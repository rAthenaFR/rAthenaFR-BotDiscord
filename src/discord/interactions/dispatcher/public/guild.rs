use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_guild_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "info" => self.handle_guild(context, command).await,
            "members" => self.handle_guildmembers(context, command).await,
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/guild")],
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_guild(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self.respond_missing_option(context, command, "name").await;
        };

        if !self
            .ensure_database_tables(context, command, GUILD_TABLES)
            .await?
        {
            return Ok(());
        }

        let guild = self
            .state
            .database
            .find_guild(
                name,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;
        let embed = match guild {
            Some(guild) => {
                embeds::guild_detail_embed_l10n(self.locale_for_command(command), &guild)
            }
            None => embeds::guild_not_found_embed_l10n(self.locale_for_command(command), name),
        };

        self.respond_embed(context, command, embed, false).await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_guildmembers(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self.respond_missing_option(context, command, "name").await;
        };

        if !self
            .ensure_database_tables(context, command, GUILD_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let members = self
            .state
            .database
            .guild_members(
                name,
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::guild_members_embed_l10n(
                self.locale_for_command(command),
                name,
                &members,
                display_limit,
            ),
            false,
        )
        .await
    }
}
