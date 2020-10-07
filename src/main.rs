use std::{
    collections::HashSet,
    env,
    sync::Arc,
};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::StandardFramework,
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

mod commands;
use commands::*;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct SledContainer;

impl TypeMapKey for SledContainer {
    type Value = sled::Db;
}

struct ReqwestContainer;

impl TypeMapKey for ReqwestContainer {
    type Value = reqwest::Client;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        tracing::info!("Resumed");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in the environment");
    
    let sled_path = env::var("SLED_PATH").expect("Expected SLED_PATH in environment");
    let sled = sled::open(sled_path)?;

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix("$"))
        .group(&QUIZ_GROUP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<SledContainer>(sled.clone());
        data.insert::<ReqwestContainer>(reqwest::Client::new());
    }

    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}
