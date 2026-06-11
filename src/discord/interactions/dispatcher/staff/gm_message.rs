use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_gmmsg_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, self.state.config.commands.gmmsg_min_role) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        let subcommand = subcommand_name(command).unwrap_or("server");
        let Some(message) = string_option(command, "message") else {
            return self
                .respond_missing_option(context, command, "message")
                .await;
        };
        let message = match sanitize_gm_message_l10n(
            self.locale_for_command(command),
            message,
            self.state.config.game_bridge.max_message_length,
        ) {
            Ok(message) => message,
            Err(message) => return self.respond_error(context, command, &message).await,
        };
        let discord_user_id = command.user.id.get();
        let discord_username = command.user.name.as_str();

        let result = match subcommand {
            "server" => {
                self.state
                    .game_bridge
                    .send_global_message(
                        BroadcastMode::Broadcast,
                        &message,
                        discord_user_id,
                        discord_username,
                    )
                    .await
            }
            "blue" => {
                self.state
                    .game_bridge
                    .send_global_message(
                        BroadcastMode::KamiBlue,
                        &message,
                        discord_user_id,
                        discord_username,
                    )
                    .await
            }
            "color" => {
                let Some(hex) = string_option(command, "hex") else {
                    return self.respond_missing_option(context, command, "hex").await;
                };
                let hex = match validate_hex_color_l10n(self.locale_for_command(command), hex) {
                    Ok(hex) => hex,
                    Err(message) => return self.respond_error(context, command, &message).await,
                };
                self.state
                    .game_bridge
                    .send_global_message(
                        BroadcastMode::KamiColor(hex),
                        &message,
                        discord_user_id,
                        discord_username,
                    )
                    .await
            }
            "map" => {
                let Some(map) = string_option(command, "map") else {
                    return self.respond_missing_option(context, command, "map").await;
                };
                self.state
                    .game_bridge
                    .send_map_message(map, &message, discord_user_id, discord_username)
                    .await
            }
            "test" => Ok(format!("mode test : {message}")),
            _ => Err(anyhow::anyhow!(
                "{}",
                self.tr_args(
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/gmmsg")]
                )
            )),
        };

        let (log_status, log_result) = match &result {
            Ok(details) => (
                embeds::GmmsgLogStatus::Sent,
                gmmsg_success_log_result_l10n(
                    self.locale_for_command(command),
                    subcommand,
                    details,
                ),
            ),
            Err(error) => (
                embeds::GmmsgLogStatus::Failed,
                gmmsg_error_log_result_l10n(
                    self.locale_for_command(command),
                    self.state.config.game_bridge.mode,
                    &error.to_string(),
                ),
            ),
        };
        self.staff_audit_logger(context, command)
            .log_gmmsg(GmmsgAuditEntry {
                status: log_status,
                action: subcommand,
                message: &message,
                result: &log_result,
            })
            .await;

        match result {
            Ok(details) => {
                self.respond_embed(
                    context,
                    command,
                    embeds::success_message_embed(
                        &self.tr(command, I18nKey::TitleGmMessage),
                        details,
                    ),
                    true,
                )
                .await
            }
            Err(error) => {
                self.respond_error(context, command, &error.to_string())
                    .await
            }
        }
    }
}
