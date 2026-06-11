use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_online_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("count") {
            "count" => {
                let status = self
                    .state
                    .database
                    .database_status(self.state.config.display.public_character_group_threshold())
                    .await?;
                self.respond_embed(
                    context,
                    command,
                    embeds::text_embed(
                        &self.tr_raw(command, "title-online-count"),
                        self.tr_raw_args(
                            command,
                            "online-count-description",
                            &[TranslationArg::new(
                                "count",
                                &status.online_characters.to_string(),
                            )],
                        ),
                    ),
                    false,
                )
                .await
            }
            "list" => {
                if !self.state.config.commands.online_list_public {
                    return self
                        .respond_error_raw(context, command, "error-online-list-disabled")
                        .await;
                }

                self.handle_online(context, command).await
            }
            "map" => {
                let (display_limit, query_limit) = self.list_limits(command);
                let entries = self
                    .state
                    .database
                    .map_stats(
                        self.state.config.display.public_character_group_threshold(),
                        true,
                        query_limit,
                    )
                    .await?;

                self.respond_embed(
                    context,
                    command,
                    embeds::map_stats_embed_l10n(
                        self.locale_for_command(command),
                        &entries,
                        true,
                        display_limit,
                    ),
                    false,
                )
                .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/online")],
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_online(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let characters = self
            .state
            .database
            .online_characters(
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::online_embed_l10n(self.locale_for_command(command), &characters, display_limit),
            false,
        )
        .await
    }
}
