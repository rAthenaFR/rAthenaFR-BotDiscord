use super::*;
use crate::i18n::{BotLocale, I18nKey, TranslationArg};

pub fn mvp_list_panel_embed(lines: &[String], page: usize, page_size: usize) -> CreateEmbed {
    mvp_list_panel_embed_l10n(BotLocale::DEFAULT, lines, page, page_size)
}

pub fn mvp_list_panel_embed_l10n(
    locale: BotLocale,
    lines: &[String],
    page: usize,
    page_size: usize,
) -> CreateEmbed {
    let page_size = page_size.max(1);
    let total_count = lines.len();
    let total_pages = total_count.div_ceil(page_size).max(1);
    let page = page.min(total_pages.saturating_sub(1));
    let start = page.saturating_mul(page_size);
    let end = start.saturating_add(page_size).min(total_count);

    let description = if total_count == 0 {
        t(locale, I18nKey::EmbedMvpListEmpty)
    } else {
        let mut description = lines[start..end].join("\n\n");
        if page == 0 && total_count > page_size {
            let count = (end - start).to_string();
            description.push_str("\n\n");
            description.push_str(&ta(
                locale,
                I18nKey::EmbedMvpListLimitNotice,
                &[TranslationArg::new("count", &count)],
            ));
        }
        description
    };

    let page_label = (page + 1).to_string();
    let pages_label = total_pages.to_string();
    let count_label = total_count.to_string();

    CreateEmbed::new()
        .title(brand_text(t(locale, I18nKey::EmbedMvpListTitle)))
        .description(brand_text(truncate_embed_field(&description, 3900)))
        .color(COLOR_PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(ta(
            locale,
            I18nKey::EmbedMvpListFooter,
            &[
                TranslationArg::new("page", &page_label),
                TranslationArg::new("pages", &pages_label),
                TranslationArg::new("count", &count_label),
            ],
        )))
        .timestamp(Timestamp::now())
}

pub fn mvp_last_embed(entries: &[MvpKillEntry], requested_limit: u32) -> CreateEmbed {
    mvp_last_embed_l10n(BotLocale::DEFAULT, entries, requested_limit)
}

pub fn mvp_last_embed_l10n(
    locale: BotLocale,
    entries: &[MvpKillEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            &t(locale, I18nKey::EmbedMvpLastEmptyTitle),
            t(locale, I18nKey::EmbedMvpLastEmptyDescription),
        );
    }

    let display_limit = (requested_limit as usize).clamp(1, 10);
    let displayed_entries = entries.iter().take(display_limit);
    let has_more_entries = entries.len() > display_limit;
    let mut embed = CreateEmbed::new()
        .title(t(locale, I18nKey::TitleMvpLastKills))
        .color(COLOR_PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(t(
            locale,
            I18nKey::EmbedMvpLastFooter,
        )))
        .timestamp(Timestamp::now());

    if has_more_entries {
        let count = display_limit.to_string();
        embed = embed.description(ta(
            locale,
            I18nKey::EmbedMvpLastLimitNotice,
            &[TranslationArg::new("count", &count)],
        ));
    }

    for entry in displayed_entries {
        embed = embed.field(
            truncate_embed_field(&mvp_kill_field_name(entry), 256),
            truncate_embed_field(
                &mvp_kill_field_value_l10n(locale, entry),
                EMBED_FIELD_VALUE_LIMIT,
            ),
            false,
        );
    }

    embed
}
