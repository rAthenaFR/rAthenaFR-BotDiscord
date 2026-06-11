use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_account_manage_edit(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let options = account_manage_options(command);
        let prepared = match account_manage::prepare_edit(&options) {
            Ok(prepared) => prepared,
            Err(message) if message.starts_with("Option obligatoire") => {
                return self
                    .respond_error(
                        context,
                        command,
                        &self.localize_known_error(command, &message),
                    )
                    .await;
            }
            Err(message) => {
                let account = account_manage::requested_account(&options)
                    .unwrap_or_else(|| "non renseigné".to_string());
                return self
                    .reject_account_manage(context, command, "edit", &account, &message)
                    .await;
            }
        };
        let Some(account) =
            account_manage::resolve_account(&self.state.database, prepared.lookup).await?
        else {
            let error = self.tr(command, I18nKey::ErrorAccountExactNotFound);
            return self
                .reject_account_manage(context, command, "edit", prepared.lookup, &error)
                .await;
        };
        let account_label = account_manage::account_label(&account);

        if let Err(error) = self
            .state
            .database
            .update_account_field(account.account_id, prepared.field, &prepared.value)
            .await
        {
            self.log_account_manage_sql_error(context, command, "edit", &account_label, &error)
                .await;
            return Err(error);
        }
        let updated = match self.state.database.account_status(account.account_id).await {
            Ok(Some(updated)) => updated,
            Ok(None) => account,
            Err(error) => {
                self.log_account_manage_sql_error(context, command, "edit", &account_label, &error)
                    .await;
                return Err(error);
            }
        };
        let account_label = account_manage::account_label(&updated);
        let result = account_manage::edit_result(prepared.field, &updated);

        self.staff_audit_logger(context, command)
            .log_account_manage(AccountManageAuditEntry {
                status: embeds::AccountManageLogStatus::Success,
                action: "edit",
                account: &account_label,
                result: &result,
                reason: prepared.reason.as_deref(),
            })
            .await;

        self.respond_embed(
            context,
            command,
            embeds::success_message_embed_key(
                self.locale_for_command(command),
                I18nKey::EmbedAccountModifiedTitle,
                I18nKey::EmbedAccountEditSuccess,
            )
            .field(self.tr(command, I18nKey::FieldAction), "`edit`", true)
            .field(
                self.tr(command, I18nKey::FieldAccount),
                account_manage::summary(&updated),
                false,
            )
            .field(
                self.tr(command, I18nKey::FieldField),
                format!("`{}`", prepared.field.name()),
                true,
            )
            .field(
                self.tr(command, I18nKey::FieldValue),
                format!("`{}`", prepared.value),
                true,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_account_manage_ban(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let options = account_manage_options(command);
        let prepared = match account_manage::prepare_ban(&options) {
            Ok(prepared) => prepared,
            Err(message) => {
                return self
                    .respond_error(
                        context,
                        command,
                        &self.localize_known_error(command, &message),
                    )
                    .await
            }
        };
        let Some(account) =
            account_manage::resolve_account(&self.state.database, prepared.lookup).await?
        else {
            let error = self.tr(command, I18nKey::ErrorAccountExactNotFound);
            return self
                .reject_account_manage(context, command, "ban", prepared.lookup, &error)
                .await;
        };
        let account_label = account_manage::account_label(&account);

        if let Err(error) = self
            .state
            .database
            .ban_account(account.account_id, prepared.until)
            .await
        {
            self.log_account_manage_sql_error(context, command, "ban", &account_label, &error)
                .await;
            return Err(error);
        }
        let updated = match self.state.database.account_status(account.account_id).await {
            Ok(Some(updated)) => updated,
            Ok(None) => account,
            Err(error) => {
                self.log_account_manage_sql_error(context, command, "ban", &account_label, &error)
                    .await;
                return Err(error);
            }
        };
        let account_label = account_manage::account_label(&updated);
        let result = account_manage::ban_result(prepared.until, &updated);

        self.staff_audit_logger(context, command)
            .log_account_manage(AccountManageAuditEntry {
                status: embeds::AccountManageLogStatus::Success,
                action: "ban",
                account: &account_label,
                result: &result,
                reason: prepared.reason.as_deref(),
            })
            .await;

        self.respond_embed(
            context,
            command,
            embeds::success_message_embed_key(
                self.locale_for_command(command),
                I18nKey::EmbedAccountModifiedTitle,
                I18nKey::EmbedAccountBanSuccess,
            )
            .field(self.tr(command, I18nKey::FieldAction), "`ban`", true)
            .field(
                self.tr(command, I18nKey::FieldAccount),
                account_manage::summary(&updated),
                false,
            )
            .field(
                self.tr(command, I18nKey::FieldBanEnd),
                prepared
                    .until
                    .map(|value| format!("`{value}`"))
                    .unwrap_or_else(|| "`0`".to_string()),
                true,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_account_manage_unban(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let options = account_manage_options(command);
        let prepared = match account_manage::prepare_unban(&options) {
            Ok(prepared) => prepared,
            Err(message) => {
                return self
                    .respond_error(
                        context,
                        command,
                        &self.localize_known_error(command, &message),
                    )
                    .await
            }
        };
        let Some(account) =
            account_manage::resolve_account(&self.state.database, prepared.lookup).await?
        else {
            let error = self.tr(command, I18nKey::ErrorAccountExactNotFound);
            return self
                .reject_account_manage(context, command, "unban", prepared.lookup, &error)
                .await;
        };
        let account_label = account_manage::account_label(&account);

        if let Err(error) = self.state.database.unban_account(account.account_id).await {
            self.log_account_manage_sql_error(context, command, "unban", &account_label, &error)
                .await;
            return Err(error);
        }
        let updated = match self.state.database.account_status(account.account_id).await {
            Ok(Some(updated)) => updated,
            Ok(None) => account,
            Err(error) => {
                self.log_account_manage_sql_error(
                    context,
                    command,
                    "unban",
                    &account_label,
                    &error,
                )
                .await;
                return Err(error);
            }
        };
        let account_label = account_manage::account_label(&updated);
        let result = account_manage::unban_result(&updated);

        self.staff_audit_logger(context, command)
            .log_account_manage(AccountManageAuditEntry {
                status: embeds::AccountManageLogStatus::Success,
                action: "unban",
                account: &account_label,
                result: &result,
                reason: prepared.reason.as_deref(),
            })
            .await;

        self.respond_embed(
            context,
            command,
            embeds::success_message_embed_key(
                self.locale_for_command(command),
                I18nKey::EmbedAccountModifiedTitle,
                I18nKey::EmbedAccountUnbanSuccess,
            )
            .field(self.tr(command, I18nKey::FieldAction), "`unban`", true)
            .field(
                self.tr(command, I18nKey::FieldAccount),
                account_manage::summary(&updated),
                false,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_account_manage_delete(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let options = account_manage_options(command);
        let prepared =
            match account_manage::prepare_delete(&self.state.config.account_commands, &options) {
                Ok(prepared) => prepared,
                Err(message) if message.starts_with("Option obligatoire") => {
                    return self
                        .respond_error(
                            context,
                            command,
                            &self.localize_known_error(command, &message),
                        )
                        .await;
                }
                Err(message) => {
                    let account = account_manage::requested_account(&options)
                        .unwrap_or_else(|| "non renseigné".to_string());
                    return self
                        .reject_account_manage(context, command, "delete", &account, &message)
                        .await;
                }
            };

        let Some(account) = self
            .state
            .database
            .account_status(prepared.account_id)
            .await?
        else {
            let account_id = prepared.account_id.to_string();
            let error = self.tr(command, I18nKey::ErrorAccountIdExactNotFound);
            return self
                .reject_account_manage(context, command, "delete", &account_id, &error)
                .await;
        };
        let before_summary = account_manage::summary(&account);
        let account_label = account_manage::account_label(&account);

        if let Err(error) = self
            .state
            .database
            .strongly_disable_account(account.account_id)
            .await
        {
            self.log_account_manage_sql_error(context, command, "delete", &account_label, &error)
                .await;
            return Err(error);
        }
        let updated = match self.state.database.account_status(account.account_id).await {
            Ok(Some(updated)) => updated,
            Ok(None) => account,
            Err(error) => {
                self.log_account_manage_sql_error(
                    context,
                    command,
                    "delete",
                    &account_label,
                    &error,
                )
                .await;
                return Err(error);
            }
        };
        let account_label = account_manage::account_label(&updated);
        let result = account_manage::delete_result(&updated);

        self.staff_audit_logger(context, command)
            .log_account_manage(AccountManageAuditEntry {
                status: embeds::AccountManageLogStatus::Success,
                action: "delete",
                account: &account_label,
                result: &result,
                reason: prepared.reason.as_deref(),
            })
            .await;

        self.respond_embed(
            context,
            command,
            embeds::success_message_embed_key(
                self.locale_for_command(command),
                I18nKey::EmbedAccountModifiedTitle,
                I18nKey::EmbedAccountDeleteSuccess,
            )
            .field(
                self.tr(command, I18nKey::FieldAction),
                "`delete` soft",
                true,
            )
            .field(
                self.tr(command, I18nKey::FieldBeforeSummary),
                before_summary,
                false,
            )
            .field(
                self.tr(command, I18nKey::FieldAfterSummary),
                account_manage::summary(&updated),
                false,
            ),
            true,
        )
        .await
    }
}
