use super::*;
use crate::i18n::BotLocale;

pub fn online_embed(characters: &[CharacterSummary], requested_limit: u32) -> CreateEmbed {
    online_embed_l10n(BotLocale::DEFAULT, characters, requested_limit)
}

pub fn online_embed_l10n(
    locale: BotLocale,
    characters: &[CharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if characters.is_empty() {
        return warning_embed(
            &ts(locale, "embed-online-title"),
            ts(locale, "embed-online-empty"),
        );
    }

    let list = limited_list(characters, requested_limit, |index, character| {
        format!(
            "`{:>2}.` **{}** — Base `{}` / Job `{}` — {} — {} `{}`",
            index + 1,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
            ts(locale, "field-map"),
            character.map,
        )
    });

    success_embed(
        &ts(locale, "embed-online-title"),
        ts(locale, "embed-online-description"),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-online-characters")),
        false,
    )
    .field(ts(locale, "field-characters"), list.value, false)
}

pub fn ranking_embed(entries: &[RankingEntry], requested_limit: u32) -> CreateEmbed {
    ranking_embed_l10n(BotLocale::DEFAULT, entries, requested_limit)
}

pub fn ranking_embed_l10n(
    locale: BotLocale,
    entries: &[RankingEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            &ts(locale, "embed-ranking-title"),
            ts(locale, "embed-ranking-empty"),
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — Base `{}` / Job `{}` — {} — {} `{}`",
            entry.rank,
            entry.name,
            entry.base_level,
            entry.job_level,
            job_name(entry.class_id),
            ts(locale, "field-map"),
            entry.map,
        )
    });

    info_embed(
        &ts(locale, "embed-ranking-title"),
        ts(locale, "embed-ranking-description"),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-ranking-entries")),
        false,
    )
    .field(ts(locale, "field-ranking"), list.value, false)
}

pub fn top_zeny_embed(entries: &[ZenyRankingEntry], requested_limit: u32) -> CreateEmbed {
    top_zeny_embed_l10n(BotLocale::DEFAULT, entries, requested_limit)
}

pub fn top_zeny_embed_l10n(
    locale: BotLocale,
    entries: &[ZenyRankingEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            &ts(locale, "embed-top-zeny-title"),
            ts(locale, "embed-ranking-empty"),
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny — Base `{}` / Job `{}` — {}",
            entry.rank,
            entry.name,
            format_number(entry.zeny),
            entry.base_level,
            entry.job_level,
            job_name(entry.class_id),
        )
    });

    base_embed(
        &ts(locale, "embed-top-zeny-title"),
        ts(locale, "embed-top-zeny-description"),
        COLOR_PURPLE,
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-ranking-entries")),
        false,
    )
    .field(ts(locale, "field-ranking"), list.value, false)
}
