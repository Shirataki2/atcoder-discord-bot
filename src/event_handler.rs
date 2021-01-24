use crate::{
    data::DatabasePool,
    tasks, models
};

use std::{
    sync::Arc
};

use serenity::{
    prelude::*,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        id::GuildId,
        prelude::*
    },
    async_trait,
};

pub struct Handler {
    run_loops: Mutex<bool>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache is ready.");
        if *self.run_loops.lock().await {
            *self.run_loops.lock().await = false;
            let ctx = Arc::new(ctx);
            let ac_loop = tokio::spawn(async move {
                tasks::ac_fetcher::ac_fetch(ctx).await
            });
            let _ = ac_loop.await;
            *self.run_loops.lock().await = false;
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        info!("Guild {}({}) recieved (or created)", guild.name, guild.id);
        let pool = {
            let data = ctx.data.read().await;
            data.get::<DatabasePool>().unwrap().clone()
        };
        if let Err(e) = models::guild::Guild::get(&pool, guild.id.0 as i64).await {
            match e {
                sqlx::Error::RowNotFound => {
                    if let Err(e) = models::guild::Guild::create(&pool, guild.id.0 as i64, None).await {
                        error!("Failed to create guild: {:?}", e);
                        return
                    }
                    info!("Guild {} created", guild.name)
                },
                _ => {
                    error!("Database Error: {:?}", e);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connect as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

impl Handler {
    pub fn new() -> Self {
        Self { run_loops: Mutex::new(true) }
    }
}