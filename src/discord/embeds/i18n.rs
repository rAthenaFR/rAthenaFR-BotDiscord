use super::*;
use crate::i18n::{translate, translate_with_args, BotLocale, I18nKey, TranslationArg};

pub(super) fn t(locale: BotLocale, key: I18nKey) -> String {
    translate(locale, key)
}

pub(super) fn ta(locale: BotLocale, key: I18nKey, args: &[TranslationArg<'_>]) -> String {
    translate_with_args(locale, key, args)
}

pub(super) fn ts(locale: BotLocale, key: &str) -> String {
    crate::i18n::loader::lookup(locale, key)
        .unwrap_or(key)
        .to_string()
}

pub(super) fn tsa(locale: BotLocale, key: &str, args: &[TranslationArg<'_>]) -> String {
    let mut value = ts(locale, key);

    for arg in args {
        let token = format!("{{ ${} }}", arg.name);
        value = value.replace(&token, arg.value);
    }

    value
}

pub(super) fn default_locale() -> BotLocale {
    BotLocale::DEFAULT
}

pub(super) fn arg<'a>(name: &'a str, value: &'a str) -> TranslationArg<'a> {
    TranslationArg::new(name, value)
}

pub(super) fn arg_owned<'a>(storage: &'a str, name: &'a str) -> TranslationArg<'a> {
    TranslationArg::new(name, storage)
}

pub(super) fn list_summary_l10n(locale: BotLocale, list: &LimitedList, noun: &str) -> String {
    let total_text = if list.available_count > list.row_limit {
        let count = (list.row_limit + 1).to_string();
        ta(locale, I18nKey::ListAtLeast, &[arg("count", &count)])
    } else {
        list.available_count.to_string()
    };
    let displayed = list.displayed_count.to_string();
    let mut summary = ta(
        locale,
        I18nKey::ListSummary,
        &[
            arg("displayed", &displayed),
            arg("total", &total_text),
            arg("noun", noun),
        ],
    );

    let hidden_by_row_limit = list.available_count > list.row_limit;
    let hidden_by_embed_limit = list.displayed_count < list.available_count.min(list.row_limit);
    let reason_key = match (hidden_by_row_limit, hidden_by_embed_limit) {
        (true, true) => Some(I18nKey::ListHiddenDisplayAndEmbedLimit),
        (true, false) => Some(I18nKey::ListHiddenDisplayLimit),
        (false, true) => Some(I18nKey::ListHiddenEmbedLimit),
        (false, false) => None,
    };

    if let Some(reason_key) = reason_key {
        let reason = t(locale, reason_key);
        summary = ta(
            locale,
            I18nKey::ListSummaryHidden,
            &[arg("summary", &summary), arg("reason", &reason)],
        );
    }

    summary
}

pub(super) fn service_status_lines_l10n(
    locale: BotLocale,
    services: &[RAthenaFrServiceStatus],
) -> String {
    if services.is_empty() {
        return ts(locale, "embed-service-status-empty");
    }

    services
        .iter()
        .map(|service| {
            let state = if service.online {
                format!("🟢 {}", t(locale, I18nKey::TextConnected))
            } else {
                format!("🔴 {}", t(locale, I18nKey::TextOffline))
            };

            format!("**{}**: {}", service.name, state)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn localized_status_icon(locale: BotLocale, online: bool) -> String {
    if online {
        format!("🟢 {}", t(locale, I18nKey::TextConnected))
    } else {
        format!("⚫ {}", t(locale, I18nKey::TextOffline))
    }
}

pub(super) fn none(locale: BotLocale) -> String {
    t(locale, I18nKey::TextNone)
}

pub(super) fn unavailable(locale: BotLocale) -> String {
    t(locale, I18nKey::TextUnavailable)
}

pub(super) fn account_state_l10n(locale: BotLocale, value: i64) -> String {
    match value {
        0 => format!("`0` {}", t(locale, I18nKey::TextActive)),
        5 => format!("`5` {}", t(locale, I18nKey::TextBanned)),
        other => format!("`{}`", other),
    }
}

pub(super) fn unix_time_field_l10n(locale: BotLocale, value: i64) -> String {
    if value <= 0 {
        none(locale)
    } else {
        format!("`{}`", value)
    }
}

pub(super) fn mob_drop_field_value_l10n(locale: BotLocale, drop: &MonsterDropEntry) -> String {
    format!(
        "{} : {}\nAegisName : {}\n{} : {}",
        ts(locale, "field-id"),
        drop.item_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| unavailable(locale)),
        drop.aegis_name
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(sanitize_embed_mentions)
            .unwrap_or_else(|| unavailable(locale)),
        ts(locale, "field-server-rate"),
        drop.server_rate
            .map(format_drop_rate)
            .unwrap_or_else(|| unavailable(locale)),
    )
}

pub(super) fn mvp_kill_field_value_l10n(locale: BotLocale, entry: &MvpKillEntry) -> String {
    let date = match entry.mvp_timestamp.filter(|timestamp| *timestamp > 0) {
        Some(timestamp) => format!("<t:{timestamp}:F> (<t:{timestamp}:R>)"),
        None => entry
            .mvp_date
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| unavailable(locale)),
    };
    let mvp_exp = entry
        .mvp_exp
        .filter(|value| *value > 0)
        .map(format_number_fr)
        .unwrap_or_else(|| unavailable(locale));
    let killer_name = if entry.killer_name.trim().is_empty() {
        tsa(
            locale,
            "text-character-number",
            &[arg("id", &entry.killer_id.to_string())],
        )
    } else {
        entry.killer_name.clone()
    };
    let prize_name = if entry.prize_name.trim().is_empty() {
        tsa(
            locale,
            "text-item-number",
            &[arg("id", &entry.prize_id.to_string())],
        )
    } else {
        entry.prize_name.clone()
    };
    let mut lines = vec![
        format!(
            "{} : {}",
            t(locale, I18nKey::FieldUser),
            sanitize_embed_mentions(&killer_name)
        ),
        format!(
            "{} : `{}`",
            t(locale, I18nKey::FieldMap),
            sanitize_embed_mentions(&entry.map)
        ),
        format!("{} : {date}", ts(locale, "field-date")),
        format!("{} : {mvp_exp}", ts(locale, "field-mvp-exp")),
        format!(
            "{} : {}",
            ts(locale, "field-reward"),
            sanitize_embed_mentions(&prize_name)
        ),
    ];

    if let Some(aegis_name) = entry
        .monster_aegis_name
        .as_deref()
        .filter(|value| !value.eq_ignore_ascii_case(&entry.monster_name))
    {
        lines.push(format!(
            "{} : `{}`",
            ts(locale, "field-technical-name"),
            sanitize_embed_mentions(aegis_name)
        ));
    }
    if let Some(aegis_name) = entry
        .prize_aegis_name
        .as_deref()
        .filter(|value| !value.eq_ignore_ascii_case(&prize_name))
    {
        lines.push(format!(
            "{} : `{}`",
            ts(locale, "field-technical-reward"),
            sanitize_embed_mentions(aegis_name)
        ));
    }

    lines.join("\n")
}

pub(super) fn quest_state_name_l10n(locale: BotLocale, value: &str) -> String {
    match value {
        "0" => format!("0 {}", ts(locale, "quest-state-open")),
        "1" => format!("1 {}", ts(locale, "quest-state-completed")),
        "2" => format!("2 {}", ts(locale, "quest-state-expired")),
        other => other.to_string(),
    }
}

pub(super) fn item_line_l10n(locale: BotLocale, item: &CharacterItemEntry) -> String {
    let identified = if item.identify {
        ts(locale, "text-identified-lower")
    } else {
        ts(locale, "text-unknown-lower")
    };
    let refine = if item.refine > 0 {
        format!("+{} ", item.refine)
    } else {
        String::new()
    };
    let cards = [item.card0, item.card1, item.card2, item.card3]
        .into_iter()
        .filter(|card| *card != 0)
        .map(|card| card.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let card_text = if cards.is_empty() {
        t(locale, I18nKey::TextNoCard)
    } else {
        tsa(locale, "text-cards", &[arg("cards", &cards)])
    };

    format!(
        "{}{} `{}` x`{}` — {} `{}` — {} — {} `{}` — {} `{}` — UID `{}` — {}",
        refine,
        ts(locale, "field-item"),
        item.item_id,
        format_number(item.amount),
        ts(locale, "field-equipped"),
        item.equip,
        identified,
        ts(locale, "field-bound"),
        item.bound,
        ts(locale, "field-grade"),
        item.enchant_grade,
        item.unique_id,
        card_text,
    )
}
