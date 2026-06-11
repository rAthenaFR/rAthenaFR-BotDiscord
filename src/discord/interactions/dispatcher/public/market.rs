use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_market_pack(
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

        match subcommand_name(command).unwrap_or("info") {
            "info" => {
                if !self
                    .ensure_database_tables(context, command, MARKET_TABLES)
                    .await?
                {
                    return Ok(());
                }
                let overview = self
                    .cached_market_overview(
                        item_id,
                        self.state.config.display.public_character_group_threshold(),
                    )
                    .await?;
                self.respond_embed(
                    context,
                    command,
                    embeds::market_embed_l10n(self.locale_for_command(command), &overview),
                    false,
                )
                .await
            }
            "sell" => {
                if !self
                    .ensure_database_tables(context, command, SELL_TABLES)
                    .await?
                {
                    return Ok(());
                }
                let (display_limit, query_limit) = self.list_limits(command);
                let sellers = self
                    .cached_who_sell(
                        item_id,
                        self.state.config.display.public_character_group_threshold(),
                        query_limit,
                    )
                    .await?;
                self.respond_embed(
                    context,
                    command,
                    embeds::who_sell_embed_l10n(
                        self.locale_for_command(command),
                        item_id,
                        &sellers,
                        display_limit,
                    ),
                    false,
                )
                .await
            }
            "buy" => {
                if !self
                    .ensure_database_tables(context, command, BUYING_STORE_TABLES)
                    .await?
                {
                    return Ok(());
                }
                let (display_limit, query_limit) = self.list_limits(command);
                let buyers = self
                    .cached_who_buy(
                        item_id,
                        self.state.config.display.public_character_group_threshold(),
                        query_limit,
                    )
                    .await?;
                self.respond_embed(
                    context,
                    command,
                    embeds::who_buy_embed_l10n(
                        self.locale_for_command(command),
                        item_id,
                        &buyers,
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
                    &[TranslationArg::new("command", "/market")],
                )
                .await
            }
        }
    }
}
