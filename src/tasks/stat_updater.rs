use std::{
    sync::Arc,
    time::Duration,
    collections::HashMap,
};
use serenity::{
    prelude::*,
};
use crate::{data::DatabasePool, http::{get_streak, get_problem_count, get_point_sum}, models::{user_stat::UserStat, account::Account}};

pub async fn stat_updater(ctx: Arc<Context>) {
    let ctx = Arc::clone(&ctx);
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    tokio::spawn(async move {
        loop {
            debug!("Stat Loop Start");
            let streaks = match get_streak().await {
                Ok(s) => s.iter().map(|v| (v.user_id.clone(), v.streak)).collect::<HashMap<_, _>>(),
                Err(e) => {
                    error!("Failed to get streaks: {:?}", e);
                    continue
                }
            };
            let problem_counts = match get_problem_count().await {
                Ok(s) => s.iter().map(|v| (v.user_id.clone(), v.problem_count)).collect::<HashMap<_, _>>(),
                Err(e) => {
                    error!("Failed to get problem counts: {:?}", e);
                    continue
                }
            };
            let point_sums = match get_point_sum().await {
                Ok(s) => s.iter().map(|v| (v.user_id.clone(), v.point_sum)).collect::<HashMap<_, _>>(),
                Err(e) => {
                    error!("Failed to get point sum: {:?}", e);
                    continue
                }
            };
            let accounts = match Account::list(&pool).await {
                Ok(a) => a,
                Err(e) => {
                    error!("Failed to get accounts: {:?}", e);
                    continue
                }
            };
            for account in accounts.iter() {
                let id = account.atcoder_id.clone();
                let _ = UserStat::create(&pool, &id).await;
                let streak = streaks.get(&id);
                let problem_count = problem_counts.get(&id);
                let point_sum = point_sums.get(&id);
                if let (Some(streak), Some(problem_count), Some(point_sum)) = (streak, problem_count, point_sum) {
                    let _ = UserStat::set_streak(&pool, &id, streak).await;
                    let _ = UserStat::set_problem_count(&pool, &id, problem_count).await;
                    let _ = UserStat::set_point_sum(&pool, &id, point_sum).await;
                }
            }
            debug!("Stat Loop Finished");
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}