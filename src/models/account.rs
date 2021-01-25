use tokio_compat_02::FutureExt;
use serenity::model::channel::Message;
use crate::error::{AppError, custom_error};

type ExecuteResult = Result<sqlx::postgres::PgDone, AppError>;

#[derive(Debug)]
pub struct Account {
    pub id: i64,
    pub atcoder_id: String,
}

#[derive(Debug)]
pub struct GuildAccounts {
    pub id: i32,
    pub guild_id: i64,
    pub account_id: i64,
}

impl Account {
    pub async fn list(pool: &sqlx::PgPool) -> Result<Vec<Self>, AppError> {
        let accounts = query_as!(
            Self, "SELECT * FROM account;"
        )
        .fetch_all(pool)
        .compat()
        .await?;
        Ok(accounts)
    }

    pub async fn list_guilds(pool: &sqlx::PgPool, account_id: i64) -> Result<Vec<GuildAccounts>, AppError> {
        let accounts = query_as!(
            GuildAccounts, "SELECT * FROM guild_accounts WHERE account_id = $1;", account_id
        )
        .fetch_all(pool)
        .compat()
        .await?;
        Ok(accounts)
    }

    pub async fn get(pool: &sqlx::PgPool, id: i64) -> Result<Self, AppError> {
        let account = query_as!(
            Self, "SELECT * FROM account WHERE id = $1", id
        )
        .fetch_one(pool)
        .compat()
        .await?;
        Ok(account)
    }

    pub async fn create(pool: &sqlx::PgPool, msg: &Message, atcoder_id: &str) -> ExecuteResult {
        query!(
            "INSERT INTO account(id, atcoder_id) VALUES ($1, $2)", msg.author.id.0 as i64, atcoder_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &sqlx::PgPool, msg: &Message) -> ExecuteResult {
        query!(
            "DELETE FROM account WHERE id = $1", msg.author.id.0 as i64
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn is_subscribed(pool: &sqlx::PgPool, msg: &Message) -> Result<bool, AppError> {
        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id.0 as i64,
            None => return Err(custom_error("Guild ID is None. Called in DM!"))
        };
        let data = query_as!(
            GuildAccounts, "SELECT * FROM guild_accounts WHERE guild_id = $1 AND account_id = $2", guild_id, msg.author.id.0 as i64
        )
        .fetch_one(pool)
        .compat()
        .await;
        match data {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(err) => return Err(err.into())
        }
    }

    pub async fn subscribe(pool: &sqlx::PgPool, msg: &Message) -> ExecuteResult {
        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id.0 as i64,
            None => return Err(custom_error("Guild ID is None. Called in DM!"))
        };

        query!(
            "INSERT INTO guild_accounts(guild_id, account_id) VALUES ($1, $2)", guild_id, msg.author.id.0 as i64
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn unsubscribe(pool: &sqlx::PgPool, msg: &Message) -> ExecuteResult {
        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id.0 as i64,
            None => return Err(custom_error("Guild ID is None. Called in DM!"))
        };

        query!(
            "DELETE FROM guild_accounts WHERE guild_id = $1 AND account_id = $2", guild_id, msg.author.id.0 as i64
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &sqlx::PgPool, id: i64, new_atcoder_id: &str) -> ExecuteResult {
        query!(
            "UPDATE account SET atcoder_id = $2 WHERE id = $1", id, new_atcoder_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }
}
