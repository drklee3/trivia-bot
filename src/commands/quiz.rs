use jservice_rs::{JServiceRequester, model::Clue};
use serenity::collector::message_collector::MessageCollectorBuilder;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;
use strsim::normalized_levenshtein;
use tokio::stream::StreamExt;

use crate::keys::*;
use crate::model::{UserState, UserStateDb};

#[command]
pub async fn quiz(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let client = data.get::<ReqwestContainer>().unwrap();
    let game_state = data.get::<GameState>().unwrap();

    if let Some(is_playing) = game_state.channel.get(&msg.channel_id.0) {
        if *is_playing {
            let _ = msg
                .channel_id
                .say(&ctx, "You have a quiz ongoing right now.")
                .await?;
            return Ok(());
        }
    }

    let clues = match client.get_random_clues(1).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to get clue: {}", e);
            let _ = msg.channel_id.say(&ctx, "Failed to fetch clue :(").await?;

            return Ok(());
        }
    };

    let clue = match clues.first() {
        Some(c) => c,
        None => {
            tracing::error!(?msg, "Fetched clues are empty",);
            let _ = msg.channel_id.say(&ctx, "Failed to fetch clue :(").await?;

            return Ok(());
        }
    };

    tracing::info!(?clue, "Requested clue");
    msg.channel_id.say(&ctx.http, &clue.question).await?;

    // Save state
    game_state.channel.insert(msg.channel_id.0, true);

    if let Err(e) = _quiz(&ctx, &msg, &clue).await {
        tracing::error!(?msg, "Quiz error: {}", e);
    }

    game_state.channel.insert(msg.channel_id.0, false);

    Ok(())
}

pub async fn _quiz(ctx: &Context, msg: &Message, clue: &Clue) -> CommandResult {
    let mut collector = MessageCollectorBuilder::new(&ctx)
        .channel_id(msg.channel_id)
        .timeout(Duration::from_secs(15))
        .await;

    while let Some(msg) = collector.next().await {
        if msg.content.to_lowercase() == clue.answer.to_lowercase() {
            let state = UserState::inc(&ctx, msg.author.id.0).await?;

            let _ = msg.reply(ctx, format!("Correct! +1 points ({} -> {})", state.score - 1, state.score)).await;

            break;
        } else if normalized_levenshtein(&msg.content.to_lowercase(), &clue.answer.to_lowercase())
            > 0.8
        {
            // if the answer is pretty close, react
            let _ = msg.react(ctx, '🤏').await;
        } else {
            // let _ = msg.react(ctx, '❌').await;
            // Don't do anything when user responds wrong
        }
    }

    let _ = msg
        .reply(
            ctx,
            format!("Time up! (15 seconds) The answer is: {}", clue.answer),
        )
        .await;

    Ok(())
}


