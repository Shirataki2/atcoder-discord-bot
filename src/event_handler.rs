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
            let ctx_cloned = Arc::clone(&ctx);
            let ac_loop = tokio::spawn(async move {
                tasks::ac_fetcher::ac_fetch(ctx_cloned).await
            });
            let _ = ac_loop.await;

            let ctx_cloned = Arc::clone(&ctx);
            let submit_loop = tokio::spawn(async move {
                tasks::submitter::submit_task(ctx_cloned).await
            });
            let _ = submit_loop.await;

            let ctx_cloned = Arc::clone(&ctx);
            let updater_loop = tokio::spawn(async move {
                tasks::stat_updater::stat_updater(ctx_cloned).await
            });
            let _ = updater_loop.await;

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

    async fn guild_delete(&self, ctx: Context, incomplete: GuildUnavailable, _: Option<Guild>) {
        let guild_id = incomplete.id.0 as i64;
        info!("Guild ID: {} deleted (or kicked)", guild_id);
        let pool = {
            let data = ctx.data.read().await;
            data.get::<DatabasePool>().unwrap().clone()
        };
        if let Err(e) = models::guild::Guild::remove(&pool, guild_id).await {
            error!("Failed to remove guild: {:?}", e);
            return
        }
        info!("Guild {} removed", guild_id)
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