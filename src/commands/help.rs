use std::collections::HashSet;

use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands,
        macros::help,
    },
    model::{
        channel::Message,
        id::UserId,
    },
    utils::Colour
};


#[help]
#[individual_command_tip(r"If you want more information about a specific command, just pass the command as argument.

This bot will send your AtCoder submissions to the Discord channel.

日本語のヘルプは[GitHubのREADME](https://github.com/Shirataki2/atcoder-discord-bot/blob/main/README.md)をご参照ください．

**Usage**

__1. Install this Bot __

You can install it [here](https://discord.com/api/oauth2/authorize?client_id=801783771526856704&permissions=126016&scope=bot).

__2. Run `^start` command on the channel you want to receive notifications from __

To prevent abuse, only users with `message management` permission can execute this command.

__3. Run `^register <your_atcoder_id>` to link your AtCoder ID to your Discord user data__

At this step, no AC will be sent.

__4. Run `^subscribe` to receive AC information__

AtCoder Problems API is used internally, and notifications are sent 2-5 minutes later, depending on how often the API is updated.
")]
#[command_not_found_text = "Could not find: `{}`."]
#[strikethrough_commands_tip_in_dm = "~~`Strikethrough commands`~~ are unavailabe because the bot is unable to run them."]
#[strikethrough_commands_tip_in_guild = "~~`Strikethrough commands`~~ are unavailabe because the bot is unable to run them."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Strike"]
#[wrong_channel = "Strike"]
pub async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let mut ho = help_options.clone();
    ho.embed_error_colour = Colour::from_rgb(255, 30, 30);
    ho.embed_success_colour = Colour::from_rgb(141, 91, 255);
    let _ = help_commands::with_embeds(context, msg, args, &ho, groups, owners).await;
    Ok(())
}
