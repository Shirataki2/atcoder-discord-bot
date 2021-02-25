use serenity::utils::Colour;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use crate::{
    data::DatabasePool,
    models::{account::Account, user_stat::UserStat},
};


#[command]
#[description(
    "Show guild point sum ranking"
)]
#[usage("")]
pub async fn point(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap().0 as i64;
    let pool = {
        let data = ctx.data.read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let accounts = match Account::list_accounts(&pool, guild_id).await {
        Ok(accounts) => accounts,
        Err(err) => {
            error!("Failed to change channel: {:?}", err);
            return Ok(())
        }
    };
    let mut map = Vec::new();
    for account in accounts {
        let stat = match UserStat::get(&pool, &account.0).await {
            Ok(stat) => stat,
            Err(err) => {
                error!("Failed to get user: {:?}", err);
                return Ok(())
            }
        };
        map.push((stat.point_sum, account.0, account.1));
    }
    map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    map.reverse();
    if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let embed = e.title("Top 10 Point Rank").color(Colour::from_rgb(0, 255, 55));
            let top10 = map.iter().take(10).collect::<Vec<_>>();
            let mut rank = 1;
            for (v, name, id) in top10 {
                let name = format!("#{} {}", rank, name);
                let value = format!("<@{}> - {} pts", id, v);
                embed.field(name, value, false);
                rank += 1;
            }
            embed
        })
    }).await {
        error!("Failed to send message: {:?}", e);
    };
    Ok(())
}
