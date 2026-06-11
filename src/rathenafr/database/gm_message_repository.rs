use super::*;

impl RAthenaFrDatabase {
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
}
