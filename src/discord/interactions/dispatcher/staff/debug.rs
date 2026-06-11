use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_debug_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, self.state.config.commands.debug_min_role) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        match subcommand_name(command).unwrap_or("quest") {
            "quest" => self.handle_charquests(context, command).await,
            "char-vars" => {
                self.handle_variable_command(context, command, "char_reg_num")
                    .await
            }
            "acc-vars" => {
                self.handle_variable_command(context, command, "acc_reg_num")
                    .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/debug")],
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_db_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, StaffRole::Owner) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        match subcommand_name(command).unwrap_or("health") {
            "health" => {
                let lines = self.state.database.release_health_lines().await?;
                self.respond_lines_key(context, command, I18nKey::TitleDbHealth, lines, true)
                    .await
            }
            "tables" => {
                let (_display_limit, query_limit) = self.list_limits(command);
                let lines = self
                    .state
                    .database
                    .detected_rathena_tables(query_limit)
                    .await?;
                self.respond_lines_or_empty_key(
                    context,
                    command,
                    I18nKey::TitleDebugTables,
                    lines,
                    I18nKey::ErrorNoResult,
                    true,
                )
                .await
            }
            "count" => {
                let lines = self.state.database.useful_table_counts().await?;
                self.respond_lines_or_empty_key(
                    context,
                    command,
                    I18nKey::TitleTableCounters,
                    lines,
                    I18nKey::ErrorNoResult,
                    true,
                )
                .await
            }
            "logs-size" => {
                let lines = self.state.database.log_table_sizes().await?;
                self.respond_lines_key(context, command, I18nKey::TitleLogVolume, lines, true)
                    .await
            }
            "last-update" => {
                let (_display_limit, query_limit) = self.list_limits(command);
                let lines = self.state.database.sql_updates_lines(query_limit).await?;
                self.respond_lines_key(context, command, I18nKey::TitleSqlUpdates, lines, true)
                    .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/db")],
                )
                .await
            }
        }
    }
}
