use super::*;
use crate::i18n::{BotLocale, I18nKey, TranslationArg};

pub fn account_characters_embed(
    account_id: i64,
    characters: &[AccountCharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    account_characters_embed_l10n(BotLocale::DEFAULT, account_id, characters, requested_limit)
}

pub fn account_characters_embed_l10n(
    locale: BotLocale,
    account_id: i64,
    characters: &[AccountCharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    let account = account_id.to_string();
    if characters.is_empty() {
        return warning_embed(
            &t(locale, I18nKey::EmbedAccountCharactersTitle),
            ta(
                locale,
                I18nKey::EmbedAccountCharactersEmpty,
                &[TranslationArg::new("account", &account)],
            ),
        );
    }

    let list = limited_list(characters, requested_limit, |_index, character| {
        let status = if character.online { "🟢" } else { "⚫" };
        let guild = character
            .guild_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| t(locale, I18nKey::TextNoGuild));

        format!(
            "Slot `{}` — {} **{}** — Niv. `{}` / Job `{}` — {} — `{}` — `{}` zeny — {}",
            character.slot,
            status,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
            character.map,
            format_number(character.zeny),
            guild,
        )
    });

    info_embed(
        &t(locale, I18nKey::EmbedAccountCharactersTitle),
        ta(
            locale,
            I18nKey::EmbedAccountCharactersDescription,
            &[TranslationArg::new("account", &account)],
        ),
    )
    .field(
        t(locale, I18nKey::FieldSummary),
        list_summary_l10n(locale, &list, &ts(locale, "noun-account-characters")),
        false,
    )
    .field(t(locale, I18nKey::FieldCharacters), list.value, false)
}

pub fn account_creation_disabled_embed() -> CreateEmbed {
    account_creation_disabled_embed_l10n(BotLocale::DEFAULT)
}

pub fn account_creation_disabled_embed_l10n(locale: BotLocale) -> CreateEmbed {
    warning_embed(
        &t(locale, I18nKey::EmbedAccountCreationDisabledTitle),
        t(locale, I18nKey::EmbedAccountCreationDisabledDescription),
    )
}

pub fn account_created_embed(account: &CreatedAccount) -> CreateEmbed {
    account_created_embed_l10n(BotLocale::DEFAULT, account)
}

pub fn account_created_embed_l10n(locale: BotLocale, account: &CreatedAccount) -> CreateEmbed {
    success_embed(
        &t(locale, I18nKey::EmbedAccountCreatedTitle),
        ta(
            locale,
            I18nKey::EmbedAccountCreatedDescription,
            &[TranslationArg::new("account", &account.userid)],
        ),
    )
    .field(
        t(locale, I18nKey::FieldAccountId),
        format!("`{}`", account.account_id),
        true,
    )
    .field(
        t(locale, I18nKey::FieldSex),
        format!("`{}`", account.sex),
        true,
    )
    .field(
        t(locale, I18nKey::FieldEmail),
        format!("`{}`", account.email),
        true,
    )
    .field(
        t(locale, I18nKey::FieldImportant),
        t(locale, I18nKey::EmbedAccountPasswordNotice),
        false,
    )
}

pub fn account_status_embed(status: &AccountStatus) -> CreateEmbed {
    account_status_embed_l10n(BotLocale::DEFAULT, status)
}

pub fn account_status_embed_l10n(locale: BotLocale, status: &AccountStatus) -> CreateEmbed {
    let account = status.account_id.to_string();
    success_embed(
        &t(locale, I18nKey::EmbedAccountTitle),
        ta(
            locale,
            I18nKey::EmbedAccountStatusDescription,
            &[TranslationArg::new("account", &account)],
        ),
    )
    .field(
        t(locale, I18nKey::FieldAccountId),
        format!("`{}`", status.account_id),
        true,
    )
    .field(
        t(locale, I18nKey::FieldLogin),
        format!("`{}`", status.userid),
        true,
    )
    .field(
        t(locale, I18nKey::FieldSex),
        format!("`{}`", status.sex),
        true,
    )
    .field(
        t(locale, I18nKey::FieldGroupId),
        format!("`{}`", status.group_id),
        true,
    )
    .field(
        t(locale, I18nKey::FieldState),
        account_state_l10n(locale, status.state),
        true,
    )
    .field(
        t(locale, I18nKey::FieldLoginCount),
        format!("`{}`", status.logincount),
        true,
    )
    .field(
        t(locale, I18nKey::FieldCharacters),
        format!(
            "`{}` / slots `{}`",
            status.characters, status.character_slots
        ),
        true,
    )
    .field(
        t(locale, I18nKey::FieldOnlineCharacters),
        format!("`{}`", status.online_characters),
        true,
    )
    .field(
        t(locale, I18nKey::FieldZeny),
        format!("`{}`", format_number(status.total_zeny)),
        true,
    )
    .field(
        t(locale, I18nKey::FieldLastLogin),
        status
            .lastlogin
            .as_deref()
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| t(locale, I18nKey::TextNever)),
        true,
    )
    .field(
        t(locale, I18nKey::FieldUnbanEnd),
        unix_time_field_l10n(locale, status.unban_time),
        true,
    )
    .field(
        t(locale, I18nKey::FieldExpiration),
        unix_time_field_l10n(locale, status.expiration_time),
        true,
    )
}
