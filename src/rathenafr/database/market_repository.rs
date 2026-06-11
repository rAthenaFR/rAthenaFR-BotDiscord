use super::*;

impl RAthenaFrDatabase {
    pub async fn who_sell(
        &self,
        item_id: i64,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<MarketSellEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS merchant_name,
                v.title AS shop_title,
                v.map,
                CAST(v.x AS SIGNED) AS x,
                CAST(v.y AS SIGNED) AS y,
                CAST(vi.amount AS SIGNED) AS item_amount,
                CAST(vi.price AS SIGNED) AS item_price
            FROM `vendings` v
            INNER JOIN `vending_items` vi ON vi.vending_id = v.id
            INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
            INNER JOIN `char` c ON c.char_id = v.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE ci.nameid = ? AND l.group_id < ?
            ORDER BY vi.price ASC, vi.amount DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(item_id)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des vendeurs vending")?;

        rows.into_iter()
            .map(|row| {
                Ok(MarketSellEntry {
                    merchant_name: row.try_get("merchant_name")?,
                    shop_title: row.try_get("shop_title")?,
                    map: row.try_get("map")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    amount: row.try_get("item_amount")?,
                    price: row.try_get("item_price")?,
                })
            })
            .collect()
    }

    pub async fn who_buy(
        &self,
        item_id: i64,
        group_threshold: i32,
        limit: u32,
    ) -> Result<Vec<MarketBuyEntry>> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.name AS buyer_name,
                bs.title AS shop_title,
                bs.map,
                CAST(bs.x AS SIGNED) AS x,
                CAST(bs.y AS SIGNED) AS y,
                CAST(bsi.amount AS SIGNED) AS item_amount,
                CAST(bsi.price AS SIGNED) AS item_price
            FROM `buyingstores` bs
            INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
            INNER JOIN `char` c ON c.char_id = bs.char_id
            INNER JOIN `login` l ON l.account_id = c.account_id
            WHERE bsi.item_id = ? AND l.group_id < ?
            ORDER BY bsi.price DESC, bsi.amount DESC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(item_id)
        .bind(group_threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des acheteurs buying store")?;

        rows.into_iter()
            .map(|row| {
                Ok(MarketBuyEntry {
                    buyer_name: row.try_get("buyer_name")?,
                    shop_title: row.try_get("shop_title")?,
                    map: row.try_get("map")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    amount: row.try_get("item_amount")?,
                    price: row.try_get("item_price")?,
                })
            })
            .collect()
    }

    pub async fn market_overview(
        &self,
        item_id: i64,
        group_threshold: i32,
    ) -> Result<MarketOverview> {
        let row = sqlx::query(
            r#"
            SELECT
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `vendings` v
                    INNER JOIN `vending_items` vi ON vi.vending_id = v.id
                    INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
                    INNER JOIN `char` c ON c.char_id = v.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE ci.nameid = ? AND l.group_id < ?
                ) AS sellers,
                COALESCE((
                    SELECT CAST(SUM(vi.amount) AS SIGNED)
                    FROM `vendings` v
                    INNER JOIN `vending_items` vi ON vi.vending_id = v.id
                    INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
                    INNER JOIN `char` c ON c.char_id = v.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE ci.nameid = ? AND l.group_id < ?
                ), 0) AS sell_amount,
                (
                    SELECT CAST(MIN(vi.price) AS SIGNED)
                    FROM `vendings` v
                    INNER JOIN `vending_items` vi ON vi.vending_id = v.id
                    INNER JOIN `cart_inventory` ci ON ci.id = vi.cartinventory_id
                    INNER JOIN `char` c ON c.char_id = v.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE ci.nameid = ? AND l.group_id < ?
                ) AS lowest_sell_price,
                (
                    SELECT CAST(COUNT(*) AS SIGNED)
                    FROM `buyingstores` bs
                    INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
                    INNER JOIN `char` c ON c.char_id = bs.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE bsi.item_id = ? AND l.group_id < ?
                ) AS buyers,
                COALESCE((
                    SELECT CAST(SUM(bsi.amount) AS SIGNED)
                    FROM `buyingstores` bs
                    INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
                    INNER JOIN `char` c ON c.char_id = bs.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE bsi.item_id = ? AND l.group_id < ?
                ), 0) AS buy_amount,
                (
                    SELECT CAST(MAX(bsi.price) AS SIGNED)
                    FROM `buyingstores` bs
                    INNER JOIN `buyingstore_items` bsi ON bsi.buyingstore_id = bs.id
                    INNER JOIN `char` c ON c.char_id = bs.char_id
                    INNER JOIN `login` l ON l.account_id = c.account_id
                    WHERE bsi.item_id = ? AND l.group_id < ?
                ) AS highest_buy_price
            "#,
        )
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .bind(item_id)
        .bind(group_threshold)
        .fetch_one(&self.pool)
        .await
        .context("récupération de la vue d’ensemble du marché")?;

        Ok(MarketOverview {
            item_id,
            sellers: row.try_get("sellers")?,
            sell_amount: row.try_get("sell_amount")?,
            lowest_sell_price: row.try_get("lowest_sell_price")?,
            buyers: row.try_get("buyers")?,
            buy_amount: row.try_get("buy_amount")?,
            highest_buy_price: row.try_get("highest_buy_price")?,
        })
    }
}
