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
pub struct SearchResults {
    pub characters: Vec<CharacterSummary>,
    pub items: Vec<ItemSearchEntry>,
    pub monsters: Vec<MonsterSearchEntry>,
}

impl SearchResults {
    pub fn is_empty(&self) -> bool {
        self.characters.is_empty() && self.items.is_empty() && self.monsters.is_empty()
    }
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
pub struct ClassDistributionEntry {
    pub rank: usize,
    pub class_id: i32,
    pub characters: i64,
    pub online_characters: i64,
}

#[derive(Debug, Clone)]
pub struct MapStatsEntry {
    pub rank: usize,
    pub map: String,
    pub characters: i64,
    pub online_characters: i64,
}

#[derive(Debug, Clone)]
pub struct PartyDetails {
    pub name: String,
    pub leader_name: Option<String>,
    pub members: i64,
    pub online_members: i64,
    pub exp_mode: i32,
    pub item_mode: i32,
}

#[derive(Debug, Clone)]
pub struct PartyMemberSummary {
    pub name: String,
    pub class_id: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub online: bool,
    pub map: String,
    pub is_leader: bool,
}

#[derive(Debug, Clone)]
pub struct HomunculusProfile {
    pub owner_name: String,
    pub name: String,
    pub class_id: i32,
    pub level: i32,
    pub intimacy: i32,
    pub hunger: i32,
    pub alive: bool,
    pub vaporized: bool,
    pub autofeed: bool,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub max_sp: i32,
}

#[derive(Debug, Clone)]
pub struct PetProfile {
    pub owner_name: String,
    pub name: String,
    pub class_id: i32,
    pub level: i32,
    pub intimacy: i32,
    pub hunger: i32,
    pub incubated: bool,
    pub autofeed: bool,
}

#[derive(Debug, Clone)]
pub struct ZenySummary {
    pub total_zeny: i64,
    pub average_zeny: i64,
    pub character_count: i64,
    pub richest_name: Option<String>,
    pub richest_zeny: i64,
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
pub struct GuildAllianceEntry {
    pub relation: String,
    pub target_guild_id: i64,
    pub target_name: String,
}

#[derive(Debug, Clone)]
pub struct GuildSkillEntry {
    pub skill_id: i32,
    pub level: i32,
}

#[derive(Debug, Clone)]
pub struct HomunculusRankingEntry {
    pub rank: usize,
    pub owner_name: String,
    pub name: String,
    pub class_id: i32,
    pub level: i32,
    pub intimacy: i32,
    pub hunger: i32,
}

#[derive(Debug, Clone)]
pub struct PetRankingEntry {
    pub rank: usize,
    pub owner_name: String,
    pub name: String,
    pub class_id: i32,
    pub level: i32,
    pub intimacy: i32,
    pub hunger: i32,
}

#[derive(Debug, Clone)]
pub struct QuestStats {
    pub quest_id: i64,
    pub total_characters: i64,
    pub state_0: i64,
    pub state_1: i64,
    pub state_2: i64,
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
pub struct AccountList {
    pub total_accounts: i64,
    pub page: u32,
    pub per_page: u32,
    pub offset: u32,
    pub entries: Vec<AccountListEntry>,
}

#[derive(Debug, Clone)]
pub struct AccountListEntry {
    pub account_id: i64,
    pub userid: String,
    pub sex: String,
    pub group_id: i32,
    pub state: i64,
    pub characters: i64,
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
pub enum AccountDeleteResult {
    Deleted {
        account_id: i64,
        userid: String,
        characters: i64,
        deleted_rows: u64,
    },
    HasGuildOwnership {
        account_id: i64,
        userid: String,
        guilds: i64,
    },
    NotFound {
        account_id: i64,
    },
}

#[derive(Debug, Clone, Default)]
pub struct AccountUpdateRequest {
    pub userid: Option<String>,
    pub password: Option<String>,
    pub sex: Option<String>,
    pub birthdate: Option<String>,
    pub email: Option<String>,
    pub group_id: Option<i32>,
    pub state: Option<i64>,
    pub unban_time: Option<i64>,
    pub expiration_time: Option<i64>,
    pub character_slots: Option<i32>,
}

impl AccountUpdateRequest {
    pub fn is_empty(&self) -> bool {
        self.userid.is_none()
            && self.password.is_none()
            && self.sex.is_none()
            && self.birthdate.is_none()
            && self.email.is_none()
            && self.group_id.is_none()
            && self.state.is_none()
            && self.unban_time.is_none()
            && self.expiration_time.is_none()
            && self.character_slots.is_none()
    }
}

#[derive(Debug, Clone)]
pub enum AccountUpdateResult {
    Updated {
        account_id: i64,
        userid: String,
        changed_fields: Vec<String>,
    },
    UsernameAlreadyExists {
        account_id: i64,
        userid: String,
    },
    NotFound {
        account_id: i64,
    },
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
pub struct ItemCountSummary {
    pub item_id: i64,
    pub inventory_amount: i64,
    pub cart_amount: i64,
    pub storage_amount: i64,
    pub guild_storage_amount: i64,
    pub total_amount: i64,
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

#[derive(Debug, Clone)]
pub struct VendingStoreEntry {
    pub rank: usize,
    pub merchant_name: String,
    pub shop_title: String,
    pub map: String,
    pub x: i32,
    pub y: i32,
    pub item_count: i64,
    pub total_amount: i64,
    pub min_price: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct BuyingStoreEntry {
    pub rank: usize,
    pub buyer_name: String,
    pub shop_title: String,
    pub map: String,
    pub x: i32,
    pub y: i32,
    pub item_count: i64,
    pub total_amount: i64,
    pub max_price: Option<i64>,
    pub zeny_limit: i64,
}
