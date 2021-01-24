use std::sync::Arc;
use serenity::{
    prelude::{TypeMapKey, Mutex},
    client::bridge::gateway::ShardManager,
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = sqlx::PgPool;
}
