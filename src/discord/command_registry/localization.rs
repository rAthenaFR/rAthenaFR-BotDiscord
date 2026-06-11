use crate::i18n::{translate, BotLocale, I18nKey};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

/// Crée une commande Discord avec description française par défaut et
/// localisations Discord pour les langues prises en charge.
pub(super) fn localized_command(name: &str, description_key: I18nKey) -> CreateCommand {
    let mut command =
        CreateCommand::new(name).description(translate(BotLocale::FrFr, description_key));

    for locale in BotLocale::all() {
        if *locale == BotLocale::FrFr {
            continue;
        }

        command = command
            .description_localized(locale.discord_tag(), translate(*locale, description_key));
    }

    command
}

/// Crée une sous-commande avec description localisée depuis `locales/*.ftl`.
pub(super) fn localized_subcommand(name: &str, description_key: &str) -> CreateCommandOption {
    localized_option(CommandOptionType::SubCommand, name, description_key)
}

/// Crée une option avec description localisée depuis `locales/*.ftl`.
pub(super) fn localized_option(
    kind: CommandOptionType,
    name: &str,
    description_key: &str,
) -> CreateCommandOption {
    let mut option =
        CreateCommandOption::new(kind, name, translate_raw(BotLocale::FrFr, description_key));

    for locale in BotLocale::all() {
        if *locale == BotLocale::FrFr {
            continue;
        }

        option = option.description_localized(
            locale.discord_tag(),
            translate_raw(*locale, description_key),
        );
    }

    option
}

fn translate_raw(locale: BotLocale, key: &str) -> String {
    crate::i18n::loader::lookup(locale, key)
        .unwrap_or(key)
        .to_string()
}
