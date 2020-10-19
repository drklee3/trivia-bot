use jservice_rs::{model::Clue, JServiceRequester};
use serenity::collector::message_collector::MessageCollectorBuilder;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;
use tokio::stream::StreamExt;

use crate::keys::*;
use crate::model::{UserState, UserStateDb};

#[command]
pub async fn quiz(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let client = data.get::<ReqwestContainer>().unwrap();
    let game_state = data.get::<GameState>().unwrap();

    let num_questions = args.single::<u64>().unwrap_or(3);

    if let Some(is_playing) = game_state.channel.get(&msg.channel_id.0) {
        if *is_playing {
            let _ = msg
                .channel_id
                .say(&ctx, "There is a quiz ongoing in this channel right now.")
                .await?;
            return Ok(());
        }
    }

    let clues = match client.get_random_clues(num_questions + 5).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to get clue: {}", e);
            let _ = msg.channel_id.say(&ctx, "Failed to fetch clue :(").await?;

            return Ok(());
        }
    };

    tracing::info!("Requested {} clues", clues.len());

    let clues_filtered: Vec<Clue> = clues
        .into_iter()
        .filter(|c| {
            if let Some(count) = c.invalid_count {
                count > 0
            } else {
                true
            }
        })
        .collect();

    if clues_filtered.is_empty() {
        tracing::error!(?msg, "Fetched clues are empty",);
        let _ = msg.channel_id.say(&ctx, "Failed to fetch clues :(").await?;

        return Ok(());
    };

    // Save state
    game_state.channel.insert(msg.channel_id.0, true);

    for (i, clue) in clues_filtered.iter().enumerate() {
        if let Err(e) = _quiz(&ctx, &msg, &clue, (i, num_questions as usize)).await {
            tracing::error!(?msg, "Quiz error: {}", e);
        }
    }

    game_state.channel.insert(msg.channel_id.0, false);

    Ok(())
}

pub async fn _quiz(
    ctx: &Context,
    msg: &Message,
    clue: &Clue,
    count: (usize, usize),
) -> CommandResult {
    let sent_clue_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| a.name(format!("Question #{}/{}", count.0 + 1, count.1)));

                e.title(format!(
                    "Category: {}",
                    clue.category
                        .as_ref()
                        .map_or_else(|| "No Category", |c| &c.title)
                ));

                e.description(&clue.question);

                e.field("Value", format!("{}", clue.value.unwrap_or(100)), false);

                if let Some(d) = clue.created_at {
                    e.timestamp(d.to_rfc3339());
                }

                e.footer(|f| {
                    f.text(format!(
                        "Category ID: {}, Game ID: {}",
                        clue.category_id,
                        clue.game_id
                            .map_or_else(|| "N/A".into(), |id| id.to_string())
                    ))
                });
                e.color(0x9b59b6);

                e
            })
        })
        .await?;

    tracing::info!(?clue, "Sent clue");

    let mut collector = MessageCollectorBuilder::new(&ctx)
        .channel_id(msg.channel_id)
        .timeout(Duration::from_secs(25))
        .await;

    while let Some(msg) = collector.next().await {
        let (is_match, dist) = crate::util::check_answer(&msg.content, &clue.answer);

        if is_match {
            let clue_points = clue.value.unwrap_or(100);

            let state = UserState::inc(&ctx, msg.author.id.0, clue_points).await?;

            let _ = msg
                .reply(
                    ctx,
                    format!(
                        "Correct! **+{} points** ({} → {})",
                        clue_points,
                        state.points as u64 - clue_points,
                        state.points
                    ),
                )
                .await;

            sent_clue_msg.delete(&ctx).await?;
            return Ok(());
        } else if dist > 0.8 {
            // if the answer is pretty close, react
            let _ = msg.react(ctx, '🤏').await;
        } else {
            // let _ = msg.react(ctx, '❌').await;
            // Don't do anything when user responds wrong
        }
    }

    sent_clue_msg.delete(&ctx).await?;

    let _ = msg
        .reply(
            ctx,
            format!("Time up! (15 seconds) The answer is: {}", clue.answer),
        )
        .await;

    Ok(())
}
