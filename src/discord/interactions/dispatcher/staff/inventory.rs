use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_cart(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .character_cart_lines(character, query_limit)
            .await?;
        self.respond_lines_key(context, command, I18nKey::TitleCartInventory, lines, true)
            .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_storage(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .character_storage_lines(character, query_limit)
            .await?;
        self.respond_lines_key(context, command, I18nKey::TitleAccountStorage, lines, true)
            .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_guildstorage(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(guild) = string_option(command, "guild") else {
            return self.respond_missing_option(context, command, "guild").await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .guild_storage_lines(guild, query_limit)
            .await?;
        self.respond_lines_key(context, command, I18nKey::TitleGuildStorage, lines, true)
            .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_whohas(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(item_query) = string_option(command, "item") else {
            return self.respond_missing_option(context, command, "item").await;
        };
        let Some(item_id) = self.resolve_item_id(item_query).await? else {
            return self
                .respond_error_key(context, command, I18nKey::ErrorItemNotFoundGeneric)
                .await;
        };
        if !self
            .ensure_database_tables(context, command, ITEM_STORAGE_TABLES)
            .await?
        {
            return Ok(());
        }
        let (display_limit, query_limit) = self.list_limits(command);
        let owners = self
            .state
            .database
            .item_owners(item_id, query_limit)
            .await?;
        self.respond_embed(
            context,
            command,
            embeds::item_owners_embed_l10n(
                self.locale_for_command(command),
                item_id,
                &owners,
                display_limit,
            ),
            true,
        )
        .await
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_zeny(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_missing_option(context, command, "character")
                .await;
        };
        let profile = self.state.database.find_player(i32::MAX, character).await?;
        match profile {
            Some(profile) => {
                self.respond_embed(
                    context,
                    command,
                    embeds::text_embed(
                        &self.tr_raw(command, "title-character-zeny"),
                        self.tr_raw_args(
                            command,
                            "character-zeny-description",
                            &[
                                TranslationArg::new("character", &profile.name),
                                TranslationArg::new("zeny", &profile.zeny.to_string()),
                            ],
                        ),
                    ),
                    true,
                )
                .await
            }
            None => {
                self.respond_error_key(context, command, I18nKey::ErrorPlayerNotFoundGeneric)
                    .await
            }
        }
    }
}
