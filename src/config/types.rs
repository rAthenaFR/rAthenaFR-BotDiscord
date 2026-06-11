use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub discord: DiscordConfig,
    pub database: DatabaseConfig,
    pub services: ServicesConfig,
    pub display: DisplayConfig,
    pub cache: CacheConfig,
    pub server_rates: ServerRatesConfig,
    pub account_commands: AccountCommandsConfig,
    pub commands: CommandConfig,
    pub game_bridge: GameBridgeConfig,
}

#[derive(Debug, Clone)]
pub struct DiscordConfig {
    pub token: String,
    pub application_id: u64,
    pub guild_id: u64,
    pub helper_role_ids: Vec<u64>,
    pub moderator_role_ids: Vec<u64>,
    pub gm_role_ids: Vec<u64>,
    pub staff_role_ids: Vec<u64>,
    pub admin_role_ids: Vec<u64>,
    pub owner_role_ids: Vec<u64>,
    pub staff_log_channel_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
    pub acquire_timeout_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct ServiceEndpointConfig {
    pub name: &'static str,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct ServicesConfig {
    pub login: ServiceEndpointConfig,
    pub char_server: ServiceEndpointConfig,
    pub map: ServiceEndpointConfig,
}

#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub hide_gm_characters: bool,
    pub hide_gm_from_top: bool,
    pub hide_gm_group_from_ranking: i32,
    pub default_limit: u32,
    pub max_limit: u32,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct DropRateSet {
    pub normal: u32,
    pub boss: u32,
    pub mvp: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DropRateBounds {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone)]
pub struct ServerRatesConfig {
    pub configured: bool,
    pub base_exp_rate: u32,
    pub job_exp_rate: u32,
    pub mvp_exp_rate: u32,
    pub item_rate_common: DropRateSet,
    pub item_rate_heal: DropRateSet,
    pub item_rate_use: DropRateSet,
    pub item_rate_equip: DropRateSet,
    pub item_rate_card: DropRateSet,
    pub item_rate_mvp: u32,
    pub item_drop_common: DropRateBounds,
    pub item_drop_heal: DropRateBounds,
    pub item_drop_use: DropRateBounds,
    pub item_drop_equip: DropRateBounds,
    pub item_drop_card: DropRateBounds,
    pub item_drop_mvp: DropRateBounds,
    pub logarithmic_drops: bool,
    pub drop_rate_increase: bool,
    pub item_ratio_overrides: bool,
}

#[derive(Debug, Clone)]
pub struct AccountCommandsConfig {
    pub creation_enabled: bool,
    pub password_mode: AccountPasswordMode,
    pub manage_enabled: bool,
    pub delete_enabled: bool,
    pub manage_min_role: StaffRole,
    pub delete_min_role: StaffRole,
}

#[derive(Debug, Clone)]
pub struct CommandConfig {
    pub public_pack_enabled: bool,
    pub staff_pack_enabled: bool,
    pub online_list_public: bool,
    pub top_zeny_mode: TopZenyMode,
    pub disabled_commands: Vec<String>,
    pub item_table_name: String,
    pub mob_table_name: String,
    pub gmmsg_min_role: StaffRole,
    pub debug_min_role: StaffRole,
    pub audit_min_role: StaffRole,
}

#[derive(Debug, Clone)]
pub struct GameBridgeConfig {
    pub mode: GameBridgeMode,
    pub max_message_length: usize,
    pub encoding: GameBridgeEncoding,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountPasswordMode {
    Plain,
    Md5,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TopZenyMode {
    Enabled,
    Anonymized,
    Disabled,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameBridgeMode {
    Disabled,
    Test,
    SqlQueue,
    Bridge,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameBridgeEncoding {
    Windows1252,
    Utf8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum StaffRole {
    Helper,
    Moderator,
    Gm,
    Admin,
    Owner,
}

impl Default for ServerRatesConfig {
    fn default() -> Self {
        let standard = DropRateSet {
            normal: 100,
            boss: 100,
            mvp: 100,
        };
        let bounds = DropRateBounds {
            min: 1,
            max: 10_000,
        };

        Self {
            configured: false,
            base_exp_rate: 100,
            job_exp_rate: 100,
            mvp_exp_rate: 100,
            item_rate_common: standard,
            item_rate_heal: standard,
            item_rate_use: standard,
            item_rate_equip: standard,
            item_rate_card: standard,
            item_rate_mvp: 100,
            item_drop_common: bounds,
            item_drop_heal: bounds,
            item_drop_use: bounds,
            item_drop_equip: bounds,
            item_drop_card: bounds,
            item_drop_mvp: bounds,
            logarithmic_drops: false,
            drop_rate_increase: false,
            item_ratio_overrides: false,
        }
    }
}

impl DisplayConfig {
    pub fn clamp_limit(&self, requested: Option<i64>) -> u32 {
        match requested {
            Some(value) if value > 0 => (value as u32).min(self.max_limit),
            _ => self.default_limit,
        }
    }

    pub fn public_character_group_threshold(&self) -> i32 {
        if self.hide_gm_characters {
            self.hide_gm_group_from_ranking
        } else {
            i32::MAX
        }
    }

    pub fn ranking_group_threshold(&self) -> i32 {
        if self.hide_gm_from_top {
            self.hide_gm_group_from_ranking
        } else {
            i32::MAX
        }
    }
}

impl CacheConfig {
    pub fn duration(&self, default_seconds: u64) -> Option<Duration> {
        if !self.enabled {
            return None;
        }

        let seconds = self.ttl_seconds.unwrap_or(default_seconds);
        if seconds == 0 {
            None
        } else {
            Some(Duration::from_secs(seconds))
        }
    }
}

impl ServicesConfig {
    pub fn endpoints(&self) -> [&ServiceEndpointConfig; 3] {
        [&self.login, &self.char_server, &self.map]
    }
}
