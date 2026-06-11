use super::localization::{localized_command, localized_option, localized_subcommand};
use crate::i18n::I18nKey;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        staff_command(),
        mod_command(),
        debug_command(),
        audit_command(),
        db_command(),
        gmmsg_command(),
    ]
}

fn staff_command() -> CreateCommand {
    localized_command("staff", I18nKey::CommandStaffDescription)
        .add_option(
            subcommand("player", "subcommand-staff-player-description")
                .add_sub_option(character_option()),
        )
        .add_option(
            subcommand("account", "subcommand-staff-account-description")
                .add_sub_option(character_option()),
        )
        .add_option(
            subcommand("chars", "subcommand-staff-chars-description")
                .add_sub_option(lookup_option()),
        )
        .add_option(
            subcommand("inventory", "subcommand-staff-inventory-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("equipment", "subcommand-staff-equipment-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("cart", "subcommand-staff-cart-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("storage", "subcommand-staff-storage-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("guildstorage", "subcommand-staff-guildstorage-description")
                .add_sub_option(guild_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("whohas", "subcommand-staff-whohas-description")
                .add_sub_option(item_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("item-search", "subcommand-staff-item-search-description")
                .add_sub_option(item_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("zeny", "subcommand-staff-zeny-description")
                .add_sub_option(character_option()),
        )
        .add_option(
            subcommand("zenylog", "subcommand-staff-zenylog-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("picklog", "subcommand-staff-picklog-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("trade-log", "subcommand-staff-trade-log-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("mvp-log", "subcommand-staff-mvp-log-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("loginlog", "subcommand-staff-loginlog-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("ip-accounts", "subcommand-staff-ip-accounts-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("multiaccount", "subcommand-staff-multiaccount-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("banned", "subcommand-staff-banned-description")
                .add_sub_option(limit_option()),
        )
        .add_option(account_manage_group())
}

fn mod_command() -> CreateCommand {
    localized_command("mod", I18nKey::CommandModDescription)
        .add_option(
            subcommand("chatlog", "subcommand-mod-chatlog-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("chat-search", "subcommand-mod-chat-search-description")
                .add_sub_option(text_option("text", "option-search-text-description"))
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand(
                "report-context",
                "subcommand-mod-report-context-description",
            )
            .add_sub_option(character_option())
            .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("branchlog", "subcommand-mod-branchlog-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
}

fn debug_command() -> CreateCommand {
    localized_command("debug", I18nKey::CommandDebugDescription)
        .add_option(
            subcommand("quest", "subcommand-debug-quest-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("char-vars", "subcommand-debug-char-vars-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("acc-vars", "subcommand-debug-acc-vars-description")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
}

fn audit_command() -> CreateCommand {
    localized_command("audit", I18nKey::CommandAuditDescription)
        .add_option(
            subcommand("atcommands", "subcommand-audit-atcommands-description")
                .add_sub_option(text_option("gm", "option-gm-name-description"))
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("item-created", "subcommand-audit-item-created-description")
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("zeny-created", "subcommand-audit-zeny-created-description")
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("gm-activity", "subcommand-audit-gm-activity-description")
                .add_sub_option(text_option("gm", "option-gm-name-description"))
                .add_sub_option(limit_option()),
        )
}

fn db_command() -> CreateCommand {
    localized_command("db", I18nKey::CommandDbDescription)
        .add_option(subcommand("health", "subcommand-db-health-description"))
        .add_option(
            subcommand("tables", "subcommand-db-tables-description").add_sub_option(limit_option()),
        )
        .add_option(subcommand("count", "subcommand-db-count-description"))
        .add_option(subcommand(
            "logs-size",
            "subcommand-db-logs-size-description",
        ))
        .add_option(
            subcommand("last-update", "subcommand-db-last-update-description")
                .add_sub_option(limit_option()),
        )
}

fn gmmsg_command() -> CreateCommand {
    localized_command("gmmsg", I18nKey::CommandGmmsgDescription)
        .add_option(
            subcommand("server", "subcommand-gmmsg-server-description")
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("map", "subcommand-gmmsg-map-description")
                .add_sub_option(text_option("map", "option-map-name-description"))
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("blue", "subcommand-gmmsg-blue-description")
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("color", "subcommand-gmmsg-color-description")
                .add_sub_option(text_option("hex", "option-hex-color-description"))
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("test", "subcommand-gmmsg-test-description")
                .add_sub_option(message_option()),
        )
}

fn account_manage_group() -> CreateCommandOption {
    localized_option(
        CommandOptionType::SubCommandGroup,
        "account-manage",
        "subcommand-group-account-manage-description",
    )
    .add_sub_option(
        subcommand("edit", "subcommand-account-manage-edit-description")
            .add_sub_option(account_lookup_option())
            .add_sub_option(account_field_option())
            .add_sub_option(text_option("value", "option-account-value-description"))
            .add_sub_option(optional_text_option(
                "reason",
                "option-staff-reason-description",
            )),
    )
    .add_sub_option(
        subcommand("ban", "subcommand-account-manage-ban-description")
            .add_sub_option(account_lookup_option())
            .add_sub_option(optional_integer_option(
                "until",
                "option-ban-until-description",
                0,
            ))
            .add_sub_option(optional_text_option(
                "reason",
                "option-staff-reason-description",
            )),
    )
    .add_sub_option(
        subcommand("unban", "subcommand-account-manage-unban-description")
            .add_sub_option(account_lookup_option())
            .add_sub_option(optional_text_option(
                "reason",
                "option-staff-reason-description",
            )),
    )
    .add_sub_option(
        subcommand("delete", "subcommand-account-manage-delete-description")
            .add_sub_option(
                localized_option(
                    CommandOptionType::Integer,
                    "account_id",
                    "option-account-id-description",
                )
                .min_int_value(1)
                .required(true),
            )
            .add_sub_option(text_option("confirm", "option-confirm-delete-description"))
            .add_sub_option(optional_text_option(
                "reason",
                "option-staff-reason-description",
            )),
    )
}

fn subcommand(name: &str, description_key: &str) -> CreateCommandOption {
    localized_subcommand(name, description_key)
}

fn text_option(name: &str, description_key: &str) -> CreateCommandOption {
    localized_option(CommandOptionType::String, name, description_key).required(true)
}

fn optional_text_option(name: &str, description_key: &str) -> CreateCommandOption {
    localized_option(CommandOptionType::String, name, description_key).required(false)
}

fn optional_integer_option(name: &str, description_key: &str, min: u64) -> CreateCommandOption {
    localized_option(CommandOptionType::Integer, name, description_key)
        .min_int_value(min)
        .required(false)
}

fn account_lookup_option() -> CreateCommandOption {
    text_option("account", "option-account-lookup-description")
}

fn account_field_option() -> CreateCommandOption {
    localized_option(
        CommandOptionType::String,
        "field",
        "option-account-field-description",
    )
    .required(true)
    .add_string_choice("group_id", "group_id")
    .add_string_choice("state", "state")
    .add_string_choice("unban_time", "unban_time")
    .add_string_choice("expiration_time", "expiration_time")
    .add_string_choice("logincount", "logincount")
    .add_string_choice("sex", "sex")
}

fn character_option() -> CreateCommandOption {
    text_option("character", "option-character-name-description")
}

fn lookup_option() -> CreateCommandOption {
    text_option("lookup", "option-lookup-description")
}

fn guild_option() -> CreateCommandOption {
    text_option("guild", "option-guild-lookup-description")
}

fn item_option() -> CreateCommandOption {
    text_option("item", "option-item-query-description")
}

fn message_option() -> CreateCommandOption {
    text_option("message", "option-message-description")
}

fn limit_option() -> CreateCommandOption {
    localized_option(
        CommandOptionType::Integer,
        "limit",
        "option-limit-description",
    )
    .min_int_value(1)
    .required(false)
}
