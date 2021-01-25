use serenity::{
    model::prelude::*,
    prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    }
};

use crate::{
    models::guild::Guild,
    data::DatabasePool,
    utils::send_error,
};

#[command]
#[only_in("guild")]
#[required_permissions("MANAGE_MESSAGES")]
#[description(
    "Stop sending notifications."
)]
#[usage("[channel_name]")]
#[example("")]
#[example("#general")]
#[bucket("account")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let guild_id = msg.guild_id.unwrap().0 as i64;
    match Guild::get(&pool, guild_id).await {
        Ok(_) => {
            if let Err(err) = Guild::unset_channel(&pool, guild_id).await {
                error!("Failed to change channel: {:?}", err);
                let _ = send_error(ctx, msg, "Internal Error!", REGISTRATION_ERROR).await;
                return Ok(())
            }
            let description = "Notification successfully stopped!";
            let _ = msg.reply(&ctx, description).await;
        }
        Err(err) => {
            error!("Failed to get guild: {:?}", err);
            let _ = send_error(ctx, msg, "Internal Error!", REGISTRATION_ERROR).await;
        }
    };
    Ok(())
}

const REGISTRATION_ERROR: &str = "Your registration to the database has not been completed because the internal process did not finish successfully.\
Please report the details of the error to `atcorder2021@gmail.com`.";
