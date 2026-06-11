use super::*;

impl RAthenaFrDatabase {
    pub async fn account_characters(
        &self,
        account_id: i64,
        limit: u32,
    ) -> Result<Vec<AccountCharacterSummary>> {
        let rows = sqlx::query(
            r#"
            SELECT
                CAST(c.char_num AS SIGNED) AS slot,
                c.name,
                CAST(c.class AS SIGNED) AS class_id,
                CAST(c.base_level AS SIGNED) AS base_level,
                CAST(c.job_level AS SIGNED) AS job_level,
                CAST(c.online AS SIGNED) AS online,
                c.last_map,
                CAST(c.zeny AS SIGNED) AS zeny,
                g.name AS guild_name
            FROM `login` l
            INNER JOIN `char` c ON c.account_id = l.account_id
            LEFT JOIN `guild` g ON g.guild_id = c.guild_id
            WHERE l.account_id = ?
            ORDER BY c.char_num ASC, c.name ASC
            LIMIT ?
            "#,
        )
        .bind(account_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context("récupération des personnages du compte")?;

        rows.into_iter()
            .map(|row| {
                Ok(AccountCharacterSummary {
                    slot: row.try_get("slot")?,
                    name: row.try_get("name")?,
                    class_id: row.try_get("class_id")?,
                    base_level: row.try_get("base_level")?,
                    job_level: row.try_get("job_level")?,
                    online: row.try_get::<i32, _>("online")? == 1,
                    map: row.try_get("last_map")?,
                    zeny: row.try_get("zeny")?,
                    guild_name: row.try_get("guild_name")?,
                })
            })
            .collect()
    }

    pub async fn account_status(&self, account_id: i64) -> Result<Option<AccountStatus>> {
        let row = sqlx::query(
            r#"
            SELECT
                CAST(l.account_id AS SIGNED) AS account_id,
                l.userid,
                l.sex,
                CAST(l.group_id AS SIGNED) AS group_id,
                CAST(l.state AS SIGNED) AS state,
                CAST(l.unban_time AS SIGNED) AS unban_time,
                CAST(l.expiration_time AS SIGNED) AS expiration_time,
                CAST(l.logincount AS SIGNED) AS logincount,
                CAST(l.character_slots AS SIGNED) AS character_slots,
                DATE_FORMAT(l.lastlogin, '%Y-%m-%d %H:%i:%s') AS lastlogin,
                CAST(COUNT(c.char_id) AS SIGNED) AS characters,
                CAST(COALESCE(SUM(CASE WHEN c.online = 1 THEN 1 ELSE 0 END), 0) AS SIGNED) AS online_characters,
                CAST(COALESCE(SUM(c.zeny), 0) AS SIGNED) AS total_zeny
            FROM `login` l
            LEFT JOIN `char` c ON c.account_id = l.account_id
            WHERE l.account_id = ?
            GROUP BY
                l.account_id,
                l.userid,
                l.sex,
                l.group_id,
                l.state,
                l.unban_time,
                l.expiration_time,
                l.logincount,
                l.character_slots,
                l.lastlogin
            LIMIT 1
            "#,
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await
        .context("récupération du statut du compte")?;

        match row {
            Some(row) => Ok(Some(AccountStatus {
                account_id: row.try_get("account_id")?,
                userid: row.try_get("userid")?,
                sex: row.try_get("sex")?,
                group_id: row.try_get("group_id")?,
                state: row.try_get("state")?,
                unban_time: row.try_get("unban_time")?,
                expiration_time: row.try_get("expiration_time")?,
                logincount: row.try_get("logincount")?,
                character_slots: row.try_get("character_slots")?,
                characters: row.try_get("characters")?,
                online_characters: row.try_get("online_characters")?,
                total_zeny: row.try_get("total_zeny")?,
                lastlogin: row.try_get("lastlogin")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn account_status_by_userid(&self, userid: &str) -> Result<Option<AccountStatus>> {
        let row = sqlx::query(
            r#"
            SELECT CAST(`account_id` AS SIGNED) AS account_id
            FROM `login`
            WHERE `userid` = ?
            LIMIT 1
            "#,
        )
        .bind(userid)
        .fetch_optional(&self.pool)
        .await
        .context("récupération du compte par userid")?;

        let Some(row) = row else {
            return Ok(None);
        };
        let account_id = row.try_get("account_id")?;

        self.account_status(account_id).await
    }

    pub async fn update_account_field(
        &self,
        account_id: i64,
        field: AccountManageField,
        value: &str,
    ) -> Result<()> {
        self.ensure_login_column(field.name()).await?;

        match field {
            AccountManageField::GroupId => {
                let parsed = parse_account_manage_i64(value, field.name())?;
                self.execute_account_i64_update(
                    "UPDATE `login` SET `group_id` = ? WHERE `account_id` = ?",
                    parsed,
                    account_id,
                    "modification du groupe de compte",
                )
                .await
            }
            AccountManageField::State => {
                let parsed = parse_account_manage_i64(value, field.name())?;
                self.execute_account_i64_update(
                    "UPDATE `login` SET `state` = ? WHERE `account_id` = ?",
                    parsed,
                    account_id,
                    "modification de l’état de compte",
                )
                .await
            }
            AccountManageField::UnbanTime => {
                let parsed = parse_account_manage_i64(value, field.name())?;
                self.execute_account_i64_update(
                    "UPDATE `login` SET `unban_time` = ? WHERE `account_id` = ?",
                    parsed,
                    account_id,
                    "modification du temps de déban",
                )
                .await
            }
            AccountManageField::ExpirationTime => {
                let parsed = parse_account_manage_i64(value, field.name())?;
                self.execute_account_i64_update(
                    "UPDATE `login` SET `expiration_time` = ? WHERE `account_id` = ?",
                    parsed,
                    account_id,
                    "modification de l’expiration de compte",
                )
                .await
            }
            AccountManageField::Logincount => {
                let parsed = parse_account_manage_i64(value, field.name())?;
                self.execute_account_i64_update(
                    "UPDATE `login` SET `logincount` = ? WHERE `account_id` = ?",
                    parsed,
                    account_id,
                    "modification du compteur de connexions",
                )
                .await
            }
            AccountManageField::Sex => {
                self.execute_account_text_update(
                    "UPDATE `login` SET `sex` = ? WHERE `account_id` = ?",
                    value,
                    account_id,
                    "modification du sexe de compte",
                )
                .await
            }
        }
    }

    pub async fn ban_account(&self, account_id: i64, until: Option<i64>) -> Result<()> {
        let columns = self.login_columns().await?;
        let has_state = columns.first(&["state"]).is_some();
        let has_unban_time = columns.first(&["unban_time"]).is_some();

        if until.is_some() && !has_unban_time {
            anyhow::bail!("La colonne `unban_time` est absente de `login`.");
        }
        if !has_state && !has_unban_time {
            anyhow::bail!("Aucune colonne de bannissement utilisable dans `login`.");
        }

        let mut set_clauses = Vec::new();
        if has_state {
            set_clauses.push("`state` = ?");
        }
        if has_unban_time {
            set_clauses.push("`unban_time` = ?");
        }
        let sql = format!(
            "UPDATE `login` SET {} WHERE `account_id` = ?",
            set_clauses.join(", ")
        );
        let mut query = sqlx::query(&sql);
        if has_state {
            query = query.bind(ACCOUNT_STATE_BLOCKED);
        }
        if has_unban_time {
            query = query.bind(until.unwrap_or(ACCOUNT_NO_UNBAN_TIME));
        }

        self.execute_bound_account_update(query.bind(account_id), "bannissement de compte")
            .await
    }

    pub async fn unban_account(&self, account_id: i64) -> Result<()> {
        let columns = self.login_columns().await?;
        let has_state = columns.first(&["state"]).is_some();
        let has_unban_time = columns.first(&["unban_time"]).is_some();

        if !has_state && !has_unban_time {
            anyhow::bail!("Aucune colonne de bannissement utilisable dans `login`.");
        }

        let mut set_clauses = Vec::new();
        if has_state {
            set_clauses.push("`state` = ?");
        }
        if has_unban_time {
            set_clauses.push("`unban_time` = ?");
        }
        let sql = format!(
            "UPDATE `login` SET {} WHERE `account_id` = ?",
            set_clauses.join(", ")
        );
        let mut query = sqlx::query(&sql);
        if has_state {
            query = query.bind(ACCOUNT_STATE_ACTIVE);
        }
        if has_unban_time {
            query = query.bind(ACCOUNT_NO_UNBAN_TIME);
        }

        self.execute_bound_account_update(query.bind(account_id), "déblocage de compte")
            .await
    }

    pub async fn strongly_disable_account(&self, account_id: i64) -> Result<()> {
        self.ensure_login_column("state").await?;
        self.ensure_login_column("group_id").await?;
        self.ensure_login_column("expiration_time").await?;

        let columns = self.login_columns().await?;
        let has_unban_time = columns.first(&["unban_time"]).is_some();
        let sql = if has_unban_time {
            "UPDATE `login` SET `state` = ?, `group_id` = ?, `expiration_time` = ?, `unban_time` = ? WHERE `account_id` = ?"
        } else {
            "UPDATE `login` SET `state` = ?, `group_id` = ?, `expiration_time` = ? WHERE `account_id` = ?"
        };
        let mut query = sqlx::query(sql)
            .bind(ACCOUNT_STATE_BLOCKED)
            .bind(ACCOUNT_DEFAULT_GROUP_ID)
            .bind(ACCOUNT_NO_EXPIRATION_TIME);
        if has_unban_time {
            query = query.bind(ACCOUNT_NO_UNBAN_TIME);
        }

        self.execute_bound_account_update(query.bind(account_id), "désactivation forte de compte")
            .await
    }

    pub(super) async fn login_columns(&self) -> Result<AvailableColumns> {
        self.table_columns("login")
            .await?
            .ok_or_else(|| anyhow::anyhow!("La table `login` est absente."))
    }

    pub(super) async fn ensure_login_column(&self, column_name: &str) -> Result<()> {
        let columns = self.login_columns().await?;
        if columns.first(&[column_name]).is_none() {
            anyhow::bail!("La colonne `login`.`{column_name}` est absente.");
        }

        Ok(())
    }

    pub(super) async fn execute_account_i64_update(
        &self,
        sql: &str,
        value: i64,
        account_id: i64,
        context: &str,
    ) -> Result<()> {
        let query = sqlx::query(sql).bind(value).bind(account_id);
        self.execute_bound_account_update(query, context).await
    }

    pub(super) async fn execute_account_text_update(
        &self,
        sql: &str,
        value: &str,
        account_id: i64,
        context: &str,
    ) -> Result<()> {
        let query = sqlx::query(sql).bind(value).bind(account_id);
        self.execute_bound_account_update(query, context).await
    }

    pub(super) async fn execute_bound_account_update<'q>(
        &self,
        query: sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments>,
        context: &str,
    ) -> Result<()> {
        let result = query
            .execute(&self.pool)
            .await
            .context(context.to_string())?;
        if result.rows_affected() == 0 {
            anyhow::bail!("Aucun compte n’a été modifié.");
        }

        Ok(())
    }

    pub async fn create_account(
        &self,
        userid: &str,
        password: &str,
        password_mode: AccountPasswordMode,
        sex: &str,
        birthdate: &str,
        email: &str,
    ) -> Result<CreatedAccount> {
        if self.account_userid_exists(userid).await? {
            anyhow::bail!("Le compte `{userid}` existe déjà.");
        }

        let sql = match password_mode {
            AccountPasswordMode::Plain => {
                r#"
                INSERT INTO `login` (userid, user_pass, sex, birthdate, email)
                VALUES (?, ?, ?, ?, ?)
                "#
            }
            AccountPasswordMode::Md5 => {
                r#"
                INSERT INTO `login` (userid, user_pass, sex, birthdate, email)
                VALUES (?, MD5(?), ?, ?, ?)
                "#
            }
        };

        let result = sqlx::query(sql)
            .bind(userid)
            .bind(password)
            .bind(sex)
            .bind(birthdate)
            .bind(email)
            .execute(&self.pool)
            .await
            .context("create rAthena account")?;

        Ok(CreatedAccount {
            account_id: i64::try_from(result.last_insert_id()).unwrap_or(i64::MAX),
            userid: userid.to_string(),
            sex: sex.to_string(),
            email: email.to_string(),
        })
    }

    pub async fn account_userid_exists(&self, userid: &str) -> Result<bool> {
        let existing = sqlx::query(
            r#"
            SELECT CAST(COUNT(*) AS SIGNED) AS account_count
            FROM `login`
            WHERE userid = ?
            "#,
        )
        .bind(userid)
        .fetch_one(&self.pool)
        .await
        .context("check account username availability")?;

        let account_count: i64 = existing.try_get("account_count")?;
        Ok(account_count > 0)
    }

    pub async fn account_id_for_character(&self, character_name: &str) -> Result<Option<i64>> {
        let row = sqlx::query(
            r#"
            SELECT CAST(account_id AS SIGNED) AS account_id
            FROM `char`
            WHERE name = ?
            LIMIT 1
            "#,
        )
        .bind(character_name)
        .fetch_optional(&self.pool)
        .await
        .context("récupération de l’account_id du personnage")?;

        row.map(|row| row.try_get("account_id").map_err(Into::into))
            .transpose()
    }

    pub async fn account_status_by_character(
        &self,
        character_name: &str,
    ) -> Result<Option<AccountStatus>> {
        let Some(account_id) = self.account_id_for_character(character_name).await? else {
            return Ok(None);
        };

        self.account_status(account_id).await
    }
}
