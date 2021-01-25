use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
#[description(
    "Get invitation link"
)]
#[usage("")]
pub async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    let link = std::env::var("INVITATION_URL").unwrap_or_else(|_| "http://discordapp.com".to_string());
    msg.channel_id.say(&ctx.http, link).await?;
    Ok(())
}
