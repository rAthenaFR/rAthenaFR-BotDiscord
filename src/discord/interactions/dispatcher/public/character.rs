use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_charquests(
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

        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, QUEST_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let quests = self
            .state
            .database
            .character_quests(character, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::character_quests_embed_l10n(
                self.locale_for_command(command),
                character,
                &quests,
                display_limit,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_charequipment(
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

        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, INVENTORY_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let items = self
            .state
            .database
            .character_equipment(character, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::character_equipment_embed_l10n(
                self.locale_for_command(command),
                character,
                &items,
                display_limit,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_charinventory(
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

        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, INVENTORY_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let items = self
            .state
            .database
            .character_inventory(character, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::character_inventory_embed_l10n(
                self.locale_for_command(command),
                character,
                &items,
                display_limit,
            ),
            true,
        )
        .await
    }
}
