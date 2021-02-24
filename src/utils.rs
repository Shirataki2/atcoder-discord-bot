use serenity::{
    prelude::Context, model::channel::Message,
    utils::Colour,
    framework::standard::CommandError
};
use std::collections::VecDeque;
use chrono::prelude::*;
use crate::{
    data::{DatabasePool, SubmissionQueue},
    http::{get_problem_name, get_contest_name},
    models::{account::Account, submission::Submission, guild::Guild},
    error::AppError
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
#[allow(dead_code)]
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

pub async fn insert_submission(ctx: &Context, account: &Account, submission: &Submission) -> Result<(), AppError> {
    let data = ctx.data.read().await;
    let dequeue_map = data.get::<SubmissionQueue>().unwrap();

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

    info!("Recieve new Accepted Submission!: {:?}", submission);
    for guild in guilds.iter() {
        let guild_id = guild.guild_id;
        dequeue_map
            .lock()
            .await
            .entry(guild_id)
            .or_insert(VecDeque::new())
            .push_back((account.clone(), submission.clone()));
    }
    
    Ok(())
}

#[allow(clippy::eval_order_dependence)]
pub async fn send_accepted_single(ctx: &Context, guild_id: i64, account: &Account, submission: &Submission) -> Result<(), serenity::Error> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };

    let channel = match Guild::get(&pool, guild_id).await {
        Ok(data) => match data.channel_id {
            Some(channel_id) => channel_id as u64,
            None => {
                info!("Unsubscribe guild: {}", guild_id);
                return Ok(())
            }
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

    Ok(())
}


#[allow(clippy::eval_order_dependence)]
pub async fn send_accepted_multiple(ctx: &Context, guild_id: i64, accounts_submissions: Vec<(Account, Submission)>) -> Result<(), serenity::Error> {
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };

    let channel = match Guild::get(&pool, guild_id).await {
        Ok(data) => match data.channel_id {
            Some(channel_id) => channel_id as u64,
            None => {
                info!("Unsubscribe guild: {}", guild_id);
                return Ok(())
            }
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
    info!("Send new Accepted Submission!: {:?}", accounts_submissions);

    let mut problem_names = vec![];
    for (_account, submission) in accounts_submissions.iter() {
        let description = match (get_contest_name(&submission.contest_id).await, get_problem_name(&submission.problem_id).await) {
            (Ok(Some(contest)), Ok(Some(problem))) => format!("{} - {}", contest, problem),
            _ => format!("Unknown Problem")
        };
        problem_names.push(description);
    }

    if let Err(e) = channel.id().send_message(ctx, |m| {
        m
            .embed(|e| {
                let embed = e
                    .title("Accepted!!")
                    .colour(Colour::from_rgb(0, 255, 55));
                for ((account, submission), problem) in accounts_submissions.iter().zip(problem_names.iter()) {
                    let author = format!("<@{}>", account.id);
                    let point = format!("{} pts.", submission.point);
                    let execution_time = format!("{} ms", submission.execution_time);
                    let lang = format!("{}", submission.language);
                    let url = format!("[View](https://atcoder.jp/contests/{}/submissions/{})", submission.contest_id, submission.id);
                    let value = vec![author, point, execution_time, lang, url].join(" | ");
                    embed.field(problem, value, false);
                }
                embed
            })
    }).await {
        error!("Failed to send message: {:?}", e);
    }

    Ok(())
}


pub fn unknown_error() -> CommandError {
    CommandError::from("Unknown error has occurred.\n\
        If you get this error repeatedly, please contact `admin@sample.com`.")
}
