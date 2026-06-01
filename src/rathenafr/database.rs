use crate::config::{AccountPasswordMode, DatabaseConfig};
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, MySql, MySqlPool, Row, Transaction};
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
    GuildAlliance,
    GuildCastle,
    GuildMember,
    GuildSkill,
    GuildStorage,
    Homunculus,
    Inventory,
    Party,
    Pet,
    Quest,
    Storage,
    VendingItems,
    Vendings,
}

impl DatabaseTable {
    pub const fn name(self) -> &'static str {
        match self {
            Self::BuyingStoreItems => "buyingstore_items",
            Self::BuyingStores => "buyingstores",
            Self::CartInventory => "cart_inventory",
            Self::GuildAlliance => "guild_alliance",
            Self::GuildCastle => "guild_castle",
            Self::GuildMember => "guild_member",
            Self::GuildSkill => "guild_skill",
            Self::GuildStorage => "guild_storage",
            Self::Homunculus => "homunculus",
            Self::Inventory => "inventory",
            Self::Party => "party",
            Self::Pet => "pet",
            Self::Quest => "quest",
            Self::Storage => "storage",
            Self::VendingItems => "vending_items",
            Self::Vendings => "vendings",
        }
    }
}

use crate::rathenafr::models::*;

const ITEM_SEARCH_TABLES: &[&str] = &["item_db", "item_db_re"];
const MOB_SEARCH_TABLES: &[&str] = &["mob_db", "mob_db_re"];
const ACCOUNT_DELETE_ACCOUNT_ID_SKIP_TABLES: &[&str] = &["char", "login"];
const ACCOUNT_DELETE_CHAR_ID_SKIP_TABLES: &[&str] = &["char", "guild"];

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

    async fn table_exists(&self, table_name: &str) -> Result<bool> {
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

    async fn table_has_columns(&self, table_name: &str, column_names: &[&str]) -> Result<bool> {
        if !self.table_exists(table_name).await? {
            return Ok(false);
        }

        for column_name in column_names {
            let row = sqlx::query(
                r#"
                SELECT CAST(COUNT(*) AS SIGNED) AS column_count
                FROM information_schema.columns
                WHERE table_schema = DATABASE()
                  AND table_name = ?
                  AND column_name = ?
                "#,
            )
            .bind(table_name)
            .bind(column_name)
            .fetch_one(&self.pool)
            .await
            .context("vérification de disponibilité des colonnes rAthena")?;

            let count: i64 = row.try_get("column_count")?;
            if count == 0 {
                return Ok(false);
            }
        }

        Ok(true)
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
        .context("fetch online characters")?;

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
        .context("fetch top characters")?;

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
        .context("fetch top zeny characters")?;

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

    pub async fn search_characters(
        &self,
        group_threshold: i32,
        query: &str,
        limit: u32,
    ) -> Result<Vec<CharacterSummary>> {
        let pattern = format!("%{}%", query);
        let prefix = format!("{}%", query);

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
            WHERE c.name LIKE ? AND l.group_id < ?
            ORDER BY
                CASE
                    WHEN c.name = ? THEN 0
                    WHEN c.name LIKE ? THEN 1
                    ELSE 2
                END,
                c.name ASC
            LIMIT ?
            "#,
        )
        .bind(&pattern)
        .bind(group_threshold)
        .bind(query)
        .bind(&prefix)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("search characters")?;

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

    pub async fn search_all(
        &self,
        group_threshold: i32,
        query: &str,
        limit: u32,
    ) -> Result<SearchResults> {
        Ok(SearchResults {
            characters: self
                .search_characters(group_threshold, query, limit)
                .await
                .context("search characters for combined search")?,
            items: self
                .search_items(query, limit)
                .await
                .context("search items for combined search")?,
            monsters: self
                .search_monsters(query, limit)
                .await
                .context("search monsters for combined search")?,
        })
    }

    pub async fn search_items(&self, query: &str, limit: u32) -> Result<Vec<ItemSearchEntry>> {
        let mut entries = Vec::new();

        for table_name in ITEM_SEARCH_TABLES {
            if !self
                .table_has_columns(table_name, &["id", "name_aegis", "name_english", "type"])
                .await?
            {
                continue;
            }

            let table_entries = self.search_items_in_table(table_name, query, limit).await?;
            entries.extend(table_entries);

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
        query: &str,
        limit: u32,
    ) -> Result<Vec<ItemSearchEntry>> {
        let pattern = format!("%{}%", query);
        let prefix = format!("{}%", query);
        let sql = format!(
            r#"
            SELECT
                ? AS source_table,
                CAST(id AS SIGNED) AS item_id,
                name_aegis,
                name_english,
                CAST(`type` AS CHAR) AS item_type
            FROM `{table_name}`
            WHERE CAST(id AS CHAR) = ?
               OR name_aegis LIKE ?
               OR name_english LIKE ?
            ORDER BY
                CASE
                    WHEN CAST(id AS CHAR) = ? THEN 0
                    WHEN name_english = ? OR name_aegis = ? THEN 1
                    WHEN name_english LIKE ? OR name_aegis LIKE ? THEN 2
                    ELSE 3
                END,
                id ASC
            LIMIT ?
            "#
        );

        let rows = sqlx::query(&sql)
            .bind(table_name)
            .bind(query)
            .bind(&pattern)
            .bind(&pattern)
            .bind(query)
            .bind(query)
            .bind(query)
            .bind(&prefix)
            .bind(&prefix)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("search items in {table_name}"))?;

        rows.into_iter()
            .map(|row| {
                let english_name = row.try_get::<String, _>("name_english")?;
                let aegis_name = row.try_get::<String, _>("name_aegis")?;
                let display_name = if english_name.trim().is_empty() {
                    aegis_name.clone()
                } else {
                    english_name
                };

                Ok(ItemSearchEntry {
                    item_id: row.try_get("item_id")?,
                    aegis_name,
                    display_name,
                    item_type: row.try_get("item_type")?,
                    source_table: row.try_get("source_table")?,
                })
            })
            .collect()
    }

    pub async fn search_monsters(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<MonsterSearchEntry>> {
        let mut entries = Vec::new();

        for table_name in MOB_SEARCH_TABLES {
            if !self
                .table_has_columns(
                    table_name,
                    &["id", "sprite", "kROName", "iROName", "LV", "HP"],
                )
                .await?
            {
                continue;
            }

            let table_entries = self
                .search_monsters_in_table(table_name, query, limit)
                .await?;
            entries.extend(table_entries);

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
        query: &str,
        limit: u32,
    ) -> Result<Vec<MonsterSearchEntry>> {
        let pattern = format!("%{}%", query);
        let prefix = format!("{}%", query);
        let sql = format!(
            r#"
            SELECT
                ? AS source_table,
                CAST(id AS SIGNED) AS monster_id,
                sprite,
                kROName,
                iROName,
                CAST(LV AS SIGNED) AS monster_level,
                CAST(HP AS SIGNED) AS monster_hp
            FROM `{table_name}`
            WHERE CAST(id AS CHAR) = ?
               OR sprite LIKE ?
               OR kROName LIKE ?
               OR iROName LIKE ?
            ORDER BY
                CASE
                    WHEN CAST(id AS CHAR) = ? THEN 0
                    WHEN iROName = ? OR kROName = ? OR sprite = ? THEN 1
                    WHEN iROName LIKE ? OR kROName LIKE ? OR sprite LIKE ? THEN 2
                    ELSE 3
                END,
                id ASC
            LIMIT ?
            "#
        );

        let rows = sqlx::query(&sql)
            .bind(table_name)
            .bind(query)
            .bind(&pattern)
            .bind(&pattern)
            .bind(&pattern)
            .bind(query)
            .bind(query)
            .bind(query)
            .bind(query)
            .bind(&prefix)
            .bind(&prefix)
            .bind(&prefix)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("search monsters in {table_name}"))?;

        rows.into_iter()
            .map(|row| {
                let iro_name = row.try_get::<String, _>("iROName")?;
                let kro_name = row.try_get::<String, _>("kROName")?;
                let sprite = row.try_get::<String, _>("sprite")?;
                let display_name = if !iro_name.trim().is_empty() {
                    iro_name
                } else if !kro_name.trim().is_empty() {
                    kro_name
                } else {
                    sprite.clone()
                };

                Ok(MonsterSearchEntry {
                    monster_id: row.try_get("monster_id")?,
                    sprite,
                    display_name,
                    level: row.try_get("monster_level")?,
                    hp: row.try_get("monster_hp")?,
                    source_table: row.try_get("source_table")?,
                })
            })
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

    pub async fn top_guilds(&self, limit: u32) -> Result<Vec<GuildSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                g.name AS guild_name,
                g.master AS guild_master,
                CAST(g.guild_lv AS SIGNED) AS guild_level,
                CAST(g.connect_member AS SIGNED) AS online_members,
                CAST(g.max_member AS SIGNED) AS max_members,
                CAST(COUNT(c.char_id) AS SIGNED) AS members
            FROM `guild` g
            LEFT JOIN `char` c ON c.guild_id = g.guild_id
            GROUP BY
                g.guild_id,
                g.name,
                g.master,
                g.guild_lv,
                g.connect_member,
                g.max_member
            ORDER BY g.guild_lv DESC, members DESC, g.name ASC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch guild ranking")?;

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

    pub async fn find_guild(&self, name: &str) -> Result<Option<GuildDetails>> {
        let row = sqlx::query(
            r#"
            SELECT
                g.name AS guild_name,
                g.master AS guild_master,
                CAST(g.guild_lv AS SIGNED) AS guild_level,
                CAST(g.connect_member AS SIGNED) AS online_members,
                CAST(g.max_member AS SIGNED) AS max_members,
                CAST(g.average_lv AS SIGNED) AS average_level,
                CAST(g.exp AS SIGNED) AS guild_exp,
                CAST(g.next_exp AS SIGNED) AS next_exp,
                CAST(COUNT(c.char_id) AS SIGNED) AS members
            FROM `guild` g
            LEFT JOIN `char` c ON c.guild_id = g.guild_id
            WHERE g.name = ?
            GROUP BY
                g.guild_id,
                g.name,
                g.master,
                g.guild_lv,
                g.connect_member,
                g.max_member,
                g.average_lv,
                g.exp,
                g.next_exp
            LIMIT 1
            "#,
        )
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
        .context("fetch guild members")?;

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

    pub async fn class_distribution(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<ClassDistributionEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(c.class AS SIGNED) AS class_id,
                CAST(COUNT(*) AS SIGNED) AS characters,
                CAST(SUM(CASE WHEN c.online = 1 THEN 1 ELSE 0 END) AS SIGNED) AS online_characters
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            GROUP BY c.class
            ORDER BY characters DESC, online_characters DESC, class_id ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch class distribution")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(ClassDistributionEntry {
                    rank: index + 1,
                    class_id: row.try_get("class_id")?,
                    characters: row.try_get("characters")?,
                    online_characters: row.try_get("online_characters")?,
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
        .context("fetch map statistics")?;

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

    pub async fn map_online_characters(
        &self,
        group_threshold: i32,
        map: &str,
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
            WHERE c.online = 1
              AND c.last_map = ?
              AND l.group_id < ?
            ORDER BY c.base_level DESC, c.job_level DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(map)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch online map characters")?;

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

    pub async fn find_party(
        &self,
        name: &str,
        group_threshold: i32,
    ) -> Result<Option<PartyDetails>> {
        let row = sqlx::query(
            r#"
            SELECT
                p.name AS party_name,
                leader.name AS leader_name,
                CAST(p.exp AS SIGNED) AS exp_mode,
                CAST(p.item AS SIGNED) AS item_mode,
                CAST(COUNT(c.char_id) AS SIGNED) AS members,
                CAST(COALESCE(SUM(CASE WHEN c.online = 1 THEN 1 ELSE 0 END), 0) AS SIGNED) AS online_members
            FROM `party` p
            LEFT JOIN `char` leader ON leader.char_id = p.leader_char
            LEFT JOIN `char` c ON c.party_id = p.party_id
            LEFT JOIN `login` l ON l.account_id = c.account_id
            WHERE p.name = ?
              AND (c.char_id IS NULL OR l.group_id < ?)
            GROUP BY p.party_id, p.name, leader.name, p.exp, p.item
            LIMIT 1
            "#,
        )
        .bind(name)
        .bind(group_threshold)
        .fetch_optional(&self.pool)
        .await
        .context("find party")?;

        match row {
            Some(row) => Ok(Some(PartyDetails {
                name: row.try_get("party_name")?,
                leader_name: row.try_get("leader_name")?,
                members: row.try_get("members")?,
                online_members: row.try_get("online_members")?,
                exp_mode: row.try_get("exp_mode")?,
                item_mode: row.try_get("item_mode")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn party_members(
        &self,
        party_name: &str,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<PartyMemberSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.online AS SIGNED) AS online,
                c.last_map,
                CAST(CASE WHEN c.char_id = p.leader_char THEN 1 ELSE 0 END AS SIGNED) AS is_leader
            FROM `party` p
            INNER JOIN `char` c ON c.party_id = p.party_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE p.name = ? AND l.group_id < ?
            ORDER BY is_leader DESC, c.online DESC, c.base_level DESC, c.job_level DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(party_name)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch party members")?;

        rows.into_iter()
            .map(|row| {
                Ok(PartyMemberSummary {
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    online: row.try_get::<i32, _>("online")? == 1,
                    map: row.try_get("last_map")?,
                    is_leader: row.try_get::<i32, _>("is_leader")? == 1,
                })
            })
            .collect()
    }

    pub async fn find_homunculus(
        &self,
        character_name: &str,
        group_threshold: i32,
    ) -> Result<Option<HomunculusProfile>> {
        let row = sqlx::query(
            r#"
            SELECT
                c.name AS owner_name,
                h.name AS homunculus_name,
                CAST(h.class AS SIGNED) AS class_id,
                CAST(h.level AS SIGNED) AS level,
                CAST(h.intimacy AS SIGNED) AS intimacy,
                CAST(h.hunger AS SIGNED) AS hunger,
                CAST(h.alive AS SIGNED) AS alive,
                CAST(h.vaporize AS SIGNED) AS vaporize,
                CAST(h.autofeed AS SIGNED) AS autofeed,
                CAST(h.hp AS SIGNED) AS hp,
                CAST(h.max_hp AS SIGNED) AS max_hp,
                CAST(h.sp AS SIGNED) AS sp,
                CAST(h.max_sp AS SIGNED) AS max_sp
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            INNER JOIN `homunculus` h ON h.char_id = c.char_id
            WHERE c.name = ? AND l.group_id < ?
            ORDER BY h.homun_id DESC
            LIMIT 1
            "#,
        )
        .bind(character_name)
        .bind(group_threshold)
        .fetch_optional(&self.pool)
        .await
        .context("find homunculus")?;

        match row {
            Some(row) => Ok(Some(HomunculusProfile {
                owner_name: row.try_get("owner_name")?,
                name: row.try_get("homunculus_name")?,
                class_id: row.try_get("class_id")?,
                level: row.try_get("level")?,
                intimacy: row.try_get("intimacy")?,
                hunger: row.try_get("hunger")?,
                alive: row.try_get::<i32, _>("alive")? == 1,
                vaporized: row.try_get::<i32, _>("vaporize")? == 1,
                autofeed: row.try_get::<i32, _>("autofeed")? == 1,
                hp: row.try_get("hp")?,
                max_hp: row.try_get("max_hp")?,
                sp: row.try_get("sp")?,
                max_sp: row.try_get("max_sp")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn find_pet(
        &self,
        character_name: &str,
        group_threshold: i32,
    ) -> Result<Option<PetProfile>> {
        let row = sqlx::query(
            r#"
            SELECT
                c.name AS owner_name,
                p.name AS pet_name,
                CAST(p.class AS SIGNED) AS class_id,
                CAST(p.level AS SIGNED) AS level,
                CAST(p.intimate AS SIGNED) AS intimacy,
                CAST(p.hungry AS SIGNED) AS hunger,
                CAST(p.incubate AS SIGNED) AS incubate,
                CAST(p.autofeed AS SIGNED) AS autofeed
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            INNER JOIN `pet` p ON p.char_id = c.char_id
            WHERE c.name = ? AND l.group_id < ?
            ORDER BY p.pet_id DESC
            LIMIT 1
            "#,
        )
        .bind(character_name)
        .bind(group_threshold)
        .fetch_optional(&self.pool)
        .await
        .context("find pet")?;

        match row {
            Some(row) => Ok(Some(PetProfile {
                owner_name: row.try_get("owner_name")?,
                name: row.try_get("pet_name")?,
                class_id: row.try_get("class_id")?,
                level: row.try_get("level")?,
                intimacy: row.try_get("intimacy")?,
                hunger: row.try_get("hunger")?,
                incubated: row.try_get::<i32, _>("incubate")? == 1,
                autofeed: row.try_get::<i32, _>("autofeed")? == 1,
            })),
            None => Ok(None),
        }
    }

    pub async fn zeny_summary(&self, group_threshold: i32) -> Result<ZenySummary> {
        let row = sqlx::query(
            r#"
            SELECT
                CAST(COUNT(*) AS SIGNED) AS character_count,
                CAST(COALESCE(SUM(c.zeny), 0) AS SIGNED) AS total_zeny,
                CAST(COALESCE(AVG(c.zeny), 0) AS SIGNED) AS average_zeny,
                (
                    SELECT rc.name
                    FROM `char` rc
                    INNER JOIN `login` rl ON rl.account_id = rc.account_id
                    WHERE rl.group_id < ?
                    ORDER BY rc.zeny DESC, rc.base_level DESC, rc.name ASC
                    LIMIT 1
                ) AS richest_name,
                COALESCE((
                    SELECT CAST(rc.zeny AS SIGNED)
                    FROM `char` rc
                    INNER JOIN `login` rl ON rl.account_id = rc.account_id
                    WHERE rl.group_id < ?
                    ORDER BY rc.zeny DESC, rc.base_level DESC, rc.name ASC
                    LIMIT 1
                ), 0) AS richest_zeny
            FROM `char` c
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            "#,
        )
        .bind(group_threshold)
        .bind(group_threshold)
        .bind(group_threshold)
        .fetch_one(&self.pool)
        .await
        .context("fetch zeny summary")?;

        Ok(ZenySummary {
            total_zeny: row.try_get("total_zeny")?,
            average_zeny: row.try_get("average_zeny")?,
            character_count: row.try_get("character_count")?,
            richest_name: row.try_get("richest_name")?,
            richest_zeny: row.try_get("richest_zeny")?,
        })
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
        .context("fetch castle list")?;

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
        .context("fetch castle details")?;

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

    pub async fn guild_alliances(
        &self,
        guild_name: &str,
        limit: u32,
    ) -> Result<Vec<GuildAllianceEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(ga.opposition AS SIGNED) AS opposition,
                CAST(ga.alliance_id AS SIGNED) AS target_guild_id,
                COALESCE(NULLIF(target.name, ''), NULLIF(ga.name, ''), CONCAT('Guilde ', ga.alliance_id)) AS target_name
            FROM `guild` g
            INNER JOIN `guild_alliance` ga ON ga.guild_id = g.guild_id
            LEFT JOIN `guild` target ON target.guild_id = ga.alliance_id
            WHERE g.name = ?
            ORDER BY ga.opposition ASC, target_name ASC
            LIMIT ?
            "#,
        )
        .bind(guild_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch guild alliances")?;

        rows.into_iter()
            .map(|row| {
                let opposition: i32 = row.try_get("opposition")?;
                Ok(GuildAllianceEntry {
                    relation: if opposition == 1 {
                        "Opposition".to_string()
                    } else {
                        "Alliance".to_string()
                    },
                    target_guild_id: row.try_get("target_guild_id")?,
                    target_name: row.try_get("target_name")?,
                })
            })
            .collect()
    }

    pub async fn guild_skills(&self, guild_name: &str, limit: u32) -> Result<Vec<GuildSkillEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(gs.id AS SIGNED) AS skill_id,
                CAST(gs.lv AS SIGNED) AS skill_level
            FROM `guild` g
            INNER JOIN `guild_skill` gs ON gs.guild_id = g.guild_id
            WHERE g.name = ? AND gs.lv > 0
            ORDER BY gs.id ASC
            LIMIT ?
            "#,
        )
        .bind(guild_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch guild skills")?;

        rows.into_iter()
            .map(|row| {
                Ok(GuildSkillEntry {
                    skill_id: row.try_get("skill_id")?,
                    level: row.try_get("skill_level")?,
                })
            })
            .collect()
    }

    pub async fn top_homunculus(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<HomunculusRankingEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS owner_name,
                h.name AS homunculus_name,
                CAST(h.class AS SIGNED) AS class_id,
                CAST(h.level AS SIGNED) AS level,
                CAST(h.intimacy AS SIGNED) AS intimacy,
                CAST(h.hunger AS SIGNED) AS hunger
            FROM `homunculus` h
            INNER JOIN `char` c ON c.char_id = h.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            ORDER BY h.level DESC, h.intimacy DESC, h.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch top homunculus entries")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(HomunculusRankingEntry {
                    rank: index + 1,
                    owner_name: row.try_get("owner_name")?,
                    name: row.try_get("homunculus_name")?,
                    class_id: row.try_get("class_id")?,
                    level: row.try_get("level")?,
                    intimacy: row.try_get("intimacy")?,
                    hunger: row.try_get("hunger")?,
                })
            })
            .collect()
    }

    pub async fn top_pets(&self, group_threshold: i32, limit: u32) -> Result<Vec<PetRankingEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS owner_name,
                p.name AS pet_name,
                CAST(p.class AS SIGNED) AS class_id,
                CAST(p.level AS SIGNED) AS level,
                CAST(p.intimate AS SIGNED) AS intimacy,
                CAST(p.hungry AS SIGNED) AS hunger
            FROM `pet` p
            INNER JOIN `char` c ON c.char_id = p.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE l.group_id < ?
            ORDER BY p.intimate DESC, p.level DESC, p.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch top pet entries")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(PetRankingEntry {
                    rank: index + 1,
                    owner_name: row.try_get("owner_name")?,
                    name: row.try_get("pet_name")?,
                    class_id: row.try_get("class_id")?,
                    level: row.try_get("level")?,
                    intimacy: row.try_get("intimacy")?,
                    hunger: row.try_get("hunger")?,
                })
            })
            .collect()
    }

    pub async fn quest_stats(&self, quest_id: i64, group_threshold: i32) -> Result<QuestStats> {
        let row = sqlx::query(
            r#"
            SELECT
                CAST(COUNT(*) AS SIGNED) AS total_characters,
                CAST(COALESCE(SUM(CASE WHEN q.state = '0' THEN 1 ELSE 0 END), 0) AS SIGNED) AS state_0,
                CAST(COALESCE(SUM(CASE WHEN q.state = '1' THEN 1 ELSE 0 END), 0) AS SIGNED) AS state_1,
                CAST(COALESCE(SUM(CASE WHEN q.state = '2' THEN 1 ELSE 0 END), 0) AS SIGNED) AS state_2
            FROM `quest` q
            INNER JOIN `char` c ON c.char_id = q.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE q.quest_id = ? AND l.group_id < ?
            "#,
        )
        .bind(quest_id)
        .bind(group_threshold)
        .fetch_one(&self.pool)
        .await
        .context("fetch quest statistics")?;

        Ok(QuestStats {
            quest_id,
            total_characters: row.try_get("total_characters")?,
            state_0: row.try_get("state_0")?,
            state_1: row.try_get("state_1")?,
            state_2: row.try_get("state_2")?,
        })
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
        .context("fetch account characters")?;

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
        .context("fetch account status")?;

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

    pub async fn account_list(&self, limit: u32, page: u32) -> Result<AccountList> {
        let page = page.max(1);
        let offset = page.saturating_sub(1).saturating_mul(limit);
        let total_row = sqlx::query(
            r#"
            SELECT CAST(COUNT(*) AS SIGNED) AS total_accounts
            FROM `login`
            "#,
        )
        .fetch_one(&self.pool)
        .await
        .context("count accounts")?;

        let rows = sqlx::query(
            r#"
            SELECT
                CAST(l.account_id AS SIGNED) AS account_id,
                l.userid,
                l.sex,
                CAST(l.group_id AS SIGNED) AS group_id,
                CAST(l.state AS SIGNED) AS account_state,
                DATE_FORMAT(l.lastlogin, '%Y-%m-%d %H:%i:%s') AS lastlogin,
                CAST(COUNT(c.char_id) AS SIGNED) AS characters
            FROM `login` l
            LEFT JOIN `char` c ON c.account_id = l.account_id
            GROUP BY
                l.account_id,
                l.userid,
                l.sex,
                l.group_id,
                l.state,
                l.lastlogin
            ORDER BY l.account_id DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .context("fetch account list")?;

        let entries = rows
            .into_iter()
            .map(|row| {
                Ok(AccountListEntry {
                    account_id: row.try_get("account_id")?,
                    userid: row.try_get("userid")?,
                    sex: row.try_get("sex")?,
                    group_id: row.try_get("group_id")?,
                    state: row.try_get("account_state")?,
                    characters: row.try_get("characters")?,
                    lastlogin: row.try_get("lastlogin")?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(AccountList {
            total_accounts: total_row.try_get("total_accounts")?,
            page,
            per_page: limit,
            offset,
            entries,
        })
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

    pub async fn update_account(
        &self,
        account_id: i64,
        update: AccountUpdateRequest,
        password_mode: AccountPasswordMode,
    ) -> Result<AccountUpdateResult> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("start account update transaction")?;

        let row = sqlx::query(
            r#"
            SELECT userid
            FROM `login`
            WHERE account_id = ?
            FOR UPDATE
            "#,
        )
        .bind(account_id)
        .fetch_optional(&mut *tx)
        .await
        .context("fetch account before update")?;

        let Some(row) = row else {
            tx.commit()
                .await
                .context("commit missing account update transaction")?;
            return Ok(AccountUpdateResult::NotFound { account_id });
        };

        let current_userid: String = row.try_get("userid")?;
        let mut resulting_userid = current_userid.clone();
        let mut changed_fields = Vec::new();

        if let Some(userid) = update.userid {
            let existing = sqlx::query(
                r#"
                SELECT CAST(COUNT(*) AS SIGNED) AS account_count
                FROM `login`
                WHERE userid = ?
                  AND account_id <> ?
                "#,
            )
            .bind(&userid)
            .bind(account_id)
            .fetch_one(&mut *tx)
            .await
            .context("check updated account username availability")?;

            let account_count: i64 = existing.try_get("account_count")?;
            if account_count > 0 {
                tx.commit()
                    .await
                    .context("commit refused account update transaction")?;
                return Ok(AccountUpdateResult::UsernameAlreadyExists { account_id, userid });
            }

            sqlx::query(
                r#"
                UPDATE `login`
                SET userid = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(&userid)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account username")?;

            resulting_userid = userid;
            changed_fields.push("Login".to_string());
        }

        if let Some(password) = update.password {
            let sql = match password_mode {
                AccountPasswordMode::Plain => {
                    r#"
                    UPDATE `login`
                    SET user_pass = ?
                    WHERE account_id = ?
                    LIMIT 1
                    "#
                }
                AccountPasswordMode::Md5 => {
                    r#"
                    UPDATE `login`
                    SET user_pass = MD5(?)
                    WHERE account_id = ?
                    LIMIT 1
                    "#
                }
            };

            sqlx::query(sql)
                .bind(password)
                .bind(account_id)
                .execute(&mut *tx)
                .await
                .context("update account password")?;

            changed_fields.push("Mot de passe".to_string());
        }

        if let Some(sex) = update.sex {
            sqlx::query(
                r#"
                UPDATE `login`
                SET sex = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(sex)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account sex")?;

            changed_fields.push("Sexe".to_string());
        }

        if let Some(birthdate) = update.birthdate {
            sqlx::query(
                r#"
                UPDATE `login`
                SET birthdate = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(birthdate)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account birthdate")?;

            changed_fields.push("Date de naissance".to_string());
        }

        if let Some(email) = update.email {
            sqlx::query(
                r#"
                UPDATE `login`
                SET email = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(email)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account email")?;

            changed_fields.push("Email".to_string());
        }

        if let Some(group_id) = update.group_id {
            sqlx::query(
                r#"
                UPDATE `login`
                SET group_id = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(group_id)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account group_id")?;

            changed_fields.push("Groupe".to_string());
        }

        if let Some(state) = update.state {
            sqlx::query(
                r#"
                UPDATE `login`
                SET state = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(state)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account state")?;

            changed_fields.push("État".to_string());
        }

        if let Some(unban_time) = update.unban_time {
            sqlx::query(
                r#"
                UPDATE `login`
                SET unban_time = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(unban_time)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account unban_time")?;

            changed_fields.push("Fin de bannissement".to_string());
        }

        if let Some(expiration_time) = update.expiration_time {
            sqlx::query(
                r#"
                UPDATE `login`
                SET expiration_time = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(expiration_time)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account expiration_time")?;

            changed_fields.push("Expiration".to_string());
        }

        if let Some(character_slots) = update.character_slots {
            sqlx::query(
                r#"
                UPDATE `login`
                SET character_slots = ?
                WHERE account_id = ?
                LIMIT 1
                "#,
            )
            .bind(character_slots)
            .bind(account_id)
            .execute(&mut *tx)
            .await
            .context("update account character_slots")?;

            changed_fields.push("Slots personnages".to_string());
        }

        tx.commit()
            .await
            .context("commit account update transaction")?;

        Ok(AccountUpdateResult::Updated {
            account_id,
            userid: resulting_userid,
            changed_fields,
        })
    }

    pub async fn delete_account_completely(&self, account_id: i64) -> Result<AccountDeleteResult> {
        let account_id_tables = self.table_names_with_column("account_id").await?;
        let char_id_tables = self.table_names_with_column("char_id").await?;
        let friend_id_tables = self.friend_relation_tables().await?;
        let has_guild_owner_column = self.table_has_columns("guild", &["char_id"]).await?;
        let has_mail_character_columns = self
            .table_has_columns("mail", &["send_id", "dest_id"])
            .await?;
        let has_vending_items = self
            .table_has_columns("vending_items", &["vending_id"])
            .await?
            && self
                .table_has_columns("vendings", &["id", "char_id"])
                .await?;
        let has_buyingstore_items = self
            .table_has_columns("buyingstore_items", &["buyingstore_id"])
            .await?
            && self
                .table_has_columns("buyingstores", &["id", "char_id"])
                .await?;

        let mut tx = self
            .pool
            .begin()
            .await
            .context("start full account delete transaction")?;

        let row = sqlx::query(
            r#"
            SELECT
                l.userid,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `char` c
                    WHERE c.account_id = l.account_id
                ) AS characters
            FROM `login` l
            WHERE l.account_id = ?
            FOR UPDATE
            "#,
        )
        .bind(account_id)
        .fetch_optional(&mut *tx)
        .await
        .context("fetch account before full delete")?;

        let Some(row) = row else {
            tx.commit()
                .await
                .context("commit missing account full delete transaction")?;
            return Ok(AccountDeleteResult::NotFound { account_id });
        };

        let userid: String = row.try_get("userid")?;
        let characters: i64 = row.try_get("characters")?;

        if has_guild_owner_column {
            let row = sqlx::query(
                r#"
                SELECT CAST(COUNT(*) AS SIGNED) AS owned_guilds
                FROM `guild`
                WHERE char_id IN (
                    SELECT char_id
                    FROM `char`
                    WHERE account_id = ?
                )
                "#,
            )
            .bind(account_id)
            .fetch_one(&mut *tx)
            .await
            .context("count owned guilds before full account delete")?;

            let guilds: i64 = row.try_get("owned_guilds")?;
            if guilds > 0 {
                tx.commit()
                    .await
                    .context("commit refused full account delete transaction")?;
                return Ok(AccountDeleteResult::HasGuildOwnership {
                    account_id,
                    userid,
                    guilds,
                });
            }
        }

        let mut deleted_rows = 0;

        if has_vending_items {
            deleted_rows += self
                .delete_vending_items_for_account(&mut tx, account_id)
                .await?;
        }

        if has_buyingstore_items {
            deleted_rows += self
                .delete_buyingstore_items_for_account(&mut tx, account_id)
                .await?;
        }

        if has_mail_character_columns {
            deleted_rows += self
                .delete_character_relation_rows(&mut tx, "mail", "send_id", account_id)
                .await?;
            deleted_rows += self
                .delete_character_relation_rows(&mut tx, "mail", "dest_id", account_id)
                .await?;
        }

        for table_name in friend_id_tables {
            deleted_rows += self
                .delete_character_relation_rows(&mut tx, &table_name, "friend_id", account_id)
                .await?;
        }

        for table_name in char_id_tables {
            if contains_table_name(ACCOUNT_DELETE_CHAR_ID_SKIP_TABLES, &table_name) {
                continue;
            }

            deleted_rows += self
                .delete_character_relation_rows(&mut tx, &table_name, "char_id", account_id)
                .await?;
        }

        for table_name in account_id_tables {
            if contains_table_name(ACCOUNT_DELETE_ACCOUNT_ID_SKIP_TABLES, &table_name) {
                continue;
            }

            deleted_rows += self
                .delete_account_relation_rows(&mut tx, &table_name, account_id)
                .await?;
        }

        deleted_rows += sqlx::query(
            r#"
            DELETE FROM `char`
            WHERE account_id = ?
            "#,
        )
        .bind(account_id)
        .execute(&mut *tx)
        .await
        .context("delete account characters")?
        .rows_affected();

        deleted_rows += sqlx::query(
            r#"
            DELETE FROM `login`
            WHERE account_id = ?
            LIMIT 1
            "#,
        )
        .bind(account_id)
        .execute(&mut *tx)
        .await
        .context("delete account login")?
        .rows_affected();

        tx.commit()
            .await
            .context("commit full account delete transaction")?;

        Ok(AccountDeleteResult::Deleted {
            account_id,
            userid,
            characters,
            deleted_rows,
        })
    }

    async fn table_names_with_column(&self, column_name: &str) -> Result<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT c.table_name
            FROM information_schema.columns c
            INNER JOIN information_schema.tables t
              ON t.table_schema = c.table_schema
             AND t.table_name = c.table_name
            WHERE c.table_schema = DATABASE()
              AND c.column_name = ?
              AND t.table_type = 'BASE TABLE'
            ORDER BY c.table_name ASC
            "#,
        )
        .bind(column_name)
        .fetch_all(&self.pool)
        .await
        .with_context(|| format!("list rAthena tables with column {column_name}"))?;

        rows.into_iter()
            .map(|row| row.try_get("table_name").map_err(Into::into))
            .collect()
    }

    async fn friend_relation_tables(&self) -> Result<Vec<String>> {
        let mut tables = Vec::new();

        for table_name in self.table_names_with_column("friend_id").await? {
            if self
                .table_has_columns(&table_name, &["char_id", "friend_id"])
                .await?
            {
                tables.push(table_name);
            }
        }

        Ok(tables)
    }

    async fn delete_vending_items_for_account(
        &self,
        tx: &mut Transaction<'_, MySql>,
        account_id: i64,
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE vi
            FROM `vending_items` vi
            INNER JOIN `vendings` v ON v.id = vi.vending_id
            WHERE v.char_id IN (
                SELECT char_id
                FROM `char`
                WHERE account_id = ?
            )
            "#,
        )
        .bind(account_id)
        .execute(&mut **tx)
        .await
        .context("delete vending items for account")?;

        Ok(result.rows_affected())
    }

    async fn delete_buyingstore_items_for_account(
        &self,
        tx: &mut Transaction<'_, MySql>,
        account_id: i64,
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE bsi
            FROM `buyingstore_items` bsi
            INNER JOIN `buyingstores` bs ON bs.id = bsi.buyingstore_id
            WHERE bs.char_id IN (
                SELECT char_id
                FROM `char`
                WHERE account_id = ?
            )
            "#,
        )
        .bind(account_id)
        .execute(&mut **tx)
        .await
        .context("delete buying store items for account")?;

        Ok(result.rows_affected())
    }

    async fn delete_character_relation_rows(
        &self,
        tx: &mut Transaction<'_, MySql>,
        table_name: &str,
        column_name: &str,
        account_id: i64,
    ) -> Result<u64> {
        let sql = format!(
            r#"
            DELETE FROM {}
            WHERE {} IN (
                SELECT char_id
                FROM `char`
                WHERE account_id = ?
            )
            "#,
            quote_identifier(table_name),
            quote_identifier(column_name),
        );

        let result = sqlx::query(&sql)
            .bind(account_id)
            .execute(&mut **tx)
            .await
            .with_context(|| {
                format!("delete {table_name}.{column_name} rows for account {account_id}")
            })?;

        Ok(result.rows_affected())
    }

    async fn delete_account_relation_rows(
        &self,
        tx: &mut Transaction<'_, MySql>,
        table_name: &str,
        account_id: i64,
    ) -> Result<u64> {
        let sql = format!(
            r#"
            DELETE FROM {}
            WHERE `account_id` = ?
            "#,
            quote_identifier(table_name),
        );

        let result = sqlx::query(&sql)
            .bind(account_id)
            .execute(&mut **tx)
            .await
            .with_context(|| format!("delete {table_name}.account_id rows for account"))?;

        Ok(result.rows_affected())
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
        .context("fetch character quests")?;

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
        .context("fetch character inventory items")?;

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

    pub async fn item_count(&self, item_id: i64) -> Result<ItemCountSummary> {
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE((SELECT CAST(SUM(amount) AS SIGNED) FROM `inventory` WHERE nameid = ?), 0) AS inventory_amount,
                COALESCE((SELECT CAST(SUM(amount) AS SIGNED) FROM `cart_inventory` WHERE nameid = ?), 0) AS cart_amount,
                COALESCE((SELECT CAST(SUM(amount) AS SIGNED) FROM `storage` WHERE nameid = ?), 0) AS storage_amount,
                COALESCE((SELECT CAST(SUM(amount) AS SIGNED) FROM `guild_storage` WHERE nameid = ?), 0) AS guild_storage_amount
            "#,
        )
        .bind(item_id)
        .bind(item_id)
        .bind(item_id)
        .bind(item_id)
        .fetch_one(&self.pool)
        .await
        .context("count item across rAthenaFR inventory tables")?;

        let inventory_amount = row.try_get("inventory_amount")?;
        let cart_amount = row.try_get("cart_amount")?;
        let storage_amount = row.try_get("storage_amount")?;
        let guild_storage_amount = row.try_get("guild_storage_amount")?;

        Ok(ItemCountSummary {
            item_id,
            inventory_amount,
            cart_amount,
            storage_amount,
            guild_storage_amount,
            total_amount: inventory_amount + cart_amount + storage_amount + guild_storage_amount,
        })
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
        .context("fetch item owners")?;

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
        .context("fetch ban list")?;

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
        .context("fetch vending sellers")?;

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
        .context("fetch buying store buyers")?;

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
        .context("fetch market overview")?;

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

    pub async fn vending_stores(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<VendingStoreEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS merchant_name,
                v.title AS shop_title,
                v.map,
                CAST(v.x AS SIGNED) AS x,
                CAST(v.y AS SIGNED) AS y,
                CAST(COUNT(vi.`index`) AS SIGNED) AS item_count,
                CAST(COALESCE(SUM(vi.amount), 0) AS SIGNED) AS total_amount,
                CAST(MIN(vi.price) AS SIGNED) AS min_price
            FROM `vendings` v
            INNER JOIN `char` c ON c.char_id = v.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            LEFT JOIN `vending_items` vi ON vi.vending_id = v.id
            WHERE l.group_id < ?
            GROUP BY c.name, v.title, v.map, v.x, v.y
            ORDER BY item_count DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch active vending stores")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(VendingStoreEntry {
                    rank: index + 1,
                    merchant_name: row.try_get("merchant_name")?,
                    shop_title: row.try_get("shop_title")?,
                    map: row.try_get("map")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    item_count: row.try_get("item_count")?,
                    total_amount: row.try_get("total_amount")?,
                    min_price: row.try_get("min_price")?,
                })
            })
            .collect()
    }

    pub async fn buying_stores(
        &self,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<BuyingStoreEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS buyer_name,
                bs.title AS shop_title,
                bs.map,
                CAST(bs.x AS SIGNED) AS x,
                CAST(bs.y AS SIGNED) AS y,
                CAST(bs.`limit` AS SIGNED) AS zeny_limit,
                CAST(COUNT(bsi.`index`) AS SIGNED) AS item_count,
                CAST(COALESCE(SUM(bsi.amount), 0) AS SIGNED) AS total_amount,
                CAST(MAX(bsi.price) AS SIGNED) AS max_price
            FROM `buyingstores` bs
            INNER JOIN `char` c ON c.char_id = bs.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            LEFT JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
            WHERE l.group_id < ?
            GROUP BY c.name, bs.title, bs.map, bs.x, bs.y, bs.`limit`
            ORDER BY item_count DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("fetch active buying stores")?;

        rows.into_iter()
            .enumerate()
            .map(|(index, row)| {
                Ok(BuyingStoreEntry {
                    rank: index + 1,
                    buyer_name: row.try_get("buyer_name")?,
                    shop_title: row.try_get("shop_title")?,
                    map: row.try_get("map")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    item_count: row.try_get("item_count")?,
                    total_amount: row.try_get("total_amount")?,
                    max_price: row.try_get("max_price")?,
                    zeny_limit: row.try_get("zeny_limit")?,
                })
            })
            .collect()
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

fn contains_table_name(table_names: &[&str], table_name: &str) -> bool {
    table_names
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(table_name))
}

fn quote_identifier(identifier: &str) -> String {
    format!("`{}`", identifier.replace('`', "``"))
}
