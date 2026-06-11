#[derive(Debug, Clone)]
pub struct MarketSellEntry {
    pub merchant_name: String,
    pub shop_title: String,
    pub map: String,
    pub x: i32,
    pub y: i32,
    pub amount: i64,
    pub price: i64,
}

#[derive(Debug, Clone)]
pub struct MarketBuyEntry {
    pub buyer_name: String,
    pub shop_title: String,
    pub map: String,
    pub x: i32,
    pub y: i32,
    pub amount: i64,
    pub price: i64,
}

#[derive(Debug, Clone)]
pub struct MarketOverview {
    pub item_id: i64,
    pub sellers: i64,
    pub sell_amount: i64,
    pub lowest_sell_price: Option<i64>,
    pub buyers: i64,
    pub buy_amount: i64,
    pub highest_buy_price: Option<i64>,
}
