use super::*;

impl RAthenaFrDatabase {
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
}
