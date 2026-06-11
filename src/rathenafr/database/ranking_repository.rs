use super::*;

impl RAthenaFrDatabase {
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
}
