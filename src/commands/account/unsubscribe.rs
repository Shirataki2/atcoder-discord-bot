use serenity::{
    model::prelude::*,
    prelude::*,
    framework::standard::{
        CommandResult,
        CommandError,
        macros::command,
    }
};

use crate::{
    models::account::Account,
    data::DatabasePool,
    utils::*
};

#[command]
#[aliases("u")]
#[only_in("guild")]
#[description(
    "Disable notifications in this server."
)]
#[usage("")]
#[bucket("account")]
pub async fn unsubscribe(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    if Account::get(&pool, msg.author.id.0 as i64).await.is_ok() {
        match Account::is_subscribed(&pool, msg).await {
            Ok(false) => {
                let _ = msg.reply(&ctx, "You have not subscribed yet!").await;
            },
            Ok(true) => {
                if let Err(e) = Account::unsubscribe(&pool, msg).await {
                    error!("Database Error: {:?}", e);
                    return Err(unknown_error());    
                }
                let _ = msg.reply(&ctx, "Successfully unsubscribed!").await;
            },
            Err(e) => {
                error!("Database Error: {:?}", e);
                return Err(unknown_error());    
            }
        }
        
        Ok(())
    } else {
        Err(CommandError::from("You have not run registration yet!\n\n\
        First, please enter your AtCoder ID with the `register` command."))
    }
}
