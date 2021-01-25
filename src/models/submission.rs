use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::error::AppError;
use tokio_compat_02::FutureExt;

#[derive(Debug)]
pub struct Submission {
    pub id: i32,
    pub epoch_second: NaiveDateTime,
    pub problem_id: String,
    pub contest_id: String,
    pub result: String,
    pub atcoder_id: String,
    pub language: String,
    pub point: i32,
    pub length: i32,
    pub execution_time: i32,
    pub account_id: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawSubmission {
    pub id: i32,
    epoch_second: i64,
    problem_id: String,
    contest_id: String,
    pub result: String,
    user_id: String,
    language: String,
    point: f64,
    length: i32,
    execution_time: Option<i32>,
}

impl From<RawSubmission> for Submission {
    fn from(raw: RawSubmission) -> Submission {
        Submission {
            id: raw.id,
            epoch_second: NaiveDateTime::from_timestamp(raw.epoch_second, 0),
            problem_id: raw.problem_id,
            contest_id: raw.contest_id,
            result: raw.result,
            atcoder_id: raw.user_id,
            language: raw.language,
            point: raw.point as i32,
            length: raw.length,
            execution_time: raw.execution_time.unwrap_or(0),
            account_id: 0,
        }
    }
}


impl Submission {
    pub async fn get(pool: &sqlx::PgPool, account_id: i64) -> Result<Vec<Submission>, AppError> {
        let submissions = query_as!(
            Submission,
            "SELECT * FROM submission WHERE account_id = $1;",
            account_id
        )
        .fetch_all(pool)
        .compat()
        .await?;
        Ok(submissions)
    }

    pub async fn bulk_insert(pool: &sqlx::PgPool, account_id: i64, submissions: &[RawSubmission]) -> Result<(), AppError> {
        for submission in submissions.iter().cloned() {
            let submission = Submission::from(submission);
            if let Err(err) = query!(
                "INSERT INTO submission 
                (id, epoch_second, problem_id, contest_id, result, atcoder_id, language, point, length, execution_time, account_id)
                VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT DO NOTHING",
                submission.id,
                submission.epoch_second,
                submission.problem_id,
                submission.contest_id,
                submission.result,
                submission.atcoder_id,
                submission.language,
                submission.point,
                submission.length,
                submission.execution_time,
                account_id
            )
            .execute(pool)
            .compat()
            .await {
                error!("Failed to insert submission: {:?}", err);
            }
        }
        Ok(())
    }
}
