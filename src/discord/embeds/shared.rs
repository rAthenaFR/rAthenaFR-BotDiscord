use super::*;

pub(super) fn service_status_lines(services: &[RAthenaFrServiceStatus]) -> String {
    if services.is_empty() {
        return "Aucun service rAthena n’est configuré.".to_string();
    }

    services
        .iter()
        .map(|service| {
            let state = if service.online {
                "🟢 Connecté"
            } else {
                "🔴 Hors ligne"
            };

            format!("**{}**: {}", service.name, state)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn success_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    base_embed(title, description, COLOR_SUCCESS)
}

pub(super) fn warning_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    base_embed(title, description, COLOR_WARNING)
}

pub(super) fn info_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    base_embed(title, description, COLOR_INFO)
}

pub(super) fn base_embed(
    title: &str,
    description: impl Into<String>,
    color: Colour,
) -> CreateEmbed {
    CreateEmbed::new()
        .title(brand_text(title))
        .description(brand_text(description.into()))
        .color(color)
        .footer(serenity::all::CreateEmbedFooter::new(footer_text()))
        .timestamp(Timestamp::now())
}

pub(super) fn display_name() -> String {
    std::env::var("RATHENAFR_DISPLAY_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "rAthenaFR".to_string())
}

pub(super) fn footer_text() -> String {
    format!("Bot Discord {}", display_name())
}

pub(super) fn brand_text(value: impl Into<String>) -> String {
    value.into().replace("rAthenaFR", COMMAND_DISPLAY_NAME)
}

pub(super) fn sanitize_embed_mentions(value: &str) -> String {
    value
        .replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
}

pub(super) fn truncate_embed_field(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value.to_string();
    }

    let mut output = value
        .chars()
        .take(limit.saturating_sub(1))
        .collect::<String>();
    output.push('…');
    output
}

pub(super) fn mob_drop_field_value(drop: &MonsterDropEntry) -> String {
    format!(
        "ID : {}\nAegisName : {}\nTaux serveur : {}",
        drop.item_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "Non disponible".to_string()),
        drop.aegis_name
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(sanitize_embed_mentions)
            .unwrap_or_else(|| "Non disponible".to_string()),
        drop.server_rate
            .map(format_drop_rate)
            .unwrap_or_else(|| "Non disponible".to_string()),
    )
}

pub(super) fn format_drop_rate(rate: f64) -> String {
    if rate >= 100.0 {
        "100%".to_string()
    } else {
        format!("{rate:.2}%")
    }
}

pub(super) fn account_state(value: i64) -> String {
    match value {
        0 => "`0` Actif".to_string(),
        5 => "`5` Banni".to_string(),
        other => format!("`{}`", other),
    }
}

pub(super) fn unix_time_field(value: i64) -> String {
    if value <= 0 {
        "Aucun".to_string()
    } else {
        format!("`{}`", value)
    }
}

pub(super) fn quest_state_name(value: &str) -> String {
    match value {
        "0" => "0 Ouverte".to_string(),
        "1" => "1 Terminée".to_string(),
        "2" => "2 Expirée".to_string(),
        other => other.to_string(),
    }
}

pub(super) fn item_line(item: &CharacterItemEntry) -> String {
    let identified = if item.identify {
        "identifié"
    } else {
        "inconnu"
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
        "Aucune carte".to_string()
    } else {
        format!("Cartes `{}`", cards)
    };

    format!(
        "{}Objet `{}` x`{}` — Équipé `{}` — {} — Lié `{}` — Grade `{}` — UID `{}` — {}",
        refine,
        item.item_id,
        format_number(item.amount),
        item.equip,
        identified,
        item.bound,
        item.enchant_grade,
        item.unique_id,
        card_text,
    )
}

pub(super) fn mvp_kill_field_name(entry: &MvpKillEntry) -> String {
    let name = if entry.monster_name.trim().is_empty() {
        format!("MVP #{}", entry.monster_id)
    } else {
        entry.monster_name.clone()
    };

    sanitize_embed_mentions(&name)
}

pub(super) fn mvp_kill_field_value(entry: &MvpKillEntry) -> String {
    let date = match entry.mvp_timestamp.filter(|timestamp| *timestamp > 0) {
        Some(timestamp) => format!("<t:{timestamp}:F> (<t:{timestamp}:R>)"),
        None => entry
            .mvp_date
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Non disponible")
            .to_string(),
    };
    let mvp_exp = entry
        .mvp_exp
        .filter(|value| *value > 0)
        .map(format_number_fr)
        .unwrap_or_else(|| "Non disponible".to_string());
    let killer_name = if entry.killer_name.trim().is_empty() {
        format!("Personnage #{}", entry.killer_id)
    } else {
        entry.killer_name.clone()
    };
    let prize_name = if entry.prize_name.trim().is_empty() {
        format!("Item #{}", entry.prize_id)
    } else {
        entry.prize_name.clone()
    };
    let mut lines = vec![
        format!("Joueur : {}", sanitize_embed_mentions(&killer_name)),
        format!("Carte : `{}`", sanitize_embed_mentions(&entry.map)),
        format!("Date : {date}"),
        format!("EXP MVP attribuée : {mvp_exp}"),
        format!("Récompense : {}", sanitize_embed_mentions(&prize_name)),
    ];

    if let Some(aegis_name) = entry
        .monster_aegis_name
        .as_deref()
        .filter(|value| !value.eq_ignore_ascii_case(&entry.monster_name))
    {
        lines.push(format!(
            "Nom technique : `{}`",
            sanitize_embed_mentions(aegis_name)
        ));
    }
    if let Some(aegis_name) = entry
        .prize_aegis_name
        .as_deref()
        .filter(|value| !value.eq_ignore_ascii_case(&prize_name))
    {
        lines.push(format!(
            "Récompense technique : `{}`",
            sanitize_embed_mentions(aegis_name)
        ));
    }

    lines.join("\n")
}

pub(super) fn format_number_fr(value: i64) -> String {
    let raw = value.unsigned_abs().to_string();
    let mut output = String::new();

    for (index, character) in raw.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            output.push(' ');
        }
        output.push(character);
    }

    let mut formatted = output.chars().rev().collect::<String>();
    if value < 0 {
        formatted.insert(0, '-');
    }
    formatted
}

pub(super) fn format_number(value: i64) -> String {
    let raw = value.abs().to_string();
    let mut output = String::new();

    for (index, character) in raw.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            output.push(',');
        }
        output.push(character);
    }

    let mut formatted = output.chars().rev().collect::<String>();
    if value < 0 {
        formatted.insert(0, '-');
    }
    formatted
}

pub(super) fn limited_list<T, F>(items: &[T], requested_limit: u32, formatter: F) -> LimitedList
where
    F: Fn(usize, &T) -> String,
{
    let row_limit = display_limit(requested_limit);
    let mut lines = Vec::new();
    let mut value_len = 0;

    for (index, item) in items.iter().take(row_limit).enumerate() {
        if !push_limited_line(
            &mut lines,
            &mut value_len,
            format_list_line(formatter(index, item)),
        ) {
            break;
        }
    }

    let displayed_count = lines.len();

    LimitedList {
        value: lines.join("\n\n"),
        displayed_count,
        available_count: items.len(),
        row_limit,
    }
}

pub(super) fn list_summary(list: &LimitedList, noun: &str) -> String {
    let total_text = if list.available_count > list.row_limit {
        format!("au moins {}", list.row_limit + 1)
    } else {
        list.available_count.to_string()
    };

    let mut summary = format!(
        "{} affiché(s) sur {} {}.",
        list.displayed_count, total_text, noun
    );

    let hidden_by_row_limit = list.available_count > list.row_limit;
    let hidden_by_embed_limit = list.displayed_count < list.available_count.min(list.row_limit);
    let hidden_reason = match (hidden_by_row_limit, hidden_by_embed_limit) {
        (true, true) => {
            Some("la limite d’affichage configurée et les limites de champ des embeds Discord")
        }
        (true, false) => Some("la limite d’affichage configurée"),
        (false, true) => Some("les limites de champ des embeds Discord"),
        (false, false) => None,
    };

    if let Some(reason) = hidden_reason {
        summary.push_str(" Masqué par ");
        summary.push_str(reason);
        summary.push('.');
    }

    summary
}

pub(super) fn display_limit(requested_limit: u32) -> usize {
    (requested_limit as usize).max(1)
}

pub(super) fn push_limited_line(
    lines: &mut Vec<String>,
    value_len: &mut usize,
    line: String,
) -> bool {
    let separator_len = if lines.is_empty() {
        0
    } else {
        EMBED_LIST_SEPARATOR_LEN
    };
    let available_len = EMBED_FIELD_VALUE_LIMIT.saturating_sub(*value_len + separator_len);

    if available_len == 0 {
        return false;
    }

    let line_len = line.chars().count();
    if line_len > available_len {
        if lines.is_empty() {
            let trimmed = trim_line(line, available_len);
            *value_len += separator_len + trimmed.chars().count();
            lines.push(trimmed);
            return true;
        }

        return false;
    }

    *value_len += separator_len + line_len;
    lines.push(line);
    true
}

pub(super) fn format_list_line(value: String) -> String {
    let parts = value
        .split(" — ")
        .filter(|part| !part.trim().is_empty())
        .collect::<Vec<_>>();

    if parts.len() <= 1 {
        return value;
    }

    let mut formatted = parts[0].trim().to_string();

    for detail in parts.iter().skip(1) {
        formatted.push_str("\n• ");
        formatted.push_str(detail.trim());
    }

    formatted
}

pub(super) fn trim_line(value: String, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value;
    }

    if limit == 0 {
        return String::new();
    }

    if limit <= 3 {
        return ".".repeat(limit);
    }

    let mut trimmed = value.chars().take(limit - 3).collect::<String>();
    trimmed.push_str("...");
    trimmed
}
