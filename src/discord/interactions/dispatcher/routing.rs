use super::*;

pub(super) fn command_path(command: &CommandInteraction) -> String {
    command_path_from_options(&command.data.name, &command.data.options)
}

pub(super) fn command_path_from_options(
    command_name: &str,
    options: &[CommandDataOption],
) -> String {
    command_path_from_parts(command_name, &subcommand_path(options))
}

pub(super) fn command_path_from_parts(command_name: &str, subcommands: &[&str]) -> String {
    let mut parts = Vec::with_capacity(subcommands.len() + 1);
    parts.push(command_name);
    parts.extend(subcommands.iter().copied());
    parts.join(" ")
}

pub(super) fn subcommand_path(options: &[CommandDataOption]) -> Vec<&str> {
    for option in options {
        match &option.value {
            CommandDataOptionValue::SubCommand(options)
            | CommandDataOptionValue::SubCommandGroup(options) => {
                let mut path = vec![option.name.as_str()];
                path.extend(subcommand_path(options));
                return path;
            }
            _ => {}
        }
    }

    Vec::new()
}

pub(super) fn is_public_pack_root(command_name: &str) -> bool {
    matches!(
        command_name,
        "server"
            | "online"
            | "player"
            | "guild"
            | "castle"
            | "item"
            | "who-drops"
            | "mob"
            | "mvp"
            | "top"
            | "rank"
            | "market"
    )
}

pub(super) fn is_staff_pack_root(command_name: &str) -> bool {
    matches!(
        command_name,
        "staff" | "mod" | "debug" | "audit" | "db" | "gmmsg"
    )
}

pub(super) fn subcommand_name(command: &CommandInteraction) -> Option<&str> {
    command
        .data
        .options
        .iter()
        .find_map(|option| match &option.value {
            CommandDataOptionValue::SubCommand(_) | CommandDataOptionValue::SubCommandGroup(_) => {
                Some(option.name.as_str())
            }
            _ => None,
        })
}

pub(super) fn subcommand_leaf_name(command: &CommandInteraction) -> Option<&str> {
    command
        .data
        .options
        .iter()
        .find_map(|option| match &option.value {
            CommandDataOptionValue::SubCommand(options)
            | CommandDataOptionValue::SubCommandGroup(options) => {
                Some(deepest_subcommand_name(option.name.as_str(), options))
            }
            _ => None,
        })
}

pub(super) fn deepest_subcommand_name<'a>(
    current: &'a str,
    options: &'a [CommandDataOption],
) -> &'a str {
    options
        .iter()
        .find_map(|option| match &option.value {
            CommandDataOptionValue::SubCommand(options)
            | CommandDataOptionValue::SubCommandGroup(options) => {
                Some(deepest_subcommand_name(option.name.as_str(), options))
            }
            _ => None,
        })
        .unwrap_or(current)
}

pub(super) fn option_value<'a>(
    options: &'a [CommandDataOption],
    name: &str,
) -> Option<&'a CommandDataOptionValue> {
    for option in options {
        if option.name == name {
            return Some(&option.value);
        }

        match &option.value {
            CommandDataOptionValue::SubCommand(options)
            | CommandDataOptionValue::SubCommandGroup(options) => {
                if let Some(value) = option_value(options, name) {
                    return Some(value);
                }
            }
            _ => {}
        }
    }

    None
}

pub(super) fn string_option<'a>(command: &'a CommandInteraction, name: &str) -> Option<&'a str> {
    option_value(&command.data.options, name).and_then(|value| match value {
        CommandDataOptionValue::String(value) => Some(value.as_str()),
        _ => None,
    })
}

pub(super) fn non_negative_integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    integer_option(command, name).filter(|value| *value >= 0)
}

pub(super) fn integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    option_value(&command.data.options, name).and_then(|value| match value {
        CommandDataOptionValue::Integer(value) => Some(*value),
        _ => None,
    })
}

pub(super) fn account_manage_options(command: &CommandInteraction) -> account_manage::Options<'_> {
    account_manage::Options {
        account: string_option(command, "account"),
        account_id: integer_option(command, "account_id"),
        field: string_option(command, "field"),
        value: string_option(command, "value"),
        confirm: string_option(command, "confirm"),
        reason: string_option(command, "reason"),
        until: non_negative_integer_option(command, "until"),
    }
}
