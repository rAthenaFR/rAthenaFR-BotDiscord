use super::*;
use crate::i18n::{BotLocale, I18nKey, TranslationArg};

pub fn mob_drops_embed(result: &MonsterDrops) -> CreateEmbed {
    mob_drops_embed_l10n(BotLocale::DEFAULT, result)
}

pub fn mob_drops_embed_l10n(locale: BotLocale, result: &MonsterDrops) -> CreateEmbed {
    if result.drops.is_empty() {
        return warning_embed(
            &t(locale, I18nKey::EmbedMobNoDropsTitle),
            t(locale, I18nKey::EmbedMobNoDropsDescription),
        );
    }

    let display_limit = 10;
    let mut description = format!(
        "{}\nID : {}",
        sanitize_embed_mentions(&result.monster_name),
        result.monster_id
    );
    if result.drops.len() > display_limit {
        let count = display_limit.to_string();
        description.push_str("\n\n");
        description.push_str(&ta(
            locale,
            I18nKey::EmbedMobDropsLimitNotice,
            &[TranslationArg::new("count", &count)],
        ));
    }

    let mut embed = CreateEmbed::new()
        .author(embed_author())
        .title(t(locale, I18nKey::TitleMobDrops))
        .description(truncate_embed_field(&description, 4096))
        .color(COLOR_INFO)
        .footer(serenity::all::CreateEmbedFooter::new(t(
            locale,
            I18nKey::EmbedMobDropsFooter,
        )))
        .timestamp(Timestamp::now());

    for drop in result.drops.iter().take(display_limit) {
        embed = embed.field(
            truncate_embed_field(&sanitize_embed_mentions(&drop.item_name), 256),
            truncate_embed_field(
                &mob_drop_field_value_l10n(locale, drop),
                EMBED_FIELD_VALUE_LIMIT,
            ),
            false,
        );
    }

    embed
}

pub fn monster_not_found_embed() -> CreateEmbed {
    monster_not_found_embed_l10n(BotLocale::DEFAULT)
}

pub fn monster_not_found_embed_l10n(locale: BotLocale) -> CreateEmbed {
    warning_embed(
        &t(locale, I18nKey::EmbedMobNotFoundTitle),
        t(locale, I18nKey::EmbedMobNotFoundDescription),
    )
}
