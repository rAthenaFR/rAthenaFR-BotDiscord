use super::*;
use crate::i18n::{BotLocale, TranslationArg};

pub fn character_quests_embed(
    character: &str,
    quests: &[CharacterQuestEntry],
    requested_limit: u32,
) -> CreateEmbed {
    character_quests_embed_l10n(BotLocale::DEFAULT, character, quests, requested_limit)
}

pub fn character_quests_embed_l10n(
    locale: BotLocale,
    character: &str,
    quests: &[CharacterQuestEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if quests.is_empty() {
        return warning_embed(
            &ts(locale, "embed-character-quests-title"),
            tsa(
                locale,
                "embed-character-quests-empty",
                &[TranslationArg::new("character", character)],
            ),
        );
    }

    let list = limited_list(quests, requested_limit, |_index, quest| {
        format!(
            "{} `{}` — {} `{}` — {} `{}` — {} `{}/{}/{}`",
            ts(locale, "field-quest"),
            quest.quest_id,
            ts(locale, "field-state"),
            quest_state_name_l10n(locale, &quest.state),
            ts(locale, "field-time"),
            quest.time,
            ts(locale, "field-counters"),
            quest.count1,
            quest.count2,
            quest.count3,
        )
    });

    info_embed(
        &ts(locale, "embed-character-quests-title"),
        tsa(
            locale,
            "embed-character-quests-description",
            &[TranslationArg::new("character", character)],
        ),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-quest-entries")),
        false,
    )
    .field(ts(locale, "field-quests"), list.value, false)
}

pub fn character_equipment_embed(
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    character_equipment_embed_l10n(BotLocale::DEFAULT, character, items, requested_limit)
}

pub fn character_equipment_embed_l10n(
    locale: BotLocale,
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if items.is_empty() {
        return warning_embed(
            &ts(locale, "embed-character-equipment-title"),
            tsa(
                locale,
                "embed-character-equipment-empty",
                &[TranslationArg::new("character", character)],
            ),
        );
    }

    let list = limited_list(items, requested_limit, |_index, item| {
        item_line_l10n(locale, item)
    });

    info_embed(
        &ts(locale, "embed-character-equipment-title"),
        tsa(
            locale,
            "embed-character-equipment-description",
            &[TranslationArg::new("character", character)],
        ),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-equipped-items")),
        false,
    )
    .field(ts(locale, "field-equipment"), list.value, false)
}

pub fn character_inventory_embed(
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    character_inventory_embed_l10n(BotLocale::DEFAULT, character, items, requested_limit)
}

pub fn character_inventory_embed_l10n(
    locale: BotLocale,
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if items.is_empty() {
        return warning_embed(
            &ts(locale, "embed-character-inventory-title"),
            tsa(
                locale,
                "embed-character-inventory-empty",
                &[TranslationArg::new("character", character)],
            ),
        );
    }

    let list = limited_list(items, requested_limit, |_index, item| {
        item_line_l10n(locale, item)
    });

    info_embed(
        &ts(locale, "embed-character-inventory-title"),
        tsa(
            locale,
            "embed-character-inventory-description",
            &[TranslationArg::new("character", character)],
        ),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-inventory-items")),
        false,
    )
    .field(ts(locale, "field-items"), list.value, false)
}

pub fn item_owners_embed(
    item_id: i64,
    owners: &[ItemOwnerEntry],
    requested_limit: u32,
) -> CreateEmbed {
    item_owners_embed_l10n(BotLocale::DEFAULT, item_id, owners, requested_limit)
}

pub fn item_owners_embed_l10n(
    locale: BotLocale,
    item_id: i64,
    owners: &[ItemOwnerEntry],
    requested_limit: u32,
) -> CreateEmbed {
    let item = item_id.to_string();
    if owners.is_empty() {
        return warning_embed(
            &ts(locale, "embed-item-owners-title"),
            tsa(
                locale,
                "embed-item-owners-empty",
                &[TranslationArg::new("item", &item)],
            ),
        );
    }

    let list = limited_list(owners, requested_limit, |_index, owner| {
        let account = owner
            .account_id
            .map(|value| format!(" — {} `{}`", ts(locale, "field-account"), value))
            .unwrap_or_default();

        format!(
            "**{}** — `{}` — {} `{}`{}",
            owner.owner_name,
            owner.source,
            ts(locale, "field-amount"),
            format_number(owner.amount),
            account,
        )
    });

    info_embed(
        &ts(locale, "embed-item-owners-title"),
        tsa(
            locale,
            "embed-item-owners-description",
            &[TranslationArg::new("item", &item)],
        ),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-item-owners")),
        false,
    )
    .field(ts(locale, "field-owners"), list.value, false)
}

pub fn ban_list_embed(entries: &[BanEntry], requested_limit: u32) -> CreateEmbed {
    ban_list_embed_l10n(BotLocale::DEFAULT, entries, requested_limit)
}

pub fn ban_list_embed_l10n(
    locale: BotLocale,
    entries: &[BanEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return success_embed(
            &ts(locale, "embed-ban-list-title"),
            ts(locale, "embed-ban-list-empty"),
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "{} `{}` — `{}` — {} {} — {} `{}` — {} `{}` — {} `{}` — {} `{}` — {} `{}`",
            ts(locale, "field-account"),
            entry.account_id,
            entry.userid,
            ts(locale, "field-state"),
            account_state_l10n(locale, entry.state),
            ts(locale, "field-group-id"),
            entry.group_id,
            ts(locale, "field-unban-end"),
            unix_time_field_l10n(locale, entry.unban_time),
            ts(locale, "field-expiration"),
            unix_time_field_l10n(locale, entry.expiration_time),
            ts(locale, "field-last-login"),
            entry
                .lastlogin
                .as_deref()
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| ts(locale, "text-never")),
            ts(locale, "field-characters"),
            entry.characters,
        )
    });

    info_embed(
        &ts(locale, "embed-ban-list-title"),
        ts(locale, "embed-ban-list-description"),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-blocked-accounts")),
        false,
    )
    .field(ts(locale, "field-accounts"), list.value, false)
}
