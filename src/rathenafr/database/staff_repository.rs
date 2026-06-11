use super::*;

impl RAthenaFrDatabase {
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

    pub(super) async fn filtered_log_lines(
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
        let order_column = applied.as_ref().unwrap_or(&revision);
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
            quote_identifier(order_column),
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

    pub(super) async fn table_presence_lines(
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
