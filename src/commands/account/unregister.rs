use serenity::{
    model::prelude::*,
    prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    }
};

use crate::{
    models::account::Account,
    data::DatabasePool,
    utils::unknown_error,
};

#[command]
#[only_in("guild")]
#[aliases("ur")]
#[description(
    "Delete your account of this bot."
)]
#[example("")]
#[bucket("account")]
pub async fn unregister(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    if let Ok(account) = Account::get(&pool, msg.author.id.0 as i64).await {
    
        if let Err(e) = Account::delete(&pool, msg).await {
            error!("Failed to delete account: {:?}", e);
            return Err(unknown_error())
        }
        info!("Delete Account: {}", &account.atcoder_id);
        let _ = msg.reply(ctx, "Delete your registration!").await;
    
    } else {

        let _ = msg.reply(ctx, "You have not registered yet!").await;
    }
    Ok(())
}
