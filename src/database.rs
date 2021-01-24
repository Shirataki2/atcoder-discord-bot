use tokio_compat_02::FutureExt;

use std::env;
use sqlx::{PgPool, postgres::PgPoolOptions};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn create_pgpool() -> Result<PgPool, Error> {
    let pg_url = env::var("DATABASE_URL")
        .expect("Expected a database url in the environment (DATABASE_URL)");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&pg_url)
        .compat()
        .await
        .unwrap();
    Ok(pool)
}