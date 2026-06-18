use crate::config::{
    AccountPasswordMode, DatabaseConfig, DropRateBounds, DropRateSet, ServerRatesConfig,
};
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, mysql::MySqlRow, MySqlPool, Row};
use std::time::Duration;

#[derive(Clone)]
pub struct RAthenaFrDatabase {
    pool: MySqlPool,
}

mod account_repository;
mod ban_repository;
mod character_repository;
mod connection;
mod gm_message_repository;
mod item_repository;
mod market_repository;
mod mob_repository;
mod mvp_repository;
mod ranking_repository;
mod server_repository;
mod staff_repository;

mod tables;
pub use tables::DatabaseTable;

use crate::rathenafr::models::*;

const ITEM_SEARCH_TABLES: &[&str] = &["item_db", "item_db_re"];
const RATHENAFR_ITEM_SEARCH_TABLE: &str = "rathenafr_item_search";
const MOB_SEARCH_TABLES: &[&str] = &["mob_db", "mob_db_re"];
const ITEM_AEGIS_COLUMN_CANDIDATES: &[&str] = &[
    "name_aegis",
    "aegis_name",
    "name_japanese",
    "name_english",
    "name",
];
const ITEM_DISPLAY_COLUMN_CANDIDATES: &[&str] = &[
    "name_english",
    "name_japanese",
    "name_aegis",
    "aegis_name",
    "name",
];
const ITEM_TYPE_COLUMN_CANDIDATES: &[&str] = &["type", "item_type"];
const MONSTER_SPRITE_COLUMN_CANDIDATES: &[&str] = &["sprite", "name_aegis", "aegis_name"];
const MONSTER_DISPLAY_COLUMN_CANDIDATES: &[&str] = &[
    "iROName",
    "iro_name",
    "name_english",
    "name",
    "kROName",
    "kro_name",
    "name_japanese",
    "name_aegis",
    "aegis_name",
    "sprite",
];
const MONSTER_LEVEL_COLUMN_CANDIDATES: &[&str] = &["LV", "level", "lv"];
const MONSTER_HP_COLUMN_CANDIDATES: &[&str] = &["HP", "hp"];
const RELEASE_REQUIRED_TABLES: &[DatabaseTable] = &[
    DatabaseTable::Login,
    DatabaseTable::Char,
    DatabaseTable::Guild,
    DatabaseTable::GuildMember,
];
const RELEASE_OPTIONAL_TABLES: &[DatabaseTable] = &[
    DatabaseTable::GuildPosition,
    DatabaseTable::GuildSkill,
    DatabaseTable::GuildCastle,
    DatabaseTable::GuildStorage,
    DatabaseTable::Party,
    DatabaseTable::Inventory,
    DatabaseTable::CartInventory,
    DatabaseTable::Storage,
    DatabaseTable::Mail,
    DatabaseTable::MailAttachments,
    DatabaseTable::Skill,
    DatabaseTable::Quest,
    DatabaseTable::Pet,
    DatabaseTable::Homunculus,
    DatabaseTable::CharRegNum,
    DatabaseTable::CharRegStr,
    DatabaseTable::AccRegNum,
    DatabaseTable::AccRegStr,
    DatabaseTable::ItemDb,
    DatabaseTable::ItemDbRe,
    DatabaseTable::RAthenaFrItemSearch,
    DatabaseTable::MobDb,
    DatabaseTable::MobDbRe,
    DatabaseTable::MobSkillDb,
    DatabaseTable::Vendings,
    DatabaseTable::VendingItems,
    DatabaseTable::BuyingStores,
    DatabaseTable::BuyingStoreItems,
    DatabaseTable::SqlUpdates,
];
const RELEASE_LOG_TABLES: &[DatabaseTable] = &[
    DatabaseTable::MvpLog,
    DatabaseTable::PickLog,
    DatabaseTable::ZenyLog,
    DatabaseTable::LoginLog,
    DatabaseTable::ChatLog,
    DatabaseTable::AtCommandLog,
    DatabaseTable::BranchLog,
    DatabaseTable::CharLog,
];
const TOP_GUILDS_SQL: &str = r#"
            SELECT
                g.name AS guild_name,
                g.master AS guild_master,
                CAST(g.guild_lv AS SIGNED) AS guild_level,
                CAST(g.max_member AS SIGNED) AS max_members,
                CAST(COUNT(CASE WHEN l.account_id IS NOT NULL THEN gm.char_id END) AS SIGNED) AS members,
                CAST(COALESCE(SUM(CASE WHEN l.account_id IS NOT NULL AND `char`.`online` > 0 THEN 1 ELSE 0 END), 0) AS SIGNED) AS online_members
            FROM `guild` g
            LEFT JOIN `guild_member` gm ON gm.guild_id = g.guild_id
            LEFT JOIN `char` ON `char`.`char_id` = gm.char_id
            LEFT JOIN `login` l ON l.account_id = `char`.`account_id` AND l.group_id < ?
            GROUP BY
                g.guild_id,
                g.name,
                g.master,
                g.guild_lv,
                g.max_member
            ORDER BY g.guild_lv DESC, members DESC, g.name ASC
            LIMIT ?
            "#;
const FIND_GUILD_SQL: &str = r#"
            SELECT
                g.name AS guild_name,
                g.master AS guild_master,
                CAST(g.guild_lv AS SIGNED) AS guild_level,
                CAST(g.max_member AS SIGNED) AS max_members,
                CAST(g.average_lv AS SIGNED) AS average_level,
                CAST(g.exp AS SIGNED) AS guild_exp,
                CAST(g.next_exp AS SIGNED) AS next_exp,
                CAST(COUNT(CASE WHEN l.account_id IS NOT NULL THEN gm.char_id END) AS SIGNED) AS members,
                CAST(COALESCE(SUM(CASE WHEN l.account_id IS NOT NULL AND `char`.`online` > 0 THEN 1 ELSE 0 END), 0) AS SIGNED) AS online_members
            FROM `guild` g
            LEFT JOIN `guild_member` gm ON gm.guild_id = g.guild_id
            LEFT JOIN `char` ON `char`.`char_id` = gm.char_id
            LEFT JOIN `login` l ON l.account_id = `char`.`account_id` AND l.group_id < ?
            WHERE g.name = ?
            GROUP BY
                g.guild_id,
                g.name,
                g.master,
                g.guild_lv,
                g.max_member,
                g.average_lv,
                g.exp,
                g.next_exp
            LIMIT 1
            "#;
const MOB_RACE_COLUMNS: &[&str] = &["race", "Race"];
const MOB_ELEMENT_COLUMNS: &[&str] = &["element", "Element"];
const MOB_SIZE_COLUMNS: &[&str] = &["scale", "size", "Scale"];
const MOB_MVP_EXP_COLUMNS: &[&str] = &["mvp_exp", "mvpexp", "mexp"];
const MVP_LOG_EMPTY_MESSAGE: &str = "Aucun MVP n’a encore été enregistré dans les logs.";
const MVP_LOG_KILLER_ID_COLUMNS: &[&str] =
    &["kill_char_id", "killer_char_id", "char_id", "src_charid"];
const MVP_LOG_KILLER_NAME_COLUMNS: &[&str] = &["killer_name", "char_name", "name"];
const MVP_LOG_DATE_COLUMNS: &[&str] = &["mvp_date", "time", "date", "logtime", "atcommand_date"];

#[derive(Debug, Clone)]
struct AvailableColumns {
    names: Vec<String>,
}

impl AvailableColumns {
    fn first(&self, candidates: &[&str]) -> Option<String> {
        candidates.iter().find_map(|candidate| {
            self.names
                .iter()
                .find(|name| name.eq_ignore_ascii_case(candidate))
                .cloned()
        })
    }

    fn all(&self, candidates: &[&str]) -> Vec<String> {
        let mut matches = Vec::new();

        for candidate in candidates {
            if let Some(name) = self.first(&[*candidate]) {
                if !matches
                    .iter()
                    .any(|existing: &String| existing.eq_ignore_ascii_case(&name))
                {
                    matches.push(name);
                }
            }
        }

        matches
    }
}

#[derive(Debug, Clone)]
struct ItemSearchColumns {
    id: String,
    aegis_name: String,
    display_name: String,
    item_type: Option<String>,
    searchable_names: Vec<String>,
}

#[derive(Debug, Clone)]
struct MonsterSearchColumns {
    id: String,
    sprite: Option<String>,
    display_name: String,
    level: Option<String>,
    hp: Option<String>,
    searchable_names: Vec<String>,
}

#[derive(Debug, Clone)]
struct MvpLogColumns {
    killer_id: Option<String>,
    killer_name: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Clone)]
struct MvpTimerRow {
    monster_id: i64,
    monster_name: String,
    map_name: String,
    respawn_minutes: i64,
    respawn_variance_minutes: i64,
    last_kill_ts: Option<i64>,
    earliest_spawn_ts: Option<i64>,
    latest_spawn_ts: Option<i64>,
    spawn_state: String,
}

fn cast_column_as_char(column_name: &str) -> String {
    format!("CAST({} AS CHAR)", quote_identifier(column_name))
}

fn cast_column_as_signed(column_name: &str) -> String {
    format!("CAST({} AS SIGNED)", quote_identifier(column_name))
}

fn qualified_column(table_ref: &str, column_name: &str) -> String {
    format!("{}.{}", table_ref, quote_identifier(column_name))
}

fn cast_qualified_column_as_char(table_ref: &str, column_name: &str) -> String {
    format!("CAST({} AS CHAR)", qualified_column(table_ref, column_name))
}

fn cast_qualified_column_as_signed(table_ref: &str, column_name: &str) -> String {
    format!(
        "CAST({} AS SIGNED)",
        qualified_column(table_ref, column_name)
    )
}

fn exact_conditions(column_names: &[String]) -> String {
    column_names
        .iter()
        .map(|column_name| format!("{} = ?", cast_column_as_char(column_name)))
        .collect::<Vec<_>>()
        .join(" OR ")
}

fn like_conditions(column_names: &[String]) -> String {
    column_names
        .iter()
        .map(|column_name| format!("{} LIKE ?", cast_column_as_char(column_name)))
        .collect::<Vec<_>>()
        .join(" OR ")
}

fn parse_search_id(query: &str) -> Option<i64> {
    let trimmed = query.trim();

    if trimmed.is_empty() || !trimmed.chars().all(|character| character.is_ascii_digit()) {
        return None;
    }

    trimmed.parse::<i64>().ok().filter(|value| *value > 0)
}

fn optional_signed_select(columns: &AvailableColumns, candidates: &[&str], alias: &str) -> String {
    columns
        .first(candidates)
        .map(|column| format!("{} AS {alias}", cast_column_as_signed(&column)))
        .unwrap_or_else(|| format!("NULL AS {alias}"))
}

fn optional_char_select(columns: &AvailableColumns, candidates: &[&str], alias: &str) -> String {
    columns
        .first(candidates)
        .map(|column| format!("{} AS {alias}", cast_column_as_char(&column)))
        .unwrap_or_else(|| format!("NULL AS {alias}"))
}

fn find_column_dynamic(columns: &AvailableColumns, candidates: &[String]) -> Option<String> {
    candidates.iter().find_map(|candidate| {
        columns
            .names
            .iter()
            .find(|name| name.eq_ignore_ascii_case(candidate))
            .cloned()
    })
}

#[cfg(test)]
fn mvp_drop_id_columns(columns: &AvailableColumns) -> Vec<String> {
    let mut matches = Vec::new();

    for index in 1..=3 {
        let id_candidates = vec![
            format!("MvpDrop{index}id"),
            format!("MVPDrop{index}id"),
            format!("MvpDrop{index}ID"),
            format!("mvpdrop{index}id"),
            format!("mvp_drop{index}_id"),
            format!("mvpdrop{index}_item"),
            format!("mvp_drop{index}_item"),
        ];

        if let Some(id_column) = find_column_dynamic(columns, &id_candidates) {
            matches.push(id_column);
        }
    }

    matches
}

fn mvp_log_columns(columns: &AvailableColumns) -> MvpLogColumns {
    MvpLogColumns {
        killer_id: columns.first(MVP_LOG_KILLER_ID_COLUMNS),
        killer_name: columns.first(MVP_LOG_KILLER_NAME_COLUMNS),
        date: columns.first(MVP_LOG_DATE_COLUMNS),
    }
}

fn mvp_killer_name_expression(columns: &MvpLogColumns, can_join_char: bool) -> String {
    let fallback_from_log_name = columns
        .killer_name
        .as_ref()
        .map(|column| {
            format!(
                "NULLIF({}, '')",
                cast_qualified_column_as_char("ml", column)
            )
        })
        .unwrap_or_else(|| "NULL".to_string());

    let fallback_from_id = columns
        .killer_id
        .as_ref()
        .map(|column| {
            format!(
                "CONCAT('Personnage #', {})",
                cast_qualified_column_as_char("ml", column)
            )
        })
        .unwrap_or_else(|| "NULL".to_string());

    if can_join_char {
        format!(
            "COALESCE(NULLIF(`char`.`name`, ''), {fallback_from_log_name}, {fallback_from_id}, 'Tueur inconnu')"
        )
    } else {
        format!("COALESCE({fallback_from_log_name}, {fallback_from_id}, 'Tueur inconnu')")
    }
}

fn drop_column_pairs(columns: &AvailableColumns) -> Vec<(String, Option<String>, String)> {
    let mut pairs = Vec::new();

    for index in 1..=10 {
        let id_candidates = vec![
            format!("Drop{index}id"),
            format!("Drop{index}ID"),
            format!("drop{index}id"),
            format!("drop{index}_id"),
            format!("drop{index}_item"),
        ];
        let rate_candidates = vec![
            format!("Drop{index}per"),
            format!("Drop{index}rate"),
            format!("drop{index}per"),
            format!("drop{index}_rate"),
        ];

        if let Some(id_column) = find_column_dynamic(columns, &id_candidates) {
            pairs.push((
                id_column,
                find_column_dynamic(columns, &rate_candidates),
                format!("Drop {index}"),
            ));
        }
    }

    for index in 1..=3 {
        let id_candidates = vec![
            format!("MvpDrop{index}id"),
            format!("MVPDrop{index}id"),
            format!("MvpDrop{index}ID"),
            format!("mvpdrop{index}id"),
            format!("mvp_drop{index}_id"),
            format!("mvpdrop{index}_item"),
            format!("mvp_drop{index}_item"),
        ];
        let rate_candidates = vec![
            format!("MvpDrop{index}per"),
            format!("MVPDrop{index}per"),
            format!("mvpdrop{index}per"),
            format!("mvpdrop{index}_rate"),
            format!("mvp_drop{index}_rate"),
        ];

        if let Some(id_column) = find_column_dynamic(columns, &id_candidates) {
            pairs.push((
                id_column,
                find_column_dynamic(columns, &rate_candidates),
                format!("MVP drop {index}"),
            ));
        }
    }

    pairs
}

fn drop_item_match_condition(column_name: &str) -> String {
    format!(
        "(BINARY {} = BINARY ? OR {} = ?)",
        cast_column_as_char(column_name),
        cast_column_as_signed(column_name)
    )
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MonsterRateKind {
    Normal,
    Boss,
    Mvp,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ItemDropCategory {
    Common,
    Heal,
    Use,
    Equip,
    Card,
}

fn monster_rate_kind(
    mode_mvp: Option<i64>,
    monster_class: Option<&str>,
    mvp_exp: Option<i64>,
) -> MonsterRateKind {
    if mode_mvp.is_some_and(|value| value > 0) || mvp_exp.is_some_and(|value| value > 0) {
        MonsterRateKind::Mvp
    } else if monster_class.is_some_and(|value| value.eq_ignore_ascii_case("Boss")) {
        MonsterRateKind::Boss
    } else {
        MonsterRateKind::Normal
    }
}

fn item_drop_category(item_type: &str) -> ItemDropCategory {
    match item_type {
        "Healing" => ItemDropCategory::Heal,
        "Usable" | "Cash" => ItemDropCategory::Use,
        "Weapon" | "Armor" | "PetArmor" => ItemDropCategory::Equip,
        "Card" => ItemDropCategory::Card,
        _ => ItemDropCategory::Common,
    }
}

fn rate_for_monster(rate: DropRateSet, monster_kind: MonsterRateKind) -> u32 {
    match monster_kind {
        MonsterRateKind::Normal => rate.normal,
        MonsterRateKind::Boss => rate.boss,
        MonsterRateKind::Mvp => rate.mvp,
    }
}

fn drop_rate_parameters(
    rates: &ServerRatesConfig,
    category: ItemDropCategory,
    monster_kind: MonsterRateKind,
    is_mvp_reward: bool,
) -> (u32, DropRateBounds) {
    if is_mvp_reward {
        return (rates.item_rate_mvp, rates.item_drop_mvp);
    }

    match category {
        ItemDropCategory::Common => (
            rate_for_monster(rates.item_rate_common, monster_kind),
            rates.item_drop_common,
        ),
        ItemDropCategory::Heal => (
            rate_for_monster(rates.item_rate_heal, monster_kind),
            rates.item_drop_heal,
        ),
        ItemDropCategory::Use => (
            rate_for_monster(rates.item_rate_use, monster_kind),
            rates.item_drop_use,
        ),
        ItemDropCategory::Equip => (
            rate_for_monster(rates.item_rate_equip, monster_kind),
            rates.item_drop_equip,
        ),
        ItemDropCategory::Card => (
            rate_for_monster(rates.item_rate_card, monster_kind),
            rates.item_drop_card,
        ),
    }
}

fn adjusted_drop_rate(
    base_rate: i64,
    rate_adjust: u32,
    bounds: DropRateBounds,
    logarithmic: bool,
) -> u32 {
    if rate_adjust == 0 {
        return 0;
    }

    let base_rate = base_rate.max(0) as f64;
    let adjusted = if logarithmic && rate_adjust > 0 && rate_adjust != 100 && base_rate > 0.0 {
        base_rate * (5.0 - base_rate.log10()).powf((rate_adjust as f64 / 100.0).ln() / 5.0_f64.ln())
            + 0.5
    } else {
        base_rate * rate_adjust as f64 / 100.0
    };

    (adjusted as u32).clamp(bounds.min, bounds.max)
}

fn server_drop_rate(
    base_rate: Option<i64>,
    item_type: &str,
    monster_kind: MonsterRateKind,
    is_mvp_reward: bool,
    rates: &ServerRatesConfig,
) -> Option<f64> {
    let mut base_rate = base_rate.filter(|value| *value > 0)?;

    if !rates.configured || rates.item_ratio_overrides || (item_type.is_empty() && !is_mvp_reward) {
        return None;
    }
    if rates.drop_rate_increase && !is_mvp_reward && base_rate < 5_000 {
        base_rate += 1;
    }

    let (rate_adjust, bounds) = drop_rate_parameters(
        rates,
        item_drop_category(item_type),
        monster_kind,
        is_mvp_reward,
    );
    let adjusted = adjusted_drop_rate(base_rate, rate_adjust, bounds, rates.logarithmic_drops);

    Some(adjusted as f64 / 100.0)
}

fn apply_exp_rate(base_exp: i64, rate: u32) -> i64 {
    ((base_exp.max(0) as i128 * rate as i128) / 100).min(i64::MAX as i128) as i64
}

fn format_number_fr(value: i64) -> String {
    let raw = value.to_string();
    let mut formatted = String::with_capacity(raw.len() + raw.len() / 3);

    for (index, character) in raw.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            formatted.push(' ');
        }
        formatted.push(character);
    }

    formatted.chars().rev().collect()
}

fn is_sensitive_column(column_name: &str) -> bool {
    let lower = column_name.to_ascii_lowercase();
    lower.contains("password")
        || lower.contains("user_pass")
        || lower.contains("hash")
        || lower == "email"
}

fn mask_sensitive_value(column_name: &str, value: String) -> String {
    if !column_name.to_ascii_lowercase().contains("ip") {
        return value;
    }

    let mut parts = value.split('.').collect::<Vec<_>>();
    if parts.len() == 4 {
        parts[3] = "x";
        return parts.join(".");
    }

    "ip masquée".to_string()
}

fn join_limited_lines(lines: Vec<String>, empty_message: &str) -> Vec<String> {
    if lines.is_empty() {
        vec![empty_message.to_string()]
    } else {
        lines
    }
}

fn discord_relative_timestamp(timestamp: Option<i64>) -> String {
    match timestamp {
        Some(value) => format!("<t:{value}:R>"),
        None => "Non disponible".to_string(),
    }
}

fn format_mvp_spawn_state(state: &str) -> &'static str {
    match state {
        "waiting" => "En attente",
        "window" => "Fenêtre de respawn ouverte",
        "available" => "Disponible probable",
        "unknown" => "Timer inconnu",
        _ => "Statut inconnu",
    }
}

fn format_mvp_respawn_duration(respawn_minutes: i64, variance_minutes: i64) -> String {
    if respawn_minutes > 0 && variance_minutes > 0 {
        format!("{respawn_minutes} min ± {variance_minutes} min")
    } else if respawn_minutes > 0 {
        format!("{respawn_minutes} min")
    } else {
        "Respawn inconnu".to_string()
    }
}

fn format_mvp_timer_line(timer: &MvpTimerRow) -> String {
    let header = format!(
        "**{}**\nID : `{}`\nCarte : `{}`",
        timer.monster_name, timer.monster_id, timer.map_name
    );
    let last_kill = discord_relative_timestamp(timer.last_kill_ts);
    let status = format_mvp_spawn_state(&timer.spawn_state);

    match timer.spawn_state.as_str() {
        "unknown" => format!(
            "{header}\nDernier kill : Aucun log connu\nRespawn : {}\nStatut : {status}, MVP peut être disponible",
            format_mvp_respawn_duration(
                timer.respawn_minutes,
                timer.respawn_variance_minutes
            )
        ),
        "waiting" => format!(
            "{header}\nDernier kill : {last_kill}\nRespawn au plus tôt : {}\nRespawn au plus tard : {}\nStatut : {status}",
            discord_relative_timestamp(timer.earliest_spawn_ts),
            discord_relative_timestamp(timer.latest_spawn_ts)
        ),
        "window" => format!(
            "{header}\nDernier kill : {last_kill}\nFenêtre ouverte depuis : {}\nRespawn maximum : {}\nStatut : {status}",
            discord_relative_timestamp(timer.earliest_spawn_ts),
            discord_relative_timestamp(timer.latest_spawn_ts)
        ),
        "available" => format!(
            "{header}\nDernier kill : {last_kill}\nRespawn maximum dépassé depuis : {}\nStatut : {status}",
            discord_relative_timestamp(timer.latest_spawn_ts)
        ),
        _ => format!(
            "{header}\nDernier kill : {last_kill}\nRespawn au plus tôt : {}\nRespawn au plus tard : {}\nStatut : {status}",
            discord_relative_timestamp(timer.earliest_spawn_ts),
            discord_relative_timestamp(timer.latest_spawn_ts)
        ),
    }
}

fn mvp_timer_row_from_row(row: MySqlRow) -> Result<MvpTimerRow> {
    let monster_id = row.try_get::<i64, _>("monster_id")?;
    let monster_name = row
        .try_get::<Option<String>, _>("monster_name")?
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| format!("MVP #{monster_id}"));
    let map_name = row
        .try_get::<Option<String>, _>("map_name")?
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "inconnue".to_string());

    Ok(MvpTimerRow {
        monster_id,
        monster_name,
        map_name,
        respawn_minutes: row
            .try_get::<Option<i64>, _>("respawn_minutes")?
            .unwrap_or_default(),
        respawn_variance_minutes: row
            .try_get::<Option<i64>, _>("respawn_variance_minutes")?
            .unwrap_or_default(),
        last_kill_ts: row.try_get("last_kill_ts")?,
        earliest_spawn_ts: row.try_get("earliest_spawn_ts")?,
        latest_spawn_ts: row.try_get("latest_spawn_ts")?,
        spawn_state: row
            .try_get::<Option<String>, _>("spawn_state")?
            .unwrap_or_else(|| "unknown".to_string()),
    })
}

fn mvp_kill_entry_from_row(row: MySqlRow) -> Result<MvpKillEntry> {
    let killer_id = row.try_get::<i64, _>("kill_char_id")?;
    let monster_id = row.try_get::<i64, _>("monster_id")?;
    let prize_id = row.try_get::<i64, _>("prize")?;

    Ok(MvpKillEntry {
        mvp_date: row
            .try_get::<Option<String>, _>("mvp_date")?
            .filter(|value| !value.trim().is_empty()),
        mvp_timestamp: row.try_get("mvp_ts")?,
        killer_id,
        killer_name: row
            .try_get::<Option<String>, _>("killer_name")?
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("Personnage #{killer_id}")),
        monster_id,
        monster_name: row
            .try_get::<Option<String>, _>("monster_name")?
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("MVP #{monster_id}")),
        monster_aegis_name: row
            .try_get::<Option<String>, _>("monster_aegis_name")?
            .filter(|value| !value.trim().is_empty()),
        map: row
            .try_get::<Option<String>, _>("map_name")?
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Carte inconnue".to_string()),
        mvp_exp: row.try_get("mvp_exp")?,
        prize_id,
        prize_name: row
            .try_get::<Option<String>, _>("prize_name")?
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| format!("Item #{prize_id}")),
        prize_aegis_name: row
            .try_get::<Option<String>, _>("prize_aegis_name")?
            .filter(|value| !value.trim().is_empty()),
    })
}

fn item_search_entry_from_row(row: MySqlRow) -> Result<ItemSearchEntry> {
    let item_id = row.try_get("item_id")?;
    let display_name = row
        .try_get::<Option<String>, _>("item_display_name")?
        .unwrap_or_default();
    let aegis_name = row
        .try_get::<Option<String>, _>("item_aegis_name")?
        .unwrap_or_default();
    let display_name = if display_name.trim().is_empty() {
        aegis_name.clone()
    } else {
        display_name
    };
    let display_name = if display_name.trim().is_empty() {
        format!("Objet {item_id}")
    } else {
        display_name
    };
    let item_type = row
        .try_get::<Option<String>, _>("item_type")?
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "inconnu".to_string());
    let aegis_name = if aegis_name.trim().is_empty() {
        display_name.clone()
    } else {
        aegis_name
    };

    Ok(ItemSearchEntry {
        item_id,
        aegis_name,
        display_name,
        item_type,
    })
}

fn monster_search_entry_from_row(row: MySqlRow) -> Result<MonsterSearchEntry> {
    let monster_id = row.try_get("monster_id")?;
    let sprite = row
        .try_get::<Option<String>, _>("monster_sprite")?
        .unwrap_or_default();
    let display_name = row
        .try_get::<Option<String>, _>("monster_display_name")?
        .unwrap_or_default();
    let display_name = if !display_name.trim().is_empty() {
        display_name
    } else if !sprite.trim().is_empty() {
        sprite.clone()
    } else {
        format!("Monstre {monster_id}")
    };
    let sprite = if sprite.trim().is_empty() {
        display_name.clone()
    } else {
        sprite
    };

    Ok(MonsterSearchEntry {
        monster_id,
        sprite,
        display_name,
        level: row.try_get("monster_level")?,
        hp: row.try_get("monster_hp")?,
        source_table: row.try_get("source_table")?,
    })
}

fn database_engine_name(version: &str) -> String {
    let lower = version.to_ascii_lowercase();

    if lower.contains("mariadb") {
        "MariaDB".to_string()
    } else if lower.contains("mysql") {
        "MySQL".to_string()
    } else {
        "MariaDB".to_string()
    }
}

fn quote_identifier(identifier: &str) -> String {
    format!("`{}`", identifier.replace('`', "``"))
}

fn parse_account_manage_i64(value: &str, field: &str) -> Result<i64> {
    let parsed = value
        .trim()
        .parse::<i64>()
        .with_context(|| format!("Valeur invalide pour `login`.`{field}`"))?;
    if parsed < 0 {
        anyhow::bail!("La valeur de `login`.`{field}` doit être positive ou nulle.");
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests;
