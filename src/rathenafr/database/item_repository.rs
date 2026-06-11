use super::*;

impl RAthenaFrDatabase {
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

    pub(super) async fn search_items_in_table(
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

    pub(super) async fn search_items_in_table_by_id(
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

    pub async fn item_detail_lines(
        &self,
        query: &str,
        _preferred_table: &str,
    ) -> Result<Option<Vec<String>>> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(None);
        }

        let item_id = parse_search_id(query).unwrap_or(0);
        let row = sqlx::query(
            r#"
            SELECT
                CAST(item_id AS SIGNED) AS item_id,
                item_name,
                aegis_name,
                item_type,
                item_subtype,
                CAST(slots AS SIGNED) AS slots,
                CAST(buy AS SIGNED) AS buy,
                CAST(sell AS SIGNED) AS sell,
                CAST(weight AS SIGNED) AS weight,
                CAST(attack AS SIGNED) AS attack,
                CAST(magic_attack AS SIGNED) AS magic_attack,
                CAST(defense AS SIGNED) AS defense,
                CAST(equip_level_min AS SIGNED) AS equip_level_min
            FROM `rathenafr_item_search`
            WHERE item_id = ?
               OR LOWER(item_name) = LOWER(?)
               OR LOWER(aegis_name) = LOWER(?)
               OR item_name LIKE CONCAT('%', ?, '%')
               OR aegis_name LIKE CONCAT('%', ?, '%')
            ORDER BY
              CASE
                WHEN item_id = ? THEN 0
                WHEN LOWER(item_name) = LOWER(?) THEN 1
                WHEN LOWER(aegis_name) = LOWER(?) THEN 2
                ELSE 3
              END,
              item_name ASC
            LIMIT 1
            "#,
        )
        .bind(item_id)
        .bind(query)
        .bind(query)
        .bind(query)
        .bind(query)
        .bind(item_id)
        .bind(query)
        .bind(query)
        .fetch_optional(&self.pool)
        .await
        .with_context(|| {
            format!("récupération du détail d’item dans {RATHENAFR_ITEM_SEARCH_TABLE}")
        })?;
        let Some(row) = row else {
            return Ok(None);
        };

        let text_value = |value: Option<String>| {
            value
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "N/A".to_string())
        };
        let number_value = |value: Option<i64>| {
            value
                .map(|value| value.to_string())
                .unwrap_or_else(|| "N/A".to_string())
        };

        let mut lines = Vec::new();
        lines.push(format!("ID: `{}`", row.try_get::<i64, _>("item_id")?));
        lines.push(format!(
            "Nom: `{}`",
            text_value(row.try_get::<Option<String>, _>("item_name")?)
        ));
        lines.push(format!(
            "AegisName: `{}`",
            text_value(row.try_get::<Option<String>, _>("aegis_name")?)
        ));
        lines.push(format!(
            "Type: `{}`",
            text_value(row.try_get::<Option<String>, _>("item_type")?)
        ));
        lines.push(format!(
            "Sous-type: `{}`",
            text_value(row.try_get::<Option<String>, _>("item_subtype")?)
        ));
        lines.push(format!(
            "Slots: `{}`",
            number_value(row.try_get::<Option<i64>, _>("slots")?)
        ));
        lines.push(format!(
            "Prix achat: `{}`",
            number_value(row.try_get::<Option<i64>, _>("buy")?)
        ));
        lines.push(format!(
            "Prix vente: `{}`",
            number_value(row.try_get::<Option<i64>, _>("sell")?)
        ));
        lines.push(format!(
            "Poids: `{}`",
            number_value(row.try_get::<Option<i64>, _>("weight")?)
        ));
        lines.push(format!(
            "ATK: `{}`",
            number_value(row.try_get::<Option<i64>, _>("attack")?)
        ));
        lines.push(format!(
            "MATK: `{}`",
            number_value(row.try_get::<Option<i64>, _>("magic_attack")?)
        ));
        lines.push(format!(
            "DEF: `{}`",
            number_value(row.try_get::<Option<i64>, _>("defense")?)
        ));
        lines.push(format!(
            "Niveau minimum: `{}`",
            number_value(row.try_get::<Option<i64>, _>("equip_level_min")?)
        ));

        Ok(Some(lines))
    }
}
