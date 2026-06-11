use super::*;
use crate::i18n::{BotLocale, I18nKey, TranslationArg};

pub fn player_embed(profile: &PlayerProfile) -> CreateEmbed {
    player_embed_l10n(BotLocale::DEFAULT, profile)
}

pub fn player_embed_l10n(locale: BotLocale, profile: &PlayerProfile) -> CreateEmbed {
    let status = localized_status_icon(locale, profile.online);
    let player = profile.name.as_str();

    success_embed(
        &ta(
            locale,
            I18nKey::EmbedPlayerTitle,
            &[TranslationArg::new("player", player)],
        ),
        ta(
            locale,
            I18nKey::EmbedPlayerDescription,
            &[TranslationArg::new("player", player)],
        ),
    )
    .field(t(locale, I18nKey::FieldStatus), status, true)
    .field(
        t(locale, I18nKey::FieldClass),
        job_name(profile.class_id),
        true,
    )
    .field(
        t(locale, I18nKey::FieldLevels),
        format!(
            "Base `{}` / Job `{}`",
            profile.base_level, profile.job_level
        ),
        true,
    )
    .field(
        t(locale, I18nKey::FieldMap),
        format!("`{}`", profile.map),
        true,
    )
    .field(
        t(locale, I18nKey::FieldGuild),
        profile
            .guild_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| t(locale, I18nKey::TextNone)),
        true,
    )
    .field(
        t(locale, I18nKey::FieldZeny),
        format!("`{}`", format_number(profile.zeny)),
        true,
    )
}

pub fn player_not_found_embed(name: &str) -> CreateEmbed {
    player_not_found_embed_l10n(BotLocale::DEFAULT, name)
}

pub fn player_not_found_embed_l10n(locale: BotLocale, name: &str) -> CreateEmbed {
    warning_embed(
        &t(locale, I18nKey::EmbedPlayerNotFoundTitle),
        ta(
            locale,
            I18nKey::EmbedPlayerNotFoundDescription,
            &[TranslationArg::new("player", name)],
        ),
    )
}
