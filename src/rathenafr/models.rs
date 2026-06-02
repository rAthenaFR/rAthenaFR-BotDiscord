#[derive(Debug, Clone)]
pub struct DatabaseStatus {
    pub database_name: String,
    pub database_engine: String,
    pub online_characters: i64,
    pub characters: i64,
    pub accounts: i64,
    pub guilds: i64,
}

#[derive(Debug, Clone)]
pub struct CharacterSummary {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub map: String,
}

#[derive(Debug, Clone)]
pub struct ItemSearchEntry {
    pub item_id: i64,
    pub aegis_name: String,
    pub display_name: String,
    pub item_type: String,
    pub source_table: String,
}

#[derive(Debug, Clone)]
pub struct MonsterSearchEntry {
    pub monster_id: i64,
    pub sprite: String,
    pub display_name: String,
    pub level: i32,
    pub hp: i64,
    pub source_table: String,
}

#[derive(Debug, Clone)]
pub struct RankingEntry {
    pub rank: usize,
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub map: String,
}

#[derive(Debug, Clone)]
pub struct ZenyRankingEntry {
    pub rank: usize,
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub zeny: i64,
}

#[derive(Debug, Clone)]
pub struct PlayerProfile {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub map: String,
    pub zeny: i64,
    pub guild_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GuildSummary {
    pub name: String,
    pub master: String,
    pub level: i32,
    pub members: i64,
    pub online_members: i32,
    pub max_members: i32,
}

#[derive(Debug, Clone)]
pub struct GuildDetails {
    pub name: String,
    pub master: String,
    pub level: i32,
    pub members: i64,
    pub online_members: i32,
    pub max_members: i32,
    pub average_level: i32,
    pub exp: i64,
    pub next_exp: i64,
}

#[derive(Debug, Clone)]
pub struct GuildMemberSummary {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub position: i32,
    pub map: String,
}

#[derive(Debug, Clone)]
pub struct MapStatsEntry {
    pub rank: usize,
    pub map: String,
    pub characters: i64,
    pub online_characters: i64,
}

#[derive(Debug, Clone)]
pub struct AccountCharacterSummary {
    pub slot: i32,
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub map: String,
    pub zeny: i64,
    pub guild_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CastleSummary {
    pub castle_id: i32,
    pub owner_name: Option<String>,
    pub economy: i32,
    pub defense: i32,
    pub visible_c: i32,
}

#[derive(Debug, Clone)]
pub struct CastleDetails {
    pub castle_id: i32,
    pub owner_guild_id: i64,
    pub owner_name: Option<String>,
    pub economy: i32,
    pub defense: i32,
    pub trigger_e: i32,
    pub trigger_d: i32,
    pub next_time: i64,
    pub pay_time: i64,
    pub create_time: i64,
    pub visible_c: i32,
}

#[derive(Debug, Clone)]
pub struct AccountStatus {
    pub account_id: i64,
    pub userid: String,
    pub sex: String,
    pub group_id: i32,
    pub state: i64,
    pub unban_time: i64,
    pub expiration_time: i64,
    pub logincount: i64,
    pub character_slots: i32,
    pub characters: i64,
    pub online_characters: i64,
    pub total_zeny: i64,
    pub lastlogin: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreatedAccount {
    pub account_id: i64,
    pub userid: String,
    pub sex: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct CharacterQuestEntry {
    pub quest_id: i64,
    pub state: String,
    pub time: i64,
    pub count1: i64,
    pub count2: i64,
    pub count3: i64,
}

#[derive(Debug, Clone)]
pub struct CharacterItemEntry {
    pub item_id: i64,
    pub amount: i64,
    pub equip: i64,
    pub refine: i32,
    pub identify: bool,
    pub bound: i32,
    pub unique_id: i64,
    pub enchant_grade: i32,
    pub card0: i64,
    pub card1: i64,
    pub card2: i64,
    pub card3: i64,
}

#[derive(Debug, Clone)]
pub struct ItemOwnerEntry {
    pub source: String,
    pub owner_name: String,
    pub account_id: Option<i64>,
    pub amount: i64,
}

#[derive(Debug, Clone)]
pub struct BanEntry {
    pub account_id: i64,
    pub userid: String,
    pub group_id: i32,
    pub state: i64,
    pub unban_time: i64,
    pub expiration_time: i64,
    pub lastlogin: Option<String>,
    pub characters: i64,
}

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
