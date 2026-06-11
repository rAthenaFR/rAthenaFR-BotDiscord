use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_top_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("level") {
            "level" => self.handle_top(context, command).await,
            "job" => {
                let (display_limit, query_limit) = self.list_limits(command);
                let entries = self
                    .state
                    .database
                    .top_characters_by_job(
                        self.state.config.display.ranking_group_threshold(),
                        query_limit,
                    )
                    .await?;

                self.respond_embed(
                    context,
                    command,
                    embeds::ranking_embed_l10n(
                        self.locale_for_command(command),
                        &entries,
                        display_limit,
                    ),
                    false,
                )
                .await
            }
            "guild" => self.handle_guilds(context, command).await,
            "zeny" => self.handle_top_zeny_configured(context, command).await,
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/top")],
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_top_zeny_configured(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match self.state.config.commands.top_zeny_mode {
            TopZenyMode::Disabled => {
                self.respond_error_raw(context, command, "error-top-zeny-disabled")
                    .await
            }
            TopZenyMode::Enabled => self.handle_topzeny(context, command).await,
            TopZenyMode::Anonymized => {
                let (display_limit, query_limit) = self.list_limits(command);
                let mut entries = self
                    .state
                    .database
                    .top_zeny(
                        self.state.config.display.ranking_group_threshold(),
                        query_limit,
                    )
                    .await?;

                for entry in &mut entries {
                    entry.name = self.tr_raw_args(
                        command,
                        "text-character-number",
                        &[TranslationArg::new("id", &entry.rank.to_string())],
                    );
                }

                self.respond_embed(
                    context,
                    command,
                    embeds::top_zeny_embed_l10n(
                        self.locale_for_command(command),
                        &entries,
                        display_limit,
                    ),
                    false,
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_rank(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self.respond_missing_option(context, command, "name").await;
        };
        let lines = self
            .state
            .database
            .rank_summary_lines(
                name,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;

        match lines {
            Some(lines) => {
                self.respond_lines_key(context, command, I18nKey::TitlePlayerRank, lines, false)
                    .await
            }
            None => {
                self.respond_error_key(context, command, I18nKey::ErrorPlayerNotFoundGeneric)
                    .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_top(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self
            .state
            .database
            .top_characters(
                self.state.config.display.ranking_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::ranking_embed_l10n(self.locale_for_command(command), &entries, display_limit),
            false,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_guilds(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, GUILD_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.ranking_group_threshold();
        let guilds = cached_data(
            "guildes",
            format!("limit={query_limit};group_threshold={group_threshold}"),
            self.state.config.cache.duration(GUILDS_CACHE_TTL_SECONDS),
            &self.state.cache.guilds,
            async {
                self.state
                    .database
                    .top_guilds(group_threshold, query_limit)
                    .await
            },
        )
        .await?;

        self.respond_embed(
            context,
            command,
            embeds::guilds_embed_l10n(self.locale_for_command(command), &guilds, display_limit),
            false,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_topzeny(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self
            .state
            .database
            .top_zeny(
                self.state.config.display.ranking_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::top_zeny_embed_l10n(self.locale_for_command(command), &entries, display_limit),
            false,
        )
        .await
    }
}
