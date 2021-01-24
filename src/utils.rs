use serenity::{
    prelude::Context, model::channel::Message,
    utils::Colour,
    framework::standard::CommandError
};
use chrono::prelude::*;
use crate::{
    data::DatabasePool,
    http::{get_problem_name, get_contest_name},
    models::{account::Account, submission::Submission, guild::Guild}
};

pub async fn send_error(ctx: &Context, msg: &Message, title: &str, description: &str) -> Result<Message, serenity::Error> {
    msg.channel_id.send_message(ctx, |m| {
        m
        .reference_message(msg)
        .allowed_mentions(|f| f.replied_user(false))
        .embed(|e| {
            e
            .title(title)
            .description(description)
            .colour(Colour::from_rgb(255, 50, 50))
            .footer(|f| f.text("To show usage, type \"^help <command>\""))
            .timestamp(&Utc::now())
        })
    }).await
}

#[allow(clippy::eval_order_dependence)]
pub async fn send_accepted_notification(ctx: &Context, account: &Account, submission: &Submission) -> Result<(), serenity::Error> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let guilds = match Account::list_guilds(&pool, account.id).await {
        Ok(guilds) => guilds,
        Err(e) => {
            error!("Failed to get guilds: {:?}", e);
            return Ok(());
        }
    };

    for guild in guilds.iter() {
        let channel = match Guild::get(&pool, guild.guild_id).await {
            Ok(data) => match data.channel_id {
                Some(channel_id) => channel_id as u64,
                None => continue
            },
            Err(e) => {
                error!("Failed to get channel: {:?}", e);
                return Ok(());
            }
        };
        let channel = match ctx.http.get_channel(channel).await {
            Ok(channel) => channel,
            Err(e) => {
                error!("Failed to get channel: {:?}", e);
                return Ok(());
            }
        };
        let avatar = match ctx.http.get_user(account.id as u64).await {
            Ok(user) => user
                .avatar
                .map(|a| format!("https://cdn.discordapp.com/avatars/{}/{}.png", account.id, a))
                .unwrap_or_else(|| "https://img.atcoder.jp/assets/atcoder.png".to_string()),
            Err(_) => "https://img.atcoder.jp/assets/atcoder.png".to_string(),
        };
        info!("Avatar URL: {}", avatar);
        info!("Send new Accepted Submission!: {:?}", submission);
        let description = match (get_contest_name(&submission.contest_id).await, get_problem_name(&submission.problem_id).await) {
            (Ok(Some(contest)), Ok(Some(problem))) => format!("<@{}> get new AC!\n\n Problem:\n**{} - {}**", account.id, contest, problem),
            _ => format!("<@{}> get new AC!", account.id)
        };
        if let Err(e) = channel.id().send_message(ctx, |m| {
            m
            .embed(|e| {
                e
                .title("Accepted!!")
                .description(description)
                .field("Point", format!("{}", submission.point), true)
                .field("Lang", &submission.language, true)
                .field("Execution Time", format!("{} ms", submission.execution_time), true)
                .field("Code Length", format!("{} Bytes", submission.length), true)
                .field("Link", format!("[View](https://atcoder.jp/contests/{}/submissions/{})", submission.contest_id, submission.id), true)
                .colour(Colour::from_rgb(0, 255, 55))
                .footer(|f| f.text("Submission Time"))
                .thumbnail(&avatar)
                .timestamp(&Utc.from_local_datetime(&submission.epoch_second).unwrap())
            })
        }).await {
            error!("Failed to send notification: {:?}", e);
        };
    }

    Ok(())
}


pub fn unknown_error() -> CommandError {
    CommandError::from("Unknown error has occurred.\n\
        If you get this error repeatedly, please contact `admin@sample.com`.")
}
