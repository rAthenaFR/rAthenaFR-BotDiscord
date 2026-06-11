use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_player(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self.respond_missing_option(context, command, "name").await;
        };

        let profile = self
            .state
            .database
            .find_player(
                self.state.config.display.public_character_group_threshold(),
                name,
            )
            .await?;

        let embed = match profile {
            Some(profile) => embeds::player_embed_l10n(self.locale_for_command(command), &profile),
            None => embeds::player_not_found_embed_l10n(self.locale_for_command(command), name),
        };

        self.respond_embed(context, command, embed, false).await
    }
}
