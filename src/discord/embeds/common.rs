use super::*;
use crate::i18n::{BotLocale, I18nKey, TranslationArg};

pub fn command_error_embed(details: &str) -> CreateEmbed {
    command_error_embed_l10n(BotLocale::DEFAULT, details)
}

pub fn command_error_embed_l10n(locale: BotLocale, details: &str) -> CreateEmbed {
    let lower = details.to_ascii_lowercase();

    if lower.contains("doesn't exist")
        || lower.contains("does not exist")
        || lower.contains("unknown table")
    {
        return warning_embed(
            &t(locale, I18nKey::EmbedDatabaseTableMissingTitle),
            t(locale, I18nKey::EmbedDatabaseTableMissingDescription),
        );
    }

    if lower.contains("unknown column") {
        return warning_embed(
            &t(locale, I18nKey::EmbedDatabaseSchemaUnsupportedTitle),
            t(locale, I18nKey::EmbedDatabaseSchemaUnsupportedDescription),
        );
    }

    if lower.contains("access denied") || lower.contains("permission") {
        return warning_embed(
            &t(locale, I18nKey::EmbedDatabasePermissionTitle),
            t(locale, I18nKey::EmbedDatabasePermissionDescription),
        );
    }

    if lower.contains("timed out") || lower.contains("pool timed out") || lower.contains("connect")
    {
        return warning_embed(
            &t(locale, I18nKey::EmbedDatabaseConnectionTitle),
            t(locale, I18nKey::EmbedDatabaseConnectionDescription),
        );
    }

    error_embed_l10n(locale, &t(locale, I18nKey::ErrorCommandFailed))
}

pub fn staff_only_embed() -> CreateEmbed {
    staff_only_embed_l10n(BotLocale::DEFAULT)
}

pub fn staff_only_embed_l10n(locale: BotLocale) -> CreateEmbed {
    error_embed_l10n(locale, &t(locale, I18nKey::EmbedStaffOnlyDescription))
}

pub fn missing_database_table_embed(table_name: &str) -> CreateEmbed {
    missing_database_table_embed_l10n(BotLocale::DEFAULT, table_name)
}

pub fn missing_database_table_embed_l10n(locale: BotLocale, table_name: &str) -> CreateEmbed {
    warning_embed(
        &t(locale, I18nKey::EmbedMissingDatabaseTableTitle),
        ta(
            locale,
            I18nKey::EmbedMissingDatabaseTableDescription,
            &[TranslationArg::new("table", table_name)],
        ),
    )
}

pub fn text_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    info_embed(title, description)
}

pub fn text_embed_key(
    locale: BotLocale,
    title_key: I18nKey,
    description: impl Into<String>,
) -> CreateEmbed {
    info_embed(&t(locale, title_key), description)
}

pub fn success_message_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    success_embed(title, description)
}

pub fn success_message_embed_key(
    locale: BotLocale,
    title_key: I18nKey,
    description_key: I18nKey,
) -> CreateEmbed {
    success_embed(&t(locale, title_key), t(locale, description_key))
}

pub fn command_disabled_embed(command_path: &str) -> CreateEmbed {
    command_disabled_embed_l10n(BotLocale::DEFAULT, command_path)
}

pub fn command_disabled_embed_l10n(locale: BotLocale, command_path: &str) -> CreateEmbed {
    warning_embed(
        &t(locale, I18nKey::EmbedCommandDisabledTitle),
        ta(
            locale,
            I18nKey::EmbedCommandDisabledDescription,
            &[TranslationArg::new("command", command_path)],
        ),
    )
}

pub fn error_embed(message: &str) -> CreateEmbed {
    error_embed_l10n(BotLocale::DEFAULT, message)
}

pub fn error_embed_l10n(locale: BotLocale, message: &str) -> CreateEmbed {
    base_embed(&t(locale, I18nKey::EmbedErrorTitle), message, COLOR_ERROR)
}
