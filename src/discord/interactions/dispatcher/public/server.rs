use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_server(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        self.handle_status(context, command).await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_status(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let cache_key = format!("group_threshold={group_threshold}");
        let cache_entry = cached_data(
            "status",
            cache_key,
            self.state.config.cache.duration(STATUS_CACHE_TTL_SECONDS),
            &self.state.cache.status,
            async {
                let status = self.state.database.database_status(group_threshold).await?;
                let endpoints = self.state.config.services.endpoints();
                let services = check_services(&endpoints).await;

                Ok(StatusCacheEntry { status, services })
            },
        )
        .await?;

        self.respond_embed(
            context,
            command,
            embeds::status_embed_l10n(
                self.locale_for_command(command),
                &cache_entry.status,
                &cache_entry.services,
            ),
            false,
        )
        .await
    }
}
