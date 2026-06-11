use super::super::*;

impl Handler {
    pub(in crate::discord::interactions::dispatcher) async fn handle_staff_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let subcommand = subcommand_name(command).unwrap_or("player");
        if subcommand == "account-manage" {
            return self.handle_staff_account_manage(context, command).await;
        }

        let required_role = match subcommand {
            "player" | "account" | "chars" => StaffRole::Helper,
            "loginlog" | "ip-accounts" | "multiaccount" | "banned" => StaffRole::Admin,
            _ => StaffRole::Gm,
        };
        if !self.has_staff_role(command, required_role) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::staff_only_embed_l10n(self.locale_for_command(command)),
                    true,
                )
                .await;
        }

        match subcommand {
            "player" => self.handle_staff_player(context, command).await,
            "account" => self.handle_staff_account(context, command).await,
            "chars" => self.handle_staff_chars(context, command).await,
            "inventory" => self.handle_charinventory(context, command).await,
            "equipment" => self.handle_charequipment(context, command).await,
            "cart" => self.handle_staff_cart(context, command).await,
            "storage" => self.handle_staff_storage(context, command).await,
            "guildstorage" => self.handle_staff_guildstorage(context, command).await,
            "whohas" | "item-search" => self.handle_staff_whohas(context, command).await,
            "zeny" => self.handle_staff_zeny(context, command).await,
            "zenylog" => {
                self.handle_character_log_command(context, command, "zenylog")
                    .await
            }
            "picklog" => {
                self.handle_character_log_command(context, command, "picklog")
                    .await
            }
            "trade-log" => {
                self.handle_character_log_command(context, command, "picklog")
                    .await
            }
            "mvp-log" => {
                self.handle_character_log_command(context, command, "mvplog")
                    .await
            }
            "loginlog" => {
                self.handle_character_log_command(context, command, "loginlog")
                    .await
            }
            "ip-accounts" | "multiaccount" => {
                self.handle_character_log_command(context, command, "loginlog")
                    .await
            }
            "banned" => self.handle_banlist(context, command).await,
            _ => {
                self.respond_error_key_args(
                    context,
                    command,
                    I18nKey::ErrorUnknownSubcommand,
                    &[TranslationArg::new("command", "/staff")],
                )
                .await
            }
        }
    }
}
