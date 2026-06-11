use super::*;

impl RAthenaFrDatabase {
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

    pub(super) async fn search_monsters_in_table(
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

    pub(super) async fn search_monsters_in_table_by_id(
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

    pub async fn mob_detail_lines(
        &self,
        query: &str,
        _preferred_table: &str,
        rates: &ServerRatesConfig,
    ) -> Result<Option<Vec<String>>> {
        let Some(monster) = self.search_monsters(query, 1).await?.into_iter().next() else {
            return Ok(None);
        };
        let table_name = monster.source_table.as_str();
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
            optional_signed_select(&columns, &["base_exp", "BaseExp"], "base_exp"),
            optional_signed_select(&columns, &["job_exp", "JobExp"], "job_exp"),
            optional_signed_select(&columns, MOB_MVP_EXP_COLUMNS, "mvp_exp"),
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

        let exp_values = [
            (
                "EXP base serveur",
                row.try_get::<Option<i64>, _>("base_exp")?,
                rates.base_exp_rate,
            ),
            (
                "EXP job serveur",
                row.try_get::<Option<i64>, _>("job_exp")?,
                rates.job_exp_rate,
            ),
            (
                "EXP MVP serveur",
                row.try_get::<Option<i64>, _>("mvp_exp")?,
                rates.mvp_exp_rate,
            ),
        ];
        if rates.configured {
            for (label, value, rate) in exp_values {
                if let Some(value) = value.filter(|value| *value > 0) {
                    lines.push(format!(
                        "{label}: `{}`",
                        format_number_fr(apply_exp_rate(value, rate))
                    ));
                }
            }
        } else if exp_values.iter().any(|(_, value, _)| value.is_some()) {
            lines.push("EXP serveur non affichée : rates non configurés dans le bot.".to_string());
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

    pub async fn mob_drops(
        &self,
        query: &str,
        _preferred_table: &str,
        rates: &ServerRatesConfig,
    ) -> Result<Option<MonsterDrops>> {
        let Some(monster) = self.search_monsters(query, 1).await?.into_iter().next() else {
            return Ok(None);
        };
        let table_name = monster.source_table.as_str();
        let Some(search_columns) = self.monster_search_columns(table_name).await? else {
            return Ok(Some(MonsterDrops {
                monster_id: monster.monster_id,
                monster_name: monster.display_name,
                drops: Vec::new(),
            }));
        };
        let Some(columns) = self.table_columns(table_name).await? else {
            return Ok(None);
        };
        let pairs = drop_column_pairs(&columns);
        if pairs.is_empty() {
            return Ok(Some(MonsterDrops {
                monster_id: monster.monster_id,
                monster_name: monster.display_name,
                drops: Vec::new(),
            }));
        }

        let drop_select_columns = pairs
            .iter()
            .flat_map(|(id_column, rate_column, label)| {
                let safe_label = label.replace(' ', "_").to_ascii_lowercase();
                let mut columns = vec![format!(
                    "{} AS {}_item",
                    cast_column_as_char(id_column),
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
        let select_columns = format!(
            "{}, {}, {}, {}",
            optional_signed_select(&columns, &["mode_mvp", "ModeMvp"], "mode_mvp"),
            optional_char_select(&columns, &["class", "Class"], "monster_class"),
            optional_signed_select(&columns, MOB_MVP_EXP_COLUMNS, "mvp_exp"),
            drop_select_columns
        );
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

        let mut drops = Vec::new();
        let monster_kind = monster_rate_kind(
            row.try_get("mode_mvp")?,
            row.try_get::<Option<String>, _>("monster_class")?
                .as_deref(),
            row.try_get("mvp_exp")?,
        );
        for (_id_column, _rate_column, label) in pairs.iter() {
            let alias = label.replace(' ', "_").to_ascii_lowercase();
            let item_reference: Option<String> = row.try_get(format!("{alias}_item").as_str())?;
            let rate: Option<i64> = row.try_get(format!("{alias}_rate").as_str())?;
            let Some(item_reference) = item_reference
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty() && value != "0")
            else {
                continue;
            };
            let item = self
                .search_items(&item_reference, 1)
                .await?
                .into_iter()
                .next();
            let item_type = item
                .as_ref()
                .map(|item| item.item_type.as_str())
                .unwrap_or("");
            let is_mvp_reward = label.starts_with("MVP drop");

            drops.push(MonsterDropEntry {
                item_id: item
                    .as_ref()
                    .map(|entry| entry.item_id)
                    .or_else(|| item_reference.parse().ok()),
                item_name: item
                    .as_ref()
                    .map(|entry| entry.display_name.clone())
                    .unwrap_or_else(|| "Non disponible".to_string()),
                aegis_name: item.as_ref().map(|entry| entry.aegis_name.clone()),
                server_rate: server_drop_rate(rate, item_type, monster_kind, is_mvp_reward, rates),
            });
        }

        Ok(Some(MonsterDrops {
            monster_id: monster.monster_id,
            monster_name: monster.display_name,
            drops,
        }))
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
            .map(|(id_column, _, _)| drop_item_match_condition(id_column))
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
            query = query.bind(&item.aegis_name).bind(item.item_id);
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
}
