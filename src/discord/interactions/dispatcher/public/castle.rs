use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_castle_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "list" => self.handle_castles(context, command).await,
            "info" => self.handle_castle(context, command).await,
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/castle")],
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_castles(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, CASTLE_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let castles = cached_data(
            "castles",
            format!("limit={query_limit}"),
            self.state.config.cache.duration(CASTLES_CACHE_TTL_SECONDS),
            &self.state.cache.castles,
            async { self.state.database.castles(query_limit).await },
        )
        .await?;

        self.respond_embed(
            context,
            command,
            embeds::castles_embed_l10n(self.locale_for_command(command), &castles, display_limit),
            false,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_castle(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(castle_id) = non_negative_integer_option(command, "castle_id") else {
            return self
                .respond_missing_option(context, command, "castle_id")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, CASTLE_TABLES)
            .await?
        {
            return Ok(());
        }

        let castle = self.state.database.castle_details(castle_id).await?;
        let embed = match castle {
            Some(castle) => {
                embeds::castle_detail_embed_l10n(self.locale_for_command(command), &castle)
            }
            None => {
                embeds::castle_not_found_embed_l10n(self.locale_for_command(command), castle_id)
            }
        };

        self.respond_embed(context, command, embed, false).await
    }
}
