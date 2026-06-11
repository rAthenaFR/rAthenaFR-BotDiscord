use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_mod_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, StaffRole::Moderator) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        match subcommand_name(command).unwrap_or("chatlog") {
            "chatlog" => {
                self.handle_character_log_command(context, command, "chatlog")
                    .await
            }
            "chat-search" => self.handle_chat_search(context, command).await,
            "report-context" => self.handle_report_context(context, command).await,
            "branchlog" => {
                self.handle_character_log_command(context, command, "branchlog")
                    .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/mod")],
                )
                .await
            }
        }
    }
}
