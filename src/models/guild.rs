use tokio_compat_02::FutureExt;

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

    pub async fn create(pool: &sqlx::PgPool, id: i64, channel_id: Option<i64>) -> Result<(), sqlx::Error> {
        let result = query!(
            "INSERT INTO guild(id, channel_id) VALUES ($1, $2)", id, channel_id
        )
        .fetch_one(pool)
        .compat()
        .await;
        match result {
            Ok(_) | Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(err) => Err(err)
        }
    }

    pub async fn change_channel(pool: &sqlx::PgPool, id: i64, channel_id: i64) -> Result<(), sqlx::Error> {
        let result = query!(
            "UPDATE guild SET channel_id = $1 WHERE id = $2", channel_id, id
        )
        .fetch_one(pool)
        .compat()
        .await;
        match result {
            Ok(_) | Err(sqlx::Error::RowNotFound) => Ok(()),
            Err(err) => Err(err)
        }
    }
}
