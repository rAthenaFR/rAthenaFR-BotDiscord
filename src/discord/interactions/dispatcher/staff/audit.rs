use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_audit_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, self.state.config.commands.audit_min_role) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        let (_display_limit, query_limit) = self.list_limits(command);
        match subcommand_name(command).unwrap_or("atcommands") {
            "atcommands" => {
                let Some(gm) = string_option(command, "gm") else {
                    return self.respond_missing_option(context, command, "gm").await;
                };
                let lines = self
                    .state
                    .database
                    .named_log_lines("atcommandlog", gm, query_limit)
                    .await?;
                self.respond_lines_key(context, command, I18nKey::TitleAuditAtcommands, lines, true)
                    .await
            }
            "item-created" => {
                let lines = self
                    .state
                    .database
                    .recent_log_lines("picklog", query_limit)
                    .await?;
                self.respond_lines_key(
                    context,
                    command,
                    I18nKey::TitleAuditCreatedItems,
                    lines,
                    true,
                )
                .await
            }
            "zeny-created" => {
                let lines = self
                    .state
                    .database
                    .recent_log_lines("zenylog", query_limit)
                    .await?;
                self.respond_lines_key(context, command, I18nKey::TitleAuditZeny, lines, true)
                    .await
            }
            "gm-activity" => {
                let Some(gm) = string_option(command, "gm") else {
                    return self.respond_missing_option(context, command, "gm").await;
                };
                let lines = self
                    .state
                    .database
                    .named_log_lines("atcommandlog", gm, query_limit)
                    .await?;
                self.respond_lines_key(context, command, I18nKey::TitleGmActivity, lines, true)
                    .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/audit")],
                )
                .await
            }
        }
    }
}
