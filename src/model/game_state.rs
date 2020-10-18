use dashmap::DashMap;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct GameState {
    pub channel: Arc<DashMap<u64, bool>>,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TypeMapKey for GameState {
    type Value = GameState;
}
