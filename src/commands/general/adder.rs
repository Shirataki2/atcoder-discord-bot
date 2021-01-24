use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let sum: i64 = args.iter::<i64>().map(|v| v.unwrap_or(0)).sum();
    msg.channel_id.say(&ctx.http, sum).await?;
    Ok(())
}