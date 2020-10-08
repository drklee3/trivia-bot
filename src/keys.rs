pub use crate::model::GameState;
use serenity::{client::bridge::gateway::ShardManager, prelude::*};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct SledContainer;

impl TypeMapKey for SledContainer {
    type Value = sled::Db;
}

pub struct ReqwestContainer;

impl TypeMapKey for ReqwestContainer {
    type Value = reqwest::Client;
}
