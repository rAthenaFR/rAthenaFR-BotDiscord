use super::*;
use crate::i18n::{BotLocale, I18nKey, TranslationArg};

pub fn who_sell_embed(
    item_id: i64,
    sellers: &[MarketSellEntry],
    requested_limit: u32,
) -> CreateEmbed {
    who_sell_embed_l10n(BotLocale::DEFAULT, item_id, sellers, requested_limit)
}

pub fn who_sell_embed_l10n(
    locale: BotLocale,
    item_id: i64,
    sellers: &[MarketSellEntry],
    requested_limit: u32,
) -> CreateEmbed {
    let item = item_id.to_string();
    if sellers.is_empty() {
        return warning_embed(
            &t(locale, I18nKey::TitleMarketSellers),
            ta(
                locale,
                I18nKey::EmbedMarketSellersEmpty,
                &[TranslationArg::new("item", &item)],
            ),
        );
    }

    let list = limited_list(sellers, requested_limit, |index, seller| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny x`{}` — `{}` sur `{}` ({}, {})",
            index + 1,
            seller.merchant_name,
            format_number(seller.price),
            format_number(seller.amount),
            seller.shop_title,
            seller.map,
            seller.x,
            seller.y,
        )
    });

    info_embed(
        &t(locale, I18nKey::TitleMarketSellers),
        ta(
            locale,
            I18nKey::EmbedMarketSellersDescription,
            &[TranslationArg::new("item", &item)],
        ),
    )
    .field(
        t(locale, I18nKey::FieldSummary),
        list_summary_l10n(locale, &list, "lignes de vendeurs"),
        false,
    )
    .field(t(locale, I18nKey::FieldSellers), list.value, false)
}

pub fn who_buy_embed(item_id: i64, buyers: &[MarketBuyEntry], requested_limit: u32) -> CreateEmbed {
    who_buy_embed_l10n(BotLocale::DEFAULT, item_id, buyers, requested_limit)
}

pub fn who_buy_embed_l10n(
    locale: BotLocale,
    item_id: i64,
    buyers: &[MarketBuyEntry],
    requested_limit: u32,
) -> CreateEmbed {
    let item = item_id.to_string();
    if buyers.is_empty() {
        return warning_embed(
            &t(locale, I18nKey::TitleMarketBuyers),
            ta(
                locale,
                I18nKey::EmbedMarketBuyersEmpty,
                &[TranslationArg::new("item", &item)],
            ),
        );
    }

    let list = limited_list(buyers, requested_limit, |index, buyer| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny x`{}` — `{}` sur `{}` ({}, {})",
            index + 1,
            buyer.buyer_name,
            format_number(buyer.price),
            format_number(buyer.amount),
            buyer.shop_title,
            buyer.map,
            buyer.x,
            buyer.y,
        )
    });

    info_embed(
        &t(locale, I18nKey::TitleMarketBuyers),
        ta(
            locale,
            I18nKey::EmbedMarketBuyersDescription,
            &[TranslationArg::new("item", &item)],
        ),
    )
    .field(
        t(locale, I18nKey::FieldSummary),
        list_summary_l10n(locale, &list, "lignes d’acheteurs"),
        false,
    )
    .field(t(locale, I18nKey::FieldBuyers), list.value, false)
}

pub fn market_embed(overview: &MarketOverview) -> CreateEmbed {
    market_embed_l10n(BotLocale::DEFAULT, overview)
}

pub fn market_embed_l10n(locale: BotLocale, overview: &MarketOverview) -> CreateEmbed {
    let lowest_sell = overview
        .lowest_sell_price
        .map(format_number)
        .unwrap_or_else(|| t(locale, I18nKey::TextNone));
    let highest_buy = overview
        .highest_buy_price
        .map(format_number)
        .unwrap_or_else(|| t(locale, I18nKey::TextNone));
    let item = overview.item_id.to_string();

    info_embed(
        &t(locale, I18nKey::TitleMarketOverview),
        ta(
            locale,
            I18nKey::EmbedMarketOverviewDescription,
            &[TranslationArg::new("item", &item)],
        ),
    )
    .field(
        t(locale, I18nKey::FieldSellers),
        format!("`{}`", overview.sellers),
        true,
    )
    .field(
        t(locale, I18nKey::FieldSellAmount),
        format!("`{}`", format_number(overview.sell_amount)),
        true,
    )
    .field(
        t(locale, I18nKey::FieldLowestSellPrice),
        format!("`{}`", lowest_sell),
        true,
    )
    .field(
        t(locale, I18nKey::FieldBuyers),
        format!("`{}`", overview.buyers),
        true,
    )
    .field(
        t(locale, I18nKey::FieldBuyAmount),
        format!("`{}`", format_number(overview.buy_amount)),
        true,
    )
    .field(
        t(locale, I18nKey::FieldHighestBuyPrice),
        format!("`{}`", highest_buy),
        true,
    )
}
