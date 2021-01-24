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
#[only_in("guild")]
#[description(
    "Enable notifications in this server when you get an AC from AtCoder."
)]
#[usage("")]
#[bucket("account")]
pub async fn subscribe(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    if Account::get(&pool, msg.author.id.0 as i64).await.is_ok() {
        match Account::is_subscribed(&pool, msg).await {
            Ok(true) => {
                let _ = msg.reply(&ctx, "You are already subscribed!").await;
            },
            Ok(false) => {
                if let Err(e) = Account::subscribe(&pool, msg).await {
                    error!("Database Error: {:?}", e);
                    return Err(unknown_error());    
                }
                let _ = msg.reply(&ctx, "Successfully subscribed!").await;
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
