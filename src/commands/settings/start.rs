use serenity::{
    model::prelude::*,
    prelude::*,
    framework::standard::{
        Args, CommandResult,
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
//#[required_permissions("MANAGE_MESSAGES")]
#[description(
    "Start sending notifications."
)]
#[max_args(1)]
#[usage("[channel_name]")]
#[example("")]
#[example("#general")]
#[bucket("account")]
pub async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel_id = args.single::<ChannelId>().unwrap_or(msg.channel_id);
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    println!("{}", channel_id);
    let guild_id = msg.guild_id.unwrap().0 as i64;
    match Guild::get(&pool, guild_id).await {
        Ok(guild) => {
            if let Err(err) = Guild::change_channel(&pool, guild_id, channel_id.0 as i64).await {
                error!("Failed to change channel: {:?}", err);
                let _ = send_error(ctx, msg, "Internal Error!", REGISTRATION_ERROR).await;
                return Ok(())
            }
            let description = match guild.channel_id {
                Some(old_channel_id) => {
                    format!("Changed the notification channel from <#{}> to <#{}>.", old_channel_id, channel_id)
                },
                None => {
                    format!("Set the notification channel to <#{}>.", channel_id)
                },
            };
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
