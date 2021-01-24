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
#[individual_command_tip =
"If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[strikethrough_commands_tip_in_dm = "~~`Strikethrough commands`~~ are unavailabe because the bot is unable to run them."]
#[strikethrough_commands_tip_in_guild = "~~`Strikethrough commands`~~ are unavailabe because the bot is unable to run them."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Hide"]
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
