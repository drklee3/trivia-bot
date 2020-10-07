use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::{command, group},
};
use std::time::Duration;
use jservice_rs::JServiceRequester;
use crate::ReqwestContainer;

#[command]
pub async fn quiz(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let client = data.get::<ReqwestContainer>().unwrap();

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

    if let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(20)).await {
        if answer.content.to_lowercase() == clue.answer {
            let _ = answer.reply(ctx, "Correct!").await;
        } else {
            let _ = answer.reply(ctx, format!("Wrong answer :( The answer is: {}", clue.answer)).await;
        }
    } else {
        let _ =  msg.reply(ctx, format!("Time is out! (20 seconds) The answer is {}", clue.answer)).await;
    };

    Ok(())
}

#[group]
#[commands(quiz)]
struct Quiz;
