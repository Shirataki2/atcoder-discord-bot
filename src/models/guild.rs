use tokio_compat_02::FutureExt;
use crate::error::AppError;

type ExecuteResult = Result<sqlx::postgres::PgDone, AppError>;

#[derive(Debug)]
pub struct Guild {
    pub id: i64,
    pub channel_id: Option<i64>,
}

impl Guild {
    pub async fn get(pool: &sqlx::PgPool, id: i64) -> Result<Self, sqlx::Error> {
        let guild = query_as!(
                Self, "SELECT * FROM guild WHERE id = $1", id
            )
        .fetch_one(pool)
        .compat()
        .await?;
        Ok(guild)
    }

    pub async fn create(pool: &sqlx::PgPool, id: i64, channel_id: Option<i64>) -> ExecuteResult {
        query!(
            "INSERT INTO guild(id, channel_id) VALUES ($1, $2)", id, channel_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn remove(pool: &sqlx::PgPool, id: i64) -> ExecuteResult {
        query!(
            "DELETE FROM guild WHERE id = $1", id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn change_channel(pool: &sqlx::PgPool, id: i64, channel_id: i64) -> ExecuteResult {
        query!(
            "UPDATE guild SET channel_id = $1 WHERE id = $2", channel_id, id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn unset_channel(pool: &sqlx::PgPool, id: i64) -> ExecuteResult {
        query!(
            "UPDATE guild SET channel_id = NULL WHERE id = $1", id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }
}
