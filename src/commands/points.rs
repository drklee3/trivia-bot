use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::{UserState, UserStateDb};

#[command]
pub async fn points(ctx: &Context, msg: &Message) -> CommandResult {
    let state = UserState::from_id(&ctx, msg.author.id.0).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(msg.author.tag());
                    a.icon_url(msg.author.face());

                    a
                });

                e.field(
                    "Points",
                    format!("{}", state.as_ref().map_or(0, |s| s.points)),
                    false,
                );
                e.field(
                    "Questions Answered",
                    format!("{}", state.as_ref().map_or(0, |s| s.questions)),
                    false,
                );

                e.color(0x9b59b6);

                e
            })
        })
        .await?;

    Ok(())
}
