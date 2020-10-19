use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::{UserState, UserStateDb};

#[command]
pub async fn points(ctx: &Context, msg: &Message) -> CommandResult {
    let state = UserState::from_id(&ctx, msg.author.id.0).await?;

    if let Some(s) = state {
        msg.reply(ctx, format!("Current points: {}", s.points))
            .await?;
    } else {
        msg.channel_id.say(ctx, "You have no points :(").await?;
    }

    Ok(())
}
