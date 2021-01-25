use serenity::{
    model::prelude::*,
    prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    }
};

use crate::{
    models::{account::Account, submission::Submission},
    data::DatabasePool,
    http::get_user_submissions,
    utils::unknown_error,
};

#[command]
#[only_in("guild")]
#[aliases("r")]
#[description(
    "Link your Discord user to AtCoder's user data. To receive notifications, type the command `^subscribe`."
)]
#[num_args(1)]
#[usage("<atcoder_user_name>")]
#[example("tourist")]
#[example("chokudai")]
#[bucket("account")]
pub async fn register(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let atcoder_id = args.single::<String>()?;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    if let Ok(account) = Account::get(&pool, msg.author.id.0 as i64).await {
    
        // 既にデータベースに登録済みの場合はIDを更新する
        if let Err(e) = Account::update(&pool, msg.author.id.0 as i64, &atcoder_id).await {
            error!("Failed to update account: {:?}", e);
            return Err(unknown_error())
        }
        info!("Update Account: {} to {}", &account.atcoder_id, &atcoder_id);
        let _ = msg.reply(ctx, format!("Update your AtCoder ID: **{}** to **{}**", account.atcoder_id, atcoder_id)).await;
    
    } else {

        if let Err(e) = Account::create(&pool, msg, &atcoder_id).await {
            error!("Failed to create account: {:?}", e);
            return Err(unknown_error())
        }
        info!("Create account: {}", &atcoder_id);
        let _ = msg.reply(ctx, format!("Registered your AtCoder ID: **{}**", atcoder_id)).await;
    
    }
    let account_id = msg.author.id.0 as i64;
    match get_user_submissions(&atcoder_id).await {
        Ok(submissions) => {
            match Submission::bulk_insert(&pool, account_id, &submissions).await {
                Ok(_) => info!("Submisstions were successfully updated!"),
                Err(e) => error!("Failed to update submissions: {:?}", e),
            }
        }
        Err(e) => {
            error!("Failed to update user submissions: {:?}", e);
        }
    }
    Ok(())
}
