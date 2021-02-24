use std::{
    sync::Arc,
    time::Duration,
};
use serenity::{
    prelude::*,
    // model::id::ChannelId
};
use crate::{
    models::{account::Account, submission::Submission},
    data::DatabasePool,
    http::get_user_submissions,
    utils::insert_submission,
};

pub async fn ac_fetch(ctx: Arc<Context>) {
    let ctx = Arc::clone(&ctx);
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    tokio::spawn(async move {
        loop {
            debug!("Fetching loop Started");
            // 1. 登録ユーザーの取得
            let accounts = match Account::list(&pool).await {
                Ok(accounts) => {accounts}
                Err(e) => {
                    error!("Failed to fetch accounts: {:?}", e);
                    continue;
                }
            };
            for account in accounts {
                // 2. 新規ACの確認
                let old_submissions = match Submission::get(&pool, account.id).await {
                    Ok(submissions) => {submissions}
                    Err(e) => {
                        error!("Failed to fetch old submissions: {:?}", e);
                        continue;
                    }
                };
                let new_submissions = match get_user_submissions(&account.atcoder_id).await {
                    Ok(submissions) => {submissions}
                    Err(e) => {
                        error!("Failed to fetch submissions: {:?}", e);
                        continue;
                    }
                };
                if let Err(e) = Submission::bulk_insert(&pool, account.id, &new_submissions).await {
                    error!("Failed to update submissions: {:?}", e);
                }
                let mut old_submission_ids = std::collections::HashSet::new();
                for submission in old_submissions { old_submission_ids.insert(submission.id); }

                for submission in new_submissions.iter().cloned() {
                    if submission.result != "AC" {
                        continue;
                    }
                    if !old_submission_ids.contains(&submission.id) {
                        // 3. キューに押し込む
                        let _ = insert_submission(&ctx, &account, &Submission::from(submission)).await;
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

        }
    });
}
