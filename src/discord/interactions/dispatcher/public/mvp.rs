use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_mvp_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        match subcommand_name(command).unwrap_or("list") {
            "list" => {
                let lines = self
                    .state
                    .database
                    .mvp_list_lines(
                        &self.state.config.commands.mob_table_name,
                        MVP_LIST_FETCH_LIMIT,
                    )
                    .await?;
                self.respond_mvp_list_panel(context, command, lines, 0, display_limit as usize)
                    .await
            }
            "last" => {
                let display_limit = display_limit.min(MVP_LAST_DISPLAY_LIMIT);
                let entries = self
                    .state
                    .database
                    .mvp_last_entries(display_limit.saturating_add(1))
                    .await?;
                self.respond_embed(
                    context,
                    command,
                    embeds::mvp_last_embed_l10n(
                        self.locale_for_command(command),
                        &entries,
                        display_limit,
                    ),
                    false,
                )
                .await
            }
            "top" => {
                let lines = self
                    .state
                    .database
                    .mvp_top_lines(&self.state.config.commands.mob_table_name, query_limit)
                    .await?;
                self.respond_lines_key(context, command, I18nKey::TitleTopMvp, lines, false)
                    .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/mvp")],
                )
                .await
            }
        }
    }
}
