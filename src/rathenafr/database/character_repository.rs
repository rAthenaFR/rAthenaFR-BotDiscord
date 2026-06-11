use super::*;

impl RAthenaFrDatabase {
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
        .context("récupération des quêtes du personnage")?;

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

    pub(super) async fn character_items(
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
        .context("récupération des items d’inventaire du personnage")?;

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

    pub async fn character_cart_lines(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !self.table_exists("cart_inventory").await? {
            return Ok(vec!["Table `cart_inventory` absente.".to_string()]);
        }
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(ci.nameid AS SIGNED) AS item_id,
                CAST(ci.amount AS SIGNED) AS amount,
                CAST(COALESCE(ci.refine, 0) AS SIGNED) AS refine
            FROM `cart_inventory` ci
            INNER JOIN `char` c ON c.char_id = ci.char_id
            WHERE c.name = ?
            ORDER BY ci.nameid ASC
            LIMIT ?
            "#,
        )
        .bind(character_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("recuperation du chariot du personnage")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    format!(
                        "Item `{}` x`{}` refine `+{}`",
                        row.try_get::<i64, _>("item_id").unwrap_or_default(),
                        row.try_get::<i64, _>("amount").unwrap_or_default(),
                        row.try_get::<i64, _>("refine").unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun item n’a été trouvé dans le chariot.",
        ))
    }

    pub async fn character_storage_lines(
        &self,
        character_name: &str,
        limit: u32,
    ) -> Result<Vec<String>> {
        if !self.table_exists("storage").await? {
            return Ok(vec!["Table `storage` absente.".to_string()]);
        }
        let rows = sqlx::query(
            r#"
            SELECT CAST(s.nameid AS SIGNED) AS item_id, CAST(s.amount AS SIGNED) AS amount
            FROM `storage` s
            INNER JOIN `char` c ON c.account_id = s.account_id
            WHERE c.name = ?
            ORDER BY s.nameid ASC
            LIMIT ?
            "#,
        )
        .bind(character_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du storage du personnage")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    format!(
                        "Item `{}` x`{}`",
                        row.try_get::<i64, _>("item_id").unwrap_or_default(),
                        row.try_get::<i64, _>("amount").unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun item n’a été trouvé dans le storage.",
        ))
    }

    pub async fn guild_storage_lines(&self, guild_name: &str, limit: u32) -> Result<Vec<String>> {
        if !self.table_exists("guild_storage").await? {
            return Ok(vec!["Table `guild_storage` absente.".to_string()]);
        }
        let rows = sqlx::query(
            r#"
            SELECT CAST(gs.nameid AS SIGNED) AS item_id, CAST(gs.amount AS SIGNED) AS amount
            FROM `guild_storage` gs
            INNER JOIN `guild` g ON g.guild_id = gs.guild_id
            WHERE g.name = ?
            ORDER BY gs.nameid ASC
            LIMIT ?
            "#,
        )
        .bind(guild_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération du storage de guilde")?;

        Ok(join_limited_lines(
            rows.into_iter()
                .map(|row| {
                    format!(
                        "Item `{}` x`{}`",
                        row.try_get::<i64, _>("item_id").unwrap_or_default(),
                        row.try_get::<i64, _>("amount").unwrap_or_default()
                    )
                })
                .collect(),
            "Aucun item n’a été trouvé dans le storage de guilde.",
        ))
    }
}
