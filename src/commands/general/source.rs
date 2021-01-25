use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
#[description(
    "Get link of source code"
)]
#[usage("")]
pub async fn source(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "https://github.com/Shirataki2/atcoder-discord-bot").await?;
    Ok(())
}
