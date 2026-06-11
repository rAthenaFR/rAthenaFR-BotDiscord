use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_item_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "info" => {
                let Some(item) = string_option(command, "item") else {
                    return self.respond_missing_option(context, command, "item").await;
                };
                let lines = self
                    .state
                    .database
                    .item_detail_lines(item, &self.state.config.commands.item_table_name)
                    .await?;
                match lines {
                    Some(lines) => {
                        self.respond_lines_key(
                            context,
                            command,
                            I18nKey::TitleItemDetails,
                            lines,
                            false,
                        )
                        .await
                    }
                    None => {
                        self.respond_error_key(context, command, I18nKey::ErrorItemNotFoundGeneric)
                            .await
                    }
                }
            }
            "search" => {
                let Some(text) = string_option(command, "text") else {
                    return self.respond_missing_option(context, command, "text").await;
                };
                let (display_limit, query_limit) = self.list_limits(command);
                let items = self.state.database.search_items(text, query_limit).await?;
                let lines = items
                    .iter()
                    .take(display_limit as usize)
                    .map(|item| {
                        format!(
                            "`{}` - {} (`{}`) - `{}`",
                            item.item_id, item.display_name, item.aegis_name, item.item_type
                        )
                    })
                    .collect::<Vec<_>>();
                self.respond_lines_or_empty_key(
                    context,
                    command,
                    I18nKey::TitleItemSearch,
                    lines,
                    I18nKey::ErrorItemNotFoundGeneric,
                    false,
                )
                .await
            }
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/item")],
                )
                .await
            }
        }
    }

    pub(in crate::discord::interactions::dispatcher) async fn handle_who_drops(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(item) = string_option(command, "item") else {
            return self.respond_missing_option(context, command, "item").await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .who_drops_lines(
                item,
                &self.state.config.commands.mob_table_name,
                query_limit,
            )
            .await?;
        match lines {
            Some(lines) => {
                self.respond_lines_key(context, command, I18nKey::TitleWhoDrops, lines, false)
                    .await
            }
            None => {
                self.respond_error_key(context, command, I18nKey::ErrorItemNotFoundGeneric)
                    .await
            }
        }
    }
}
