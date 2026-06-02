use crate::config::{AccountPasswordMode, DatabaseConfig};
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, mysql::MySqlRow, MySqlPool, Row};
use std::time::Duration;

#[derive(Clone)]
pub struct RAthenaFrDatabase {
    pool: MySqlPool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DatabaseTable {
    BuyingStoreItems,
    BuyingStores,
    CartInventory,
    Char,
    CharLog,
    CharRegNum,
    CharRegStr,
    ChatLog,
    AtCommandLog,
    BranchLog,
    LoginLog,
    MvpLog,
    PickLog,
    ZenyLog,
    AccRegNum,
    AccRegStr,
    Login,
    Guild,
    GuildCastle,
    GuildMember,
    GuildPosition,
    GuildSkill,
    GuildStorage,
    Homunculus,
    Inventory,
    ItemDb,
    ItemDbRe,
    Mail,
    MailAttachments,
    MobDb,
    MobDbRe,
    MobSkillDb,
    Party,
    Pet,
    Quest,
    Skill,
    Storage,
    SqlUpdates,
    VendingItems,
    Vendings,
}

impl DatabaseTable {
    pub const fn name(self) -> &'static str {
        match self {
            Self::BuyingStoreItems => "buyingstore_items",
            Self::BuyingStores => "buyingstores",
            Self::CartInventory => "cart_inventory",
            Self::Char => "char",
            Self::CharLog => "charlog",
            Self::CharRegNum => "char_reg_num",
            Self::CharRegStr => "char_reg_str",
            Self::ChatLog => "chatlog",
            Self::AtCommandLog => "atcommandlog",
            Self::BranchLog => "branchlog",
            Self::LoginLog => "loginlog",
            Self::MvpLog => "mvplog",
            Self::PickLog => "picklog",
            Self::ZenyLog => "zenylog",
            Self::AccRegNum => "acc_reg_num",
            Self::AccRegStr => "acc_reg_str",
            Self::Login => "login",
            Self::Guild => "guild",
            Self::GuildCastle => "guild_castle",
            Self::GuildMember => "guild_member",
            Self::GuildPosition => "guild_position",
            Self::GuildSkill => "guild_skill",
            Self::GuildStorage => "guild_storage",
            Self::Homunculus => "homunculus",
            Self::Inventory => "inventory",
            Self::ItemDb => "item_db",
            Self::ItemDbRe => "item_db_re",
            Self::Mail => "mail",
            Self::MailAttachments => "mail_attachments",
            Self::MobDb => "mob_db",
            Self::MobDbRe => "mob_db_re",
            Self::MobSkillDb => "mob_skill_db",
            Self::Party => "party",
            Self::Pet => "pet",
            Self::Quest => "quest",
            Self::Skill => "skill",
            Self::Storage => "storage",
            Self::SqlUpdates => "sql_updates",
            Self::VendingItems => "vending_items",
            Self::Vendings => "vendings",
        }
    }
}

use crate::rathenafr::models::*;

const ITEM_SEARCH_TABLES: &[&str] = &["item_db", "item_db_re"];
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
const MONSTER_SPRITE_COLUMN_CANDIDATES: &[&str] = &["sprite"];
const MONSTER_DISPLAY_COLUMN_CANDIDATES: &[&str] = &[
    "iROName",
    "iro_name",
    "name_english",
    "name",
    "kROName",
    "kro_name",
    "name_japanese",
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
const ITEM_DETAIL_BUY_COLUMNS: &[&str] = &["buy", "price_buy"];
const ITEM_DETAIL_SELL_COLUMNS: &[&str] = &["sell", "price_sell"];
const ITEM_DETAIL_WEIGHT_COLUMNS: &[&str] = &["weight"];
const ITEM_DETAIL_ATTACK_COLUMNS: &[&str] = &["atk", "attack"];
const ITEM_DETAIL_DEFENSE_COLUMNS: &[&str] = &["def", "defense"];
const ITEM_DETAIL_SLOTS_COLUMNS: &[&str] = &["slots", "slot"];
const MOB_RACE_COLUMNS: &[&str] = &["race", "Race"];
const MOB_ELEMENT_COLUMNS: &[&str] = &["element", "Element"];
const MOB_SIZE_COLUMNS: &[&str] = &["scale", "size", "Scale"];
const MOB_MVP_EXP_COLUMNS: &[&str] = &["mexp", "MEXP", "mvp_exp", "MvpExp"];

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

fn cast_column_as_char(column_name: &str) -> String {
    format!("CAST({} AS CHAR)", quote_identifier(column_name))
}

fn cast_column_as_signed(column_name: &str) -> String {
    format!("CAST({} AS SIGNED)", quote_identifier(column_name))
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

fn drop_column_pairs(columns: &AvailableColumns) -> Vec<(String, Option<String>, String)> {
    let mut pairs = Vec::new();

    for index in 1..=10 {
        let id_candidates = vec![
            format!("Drop{index}id"),
            format!("Drop{index}ID"),
            format!("drop{index}id"),
            format!("drop{index}_id"),
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
        ];
        let rate_candidates = vec![
            format!("MvpDrop{index}per"),
            format!("MVPDrop{index}per"),
            format!("mvpdrop{index}per"),
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

fn format_drop_rate(rate: Option<i64>) -> String {
    match rate {
        Some(value) if value > 0 => format!("{} ({:.2}%)", value, value as f64 / 100.0),
        Some(value) => value.to_string(),
        None => "taux inconnu".to_string(),
    }
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
        source_table: row.try_get("source_table")?,
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

impl RAthenaFrDatabase {
    pub async fn connect(config: &DatabaseConfig) -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout_seconds))
            .connect(&config.connection_url())
            .await
            .context("connexion à la base rAthenaFR")?;

        Ok(Self { pool })
    }

    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .context("ping de la base rAthenaFR")?;
        Ok(())
    }

    pub async fn first_missing_table(
        &self,
        tables: &[DatabaseTable],
    ) -> Result<Option<DatabaseTable>> {
        for table in tables {
            if !self.table_exists(table.name()).await? {
                return Ok(Some(*table));
            }
        }

        Ok(None)
    }

    pub async fn table_exists(&self, table_name: &str) -> Result<bool> {
        let row = sqlx::query(
            r#"
            SELECT CAST(COUNT(*) AS SIGNED) AS table_count
            FROM information_schema.tables
            WHERE table_schema = DATABASE()
              AND table_name = ?
            "#,
        )
        .bind(table_name)
        .fetch_one(&self.pool)
        .await
        .context("vérification de disponibilité des tables rAthenaFR")?;

        let count: i64 = row.try_get("table_count")?;
        Ok(count > 0)
    }

    async fn table_columns(&self, table_name: &str) -> Result<Option<AvailableColumns>> {
        if !self.table_exists(table_name).await? {
            return Ok(None);
        }

        let rows = sqlx::query(
            r#"
            SELECT column_name
            FROM information_schema.columns
            WHERE table_schema = DATABASE()
              AND table_name = ?
            ORDER BY ordinal_position ASC
            "#,
        )
        .bind(table_name)
        .fetch_all(&self.pool)
        .await
        .with_context(|| format!("lecture des colonnes de la table rAthena {table_name}"))?;

        let names = rows
            .into_iter()
            .map(|row| row.try_get("column_name").map_err(Into::into))
            .collect::<Result<Vec<String>>>()?;

        Ok(Some(AvailableColumns { names }))
    }

    async fn item_search_columns(&self, table_name: &str) -> Result<Option<ItemSearchColumns>> {
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(None);
        };
        let Some(id) = columns.first(&["id"]) else {
            return Ok(None);
        };
        let Some(display_name) = columns.first(ITEM_DISPLAY_COLUMN_CANDIDATES) else {
            return Ok(None);
        };

        let aegis_name = columns
            .first(ITEM_AEGIS_COLUMN_CANDIDATES)
            .unwrap_or_else(|| display_name.clone());

        Ok(Some(ItemSearchColumns {
            id,
            aegis_name,
            display_name,
            item_type: columns.first(ITEM_TYPE_COLUMN_CANDIDATES),
            searchable_names: columns.all(ITEM_DISPLAY_COLUMN_CANDIDATES),
        }))
    }

    async fn monster_search_columns(
        &self,
        table_name: &str,
    ) -> Result<Option<MonsterSearchColumns>> {
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(None);
        };
        let Some(id) = columns.first(&["id"]) else {
            return Ok(None);
        };
        let Some(display_name) = columns.first(MONSTER_DISPLAY_COLUMN_CANDIDATES) else {
            return Ok(None);
        };

        Ok(Some(MonsterSearchColumns {
            id,
            sprite: columns.first(MONSTER_SPRITE_COLUMN_CANDIDATES),
            display_name,
            level: columns.first(MONSTER_LEVEL_COLUMN_CANDIDATES),
            hp: columns.first(MONSTER_HP_COLUMN_CANDIDATES),
            searchable_names: columns.all(MONSTER_DISPLAY_COLUMN_CANDIDATES),
        }))
    }

    pub async fn database_status(&self, group_threshold: i32) -> Result<DatabaseStatus> {
        let row = sqlx::query(
            r#"
            SELECT
                DATABASE() AS database_name,
                VERSION() AS database_version,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `char` c
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE c.online = 1 AND l.group_id < ?
                ) AS online_characters,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `char` c
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE l.group_id < ?
                ) AS characters,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `login`
                    WHERE group_id < ?
                ) AS accounts,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `guild`
                ) AS guilds
            "#,
        )
        .bind(group_threshold)
        .bind(group_threshold)
        .bind(group_threshold)
        .fetch_one(&self.pool)
        .await
        .context("récupération du statut de la base rAthenaFR")?;

        let database_version: String = row.try_get("database_version")?;

        Ok(DatabaseStatus {
            database_name: row.try_get("database_name")?,
            database_engine: database_engine_name(&database_version),
            online_characters: row.try_get("online_characters")?,
            characters: row.try_get("characters")?,
            accounts: row.try_get("accounts")?,
            guilds: row.try_get("guilds")?,
        })
    }

    pub async fn online_characters(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<CharacterSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                c.last_map
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE c.online = 1 AND l.group_id < ?
            ORDER BY c.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des personnages connectés")?;

        rows.into_iter()
            .map(|row| {
                Ok(CharacterSummary {
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    map: row.try_get("last_map")?,
                })
            })
            .collect()
    }

    pub async fn top_characters(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<RankingEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                c.last_map
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            ORDER BY c.base_level DESC, c.job_level DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du classement des personnages")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(RankingEntry {
                    rank: index + 1,
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    map: row.try_get("last_map")?,
                })
            })
            .collect()
    }

    pub async fn top_zeny(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<ZenyRankingEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.zeny AS SIGNED) AS zeny
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            ORDER BY c.zeny DESC, c.base_level DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du classement zeny")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(ZenyRankingEntry {
                    rank: index + 1,
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    zeny: row.try_get("zeny")?,
                })
            })
            .collect()
    }

    pub async fn search_items(&self, query: &str, limit: u32) -> Result<Vec<ItemSearchEntry>> {
        let mut entries = Vec::new();
        let exact_id = parse_search_id(query);

        for table_name in ITEM_SEARCH_TABLES {
            let Some(columns) = self.item_search_columns(table_name).await? else {
                continue;
            };

            let table_entries = self
                .search_items_in_table(table_name, &columns, query, exact_id, limit)
                .await?;
            entries.extend(table_entries);

            if exact_id.is_some() && !entries.is_empty() {
                entries.truncate(1);
                break;
            }

            if entries.len() >= limit as usize {
                entries.truncate(limit as usize);
                break;
            }
        }

        Ok(entries)
    }

    async fn search_items_in_table(
        &self,
        table_name: &str,
        columns: &ItemSearchColumns,
        query: &str,
        exact_id: Option<i64>,
        limit: u32,
    ) -> Result<Vec<ItemSearchEntry>> {
        if let Some(item_id) = exact_id {
            return self
                .search_items_in_table_by_id(table_name, columns, item_id)
                .await;
        }

        let pattern = format!("%{}%", query);
        let prefix = format!("{}%", query);
        let table_identifier = quote_identifier(table_name);
        let id_as_char = cast_column_as_char(&columns.id);
        let name_like_conditions = like_conditions(&columns.searchable_names);
        let exact_name_conditions = exact_conditions(&columns.searchable_names);
        let prefix_name_conditions = like_conditions(&columns.searchable_names);
        let where_clause = if name_like_conditions.is_empty() {
            format!("{id_as_char} = ?")
        } else {
            format!("{id_as_char} = ? OR {name_like_conditions}")
        };
        let exact_rank = if exact_name_conditions.is_empty() {
            "FALSE".to_string()
        } else {
            exact_name_conditions
        };
        let prefix_rank = if prefix_name_conditions.is_empty() {
            "FALSE".to_string()
        } else {
            prefix_name_conditions
        };
        let item_type_expression = columns
            .item_type
            .as_deref()
            .map(cast_column_as_char)
            .unwrap_or_else(|| "NULL".to_string());
        let sql = format!(
            r#"
            SELECT
                ? AS source_table,
                {} AS item_id,
                {} AS item_aegis_name,
                {} AS item_display_name,
                {} AS item_type
            FROM {table_identifier}
            WHERE {where_clause}
            ORDER BY
                CASE
                    WHEN {id_as_char} = ? THEN 0
                    WHEN {exact_rank} THEN 1
                    WHEN {prefix_rank} THEN 2
                    ELSE 3
                END,
                {} ASC
            LIMIT ?
            "#,
            cast_column_as_signed(&columns.id),
            cast_column_as_char(&columns.aegis_name),
            cast_column_as_char(&columns.display_name),
            item_type_expression,
            quote_identifier(&columns.id),
        );

        let mut sql_query = sqlx::query(&sql).bind(table_name).bind(query);

        for _ in &columns.searchable_names {
            sql_query = sql_query.bind(&pattern);
        }

        sql_query = sql_query.bind(query);

        for _ in &columns.searchable_names {
            sql_query = sql_query.bind(query);
        }

        for _ in &columns.searchable_names {
            sql_query = sql_query.bind(&prefix);
        }

        let rows = sql_query
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("recherche d’items dans {table_name}"))?;

        rows.into_iter().map(item_search_entry_from_row).collect()
    }

    async fn search_items_in_table_by_id(
        &self,
        table_name: &str,
        columns: &ItemSearchColumns,
        item_id: i64,
    ) -> Result<Vec<ItemSearchEntry>> {
        let table_identifier = quote_identifier(table_name);
        let item_type_expression = columns
            .item_type
            .as_deref()
            .map(cast_column_as_char)
            .unwrap_or_else(|| "NULL".to_string());
        let sql = format!(
            r#"
            SELECT
                ? AS source_table,
                {} AS item_id,
                {} AS item_aegis_name,
                {} AS item_display_name,
                {} AS item_type
            FROM {table_identifier}
            WHERE {} = ?
            ORDER BY {} ASC
            LIMIT 1
            "#,
            cast_column_as_signed(&columns.id),
            cast_column_as_char(&columns.aegis_name),
            cast_column_as_char(&columns.display_name),
            item_type_expression,
            cast_column_as_signed(&columns.id),
            quote_identifier(&columns.id),
        );

        let rows = sqlx::query(&sql)
            .bind(table_name)
            .bind(item_id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("recherche de l’item ID {item_id} dans {table_name}"))?;

        rows.into_iter().map(item_search_entry_from_row).collect()
    }

    pub async fn search_monsters(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<MonsterSearchEntry>> {
        let mut entries = Vec::new();
        let exact_id = parse_search_id(query);

        for table_name in MOB_SEARCH_TABLES {
            let Some(columns) = self.monster_search_columns(table_name).await? else {
                continue;
            };

            let table_entries = self
                .search_monsters_in_table(table_name, &columns, query, exact_id, limit)
                .await?;
            entries.extend(table_entries);

            if exact_id.is_some() && !entries.is_empty() {
                entries.truncate(1);
                break;
            }

            if entries.len() >= limit as usize {
                entries.truncate(limit as usize);
                break;
            }
        }

        Ok(entries)
    }

    async fn search_monsters_in_table(
        &self,
        table_name: &str,
        columns: &MonsterSearchColumns,
        query: &str,
        exact_id: Option<i64>,
        limit: u32,
    ) -> Result<Vec<MonsterSearchEntry>> {
        if let Some(monster_id) = exact_id {
            return self
                .search_monsters_in_table_by_id(table_name, columns, monster_id)
                .await;
        }

        let pattern = format!("%{}%", query);
        let prefix = format!("{}%", query);
        let table_identifier = quote_identifier(table_name);
        let id_as_char = cast_column_as_char(&columns.id);
        let name_like_conditions = like_conditions(&columns.searchable_names);
        let exact_name_conditions = exact_conditions(&columns.searchable_names);
        let prefix_name_conditions = like_conditions(&columns.searchable_names);
        let where_clause = if name_like_conditions.is_empty() {
            format!("{id_as_char} = ?")
        } else {
            format!("{id_as_char} = ? OR {name_like_conditions}")
        };
        let exact_rank = if exact_name_conditions.is_empty() {
            "FALSE".to_string()
        } else {
            exact_name_conditions
        };
        let prefix_rank = if prefix_name_conditions.is_empty() {
            "FALSE".to_string()
        } else {
            prefix_name_conditions
        };
        let sprite_expression = columns
            .sprite
            .as_deref()
            .map(cast_column_as_char)
            .unwrap_or_else(|| cast_column_as_char(&columns.display_name));
        let level_expression = columns
            .level
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "0".to_string());
        let hp_expression = columns
            .hp
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "0".to_string());
        let sql = format!(
            r#"
            SELECT
                ? AS source_table,
                {} AS monster_id,
                {} AS monster_sprite,
                {} AS monster_display_name,
                {} AS monster_level,
                {} AS monster_hp
            FROM {table_identifier}
            WHERE {where_clause}
            ORDER BY
                CASE
                    WHEN {id_as_char} = ? THEN 0
                    WHEN {exact_rank} THEN 1
                    WHEN {prefix_rank} THEN 2
                    ELSE 3
                END,
                {} ASC
            LIMIT ?
            "#,
            cast_column_as_signed(&columns.id),
            sprite_expression,
            cast_column_as_char(&columns.display_name),
            level_expression,
            hp_expression,
            quote_identifier(&columns.id),
        );

        let mut sql_query = sqlx::query(&sql).bind(table_name).bind(query);

        for _ in &columns.searchable_names {
            sql_query = sql_query.bind(&pattern);
        }

        sql_query = sql_query.bind(query);

        for _ in &columns.searchable_names {
            sql_query = sql_query.bind(query);
        }

        for _ in &columns.searchable_names {
            sql_query = sql_query.bind(&prefix);
        }

        let rows = sql_query
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("recherche de monstres dans {table_name}"))?;

        rows.into_iter()
            .map(monster_search_entry_from_row)
            .collect()
    }

    async fn search_monsters_in_table_by_id(
        &self,
        table_name: &str,
        columns: &MonsterSearchColumns,
        monster_id: i64,
    ) -> Result<Vec<MonsterSearchEntry>> {
        let table_identifier = quote_identifier(table_name);
        let sprite_expression = columns
            .sprite
            .as_deref()
            .map(cast_column_as_char)
            .unwrap_or_else(|| cast_column_as_char(&columns.display_name));
        let level_expression = columns
            .level
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "0".to_string());
        let hp_expression = columns
            .hp
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "0".to_string());
        let sql = format!(
            r#"
            SELECT
                ? AS source_table,
                {} AS monster_id,
                {} AS monster_sprite,
                {} AS monster_display_name,
                {} AS monster_level,
                {} AS monster_hp
            FROM {table_identifier}
            WHERE {} = ?
            ORDER BY {} ASC
            LIMIT 1
            "#,
            cast_column_as_signed(&columns.id),
            sprite_expression,
            cast_column_as_char(&columns.display_name),
            level_expression,
            hp_expression,
            cast_column_as_signed(&columns.id),
            quote_identifier(&columns.id),
        );

        let rows = sqlx::query(&sql)
            .bind(table_name)
            .bind(monster_id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("recherche du monstre ID {monster_id} dans {table_name}"))?;

        rows.into_iter()
            .map(monster_search_entry_from_row)
            .collect()
    }

    pub async fn find_player(
        &self,
        group_threshold: i32,
        name: &str,
    ) -> Result<Option<PlayerProfile>> {
        let row = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.online AS SIGNED) AS online,
                c.last_map,
                CAST(c.zeny AS SIGNED) AS zeny,
                g.name AS guild_name
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            LEFT JOIN `guild` g ON g.guild_id = c.guild_id
            WHERE c.name = ? AND l.group_id < ?
            LIMIT 1
            "#,
        )
        .bind(name)
        .bind(group_threshold)
        .fetch_optional(&self.pool)
        .await
        .context("find player profile")?;

        match row {
            Some(row) => Ok(Some(PlayerProfile {
                name: row.try_get("name")?,
                class_id: row.try_get("class_id")?,
                base_level: row.try_get("base_level")?,
                job_level: row.try_get("job_level")?,
                online: row.try_get::<i32, _>("online")? == 1,
                map: row.try_get("last_map")?,
                zeny: row.try_get("zeny")?,
                guild_name: row.try_get("guild_name")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn top_guilds(&self, group_threshold: i32, limit: u32) -> Result<Vec<GuildSummary>> {
        let rows = sqlx::query(TOP_GUILDS_SQL)
            .bind(group_threshold)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .context("récupération du classement des guildes")?;

        rows.into_iter()
            .map(|row| {
                Ok(GuildSummary {
                    name: row.try_get("guild_name")?,
                    master: row.try_get("guild_master")?,
                    level: row.try_get("guild_level")?,
                    members: row.try_get("members")?,
                    online_members: row.try_get("online_members")?,
                    max_members: row.try_get("max_members")?,
                })
            })
            .collect()
    }

    pub async fn find_guild(
        &self,
        name: &str,
        group_threshold: i32,
    ) -> Result<Option<GuildDetails>> {
        let row = sqlx::query(FIND_GUILD_SQL)
            .bind(group_threshold)
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .context("find guild")?;

        match row {
            Some(row) => Ok(Some(GuildDetails {
                name: row.try_get("guild_name")?,
                master: row.try_get("guild_master")?,
                level: row.try_get("guild_level")?,
                members: row.try_get("members")?,
                online_members: row.try_get("online_members")?,
                max_members: row.try_get("max_members")?,
                average_level: row.try_get("average_level")?,
                exp: row.try_get("guild_exp")?,
                next_exp: row.try_get("next_exp")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn guild_members(
        &self,
        guild_name: &str,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<GuildMemberSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.online AS SIGNED) AS online,
                CAST(gm.position AS SIGNED) AS guild_position,
                c.last_map
            FROM `guild` g
            INNER JOIN `guild_member` gm ON gm.guild_id = g.guild_id
            INNER JOIN `char` c ON c.char_id = gm.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE g.name = ? AND l.group_id < ?
            ORDER BY c.online DESC, c.base_level DESC, c.job_level DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(guild_name)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des membres de guilde")?;

        rows.into_iter()
            .map(|row| {
                Ok(GuildMemberSummary {
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    online: row.try_get::<i32, _>("online")? == 1,
                    position: row.try_get("guild_position")?,
                    map: row.try_get("last_map")?,
                })
            })
            .collect()
    }

    pub async fn map_stats(
        &self,
        group_threshold: i32,
        online_only: bool,
        limit: u32,
    ) -> Result<Vec<MapStatsEntry>> {
        let online_only_value = if online_only { 1 } else { 0 };
        let rows = sqlx::query(
            r#"
            SELECT
                COALESCE(NULLIF(c.last_map, ''), 'unknown') AS map_name,
                CAST(COUNT(*) AS SIGNED) AS characters,
                CAST(SUM(CASE WHEN c.online = 1 THEN 1 ELSE 0 END) AS SIGNED) AS online_characters
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
              AND (? = 0 OR c.online = 1)
            GROUP BY COALESCE(NULLIF(c.last_map, ''), 'unknown')
            ORDER BY characters DESC, online_characters DESC, map_name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(online_only_value)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des statistiques de maps")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(MapStatsEntry {
                    rank: index + 1,
                    map: row.try_get("map_name")?,
                    characters: row.try_get("characters")?,
                    online_characters: row.try_get("online_characters")?,
                })
            })
            .collect()
    }

    pub async fn castles(&self, limit: u32) -> Result<Vec<CastleSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(gc.castle_id AS SIGNED) AS castle_id,
                NULLIF(g.name, '') AS owner_name,
                CAST(gc.economy AS SIGNED) AS economy,
                CAST(gc.defense AS SIGNED) AS defense,
                CAST(gc.visibleC AS SIGNED) AS visible_c
            FROM `guild_castle` gc
            LEFT JOIN `guild` g ON g.guild_id = gc.guild_id
            ORDER BY gc.castle_id ASC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération de la liste des châteaux")?;

        rows.into_iter()
            .map(|row| {
                Ok(CastleSummary {
                    castle_id: row.try_get("castle_id")?,
                    owner_name: row.try_get("owner_name")?,
                    economy: row.try_get("economy")?,
                    defense: row.try_get("defense")?,
                    visible_c: row.try_get("visible_c")?,
                })
            })
            .collect()
    }

    pub async fn castle_details(&self, castle_id: i64) -> Result<Option<CastleDetails>> {
        let row = sqlx::query(
            r#"
            SELECT
                CAST(gc.castle_id AS SIGNED) AS castle_id,
                CAST(gc.guild_id AS SIGNED) AS owner_guild_id,
                NULLIF(g.name, '') AS owner_name,
                CAST(gc.economy AS SIGNED) AS economy,
                CAST(gc.defense AS SIGNED) AS defense,
                CAST(gc.triggerE AS SIGNED) AS trigger_e,
                CAST(gc.triggerD AS SIGNED) AS trigger_d,
                CAST(gc.nextTime AS SIGNED) AS next_time,
                CAST(gc.payTime AS SIGNED) AS pay_time,
                CAST(gc.createTime AS SIGNED) AS create_time,
                CAST(gc.visibleC AS SIGNED) AS visible_c
            FROM `guild_castle` gc
            LEFT JOIN `guild` g ON g.guild_id = gc.guild_id
            WHERE gc.castle_id = ?
            LIMIT 1
            "#,
        )
        .bind(castle_id)
        .fetch_optional(&self.pool)
        .await
        .context("récupération du détail du château")?;

        match row {
            Some(row) => Ok(Some(CastleDetails {
                castle_id: row.try_get("castle_id")?,
                owner_guild_id: row.try_get("owner_guild_id")?,
                owner_name: row.try_get("owner_name")?,
                economy: row.try_get("economy")?,
                defense: row.try_get("defense")?,
                trigger_e: row.try_get("trigger_e")?,
                trigger_d: row.try_get("trigger_d")?,
                next_time: row.try_get("next_time")?,
                pay_time: row.try_get("pay_time")?,
                create_time: row.try_get("create_time")?,
                visible_c: row.try_get("visible_c")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn account_characters(
        &self,
        account_id: i64,
        limit: u32,
    ) -> Result<Vec<AccountCharacterSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(c.char_num AS SIGNED) AS slot,
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.online AS SIGNED) AS online,
                c.last_map,
                CAST(c.zeny AS SIGNED) AS zeny,
                g.name AS guild_name
            FROM `login` l
            INNER JOIN `char` c ON c.account_id = l.account_id
            LEFT JOIN `guild` g ON g.guild_id = c.guild_id
            WHERE l.account_id = ?
            ORDER BY c.char_num ASC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(account_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des personnages du compte")?;

        rows.into_iter()
            .map(|row| {
                Ok(AccountCharacterSummary {
                    slot: row.try_get("slot")?,
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    online: row.try_get::<i32, _>("online")? == 1,
                    map: row.try_get("last_map")?,
                    zeny: row.try_get("zeny")?,
                    guild_name: row.try_get("guild_name")?,
                })
            })
            .collect()
    }

    pub async fn account_status(&self, account_id: i64) -> Result<Option<AccountStatus>> {
        let row = sqlx::query(
            r#"
            SELECT
                CAST(l.account_id AS SIGNED) AS account_id,
                l.userid,
                l.sex,
                CAST(l.group_id AS SIGNED) AS group_id,
                CAST(l.state AS SIGNED) AS state,
                CAST(l.unban_time AS SIGNED) AS unban_time,
                CAST(l.expiration_time AS SIGNED) AS expiration_time,
                CAST(l.logincount AS SIGNED) AS logincount,
                CAST(l.character_slots AS SIGNED) AS character_slots,
                DATE_FORMAT(l.lastlogin, '%Y-%m-%d %H:%i:%s') AS lastlogin,
                CAST(COUNT(c.char_id) AS SIGNED) AS characters,
                CAST(COALESCE(SUM(CASE WHEN c.online = 1 THEN 1 ELSE 0 END), 0) AS SIGNED) AS online_characters,
                CAST(COALESCE(SUM(c.zeny), 0) AS SIGNED) AS total_zeny
            FROM `login` l
            LEFT JOIN `char` c ON c.account_id = l.account_id
            WHERE l.account_id = ?
            GROUP BY
                l.account_id,
                l.userid,
                l.sex,
                l.group_id,
                l.state,
                l.unban_time,
                l.expiration_time,
                l.logincount,
                l.character_slots,
                l.lastlogin
            LIMIT 1
            "#,
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await
        .context("récupération du statut du compte")?;

        match row {
            Some(row) => Ok(Some(AccountStatus {
                account_id: row.try_get("account_id")?,
                userid: row.try_get("userid")?,
                sex: row.try_get("sex")?,
                group_id: row.try_get("group_id")?,
                state: row.try_get("state")?,
                unban_time: row.try_get("unban_time")?,
                expiration_time: row.try_get("expiration_time")?,
                logincount: row.try_get("logincount")?,
                character_slots: row.try_get("character_slots")?,
                characters: row.try_get("characters")?,
                online_characters: row.try_get("online_characters")?,
                total_zeny: row.try_get("total_zeny")?,
                lastlogin: row.try_get("lastlogin")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn create_account(
        &self,
        userid: &str,
        password: &str,
        password_mode: AccountPasswordMode,
        sex: &str,
        birthdate: &str,
        email: &str,
    ) -> Result<CreatedAccount> {
        if self.account_userid_exists(userid).await? {
            anyhow::bail!("Le compte `{userid}` existe déjà.");
        }

        let sql = match password_mode {
            AccountPasswordMode::Plain => {
                r#"
                INSERT INTO `login` (userid, user_pass, sex, birthdate, email)
                VALUES (?, ?, ?, ?, ?)
                "#
            }
            AccountPasswordMode::Md5 => {
                r#"
                INSERT INTO `login` (userid, user_pass, sex, birthdate, email)
                VALUES (?, MD5(?), ?, ?, ?)
                "#
            }
        };

        let result = sqlx::query(sql)
            .bind(userid)
            .bind(password)
            .bind(sex)
            .bind(birthdate)
            .bind(email)
            .execute(&self.pool)
            .await
            .context("create rAthena account")?;

        Ok(CreatedAccount {
            account_id: i64::try_from(result.last_insert_id()).unwrap_or(i64::MAX),
            userid: userid.to_string(),
            sex: sex.to_string(),
            email: email.to_string(),
        })
    }

    pub async fn enqueue_discord_gmmsg(
        &self,
        mode: &str,
        map: Option<&str>,
        color: Option<&str>,
        message: &[u8],
        discord_user_id: u64,
        discord_username: &str,
    ) -> Result<()> {
        if !self.table_exists("discord_gmmsg_queue").await? {
            anyhow::bail!(
                "La table `discord_gmmsg_queue` est absente. Exécutez le script SQL d’installation du bridge GMMSG."
            );
        }

        sqlx::query(
            r#"
            INSERT INTO `discord_gmmsg_queue`
                (`mode`, `map`, `color`, `message`, `discord_user_id`, `discord_username`, `status`)
            VALUES
                (?, ?, ?, ?, ?, ?, 'pending')
            "#,
        )
        .bind(mode)
        .bind(map)
        .bind(color)
        .bind(message)
        .bind(discord_user_id.to_string())
        .bind(discord_username)
        .execute(&self.pool)
        .await
        .context("ajout du message GMMSG dans la file SQL rAthena")?;

        Ok(())
    }

    pub async fn account_userid_exists(&self, userid: &str) -> Result<bool> {
        let existing = sqlx::query(
            r#"
            SELECT CAST(COUNT(*) AS SIGNED) AS account_count
            FROM `login`
            WHERE userid = ?
            "#,
        )
        .bind(userid)
        .fetch_one(&self.pool)
        .await
        .context("check account username availability")?;

        let account_count: i64 = existing.try_get("account_count")?;
        Ok(account_count > 0)
    }

    pub async fn character_quests(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<CharacterQuestEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(q.quest_id AS SIGNED) AS quest_id,
                q.state,
                CAST(q.time AS SIGNED) AS quest_time,
                CAST(q.count1 AS SIGNED) AS count1,
                CAST(q.count2 AS SIGNED) AS count2,
                CAST(q.count3 AS SIGNED) AS count3
            FROM `char` c
            INNER JOIN `quest` q ON q.char_id = c.char_id
            WHERE c.name = ?
            ORDER BY q.quest_id ASC
            LIMIT ?
            "#,
        )
        .bind(character_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des quêtes du personnage")?;

        rows.into_iter()
            .map(|row| {
                Ok(CharacterQuestEntry {
                    quest_id: row.try_get("quest_id")?,
                    state: row.try_get("state")?,
                    time: row.try_get("quest_time")?,
                    count1: row.try_get("count1")?,
                    count2: row.try_get("count2")?,
                    count3: row.try_get("count3")?,
                })
            })
            .collect()
    }

    pub async fn character_equipment(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<CharacterItemEntry>> {
        self.character_items(character_name, true, limit).await
    }

    pub async fn character_inventory(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<CharacterItemEntry>> {
        self.character_items(character_name, false, limit).await
    }

    async fn character_items(
        &self,
        character_name: &str,
        equipped_only: bool,
        limit: u32,
    ) -> Result<Vec<CharacterItemEntry>> {
        let equipped_only_value = if equipped_only { 1 } else { 0 };
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(i.nameid AS SIGNED) AS item_id,
                CAST(i.amount AS SIGNED) AS item_amount,
                CAST(i.equip AS SIGNED) AS equip,
                CAST(i.refine AS SIGNED) AS refine,
                CAST(i.identify AS SIGNED) AS identify,
                CAST(i.bound AS SIGNED) AS bound,
                CAST(i.unique_id AS SIGNED) AS unique_id,
                CAST(i.enchantgrade AS SIGNED) AS enchant_grade,
                CAST(i.card0 AS SIGNED) AS card0,
                CAST(i.card1 AS SIGNED) AS card1,
                CAST(i.card2 AS SIGNED) AS card2,
                CAST(i.card3 AS SIGNED) AS card3
            FROM `char` c
            INNER JOIN `inventory` i ON i.char_id = c.char_id
            WHERE c.name = ?
              AND ((? = 1 AND i.equip <> 0) OR (? = 0 AND i.equip = 0))
            ORDER BY i.equip DESC, i.nameid ASC, i.id ASC
            LIMIT ?
            "#,
        )
        .bind(character_name)
        .bind(equipped_only_value)
        .bind(equipped_only_value)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des items d’inventaire du personnage")?;

        rows.into_iter()
            .map(|row| {
                Ok(CharacterItemEntry {
                    item_id: row.try_get("item_id")?,
                    amount: row.try_get("item_amount")?,
                    equip: row.try_get("equip")?,
                    refine: row.try_get("refine")?,
                    identify: row.try_get::<i32, _>("identify")? != 0,
                    bound: row.try_get("bound")?,
                    unique_id: row.try_get("unique_id")?,
                    enchant_grade: row.try_get("enchant_grade")?,
                    card0: row.try_get("card0")?,
                    card1: row.try_get("card1")?,
                    card2: row.try_get("card2")?,
                    card3: row.try_get("card3")?,
                })
            })
            .collect()
    }

    pub async fn item_owners(&self, item_id: i64, limit: u32) -> Result<Vec<ItemOwnerEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT source, owner_name, account_id, CAST(SUM(amount) AS SIGNED) AS total_amount
            FROM (
                SELECT
                    'Inventaire' AS source,
                    c.name AS owner_name,
                    CAST(c.account_id AS SIGNED) AS account_id,
                    CAST(i.amount AS SIGNED) AS amount
                FROM `inventory` i
                INNER JOIN `char` c ON c.char_id = i.char_id
                WHERE i.nameid = ?

                UNION ALL

                SELECT
                    'Chariot' AS source,
                    c.name AS owner_name,
                    CAST(c.account_id AS SIGNED) AS account_id,
                    CAST(ci.amount AS SIGNED) AS amount
                FROM `cart_inventory` ci
                INNER JOIN `char` c ON c.char_id = ci.char_id
                WHERE ci.nameid = ?

                UNION ALL

                SELECT
                    'Stockage compte' AS source,
                    CONCAT(l.userid, ' (#', l.account_id, ')') AS owner_name,
                    CAST(l.account_id AS SIGNED) AS account_id,
                    CAST(s.amount AS SIGNED) AS amount
                FROM `storage` s
                INNER JOIN `login` l ON l.account_id = s.account_id
                WHERE s.nameid = ?

                UNION ALL

                SELECT
                    'Stockage guilde' AS source,
                    COALESCE(NULLIF(g.name, ''), CONCAT('Guilde #', gs.guild_id)) AS owner_name,
                    CAST(NULL AS SIGNED) AS account_id,
                    CAST(gs.amount AS SIGNED) AS amount
                FROM `guild_storage` gs
                LEFT JOIN `guild` g ON g.guild_id = gs.guild_id
                WHERE gs.nameid = ?
            ) owners
            GROUP BY source, owner_name, account_id
            ORDER BY total_amount DESC, source ASC, owner_name ASC
            LIMIT ?
            "#,
        )
        .bind(item_id)
        .bind(item_id)
        .bind(item_id)
        .bind(item_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des propriétaires d’item")?;

        rows.into_iter()
            .map(|row| {
                Ok(ItemOwnerEntry {
                    source: row.try_get("source")?,
                    owner_name: row.try_get("owner_name")?,
                    account_id: row.try_get("account_id")?,
                    amount: row.try_get("total_amount")?,
                })
            })
            .collect()
    }

    pub async fn ban_list(&self, limit: u32) -> Result<Vec<BanEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(l.account_id AS SIGNED) AS account_id,
                l.userid,
                CAST(l.group_id AS SIGNED) AS group_id,
                CAST(l.state AS SIGNED) AS account_state,
                CAST(l.unban_time AS SIGNED) AS unban_time,
                CAST(l.expiration_time AS SIGNED) AS expiration_time,
                DATE_FORMAT(l.lastlogin, '%Y-%m-%d %H:%i:%s') AS lastlogin,
                CAST(COUNT(c.char_id) AS SIGNED) AS characters
            FROM `login` l
            LEFT JOIN `char` c ON c.account_id = l.account_id
            WHERE l.state <> 0 OR l.unban_time > 0
            GROUP BY
                l.account_id,
                l.userid,
                l.group_id,
                l.state,
                l.unban_time,
                l.expiration_time,
                l.lastlogin
            ORDER BY l.state DESC, l.unban_time DESC, l.account_id ASC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération de la liste des bannissements")?;

        rows.into_iter()
            .map(|row| {
                Ok(BanEntry {
                    account_id: row.try_get("account_id")?,
                    userid: row.try_get("userid")?,
                    group_id: row.try_get("group_id")?,
                    state: row.try_get("account_state")?,
                    unban_time: row.try_get("unban_time")?,
                    expiration_time: row.try_get("expiration_time")?,
                    lastlogin: row.try_get("lastlogin")?,
                    characters: row.try_get("characters")?,
                })
            })
            .collect()
    }

    pub async fn who_sell(
        &self,
        item_id: i64,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<MarketSellEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS merchant_name,
                v.title AS shop_title,
                v.map,
                CAST(v.x AS SIGNED) AS x,
                CAST(v.y AS SIGNED) AS y,
                CAST(vi.amount AS SIGNED) AS item_amount,
                CAST(vi.price AS SIGNED) AS item_price
            FROM `vendings` v
            INNER JOIN `vending_items` vi ON vi.vending_id = v.id
            INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
            INNER JOIN `char` c ON c.char_id = v.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE ci.nameid = ? AND l.group_id < ?
            ORDER BY vi.price ASC, vi.amount DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(item_id)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des vendeurs vending")?;

        rows.into_iter()
            .map(|row| {
                Ok(MarketSellEntry {
                    merchant_name: row.try_get("merchant_name")?,
                    shop_title: row.try_get("shop_title")?,
                    map: row.try_get("map")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    amount: row.try_get("item_amount")?,
                    price: row.try_get("item_price")?,
                })
            })
            .collect()
    }

    pub async fn who_buy(
        &self,
        item_id: i64,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<MarketBuyEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS buyer_name,
                bs.title AS shop_title,
                bs.map,
                CAST(bs.x AS SIGNED) AS x,
                CAST(bs.y AS SIGNED) AS y,
                CAST(bsi.amount AS SIGNED) AS item_amount,
                CAST(bsi.price AS SIGNED) AS item_price
            FROM `buyingstores` bs
            INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
            INNER JOIN `char` c ON c.char_id = bs.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE bsi.item_id = ? AND l.group_id < ?
            ORDER BY bsi.price DESC, bsi.amount DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(item_id)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des acheteurs buying store")?;

        rows.into_iter()
            .map(|row| {
                Ok(MarketBuyEntry {
                    buyer_name: row.try_get("buyer_name")?,
                    shop_title: row.try_get("shop_title")?,
                    map: row.try_get("map")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    amount: row.try_get("item_amount")?,
                    price: row.try_get("item_price")?,
                })
            })
            .collect()
    }

    pub async fn market_overview(
        &self,
        item_id: i64,
        group_threshold: i32,
    ) -> Result<MarketOverview> {
        let row = sqlx::query(
            r#"
            SELECT
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `vendings` v
                    INNER JOIN `vending_items` vi ON vi.vending_id = v.id
                    INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
                    INNER JOIN `char` c ON c.char_id = v.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE ci.nameid = ? AND l.group_id < ?
                ) AS sellers,
                COALESCE((
                    SELECT CAST(SUM(vi.amount) AS SIGNED)
                    FROM `vendings` v
                    INNER JOIN `vending_items` vi ON vi.vending_id = v.id
                    INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
                    INNER JOIN `char` c ON c.char_id = v.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE ci.nameid = ? AND l.group_id < ?
                ), 0) AS sell_amount,
                (
                    SELECT CAST(MIN(vi.price) AS SIGNED)
                    FROM `vendings` v
                    INNER JOIN `vending_items` vi ON vi.vending_id = v.id
                    INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
                    INNER JOIN `char` c ON c.char_id = v.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE ci.nameid = ? AND l.group_id < ?
                ) AS lowest_sell_price,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `buyingstores` bs
                    INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
                    INNER JOIN `char` c ON c.char_id = bs.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE bsi.item_id = ? AND l.group_id < ?
                ) AS buyers,
                COALESCE((
                    SELECT CAST(SUM(bsi.amount) AS SIGNED)
                    FROM `buyingstores` bs
                    INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
                    INNER JOIN `char` c ON c.char_id = bs.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE bsi.item_id = ? AND l.group_id < ?
                ), 0) AS buy_amount,
                (
                    SELECT CAST(MAX(bsi.price) AS SIGNED)
                    FROM `buyingstores` bs
                    INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
                    INNER JOIN `char` c ON c.char_id = bs.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE bsi.item_id = ? AND l.group_id < ?
                ) AS highest_buy_price
            "#,
        )
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .fetch_one(&self.pool)
        .await
        .context("récupération de la vue d’ensemble du marché")?;

        Ok(MarketOverview {
            item_id,
            sellers: row.try_get("sellers")?,
            sell_amount: row.try_get("sell_amount")?,
            lowest_sell_price: row.try_get("lowest_sell_price")?,
            buyers: row.try_get("buyers")?,
            buy_amount: row.try_get("buy_amount")?,
            highest_buy_price: row.try_get("highest_buy_price")?,
        })
    }

    pub async fn item_detail_lines(
        &self,
        query: &str,
        preferred_table: &str,
    ) -> Result<Option<Vec<String>>> {
        let Some(item) = self.search_items(query, 1).await?.into_iter().next() else {
            return Ok(None);
        };
        let table_name = if self.table_exists(preferred_table).await? {
            preferred_table
        } else {
            item.source_table.as_str()
        };
        let Some(search_columns) = self.item_search_columns(table_name).await? else {
            return Ok(Some(vec![
                format!("ID: `{}`", item.item_id),
                format!("Nom: `{}`", item.display_name),
                format!("Type: `{}`", item.item_type),
                format!("Source: `{}`", item.source_table),
            ]));
        };
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(None);
        };

        let item_type_expression = search_columns
            .item_type
            .as_deref()
            .map(cast_column_as_char)
            .unwrap_or_else(|| "NULL".to_string());
        let sql = format!(
            r#"
            SELECT
                {} AS item_id,
                {} AS aegis_name,
                {} AS display_name,
                {} AS item_type,
                {},
                {},
                {},
                {},
                {},
                {}
            FROM {}
            WHERE {} = ?
            LIMIT 1
            "#,
            cast_column_as_signed(&search_columns.id),
            cast_column_as_char(&search_columns.aegis_name),
            cast_column_as_char(&search_columns.display_name),
            item_type_expression,
            optional_signed_select(&columns, ITEM_DETAIL_BUY_COLUMNS, "buy_price"),
            optional_signed_select(&columns, ITEM_DETAIL_SELL_COLUMNS, "sell_price"),
            optional_signed_select(&columns, ITEM_DETAIL_WEIGHT_COLUMNS, "item_weight"),
            optional_signed_select(&columns, ITEM_DETAIL_ATTACK_COLUMNS, "attack"),
            optional_signed_select(&columns, ITEM_DETAIL_DEFENSE_COLUMNS, "defense"),
            optional_signed_select(&columns, ITEM_DETAIL_SLOTS_COLUMNS, "slots"),
            quote_identifier(table_name),
            cast_column_as_signed(&search_columns.id),
        );

        let row = sqlx::query(&sql)
            .bind(item.item_id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("récupération du détail d’item dans {table_name}"))?;
        let Some(row) = row else {
            return Ok(None);
        };

        let mut lines = Vec::new();
        lines.push(format!("ID: `{}`", row.try_get::<i64, _>("item_id")?));
        lines.push(format!(
            "Nom: `{}` / `{}`",
            row.try_get::<Option<String>, _>("display_name")?
                .unwrap_or_else(|| item.display_name.clone()),
            row.try_get::<Option<String>, _>("aegis_name")?
                .unwrap_or_else(|| item.aegis_name.clone())
        ));
        lines.push(format!(
            "Type: `{}`",
            row.try_get::<Option<String>, _>("item_type")?
                .unwrap_or_else(|| item.item_type.clone())
        ));
        lines.push(format!("Source: `{table_name}`"));

        for (label, alias) in [
            ("Prix achat", "buy_price"),
            ("Prix vente", "sell_price"),
            ("Poids", "item_weight"),
            ("Attaque", "attack"),
            ("Defense", "defense"),
            ("Slots", "slots"),
        ] {
            if let Some(value) = row.try_get::<Option<i64>, _>(alias)? {
                lines.push(format!("{label}: `{value}`"));
            }
        }

        Ok(Some(lines))
    }

    pub async fn mob_detail_lines(
        &self,
        query: &str,
        preferred_table: &str,
    ) -> Result<Option<Vec<String>>> {
        let Some(monster) = self.search_monsters(query, 1).await?.into_iter().next() else {
            return Ok(None);
        };
        let table_name = if self.table_exists(preferred_table).await? {
            preferred_table
        } else {
            monster.source_table.as_str()
        };
        let Some(search_columns) = self.monster_search_columns(table_name).await? else {
            return Ok(Some(vec![
                format!("ID: `{}`", monster.monster_id),
                format!("Nom: `{}`", monster.display_name),
                format!("Niveau: `{}`", monster.level),
                format!("HP: `{}`", monster.hp),
                format!("Source: `{}`", monster.source_table),
            ]));
        };
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(None);
        };

        let sprite_expression = search_columns
            .sprite
            .as_deref()
            .map(cast_column_as_char)
            .unwrap_or_else(|| cast_column_as_char(&search_columns.display_name));
        let level_expression = search_columns
            .level
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "NULL".to_string());
        let hp_expression = search_columns
            .hp
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "NULL".to_string());
        let sql = format!(
            r#"
            SELECT
                {} AS monster_id,
                {} AS sprite,
                {} AS display_name,
                {} AS monster_level,
                {} AS monster_hp,
                {},
                {},
                {},
                {},
                {},
                {},
                {},
                {},
                {},
                {},
                {}
            FROM {}
            WHERE {} = ?
            LIMIT 1
            "#,
            cast_column_as_signed(&search_columns.id),
            sprite_expression,
            cast_column_as_char(&search_columns.display_name),
            level_expression,
            hp_expression,
            optional_char_select(&columns, MOB_RACE_COLUMNS, "race"),
            optional_char_select(&columns, MOB_ELEMENT_COLUMNS, "element"),
            optional_char_select(&columns, MOB_SIZE_COLUMNS, "mob_size"),
            optional_signed_select(&columns, &["str", "STR"], "str_stat"),
            optional_signed_select(&columns, &["agi", "AGI"], "agi_stat"),
            optional_signed_select(&columns, &["vit", "VIT"], "vit_stat"),
            optional_signed_select(&columns, &["int", "INT"], "int_stat"),
            optional_signed_select(&columns, &["dex", "DEX"], "dex_stat"),
            optional_signed_select(&columns, &["luk", "LUK"], "luk_stat"),
            optional_signed_select(&columns, &["def", "DEF"], "defense"),
            optional_signed_select(&columns, &["mdef", "MDEF"], "mdefense"),
            quote_identifier(table_name),
            cast_column_as_signed(&search_columns.id),
        );

        let row = sqlx::query(&sql)
            .bind(monster.monster_id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("récupération du détail de monstre dans {table_name}"))?;
        let Some(row) = row else {
            return Ok(None);
        };

        let mut lines = Vec::new();
        lines.push(format!("ID: `{}`", row.try_get::<i64, _>("monster_id")?));
        lines.push(format!(
            "Nom: `{}` / `{}`",
            row.try_get::<Option<String>, _>("display_name")?
                .unwrap_or_else(|| monster.display_name.clone()),
            row.try_get::<Option<String>, _>("sprite")?
                .unwrap_or_else(|| monster.sprite.clone())
        ));

        for (label, alias) in [
            ("Niveau", "monster_level"),
            ("HP", "monster_hp"),
            ("Defense", "defense"),
            ("MDEF", "mdefense"),
            ("STR", "str_stat"),
            ("AGI", "agi_stat"),
            ("VIT", "vit_stat"),
            ("INT", "int_stat"),
            ("DEX", "dex_stat"),
            ("LUK", "luk_stat"),
        ] {
            if let Some(value) = row.try_get::<Option<i64>, _>(alias)? {
                lines.push(format!("{label}: `{value}`"));
            }
        }

        for (label, alias) in [
            ("Race", "race"),
            ("Element", "element"),
            ("Taille", "mob_size"),
        ] {
            if let Some(value) = row.try_get::<Option<String>, _>(alias)? {
                lines.push(format!("{label}: `{value}`"));
            }
        }

        lines.push(format!("Source: `{table_name}`"));

        Ok(Some(lines))
    }

    pub async fn mob_drop_lines(
        &self,
        query: &str,
        preferred_table: &str,
        limit: u32,
    ) -> Result<Option<Vec<String>>> {
        let Some(monster) = self.search_monsters(query, 1).await?.into_iter().next() else {
            return Ok(None);
        };
        let table_name = if self.table_exists(preferred_table).await? {
            preferred_table
        } else {
            monster.source_table.as_str()
        };
        let Some(search_columns) = self.monster_search_columns(table_name).await? else {
            return Ok(Some(vec![format!(
                "Aucune colonne de drop lisible dans `{table_name}`."
            )]));
        };
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(None);
        };
        let pairs = drop_column_pairs(&columns);
        if pairs.is_empty() {
            return Ok(Some(vec![format!(
                "Aucune colonne de drop détectée dans `{table_name}`."
            )]));
        }

        let select_columns = pairs
            .iter()
            .flat_map(|(id_column, rate_column, label)| {
                let safe_label = label.replace(' ', "_").to_ascii_lowercase();
                let mut columns = vec![format!(
                    "{} AS {}_id",
                    cast_column_as_signed(id_column),
                    safe_label
                )];
                columns.push(
                    rate_column
                        .as_ref()
                        .map(|column| {
                            format!("{} AS {}_rate", cast_column_as_signed(column), safe_label)
                        })
                        .unwrap_or_else(|| format!("NULL AS {}_rate", safe_label)),
                );
                columns
            })
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT {select_columns} FROM {} WHERE {} = ? LIMIT 1",
            quote_identifier(table_name),
            cast_column_as_signed(&search_columns.id),
        );

        let Some(row) = sqlx::query(&sql)
            .bind(monster.monster_id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("récupération des drops de monstre dans {table_name}"))?
        else {
            return Ok(None);
        };

        let mut lines = vec![format!(
            "Drops de `{}` (`{}`):",
            monster.display_name, monster.monster_id
        )];
        for (_id_column, _rate_column, label) in pairs.iter() {
            if lines.len() > limit as usize {
                break;
            }
            let alias = label.replace(' ', "_").to_ascii_lowercase();
            let item_id: Option<i64> = row.try_get(format!("{alias}_id").as_str())?;
            let rate: Option<i64> = row.try_get(format!("{alias}_rate").as_str())?;
            if let Some(item_id) = item_id.filter(|value| *value > 0) {
                lines.push(format!(
                    "{label}: item `{item_id}` - {}",
                    format_drop_rate(rate)
                ));
            }
        }

        if lines.len() == 1 {
            lines.push("Aucun drop renseigne.".to_string());
        }

        Ok(Some(lines))
    }

    pub async fn who_drops_lines(
        &self,
        item_query: &str,
        preferred_mob_table: &str,
        limit: u32,
    ) -> Result<Option<Vec<String>>> {
        let Some(item) = self.search_items(item_query, 1).await?.into_iter().next() else {
            return Ok(None);
        };
        let mut table_name = preferred_mob_table.to_string();
        if !self.table_exists(&table_name).await? {
            for candidate in MOB_SEARCH_TABLES {
                if self.table_exists(candidate).await? {
                    table_name = (*candidate).to_string();
                    break;
                }
            }
        }
        if !self.table_exists(&table_name).await? {
            return Ok(Some(vec![format!(
                "Table monstres `{table_name}` absente."
            )]));
        }
        let Some(search_columns) = self.monster_search_columns(&table_name).await? else {
            return Ok(Some(vec![format!(
                "Aucune colonne monstre lisible dans `{table_name}`."
            )]));
        };
        let Some(columns) = self.table_columns(&table_name).await? else {
            return Ok(None);
        };
        let pairs = drop_column_pairs(&columns);
        if pairs.is_empty() {
            return Ok(Some(vec![format!(
                "Aucune colonne de drop détectée dans `{table_name}`."
            )]));
        }

        let conditions = pairs
            .iter()
            .map(|(id_column, _, _)| format!("{} = ?", cast_column_as_signed(id_column)))
            .collect::<Vec<_>>()
            .join(" OR ");
        let sql = format!(
            r#"
            SELECT
                {} AS monster_id,
                {} AS monster_name
            FROM {}
            WHERE {conditions}
            ORDER BY {} ASC
            LIMIT ?
            "#,
            cast_column_as_signed(&search_columns.id),
            cast_column_as_char(&search_columns.display_name),
            quote_identifier(&table_name),
            quote_identifier(&search_columns.id),
        );
        let mut query = sqlx::query(&sql);
        for _ in &pairs {
            query = query.bind(item.item_id);
        }
        let rows = query
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| {
                format!("récupération des monstres qui drop l’item dans {table_name}")
            })?;

        let mut lines = vec![format!(
            "Monstres qui drop `{}` (`{}`):",
            item.display_name, item.item_id
        )];
        lines.extend(rows.into_iter().map(|row| {
            let monster_id = row.try_get::<i64, _>("monster_id").unwrap_or_default();
            let monster_name = row
                .try_get::<Option<String>, _>("monster_name")
                .ok()
                .flatten()
                .unwrap_or_else(|| format!("Monstre {monster_id}"));
            format!("`{monster_id}` - {monster_name}")
        }));

        if lines.len() == 1 {
            lines.push("Aucun monstre n’a été trouvé.".to_string());
        }

        Ok(Some(lines))
    }

    pub async fn rank_summary_lines(
        &self,
        character_name: &str,
        group_threshold: i32,
    ) -> Result<Option<Vec<String>>> {
        let row = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.zeny AS SIGNED) AS zeny,
                (
                    SELECT CAST(COUNT(*) + 1 AS SIGNED)
                    FROM `char` c2
                    INNER JOIN `login` l2 ON l2.account_id = c2.account_id
                    WHERE l2.group_id < ?
                      AND (c2.base_level > c.base_level OR (c2.base_level = c.base_level AND c2.job_level > c.job_level))
                ) AS level_rank,
                (
                    SELECT CAST(COUNT(*) + 1 AS SIGNED)
                    FROM `char` c2
                    INNER JOIN `login` l2 ON l2.account_id = c2.account_id
                    WHERE l2.group_id < ?
                      AND (c2.job_level > c.job_level OR (c2.job_level = c.job_level AND c2.base_level > c.base_level))
                ) AS job_rank,
                (
                    SELECT CAST(COUNT(*) + 1 AS SIGNED)
                    FROM `char` c2
                    INNER JOIN `login` l2 ON l2.account_id = c2.account_id
                    WHERE l2.group_id < ?
                      AND c2.zeny > c.zeny
                ) AS zeny_rank
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE c.name = ? AND l.group_id < ?
            LIMIT 1
            "#,
        )
        .bind(group_threshold)
        .bind(group_threshold)
        .bind(group_threshold)
        .bind(character_name)
        .bind(group_threshold)
        .fetch_optional(&self.pool)
        .await
        .context("récupération du résumé de rang du personnage")?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(vec![
            format!("Personnage: `{}`", row.try_get::<String, _>("name")?),
            format!("Base level: `{}`", row.try_get::<i32, _>("base_level")?),
            format!("Job level: `{}`", row.try_get::<i32, _>("job_level")?),
            format!("Rang level: `#{}`", row.try_get::<i64, _>("level_rank")?),
            format!("Rang job: `#{}`", row.try_get::<i64, _>("job_rank")?),
            format!("Rang zeny: `#{}`", row.try_get::<i64, _>("zeny_rank")?),
        ]))
    }

    pub async fn mvp_list_lines(&self, preferred_table: &str, limit: u32) -> Result<Vec<String>> {
        let mut table_name = preferred_table.to_string();
        if !self.table_exists(&table_name).await? {
            for candidate in MOB_SEARCH_TABLES {
                if self.table_exists(candidate).await? {
                    table_name = (*candidate).to_string();
                    break;
                }
            }
        }
        if !self.table_exists(&table_name).await? {
            return Ok(vec![format!("Table monstres `{table_name}` absente.")]);
        }
        let Some(search_columns) = self.monster_search_columns(&table_name).await? else {
            return Ok(vec![format!(
                "Aucune colonne monstre lisible dans `{table_name}`."
            )]);
        };
        let Some(columns) = self.table_columns(&table_name).await? else {
            return Ok(vec![format!("Table monstres `{table_name}` absente.")]);
        };
        let Some(mvp_exp_column) = columns.first(MOB_MVP_EXP_COLUMNS) else {
            return Ok(vec![format!(
                "Aucune colonne MVP détectée dans `{table_name}`."
            )]);
        };
        let level_expression = search_columns
            .level
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "NULL".to_string());
        let hp_expression = search_columns
            .hp
            .as_deref()
            .map(cast_column_as_signed)
            .unwrap_or_else(|| "NULL".to_string());
        let sql = format!(
            r#"
            SELECT
                {} AS monster_id,
                {} AS monster_name,
                {} AS monster_level,
                {} AS monster_hp,
                {} AS mvp_exp
            FROM {}
            WHERE {} > 0
            ORDER BY {} DESC, {} ASC
            LIMIT ?
            "#,
            cast_column_as_signed(&search_columns.id),
            cast_column_as_char(&search_columns.display_name),
            level_expression,
            hp_expression,
            cast_column_as_signed(&mvp_exp_column),
            quote_identifier(&table_name),
            cast_column_as_signed(&mvp_exp_column),
            quote_identifier(&mvp_exp_column),
            quote_identifier(&search_columns.id),
        );
        let rows = sqlx::query(&sql)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .context("récupération de la liste des MVP")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    let id = row.try_get::<i64, _>("monster_id").unwrap_or_default();
                    let name = row
                        .try_get::<Option<String>, _>("monster_name")
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| format!("MVP {id}"));
                    let level = row
                        .try_get::<Option<i64>, _>("monster_level")
                        .ok()
                        .flatten();
                    let hp = row.try_get::<Option<i64>, _>("monster_hp").ok().flatten();
                    format!(
                        "`{id}` - {name} - niveau `{}` - HP `{}`",
                        level.unwrap_or_default(),
                        hp.unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun MVP n’a été trouvé.",
        ))
    }

    pub async fn character_cart_lines(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !self.table_exists("cart_inventory").await? {
            return Ok(vec!["Table `cart_inventory` absente.".to_string()]);
        }
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(ci.nameid AS SIGNED) AS item_id,
                CAST(ci.amount AS SIGNED) AS amount,
                CAST(COALESCE(ci.refine, 0) AS SIGNED) AS refine
            FROM `cart_inventory` ci
            INNER JOIN `char` c ON c.char_id = ci.char_id
            WHERE c.name = ?
            ORDER BY ci.nameid ASC
            LIMIT ?
            "#,
        )
        .bind(character_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("recuperation du chariot du personnage")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    format!(
                        "Item `{}` x`{}` refine `+{}`",
                        row.try_get::<i64, _>("item_id").unwrap_or_default(),
                        row.try_get::<i64, _>("amount").unwrap_or_default(),
                        row.try_get::<i64, _>("refine").unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun item n'a ete trouve dans le chariot.",
        ))
    }

    pub async fn character_storage_lines(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !self.table_exists("storage").await? {
            return Ok(vec!["Table `storage` absente.".to_string()]);
        }
        let rows = sqlx::query(
            r#"
            SELECT CAST(s.nameid AS SIGNED) AS item_id, CAST(s.amount AS SIGNED) AS amount
            FROM `storage` s
            INNER JOIN `char` c ON c.account_id = s.account_id
            WHERE c.name = ?
            ORDER BY s.nameid ASC
            LIMIT ?
            "#,
        )
        .bind(character_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du storage du personnage")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    format!(
                        "Item `{}` x`{}`",
                        row.try_get::<i64, _>("item_id").unwrap_or_default(),
                        row.try_get::<i64, _>("amount").unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun item n’a été trouvé dans le storage.",
        ))
    }

    pub async fn guild_storage_lines(&self, guild_name: &str, limit: u32) -> Result<Vec<String>> {
        if !self.table_exists("guild_storage").await? {
            return Ok(vec!["Table `guild_storage` absente.".to_string()]);
        }
        let rows = sqlx::query(
            r#"
            SELECT CAST(gs.nameid AS SIGNED) AS item_id, CAST(gs.amount AS SIGNED) AS amount
            FROM `guild_storage` gs
            INNER JOIN `guild` g ON g.guild_id = gs.guild_id
            WHERE g.name = ?
            ORDER BY gs.nameid ASC
            LIMIT ?
            "#,
        )
        .bind(guild_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du storage de guilde")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    format!(
                        "Item `{}` x`{}`",
                        row.try_get::<i64, _>("item_id").unwrap_or_default(),
                        row.try_get::<i64, _>("amount").unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun item n’a été trouvé dans le storage de guilde.",
        ))
    }

    pub async fn variable_lines(
        &self,
        table_name: &str,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !["char_reg_num", "char_reg_str", "acc_reg_num", "acc_reg_str"]
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(table_name))
        {
            anyhow::bail!("table de variables non autorisée");
        }
        if !self.table_exists(table_name).await? {
            return Ok(vec![format!("Table `{table_name}` absente.")]);
        }

        let account_filter = table_name.starts_with("acc_");
        let id_column = if account_filter {
            "account_id"
        } else {
            "char_id"
        };
        let owner_subquery = if account_filter {
            "(SELECT account_id FROM `char` WHERE name = ? LIMIT 1)"
        } else {
            "(SELECT char_id FROM `char` WHERE name = ? LIMIT 1)"
        };
        let sql = format!(
            r#"
            SELECT `key`, `index`, `value`
            FROM {}
            WHERE `{id_column}` = {owner_subquery}
            ORDER BY `key` ASC, `index` ASC
            LIMIT ?
            "#,
            quote_identifier(table_name),
        );
        let rows = sqlx::query(&sql)
            .bind(character_name)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("récupération des variables depuis {table_name}"))?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    let key = row
                        .try_get::<Option<String>, _>("key")
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| "var".to_string());
                    let index = row.try_get::<Option<i64>, _>("index").ok().flatten();
                    let value = row
                        .try_get::<Option<String>, _>("value")
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    format!("`{key}`[{}] = `{value}`", index.unwrap_or_default())
                })
                .collect(),
            "Aucune variable n’a été trouvée.",
        ))
    }

    pub async fn character_log_lines(
        &self,
        table_name: &str,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        self.filtered_log_lines(table_name, Some(character_name), None, limit)
            .await
    }

    pub async fn named_log_lines(
        &self,
        table_name: &str,
        name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        self.filtered_log_lines(table_name, None, Some(name), limit)
            .await
    }

    pub async fn recent_log_lines(&self, table_name: &str, limit: u32) -> Result<Vec<String>> {
        self.filtered_log_lines(table_name, None, None, limit).await
    }

    async fn filtered_log_lines(
        &self,
        table_name: &str,
        character_name: Option<&str>,
        name_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !RELEASE_LOG_TABLES
            .iter()
            .any(|table| table.name().eq_ignore_ascii_case(table_name))
        {
            anyhow::bail!("table de logs non autorisée");
        }
        if !self.table_exists(table_name).await? {
            return Ok(vec![format!("Table `{table_name}` absente.")]);
        }
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(vec![format!("Table `{table_name}` absente.")]);
        };
        let display_columns = columns
            .names
            .iter()
            .filter(|name| !is_sensitive_column(name))
            .take(6)
            .cloned()
            .collect::<Vec<_>>();
        if display_columns.is_empty() {
            return Ok(vec![format!(
                "Aucune colonne affichable dans `{table_name}`."
            )]);
        }

        let select_clause = display_columns
            .iter()
            .enumerate()
            .map(|(index, column)| format!("{} AS c{index}", cast_column_as_char(column)))
            .collect::<Vec<_>>()
            .join(", ");
        let order_column = columns
            .first(&["time", "date", "mvp_date", "logtime", "atcommand_date"])
            .or_else(|| display_columns.first().cloned());
        let order_clause = order_column
            .as_ref()
            .map(|column| format!("ORDER BY {} DESC", quote_identifier(column)))
            .unwrap_or_default();

        let mut where_clause = String::new();
        let mut bind_character = false;
        let mut bind_name = false;
        if let Some(_character_name) = character_name {
            if let Some(char_column) = columns.first(&["char_id", "src_charid", "kill_char_id"]) {
                where_clause = format!(
                    "WHERE {} IN (SELECT char_id FROM `char` WHERE name = ?)",
                    quote_identifier(&char_column)
                );
                bind_character = true;
            } else if let Some(account_column) = columns.first(&["account_id"]) {
                where_clause = format!(
                    "WHERE {} = (SELECT account_id FROM `char` WHERE name = ? LIMIT 1)",
                    quote_identifier(&account_column)
                );
                bind_character = true;
            } else if let Some(name_column) = columns.first(&["name", "char_name", "src_name"]) {
                where_clause = format!("WHERE {} = ?", quote_identifier(&name_column));
                bind_character = true;
            }
        } else if let Some(_name_filter) = name_filter {
            if let Some(name_column) = columns.first(&["name", "char_name", "gm", "user", "userid"])
            {
                where_clause = format!("WHERE {} = ?", quote_identifier(&name_column));
                bind_name = true;
            }
        }

        let sql = format!(
            "SELECT {select_clause} FROM {} {where_clause} {order_clause} LIMIT ?",
            quote_identifier(table_name),
        );
        let mut query = sqlx::query(&sql);
        if bind_character {
            if let Some(character_name) = character_name {
                query = query.bind(character_name);
            }
        }
        if bind_name {
            if let Some(name_filter) = name_filter {
                query = query.bind(name_filter);
            }
        }

        let rows = query
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("récupération des lignes de logs depuis {table_name}"))?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    display_columns
                        .iter()
                        .enumerate()
                        .map(|(index, column)| {
                            let value = row
                                .try_get::<Option<String>, _>(format!("c{index}").as_str())
                                .ok()
                                .flatten()
                                .unwrap_or_default();
                            format!("{}=`{}`", column, mask_sensitive_value(column, value))
                        })
                        .collect::<Vec<_>>()
                        .join(" - ")
                })
                .collect(),
            "Aucune ligne n’a été trouvée.",
        ))
    }

    pub async fn top_characters_by_job(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<RankingEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                c.last_map
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            ORDER BY c.job_level DESC, c.base_level DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du classement job des personnages")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(RankingEntry {
                    rank: index + 1,
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    map: row.try_get("last_map")?,
                })
            })
            .collect()
    }

    pub async fn account_id_for_character(&self, character_name: &str) -> Result<Option<i64>> {
        let row = sqlx::query(
            r#"
            SELECT CAST(account_id AS SIGNED) AS account_id
            FROM `char`
            WHERE name = ?
            LIMIT 1
            "#,
        )
        .bind(character_name)
        .fetch_optional(&self.pool)
        .await
        .context("récupération de l’account_id du personnage")?;

        row.map(|row| row.try_get("account_id").map_err(Into::into))
            .transpose()
    }

    pub async fn account_status_by_character(
        &self,
        character_name: &str,
    ) -> Result<Option<AccountStatus>> {
        let Some(account_id) = self.account_id_for_character(character_name).await? else {
            return Ok(None);
        };

        self.account_status(account_id).await
    }

    pub async fn release_health_lines(&self) -> Result<Vec<String>> {
        let mut lines = Vec::new();

        let required = self
            .table_presence_lines("requise", RELEASE_REQUIRED_TABLES)
            .await?;
        let optional = self
            .table_presence_lines("optionnelle", RELEASE_OPTIONAL_TABLES)
            .await?;
        let logs = self.table_presence_lines("log", RELEASE_LOG_TABLES).await?;

        lines.push("Tables requises:".to_string());
        lines.extend(required);
        lines.push(String::new());
        lines.push("Tables optionnelles:".to_string());
        lines.extend(optional);
        lines.push(String::new());
        lines.push("Logs SQL:".to_string());
        lines.extend(logs);

        Ok(lines)
    }

    pub async fn detected_rathena_tables(&self, limit: u32) -> Result<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = DATABASE()
              AND table_type = 'BASE TABLE'
            ORDER BY table_name ASC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("list detected rAthena tables")?;

        rows.into_iter()
            .map(|row| row.try_get("table_name").map_err(Into::into))
            .collect()
    }

    pub async fn useful_table_counts(&self) -> Result<Vec<String>> {
        let mut lines = Vec::new();

        for table in RELEASE_REQUIRED_TABLES
            .iter()
            .chain(RELEASE_OPTIONAL_TABLES.iter())
            .chain(RELEASE_LOG_TABLES.iter())
        {
            let table_name = table.name();
            if !self.table_exists(table_name).await? {
                continue;
            }

            let sql = format!(
                "SELECT CAST(COUNT(*) AS SIGNED) AS row_count FROM {}",
                quote_identifier(table_name)
            );
            let row = sqlx::query(&sql)
                .fetch_one(&self.pool)
                .await
                .with_context(|| format!("comptage des lignes dans {table_name}"))?;
            let row_count: i64 = row.try_get("row_count")?;

            lines.push(format!("`{table_name}`: `{row_count}` lignes"));
        }

        Ok(lines)
    }

    pub async fn log_table_sizes(&self) -> Result<Vec<String>> {
        let mut lines = Vec::new();

        for table in RELEASE_LOG_TABLES {
            let table_name = table.name();
            if !self.table_exists(table_name).await? {
                lines.push(format!("`{table_name}`: absente"));
                continue;
            }

            let row = sqlx::query(
                r#"
                SELECT
                    CAST(table_rows AS SIGNED) AS table_rows,
                    CAST(COALESCE(data_length, 0) + COALESCE(index_length, 0) AS SIGNED) AS bytes
                FROM information_schema.tables
                WHERE table_schema = DATABASE()
                  AND table_name = ?
                "#,
            )
            .bind(table_name)
            .fetch_one(&self.pool)
            .await
            .with_context(|| {
                format!("récupération de la taille de la table de logs {table_name}")
            })?;

            let table_rows: Option<i64> = row.try_get("table_rows")?;
            let bytes: i64 = row.try_get("bytes")?;
            lines.push(format!(
                "`{table_name}`: environ `{}` lignes, `{}` octets",
                table_rows.unwrap_or_default(),
                bytes
            ));
        }

        Ok(lines)
    }

    pub async fn sql_updates_lines(&self, limit: u32) -> Result<Vec<String>> {
        if !self.table_exists(DatabaseTable::SqlUpdates.name()).await? {
            return Ok(vec!["`sql_updates` absente.".to_string()]);
        }

        let columns = self.table_columns(DatabaseTable::SqlUpdates.name()).await?;
        let Some(columns) = columns else {
            return Ok(vec!["`sql_updates` absente.".to_string()]);
        };
        let revision = columns
            .first(&["revision", "version", "file"])
            .or_else(|| columns.names.first().cloned());
        let Some(revision) = revision else {
            return Ok(vec![
                "`sql_updates` ne contient aucune colonne lisible.".to_string()
            ]);
        };
        let applied = columns.first(&["applied", "date", "timestamp"]);
        let applied_expr = applied
            .as_ref()
            .map(|column| cast_column_as_char(column))
            .unwrap_or_else(|| "NULL".to_string());
        let sql = format!(
            r#"
            SELECT
                {} AS revision,
                {} AS applied
            FROM `sql_updates`
            ORDER BY {} DESC
            LIMIT ?
            "#,
            cast_column_as_char(&revision),
            applied_expr,
            quote_identifier(&revision),
        );

        let rows = sqlx::query(&sql)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .context("récupération des dernières lignes de sql_updates")?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let revision: Option<String> = row.try_get("revision").ok();
                let applied: Option<String> = row.try_get("applied").ok();
                format!(
                    "`{}` - `{}`",
                    revision.unwrap_or_else(|| "inconnu".to_string()),
                    applied.unwrap_or_else(|| "date inconnue".to_string())
                )
            })
            .collect())
    }

    async fn table_presence_lines(
        &self,
        label: &str,
        tables: &[DatabaseTable],
    ) -> Result<Vec<String>> {
        let mut lines = Vec::new();

        for table in tables {
            let table_name = table.name();
            let state = if self.table_exists(table_name).await? {
                "présente"
            } else {
                "manquante"
            };
            lines.push(format!("`{table_name}` ({label}): {state}"));
        }

        Ok(lines)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_columns_resolve_candidates_case_insensitively() {
        let columns = AvailableColumns {
            names: vec![
                "id".to_string(),
                "name_english".to_string(),
                "LV".to_string(),
            ],
        };

        assert_eq!(columns.first(&["ID"]).as_deref(), Some("id"));
        assert_eq!(columns.first(&["level", "lv"]).as_deref(), Some("LV"));
        assert_eq!(columns.all(&["name_english", "NAME_ENGLISH"]).len(), 1);
    }

    #[test]
    fn quote_identifier_escapes_backticks() {
        assert_eq!(quote_identifier("safe_name"), "`safe_name`");
        assert_eq!(quote_identifier("bad`name"), "`bad``name`");
    }

    #[test]
    fn guild_queries_count_online_members_from_joined_char_table() {
        for sql in [TOP_GUILDS_SQL, FIND_GUILD_SQL] {
            assert!(sql.contains("LEFT JOIN `guild_member` gm ON gm.guild_id = g.guild_id"));
            assert!(sql.contains("LEFT JOIN `char` ON `char`.`char_id` = gm.char_id"));
            assert!(sql.contains("`char`.`online` > 0"));
            assert!(sql.contains("gm.char_id"));
            assert!(!sql.contains("connect_member"));
            assert!(!sql.contains("c.guild_id"));
        }
    }

    #[test]
    fn parse_search_id_only_accepts_positive_numeric_queries() {
        assert_eq!(parse_search_id("501"), Some(501));
        assert_eq!(parse_search_id("  000501  "), Some(501));
        assert_eq!(parse_search_id("Poring"), None);
        assert_eq!(parse_search_id("501a"), None);
        assert_eq!(parse_search_id("0"), None);
    }
}
