use serenity::{framework::StandardFramework, http::Http, prelude::*};
use sqlx::SqlitePool;
use std::{collections::HashSet, env};
use tokio::signal::unix::{signal, SignalKind};

mod commands;
mod error;
mod handler;
mod keys;
mod model;
mod util;

use commands::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let sqlite_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment");

    let pool = SqlitePool::connect(&sqlite_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("$"))
        .group(&QUIZ_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<keys::ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<keys::DbPool>(pool.clone());
        data.insert::<keys::ReqwestContainer>(reqwest::Client::new());
        data.insert::<model::GameState>(model::GameState::new());
    }

    let signal_kinds = vec![
        SignalKind::hangup(),
        SignalKind::interrupt(),
        SignalKind::terminate(),
    ];

    for signal_kind in signal_kinds {
        let mut stream = signal(signal_kind).unwrap();
        let shard_manager = client.shard_manager.clone();

        tokio::spawn(async move {
            stream.recv().await;
            tracing::info!("Signal received, shutting down...");
            shard_manager.lock().await.shutdown_all().await;

            tracing::info!("bye");
        });
    }

    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}
