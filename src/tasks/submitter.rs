use std::{
    sync::Arc,
    time::Duration,
    cmp::min,
};
use serenity::{
    prelude::*,
};
use crate::{
    data::SubmissionQueue,
    utils::{send_accepted_single, send_accepted_multiple},
};


pub async fn submit_task(ctx: Arc<Context>) {
    let ctx = Arc::clone(&ctx);
    loop {
        info!("Submit Loop Start");
        let data = ctx.data.read().await;
        let dequeue_map = data.get::<SubmissionQueue>().unwrap();
            
        for (guild_id, queue) in dequeue_map.lock().await.iter_mut() {
            let que_size = queue.len();
            match que_size {
                0 => continue,
                1 => {
                    let (account, submission) = queue.pop_front().unwrap();
                    if let Err(e) = send_accepted_single(&ctx, *guild_id, &account, &submission).await {
                        error!("Failed to send accepted notification: {:?}", e);
                    }
                },
                _ => {
                    let drain_size = min(que_size, 8);
                    let accounts_submissions = queue.drain(..drain_size).collect::<Vec<_>>();
                    if let Err(e) = send_accepted_multiple(&ctx, *guild_id, accounts_submissions).await {
                        error!("Failed to send accepted notification: {:?}", e);
                    }
                },
            }
        }
        tokio::time::sleep(Duration::from_secs(180)).await;
    }
}