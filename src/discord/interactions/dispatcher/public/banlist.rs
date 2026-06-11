use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_banlist(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self.state.database.ban_list(query_limit).await?;

        self.respond_embed(
            context,
            command,
            embeds::ban_list_embed_l10n(self.locale_for_command(command), &entries, display_limit),
            true,
        )
        .await
    }
}
