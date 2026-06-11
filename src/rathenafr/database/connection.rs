use super::*;

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

    pub(super) async fn table_columns(&self, table_name: &str) -> Result<Option<AvailableColumns>> {
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

    pub(super) async fn item_search_columns(
        &self,
        table_name: &str,
    ) -> Result<Option<ItemSearchColumns>> {
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

    pub(super) async fn monster_search_columns(
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
}
