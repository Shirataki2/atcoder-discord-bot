#![warn(clippy::all)]
mod commands;
mod tasks;
mod models;
mod database;
mod utils;
mod data;
mod event_handler;
mod error;
mod http;


#[macro_use]
extern crate log;
#[macro_use]
extern crate sqlx;

use crate::{
    utils::send_error,
    data::{DatabasePool, ShardManagerContainer},
    event_handler::Handler
};

use commands::{
    help::*,
    // general::adder::*,
    account::{register::*, subscribe::*},
    settings::start::*
};

use std::{
    env,
    collections::*,
};

use serenity::{
    prelude::*,
    http::Http,
    framework::{
        StandardFramework,
        standard::{
            macros::{group, hook},
            DispatchError,
            CommandResult
        }
    },
    model::{
        channel::Message,
    },
};

use dotenv::dotenv;

#[hook]
async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!("Get command `{}` by user {}({})", command_name, msg.author.name, msg.author.id);
    true
}

#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, error: CommandResult) {
    if let Err(why) = &error {
        error!("Error while running command {}", &command_name);
        error!("{:?}", &error);

        if send_error(ctx, msg, "Error!", format!("{}", why).as_str()).await.is_err() {
            error!(
                "Unable to send messages on channel id {}",
                &msg.channel_id.0
            );
        };
    }

}

#[hook]
#[allow(clippy::useless_format)]
async fn on_dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let description = match (min, given) {
                (1, 0) => format!("This command requires one argument"),
                (m, 0) => format!("This command requires at least {} arguments to run", m),
                (m, g) => format!("This command requires at least {} arguments, but you give {} arguments", m, g),
            };
            let _ = send_error(ctx, msg, "Not Enough Arguments!", &description).await;
        }
        DispatchError::OnlyForGuilds => {
            let description = format!("This command does not work on DM.");
            let _ = send_error(ctx, msg, "Only For Guilds!", &description).await;
        }
        DispatchError::Ratelimited(dur) => {
            let description = format!("You cannot run this command for {} seconds.", dur.as_secs());
            let _ = send_error(ctx, msg, "Rate Limited!", &description).await;
        }
        _ => {
            error!("Unhandled dispatch error: {:?}", error);
        }
    }
}

// * ---- Commands ---- * //

// #[group]
// #[commands(add)]
// struct General;

#[group]
#[commands(register, subscribe)]
struct Account;

#[group]
#[commands(start)]
struct Settings;

// * ---- Main ---- * //

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    dotenv().expect("Failed to load .env");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment (DISCORD_TOKEN)");

    let http = Http::new_with_token(&token);

    let owners = http.get_current_application_info()
        .await
        .map(|info| {
            let mut map = HashSet::new();
            map.insert(info.owner.id);
            map
        })
        .unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("^"))
        .bucket("basic", |b| b.delay(1).time_span(5).limit(5)).await
        .bucket("account", |b| b.delay(3).time_span(10).limit(2)).await
        .before(before)
        .after(after)
        .on_dispatch_error(on_dispatch_error)
        .help(&MY_HELP)
        //.group(&GENERAL_GROUP)
        .group(&SETTINGS_GROUP)
        .group(&ACCOUNT_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler::new())
        .await
        .expect("Failed to create client.");
    
    {
        let mut data = client.data.write().await;
    
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    
        let pg_pool = database::create_pgpool().await.expect("Failed to connect database");
        data.insert::<DatabasePool>(pg_pool);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to register SIGINT handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
