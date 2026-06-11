use super::localization::{localized_command, localized_option, localized_subcommand};
use crate::i18n::I18nKey;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        server_command(),
        online_command(),
        player_command(),
        guild_command(),
        castle_command(),
        item_command(),
        who_drops_command(),
        mob_command(),
        mvp_command(),
        top_command(),
        rank_command(),
        market_command(),
        createaccount_command(),
    ]
}

pub(super) fn createaccount_definition() -> CreateCommand {
    createaccount_command()
}

fn server_command() -> CreateCommand {
    localized_command("server", I18nKey::CommandServerDescription)
}

fn online_command() -> CreateCommand {
    localized_command("online", I18nKey::CommandOnlineDescription)
        .add_option(subcommand("count", "subcommand-online-count-description"))
        .add_option(
            subcommand("list", "subcommand-online-list-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("map", "subcommand-online-map-description").add_sub_option(limit_option()),
        )
}

fn player_command() -> CreateCommand {
    localized_command("player", I18nKey::CommandPlayerDescription)
        .add_option(character_name_option())
}

fn guild_command() -> CreateCommand {
    localized_command("guild", I18nKey::CommandGuildDescription)
        .add_option(
            subcommand("info", "subcommand-guild-info-description")
                .add_sub_option(guild_name_option()),
        )
        .add_option(
            subcommand("members", "subcommand-guild-members-description")
                .add_sub_option(guild_name_option())
                .add_sub_option(limit_option()),
        )
}

fn castle_command() -> CreateCommand {
    localized_command("castle", I18nKey::CommandCastleDescription)
        .add_option(
            subcommand("list", "subcommand-castle-list-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("info", "subcommand-castle-info-description")
                .add_sub_option(castle_id_option()),
        )
}

fn item_command() -> CreateCommand {
    localized_command("item", I18nKey::CommandItemDescription)
        .add_option(
            subcommand("info", "subcommand-item-info-description")
                .add_sub_option(item_query_option()),
        )
        .add_option(
            subcommand("search", "subcommand-item-search-description")
                .add_sub_option(text_option("text", "option-search-text-description"))
                .add_sub_option(limit_option()),
        )
}

fn who_drops_command() -> CreateCommand {
    localized_command("who-drops", I18nKey::CommandWhoDropsDescription)
        .add_option(item_query_option())
        .add_option(limit_option())
}

fn mob_command() -> CreateCommand {
    localized_command("mob", I18nKey::CommandMobDescription)
        .add_option(
            subcommand("info", "subcommand-mob-info-description")
                .add_sub_option(mob_query_option()),
        )
        .add_option(
            subcommand("drops", "subcommand-mob-drops-description")
                .add_sub_option(mob_query_option())
                .add_sub_option(limit_option()),
        )
}

fn mvp_command() -> CreateCommand {
    localized_command("mvp", I18nKey::CommandMvpDescription)
        .add_option(
            subcommand("list", "subcommand-mvp-list-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("last", "subcommand-mvp-last-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("top", "subcommand-mvp-top-description").add_sub_option(limit_option()),
        )
}

fn top_command() -> CreateCommand {
    localized_command("top", I18nKey::CommandTopDescription)
        .add_option(
            subcommand("level", "subcommand-top-level-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("job", "subcommand-top-job-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("guild", "subcommand-top-guild-description").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("zeny", "subcommand-top-zeny-description").add_sub_option(limit_option()),
        )
}

fn rank_command() -> CreateCommand {
    localized_command("rank", I18nKey::CommandRankDescription).add_option(character_name_option())
}

fn market_command() -> CreateCommand {
    localized_command("market", I18nKey::CommandMarketDescription)
        .add_option(
            subcommand("info", "subcommand-market-info-description")
                .add_sub_option(item_query_option()),
        )
        .add_option(
            subcommand("sell", "subcommand-market-sell-description")
                .add_sub_option(item_query_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("buy", "subcommand-market-buy-description")
                .add_sub_option(item_query_option())
                .add_sub_option(limit_option()),
        )
}

fn createaccount_command() -> CreateCommand {
    localized_command("createaccount", I18nKey::CommandCreateAccountDescription)
        .add_option(
            localized_option(
                CommandOptionType::String,
                "username",
                "option-account-username-description",
            )
            .required(true),
        )
        .add_option(
            localized_option(
                CommandOptionType::String,
                "password",
                "option-account-password-description",
            )
            .required(true),
        )
        .add_option(
            localized_option(
                CommandOptionType::String,
                "sex",
                "option-account-sex-description",
            )
            .required(true)
            .add_string_choice("Homme", "M")
            .add_string_choice("Femme", "F"),
        )
        .add_option(
            localized_option(
                CommandOptionType::String,
                "birthdate",
                "option-account-birthdate-description",
            )
            .required(true),
        )
        .add_option(
            localized_option(
                CommandOptionType::String,
                "email",
                "option-account-email-description",
            )
            .required(false),
        )
}

fn subcommand(name: &str, description_key: &str) -> CreateCommandOption {
    localized_subcommand(name, description_key)
}

fn text_option(name: &str, description_key: &str) -> CreateCommandOption {
    localized_option(CommandOptionType::String, name, description_key).required(true)
}

fn character_name_option() -> CreateCommandOption {
    text_option("name", "option-character-name-description")
}

fn guild_name_option() -> CreateCommandOption {
    text_option("name", "option-guild-name-description")
}

fn castle_id_option() -> CreateCommandOption {
    localized_option(
        CommandOptionType::Integer,
        "castle_id",
        "option-castle-id-description",
    )
    .min_int_value(0)
    .required(true)
}

fn item_query_option() -> CreateCommandOption {
    text_option("item", "option-item-query-description")
}

fn mob_query_option() -> CreateCommandOption {
    text_option("mob", "option-mob-query-description")
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
