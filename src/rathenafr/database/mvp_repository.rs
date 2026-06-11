use super::*;

impl RAthenaFrDatabase {
    pub async fn mvp_list_lines(&self, _preferred_table: &str, limit: u32) -> Result<Vec<String>> {
        if !self.table_exists("rathenafr_mvp_regular_spawn").await? {
            return Ok(vec![
                "Vue `rathenafr_mvp_regular_spawn` absente. Importe d’abord la table MVP Athena puis crée la vue SQL.".to_string(),
            ]);
        }

        let limit = limit.clamp(1, 500);
        let has_mvp_log = self.table_exists("mvplog").await?;
        let query = if has_mvp_log {
            r#"
            WITH last_kill AS (
                SELECT
                    monster_id,
                    map,
                    MAX(mvp_date) AS last_kill_at
                FROM mvplog
                GROUP BY monster_id, map
            )
            SELECT
                CAST(s.monster_id AS SIGNED) AS monster_id,
                s.monster_name,
                COALESCE(NULLIF(s.map_name, ''), 'inconnue') AS map_name,
                CAST(ROUND(COALESCE(s.respawn_minutes, 0)) AS SIGNED) AS respawn_minutes,
                CAST(ROUND(COALESCE(s.respawn_variance_minutes, 0)) AS SIGNED) AS respawn_variance_minutes,
                CAST(UNIX_TIMESTAMP(lk.last_kill_at) AS SIGNED) AS last_kill_ts,
                CAST(UNIX_TIMESTAMP(DATE_ADD(
                    lk.last_kill_at,
                    INTERVAL s.respawn_minutes MINUTE
                )) AS SIGNED) AS earliest_spawn_ts,
                CAST(UNIX_TIMESTAMP(DATE_ADD(
                    lk.last_kill_at,
                    INTERVAL (s.respawn_minutes + COALESCE(s.respawn_variance_minutes, 0)) MINUTE
                )) AS SIGNED) AS latest_spawn_ts,
                CASE
                    WHEN lk.last_kill_at IS NULL THEN 'unknown'
                    WHEN NOW() < DATE_ADD(
                        lk.last_kill_at,
                        INTERVAL s.respawn_minutes MINUTE
                    ) THEN 'waiting'
                    WHEN NOW() <= DATE_ADD(
                        lk.last_kill_at,
                        INTERVAL (s.respawn_minutes + COALESCE(s.respawn_variance_minutes, 0)) MINUTE
                    ) THEN 'window'
                    ELSE 'available'
                END AS spawn_state
            FROM rathenafr_mvp_regular_spawn s
            LEFT JOIN last_kill lk
                ON lk.monster_id = s.monster_id
               AND lk.map COLLATE utf8mb4_unicode_ci
                   = s.map_name COLLATE utf8mb4_unicode_ci
            ORDER BY
                CASE
                    WHEN lk.last_kill_at IS NULL THEN 3
                    WHEN NOW() < DATE_ADD(
                        lk.last_kill_at,
                        INTERVAL s.respawn_minutes MINUTE
                    ) THEN 1
                    WHEN NOW() <= DATE_ADD(
                        lk.last_kill_at,
                        INTERVAL (s.respawn_minutes + COALESCE(s.respawn_variance_minutes, 0)) MINUTE
                    ) THEN 0
                    ELSE 2
                END,
                s.monster_name ASC,
                s.map_name ASC
            LIMIT ?
            "#
        } else {
            r#"
            SELECT
                CAST(s.monster_id AS SIGNED) AS monster_id,
                s.monster_name,
                COALESCE(NULLIF(s.map_name, ''), 'inconnue') AS map_name,
                CAST(ROUND(COALESCE(s.respawn_minutes, 0)) AS SIGNED) AS respawn_minutes,
                CAST(ROUND(COALESCE(s.respawn_variance_minutes, 0)) AS SIGNED) AS respawn_variance_minutes,
                CAST(NULL AS SIGNED) AS last_kill_ts,
                CAST(NULL AS SIGNED) AS earliest_spawn_ts,
                CAST(NULL AS SIGNED) AS latest_spawn_ts,
                'unknown' AS spawn_state
            FROM rathenafr_mvp_regular_spawn s
            ORDER BY s.monster_name ASC, s.map_name ASC
            LIMIT ?
            "#
        };

        let rows = sqlx::query(query)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .context("récupération de la liste des MVP réguliers")?;

        let mut lines = rows
            .into_iter()
            .map(mvp_timer_row_from_row)
            .map(|timer| timer.map(|timer| format_mvp_timer_line(&timer)))
            .collect::<Result<Vec<_>>>()?;

        if lines.is_empty() {
            lines.push("Aucun MVP avec spawn régulier n’a été trouvé.".to_string());
        }

        Ok(lines)
    }

    pub async fn mvp_last_entries(&self, limit: u32) -> Result<Vec<MvpKillEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(ml.mvp_date AS CHAR) AS mvp_date,
                CAST(UNIX_TIMESTAMP(ml.mvp_date) AS SIGNED) AS mvp_ts,
                CAST(ml.kill_char_id AS SIGNED) AS kill_char_id,
                COALESCE(
                    NULLIF(c.name, ''),
                    CONCAT('Personnage #', ml.kill_char_id)
                ) AS killer_name,
                CAST(ml.monster_id AS SIGNED) AS monster_id,
                COALESCE(
                    NULLIF(m.monster_name, ''),
                    CONCAT('MVP #', ml.monster_id)
                ) AS monster_name,
                NULLIF(m.aegis_name, '') AS monster_aegis_name,
                ml.map AS map_name,
                CAST(ml.mvpexp AS SIGNED) AS mvp_exp,
                CAST(ml.prize AS SIGNED) AS prize,
                COALESCE(
                    NULLIF(i.item_name, ''),
                    CONCAT('Item #', ml.prize)
                ) AS prize_name,
                NULLIF(i.aegis_name, '') AS prize_aegis_name
            FROM mvplog ml
            LEFT JOIN `char` c
                ON c.char_id = ml.kill_char_id
            LEFT JOIN (
                SELECT
                    monster_id,
                    MAX(monster_name) AS monster_name,
                    MAX(aegis_name) AS aegis_name
                FROM rathenafr_mvp_list
                GROUP BY monster_id
            ) m
                ON m.monster_id = ml.monster_id
            LEFT JOIN rathenafr_item_search i
                ON i.item_id = ml.prize
            ORDER BY ml.mvp_date DESC
            LIMIT ?
            "#,
        )
        .bind(limit.clamp(1, 11))
        .fetch_all(&self.pool)
        .await
        .context("récupération des derniers MVP")?;

        rows.into_iter().map(mvp_kill_entry_from_row).collect()
    }

    pub async fn mvp_top_lines(
        &self,
        _preferred_mob_table: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !self.table_exists("mvplog").await? {
            return Ok(vec!["Table `mvplog` absente.".to_string()]);
        }
        let Some(columns) = self.table_columns("mvplog").await? else {
            return Ok(vec!["Table `mvplog` absente.".to_string()]);
        };
        let log_columns = mvp_log_columns(&columns);
        let can_join_char = log_columns.killer_id.is_some() && self.table_exists("char").await?;
        let char_join = if can_join_char {
            let killer_id_column = log_columns.killer_id.as_deref().unwrap_or_default();
            format!(
                "LEFT JOIN `char` ON `char`.`char_id` = {}",
                cast_qualified_column_as_signed("ml", killer_id_column)
            )
        } else {
            String::new()
        };
        let killer_name_expression = mvp_killer_name_expression(&log_columns, can_join_char);
        let last_date_select = log_columns
            .date
            .as_ref()
            .map(|column| {
                format!(
                    "MAX({}) AS last_mvp_date",
                    cast_qualified_column_as_char("ml", column)
                )
            })
            .unwrap_or_else(|| "NULL AS last_mvp_date".to_string());
        let sql = format!(
            r#"
            SELECT
                {} AS killer_name,
                CAST(COUNT(*) AS SIGNED) AS kill_count,
                {last_date_select}
            FROM `mvplog` ml
            {char_join}
            GROUP BY killer_name
            ORDER BY kill_count DESC, killer_name ASC
            LIMIT ?
            "#,
            killer_name_expression,
        );
        let rows = sqlx::query(&sql)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .context("récupération du top MVP")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .enumerate()
                .map(|(index, row)| {
                    let killer = row
                        .try_get::<Option<String>, _>("killer_name")?
                        .filter(|value| !value.trim().is_empty())
                        .unwrap_or_else(|| "Tueur inconnu".to_string());
                    let count = row.try_get::<i64, _>("kill_count")?;
                    let last_date = row
                        .try_get::<Option<String>, _>("last_mvp_date")?
                        .filter(|value| !value.trim().is_empty());
                    let suffix = last_date
                        .map(|value| format!(" - dernier `{value}`"))
                        .unwrap_or_default();
                    Ok(format!(
                        "`{:>2}.` **{}** - `{}` MVP{}",
                        index + 1,
                        killer,
                        count,
                        suffix
                    ))
                })
                .collect::<Result<Vec<_>>>()?,
            MVP_LOG_EMPTY_MESSAGE,
        ))
    }
}
