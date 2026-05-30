mod options;
mod public;
mod staff;

use crate::config::DiscordConfig;
use anyhow::Result;
use serenity::all::{ApplicationId, CreateCommand, GuildId, Http};

pub fn command_definitions() -> Vec<CreateCommand> {
    let mut commands = public::command_definitions();
    commands.extend(staff::command_definitions());
    commands
}

pub async fn deploy_commands(config: &DiscordConfig) -> Result<()> {
    let http = Http::new(&config.token);
    http.set_application_id(ApplicationId::new(config.application_id));

    let guild_id = GuildId::new(config.guild_id);
    guild_id.set_commands(&http, command_definitions()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn command_names_are_unique() {
        let commands = command_definitions();
        let mut names = HashSet::new();

        for command in commands {
            let name = command_name(&command);
            assert!(names.insert(name.clone()), "duplicate command name: {name}");
        }
    }

    fn command_name(command: &CreateCommand) -> String {
        let debug = format!("{command:?}");
        let (_, after_marker) = debug
            .split_once("name: \"")
            .expect("CreateCommand debug output includes name");
        let (name, _) = after_marker
            .split_once('"')
            .expect("CreateCommand debug output closes name");

        name.to_string()
    }
}
