use crate::models::{account::Account, submission::Submission};
use std::sync::Arc;
use std::collections::{HashMap, VecDeque};
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

pub struct SubmissionQueue;

impl TypeMapKey for SubmissionQueue {
    type Value = Arc<Mutex<HashMap<i64, VecDeque<(Account, Submission)>>>>;
}
