use super::*;
use crate::i18n::{BotLocale, TranslationArg};

pub fn castles_embed(castles: &[CastleSummary], requested_limit: u32) -> CreateEmbed {
    castles_embed_l10n(BotLocale::DEFAULT, castles, requested_limit)
}

pub fn castles_embed_l10n(
    locale: BotLocale,
    castles: &[CastleSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if castles.is_empty() {
        return warning_embed(
            &ts(locale, "embed-castles-title"),
            ts(locale, "embed-castles-empty"),
        );
    }

    let list = limited_list(castles, requested_limit, |_index, castle| {
        let fallback_owner = ts(locale, "text-no-owner");
        let owner = castle
            .owner_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .unwrap_or(&fallback_owner);

        format!(
            "{} `{}` — {} **{}** — {} `{}` — {} `{}` — {} `{}`",
            ts(locale, "field-castle"),
            castle.castle_id,
            ts(locale, "field-owner"),
            owner,
            ts(locale, "field-economy"),
            castle.economy,
            ts(locale, "field-defense"),
            castle.defense,
            ts(locale, "field-visible"),
            castle.visible_c,
        )
    });

    info_embed(
        &ts(locale, "embed-castles-title"),
        ts(locale, "embed-castles-description"),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-castles")),
        false,
    )
    .field(ts(locale, "field-castles"), list.value, false)
}

pub fn castle_detail_embed(castle: &CastleDetails) -> CreateEmbed {
    castle_detail_embed_l10n(BotLocale::DEFAULT, castle)
}

pub fn castle_detail_embed_l10n(locale: BotLocale, castle: &CastleDetails) -> CreateEmbed {
    let fallback_owner = ts(locale, "text-no-owner");
    let owner = castle
        .owner_name
        .as_deref()
        .filter(|name| !name.is_empty())
        .unwrap_or(&fallback_owner);
    let castle_id = castle.castle_id.to_string();

    success_embed(
        &ts(locale, "embed-castle-profile-title"),
        tsa(
            locale,
            "embed-castle-profile-description",
            &[TranslationArg::new("castle", &castle_id)],
        ),
    )
    .field(ts(locale, "field-owner"), owner, true)
    .field(
        ts(locale, "field-owner-guild-id"),
        format!("`{}`", castle.owner_guild_id),
        true,
    )
    .field(
        ts(locale, "field-economy"),
        format!("`{}`", castle.economy),
        true,
    )
    .field(
        ts(locale, "field-defense"),
        format!("`{}`", castle.defense),
        true,
    )
    .field(
        ts(locale, "field-visible-c"),
        format!("`{}`", castle.visible_c),
        true,
    )
    .field(
        ts(locale, "field-triggers"),
        format!(
            "{} `{}` — {} `{}`",
            ts(locale, "field-economy"),
            castle.trigger_e,
            ts(locale, "field-defense"),
            castle.trigger_d,
        ),
        true,
    )
    .field(
        ts(locale, "field-timers"),
        format!(
            "{} `{}` — {} `{}` — {} `{}`",
            ts(locale, "field-next"),
            castle.next_time,
            ts(locale, "field-payment"),
            castle.pay_time,
            ts(locale, "field-created"),
            castle.create_time,
        ),
        false,
    )
}

pub fn castle_not_found_embed(castle_id: i64) -> CreateEmbed {
    castle_not_found_embed_l10n(BotLocale::DEFAULT, castle_id)
}

pub fn castle_not_found_embed_l10n(locale: BotLocale, castle_id: i64) -> CreateEmbed {
    let castle = castle_id.to_string();
    warning_embed(
        &ts(locale, "embed-castle-search-title"),
        tsa(
            locale,
            "embed-castle-not-found",
            &[TranslationArg::new("castle", &castle)],
        ),
    )
}
