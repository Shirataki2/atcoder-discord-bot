use tokio_compat_02::FutureExt;
use serde::Deserialize;
use crate::error::AppError;

type ExecuteResult = Result<sqlx::postgres::PgDone, AppError>;

#[derive(Debug, Clone)]
pub struct UserStat {
    pub atcoder_id: String,
    pub streak: i32,
    pub problem_count: i32,
    pub point_sum: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreakData {
    pub user_id: String,
    pub streak: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProblemCountData {
    pub user_id: String,
    pub problem_count: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PointSumData {
    pub user_id: String,
    pub point_sum: f64,
}

impl UserStat {
    pub async fn get(pool: &sqlx::PgPool, atcoder_id: &str) -> Result<Self, AppError> {
        let account = query_as!(
            Self, "SELECT * FROM user_stat WHERE atcoder_id = $1", atcoder_id
        )
        .fetch_one(pool)
        .compat()
        .await?;
        Ok(account)
    }

    pub async fn create(pool: &sqlx::PgPool, atcoder_id: &str) -> ExecuteResult {
        query!(
            "INSERT INTO user_stat(atcoder_id) VALUES ($1)", atcoder_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn set_streak(pool: &sqlx::PgPool, atcoder_id: &str, streak: &i32) -> ExecuteResult {
        query!(
            "UPDATE user_stat SET streak = $1 WHERE atcoder_id = $2", streak, atcoder_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn set_problem_count(pool: &sqlx::PgPool, atcoder_id: &str, problem_count: &i32) -> ExecuteResult {
        query!(
            "UPDATE user_stat SET problem_count = $1 WHERE atcoder_id = $2", problem_count, atcoder_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }

    pub async fn set_point_sum(pool: &sqlx::PgPool, atcoder_id: &str, point_sum: &f64) -> ExecuteResult {
        query!(
            "UPDATE user_stat SET point_sum = $1 WHERE atcoder_id = $2", point_sum, atcoder_id
        )
        .execute(pool)
        .compat()
        .await
        .map_err(AppError::from)
    }
}
