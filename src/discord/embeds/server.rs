use super::*;
use crate::i18n::{BotLocale, I18nKey};

pub fn status_embed(status: &DatabaseStatus, services: &[RAthenaFrServiceStatus]) -> CreateEmbed {
    status_embed_l10n(BotLocale::DEFAULT, status, services)
}

pub fn status_embed_l10n(
    locale: BotLocale,
    status: &DatabaseStatus,
    services: &[RAthenaFrServiceStatus],
) -> CreateEmbed {
    let all_services_online = services.iter().all(|service| service.online);
    let color = if all_services_online {
        COLOR_SUCCESS
    } else {
        COLOR_WARNING
    };

    base_embed(
        &t(locale, I18nKey::EmbedStatusTitle),
        t(locale, I18nKey::EmbedStatusDescription),
        color,
    )
    .field(
        t(locale, I18nKey::FieldServicesRathena),
        service_status_lines_l10n(locale, services),
        false,
    )
    .field(
        t(locale, I18nKey::FieldDatabase),
        format!("`{}`", status.database_name),
        true,
    )
    .field("MariaDB", format!("`{}`", status.database_engine), true)
    .field(
        t(locale, I18nKey::FieldOnlineCharacters),
        format!("`{}`", status.online_characters),
        true,
    )
    .field(
        t(locale, I18nKey::FieldCharacters),
        format!("`{}`", status.characters),
        true,
    )
    .field(
        t(locale, I18nKey::FieldAccounts),
        format!("`{}`", status.accounts),
        true,
    )
    .field(
        t(locale, I18nKey::FieldGuilds),
        format!("`{}`", status.guilds),
        true,
    )
}
