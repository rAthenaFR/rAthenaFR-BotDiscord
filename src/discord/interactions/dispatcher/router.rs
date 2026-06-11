use super::*;

impl Handler {
    pub(super) async fn handle_command(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let command_path = command_path(command);
        if is_public_pack_root(&command.data.name)
            && !self.state.config.commands.public_pack_enabled
        {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::command_disabled_embed_l10n(
                        self.locale_for_command(command),
                        "pack public",
                    ),
                    true,
                )
                .await;
        }
        if is_staff_pack_root(&command.data.name) && !self.state.config.commands.staff_pack_enabled
        {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::command_disabled_embed_l10n(
                        self.locale_for_command(command),
                        "pack staff",
                    ),
                    true,
                )
                .await;
        }
        if !self.state.config.commands.command_enabled(&command_path) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::command_disabled_embed_l10n(
                        self.locale_for_command(command),
                        &command_path,
                    ),
                    true,
                )
                .await;
        }

        match command.data.name.as_str() {
            "server" => self.handle_server(context, command).await,
            "online" => self.handle_online_pack(context, command).await,
            "top" => self.handle_top_pack(context, command).await,
            "player" => self.handle_player(context, command).await,
            "createaccount" => self.handle_createaccount(context, command).await,
            "guild" => self.handle_guild_pack(context, command).await,
            "castle" => self.handle_castle_pack(context, command).await,
            "item" => self.handle_item_pack(context, command).await,
            "who-drops" => self.handle_who_drops(context, command).await,
            "mob" => self.handle_mob_pack(context, command).await,
            "mvp" => self.handle_mvp_pack(context, command).await,
            "rank" => self.handle_rank(context, command).await,
            "market" => self.handle_market_pack(context, command).await,
            "staff" => self.handle_staff_pack(context, command).await,
            "mod" => self.handle_mod_pack(context, command).await,
            "debug" => self.handle_debug_pack(context, command).await,
            "audit" => self.handle_audit_pack(context, command).await,
            "db" => self.handle_db_pack(context, command).await,
            "gmmsg" => self.handle_gmmsg_pack(context, command).await,
            _ => {
                self.respond_error_key(context, command, I18nKey::ErrorUnknownCommand)
                    .await
            }
        }
    }
}
