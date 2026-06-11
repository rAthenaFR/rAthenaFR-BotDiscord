use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_mob_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "info" => {
                let Some(mob) = string_option(command, "mob") else {
                    return self.respond_missing_option(context, command, "mob").await;
                };
                let lines = self
                    .state
                    .database
                    .mob_detail_lines(
                        mob,
                        &self.state.config.commands.mob_table_name,
                        &self.state.config.server_rates,
                    )
                    .await?;
                match lines {
                    Some(lines) => {
                        self.respond_lines_key(
                            context,
                            command,
                            I18nKey::TitleMobDetails,
                            lines,
                            false,
                        )
                        .await
                    }
                    None => {
                        self.respond_error_key(context, command, I18nKey::ErrorMobNotFoundGeneric)
                            .await
                    }
                }
            }
            "drops" => {
                let Some(mob) = string_option(command, "mob") else {
                    return self.respond_missing_option(context, command, "mob").await;
                };
                let drops = self
                    .state
                    .database
                    .mob_drops(
                        mob,
                        &self.state.config.commands.mob_table_name,
                        &self.state.config.server_rates,
                    )
                    .await?;
                match drops {
                    Some(drops) => {
                        self.respond_embed(
                            context,
                            command,
                            embeds::mob_drops_embed_l10n(self.locale_for_command(command), &drops),
                            false,
                        )
                        .await
                    }
                    None => {
                        self.respond_embed(
                            context,
                            command,
                            embeds::monster_not_found_embed_l10n(self.locale_for_command(command)),
                            false,
                        )
                        .await
                    }
                }
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/mob")],
                )
                .await
            }
        }
    }
}
