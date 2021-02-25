use std::collections::HashMap;
use serde::Deserialize;
use crate::{error::AppError, models::{submission::RawSubmission, user_stat::{StreakData, ProblemCountData, PointSumData}}};

const API_ENDPOINT: &str = "https://kenkoooo.com/atcoder";

fn create_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder().gzip(true).build()
}

pub async fn get_user_submissions(user_name: &str) -> Result<Vec<RawSubmission>, AppError> {
    let url = format!("{}/atcoder-api/results?user={}", API_ENDPOINT, user_name);
    debug!("GET: {}", url);
    let client = create_client()?;
    let resp = client.get(&url)
        .send()
        .await?
        .json::<Vec<RawSubmission>>()
        .await?;
    Ok(resp)
}

#[derive(Debug, Deserialize)]
struct Problem {
    id: String,
    contest_id: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct Contest {
    id: String,
    start_epoch_second: i64,
    duration_second: i64,
    title: String,
    rate_change: String,
}

pub async fn get_problem_name(problem_id: &str) -> Result<Option<String>, AppError> {
    let url = format!("{}/resources/problems.json", API_ENDPOINT);
    debug!("GET: {}", url);
    let client = create_client()?;
    let resp = client.get(&url)
        .send()
        .await?
        .json::<Vec<Problem>>()
        .await?
        .iter()
        .map(|c| (c.id.clone(), c.title.clone()))
        .collect::<HashMap<String, String>>();
    Ok(resp.get(problem_id).cloned())
}

pub async fn get_contest_name(contest_id: &str) -> Result<Option<String>, AppError> {
    let url = format!("{}/resources/contests.json", API_ENDPOINT);
    debug!("GET: {}", url);
    let client = create_client()?;
    let resp = client.get(&url)
        .send()
        .await?
        .json::<Vec<Contest>>()
        .await?
        .iter()
        .map(|c| (c.id.clone(), c.title.clone()))
        .collect::<HashMap<String, String>>();
    Ok(resp.get(contest_id).cloned())
}

pub async fn get_streak() -> Result<Vec<StreakData>, AppError> {
    let url = format!("{}/resources/streaks.json", API_ENDPOINT);
    debug!("GET: {}", url);
    let client = create_client()?;
    let resp = client.get(&url)
        .send()
        .await?
        .json::<Vec<StreakData>>()
        .await?;
    Ok(resp)
}

pub async fn get_problem_count() -> Result<Vec<ProblemCountData>, AppError> {
    let url = format!("{}/resources/ac.json", API_ENDPOINT);
    debug!("GET: {}", url);
    let client = create_client()?;
    let resp = client.get(&url)
        .send()
        .await?
        .json::<Vec<ProblemCountData>>()
        .await?;
    Ok(resp)
}

pub async fn get_point_sum() -> Result<Vec<PointSumData>, AppError> {
    let url = format!("{}/resources/sums.json", API_ENDPOINT);
    debug!("GET: {}", url);
    let client = create_client()?;
    let resp = client.get(&url)
        .send()
        .await?
        .json::<Vec<PointSumData>>()
        .await?;
    Ok(resp)
}
