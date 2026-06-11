use super::*;

impl RAthenaFrDatabase {
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
}
