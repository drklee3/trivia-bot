pub use crate::model::GameState;
use serenity::{client::bridge::gateway::ShardManager, prelude::*};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct DbPool;

impl TypeMapKey for DbPool {
    type Value = sqlx::PgPool;
}

pub struct ReqwestContainer;

impl TypeMapKey for ReqwestContainer {
    type Value = reqwest::Client;
}
