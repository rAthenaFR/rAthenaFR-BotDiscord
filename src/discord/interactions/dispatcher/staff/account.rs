use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_player(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let profile = self.state.database.find_player(i32::MAX, character).await?;
        let embed = match profile {
            Some(profile) => embeds::player_embed_l10n(self.locale_for_command(command), &profile),
            None => {
                embeds::player_not_found_embed_l10n(self.locale_for_command(command), character)
            }
        };
        self.respond_embed(context, command, embed, true).await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_account(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let account = self
            .state
            .database
            .account_status_by_character(character)
            .await?;
        let embed = match account {
            Some(account) => {
                embeds::account_status_embed_l10n(self.locale_for_command(command), &account)
            }
            None => embeds::error_embed_l10n(
                self.locale_for_command(command),
                &self.tr(command, I18nKey::ErrorAccountNotFound),
            ),
        };
        self.respond_embed(context, command, embed, true).await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_chars(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(lookup) = string_option(command, "lookup") else {
            return self
                .respond_missing_option(context, command, "lookup")
                .await;
        };
        let account_id = match lookup.trim().parse::<i64>() {
            Ok(value) if value > 0 => Some(value),
            _ => self.state.database.account_id_for_character(lookup).await?,
        };
        let Some(account_id) = account_id else {
            return self
                .respond_error_key(context, command, I18nKey::ErrorAccountNotFound)
                .await;
        };
        let (display_limit, query_limit) = self.list_limits(command);
        let characters = self
            .state
            .database
            .account_characters(account_id, query_limit)
            .await?;
        self.respond_embed(
            context,
            command,
            embeds::account_characters_embed_l10n(
                self.locale_for_command(command),
                account_id,
                &characters,
                display_limit,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_account_manage(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let action = subcommand_leaf_name(command).unwrap_or("account-manage");
        let options = account_manage_options(command);
        let requested_account = account_manage::requested_account(&options);
        let required_role =
            account_manage::required_role(&self.state.config.account_commands, action);

        if !self.has_staff_role(command, required_role) {
            self.staff_audit_logger(context, command)
                .log_account_manage(AccountManageAuditEntry {
                    status: embeds::AccountManageLogStatus::Refused,
                    action,
                    account: requested_account.as_deref().unwrap_or("non renseigné"),
                    result: "Rôle Discord insuffisant.",
                    reason: None,
                })
                .await;
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        if !self.state.config.account_commands.manage_enabled {
            return self
                .reject_account_manage(
                    context,
                    command,
                    action,
                    requested_account.as_deref().unwrap_or("non renseigné"),
                    "La gestion des comptes est désactivée par configuration.",
                )
                .await;
        }

        if let Some(missing_table) = self
            .state
            .database
            .first_missing_table(account_manage::REQUIRED_TABLES)
            .await?
        {
            let result = format!("Table `{}` absente.", missing_table.name());
            self.staff_audit_logger(context, command)
                .log_account_manage(AccountManageAuditEntry {
                    status: embeds::AccountManageLogStatus::Refused,
                    action,
                    account: requested_account.as_deref().unwrap_or("non renseigné"),
                    result: &result,
                    reason: None,
                })
                .await;
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::missing_database_table_embed_l10n(
                        self.locale_for_command(command),
                        missing_table.name(),
                    ),
                    true,
                )
                .await;
        }

        match action {
            "edit" => self.handle_account_manage_edit(context, command).await,
            "ban" => self.handle_account_manage_ban(context, command).await,
            "unban" => self.handle_account_manage_unban(context, command).await,
            "delete" => self.handle_account_manage_delete(context, command).await,
            _ => {
                self.respond_error(
                    context,
                    command,
                    "Sous-commande /staff account-manage inconnue.",
                )
                .await
            }
        }
    }
}
