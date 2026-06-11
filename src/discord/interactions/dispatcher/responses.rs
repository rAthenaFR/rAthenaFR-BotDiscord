use super::*;

impl Handler {
    pub(super) async fn respond_embed(
        &self,
        context: &Context,
        command: &CommandInteraction,
        embed: serenity::all::CreateEmbed,
        ephemeral: bool,
    ) -> Result<()> {
        command
            .create_response(
                &context.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .ephemeral(ephemeral),
                ),
            )
            .await?;

        Ok(())
    }

    pub(super) fn locale_for_command(&self, command: &CommandInteraction) -> BotLocale {
        BotLocale::from_discord(command.locale.as_str())
    }

    pub(super) fn locale_for_component(&self, component: &ComponentInteraction) -> BotLocale {
        BotLocale::from_discord(component.locale.as_str())
    }

    pub(super) fn tr(&self, command: &CommandInteraction, key: I18nKey) -> String {
        translate(self.locale_for_command(command), key)
    }

    pub(super) fn tr_args(
        &self,
        command: &CommandInteraction,
        key: I18nKey,
        args: &[TranslationArg<'_>],
    ) -> String {
        translate_with_args(self.locale_for_command(command), key, args)
    }

    pub(super) fn tr_raw(&self, command: &CommandInteraction, key: &str) -> String {
        crate::i18n::loader::lookup(self.locale_for_command(command), key)
            .unwrap_or(key)
            .to_string()
    }

    pub(super) fn tr_raw_args(
        &self,
        command: &CommandInteraction,
        key: &str,
        args: &[TranslationArg<'_>],
    ) -> String {
        let mut value = self.tr_raw(command, key);

        for arg in args {
            let token = format!("{{ ${} }}", arg.name);
            value = value.replace(&token, arg.value);
        }

        value
    }

    pub(super) async fn respond_error_raw(
        &self,
        context: &Context,
        command: &CommandInteraction,
        key: &str,
    ) -> Result<()> {
        let message = self.tr_raw(command, key);
        self.respond_error(context, command, &message).await
    }

    pub(super) fn localize_known_error(
        &self,
        command: &CommandInteraction,
        message: &str,
    ) -> String {
        if let Some(option) = message
            .strip_prefix("Option obligatoire manquante : ")
            .and_then(|value| value.strip_suffix('.'))
        {
            return self.tr_args(
                command,
                I18nKey::ErrorMissingOption,
                &[TranslationArg::new("option", option)],
            );
        }

        if message == "Le compte doit être renseigné." {
            return self.tr(command, I18nKey::ErrorAccountRequired);
        }
        if message == "L’account_id doit être strictement positif." {
            return self.tr(command, I18nKey::ErrorAccountIdPositive);
        }
        if message == "La désactivation forte de compte est désactivée par configuration." {
            return self.tr(command, I18nKey::ErrorAccountDeleteDisabled);
        }
        if message == "Confirmation invalide. Renseigne `SUPPRIMER` exactement." {
            return self.tr(command, I18nKey::ErrorAccountDeleteConfirm);
        }
        if message == "Le sexe du compte doit être `M` ou `F`." {
            return self.tr(command, I18nKey::ErrorAccountSex);
        }

        if let Some(field) = message
            .strip_prefix("Le champ `")
            .and_then(|value| value.split_once('`'))
            .map(|(field, _)| field)
        {
            let key = if message.contains("ne peut jamais être modifié") {
                I18nKey::ErrorAccountForbiddenField
            } else if message.contains("n’est pas autorisé") {
                I18nKey::ErrorAccountInvalidField
            } else {
                return message.to_string();
            };
            return self.tr_args(command, key, &[TranslationArg::new("field", field)]);
        }

        if let Some(field) = message
            .strip_prefix("La valeur de `")
            .and_then(|value| value.split_once('`'))
            .map(|(field, _)| field)
        {
            let key = if message.contains("doit être un entier") {
                I18nKey::ErrorAccountValueInteger
            } else if message.contains("positive ou nulle") {
                I18nKey::ErrorAccountValueNonNegative
            } else {
                return message.to_string();
            };
            return self.tr_args(command, key, &[TranslationArg::new("field", field)]);
        }

        message.to_string()
    }

    pub(super) async fn respond_error(
        &self,
        context: &Context,
        command: &CommandInteraction,
        message: &str,
    ) -> Result<()> {
        self.respond_embed(
            context,
            command,
            embeds::error_embed_l10n(self.locale_for_command(command), message),
            true,
        )
        .await
    }

    pub(super) async fn respond_error_key(
        &self,
        context: &Context,
        command: &CommandInteraction,
        key: I18nKey,
    ) -> Result<()> {
        let message = self.tr(command, key);
        self.respond_error(context, command, &message).await
    }

    pub(super) async fn respond_error_key_args(
        &self,
        context: &Context,
        command: &CommandInteraction,
        key: I18nKey,
        args: &[TranslationArg<'_>],
    ) -> Result<()> {
        let message = self.tr_args(command, key, args);
        self.respond_error(context, command, &message).await
    }

    pub(super) async fn respond_missing_option(
        &self,
        context: &Context,
        command: &CommandInteraction,
        option: &str,
    ) -> Result<()> {
        self.respond_error_key_args(
            context,
            command,
            I18nKey::ErrorMissingOption,
            &[TranslationArg::new("option", option)],
        )
        .await
    }

    pub(super) async fn respond_lines(
        &self,
        context: &Context,
        command: &CommandInteraction,
        title: &str,
        lines: Vec<String>,
        ephemeral: bool,
    ) -> Result<()> {
        let empty_message = self.tr(command, I18nKey::ErrorNoResult);
        self.respond_lines_or_empty(context, command, title, lines, &empty_message, ephemeral)
            .await
    }

    pub(super) async fn respond_lines_key(
        &self,
        context: &Context,
        command: &CommandInteraction,
        title_key: I18nKey,
        lines: Vec<String>,
        ephemeral: bool,
    ) -> Result<()> {
        let title = self.tr(command, title_key);
        self.respond_lines(context, command, &title, lines, ephemeral)
            .await
    }

    pub(super) async fn respond_lines_or_empty_key(
        &self,
        context: &Context,
        command: &CommandInteraction,
        title_key: I18nKey,
        lines: Vec<String>,
        empty_key: I18nKey,
        ephemeral: bool,
    ) -> Result<()> {
        let title = self.tr(command, title_key);
        let empty_message = self.tr(command, empty_key);
        self.respond_lines_or_empty(context, command, &title, lines, &empty_message, ephemeral)
            .await
    }

    pub(super) async fn respond_lines_or_empty(
        &self,
        context: &Context,
        command: &CommandInteraction,
        title: &str,
        lines: Vec<String>,
        empty_message: &str,
        ephemeral: bool,
    ) -> Result<()> {
        let body = if lines.is_empty() {
            empty_message.to_string()
        } else {
            lines
                .into_iter()
                .filter(|line| !line.trim().is_empty())
                .take(self.state.config.display.max_limit as usize)
                .collect::<Vec<_>>()
                .join("\n")
        };

        self.respond_embed(
            context,
            command,
            embeds::text_embed(title, trim_discord_message(&body)),
            ephemeral,
        )
        .await
    }

    pub(super) async fn resolve_item_id(&self, item_query: &str) -> Result<Option<i64>> {
        Ok(self
            .state
            .database
            .search_items(item_query, 1)
            .await?
            .into_iter()
            .next()
            .map(|item| item.item_id))
    }

    pub(super) async fn reject_account_manage(
        &self,
        context: &Context,
        command: &CommandInteraction,
        action: &str,
        account: &str,
        error: &str,
    ) -> Result<()> {
        self.staff_audit_logger(context, command)
            .log_account_manage(AccountManageAuditEntry {
                status: embeds::AccountManageLogStatus::Refused,
                action,
                account,
                result: error,
                reason: None,
            })
            .await;

        let message = self.localize_known_error(command, error);
        self.respond_error(context, command, &message).await
    }

    pub(super) async fn log_account_manage_sql_error(
        &self,
        context: &Context,
        command: &CommandInteraction,
        action: &str,
        account: &str,
        error: &anyhow::Error,
    ) {
        let result = account_manage::safe_sql_error(error);
        self.staff_audit_logger(context, command)
            .log_account_manage(AccountManageAuditEntry {
                status: embeds::AccountManageLogStatus::Failed,
                action,
                account,
                result: &result,
                reason: None,
            })
            .await;
    }

    pub(super) fn staff_audit_logger<'a>(
        &self,
        context: &'a Context,
        command: &'a CommandInteraction,
    ) -> StaffAuditLogger<'a> {
        StaffAuditLogger::new(
            context,
            command,
            self.state.config.discord.staff_log_channel_id,
        )
    }

    pub(super) async fn ensure_database_tables(
        &self,
        context: &Context,
        command: &CommandInteraction,
        tables: &[DatabaseTable],
    ) -> Result<bool> {
        let Some(missing_table) = self.state.database.first_missing_table(tables).await? else {
            return Ok(true);
        };

        self.respond_embed(
            context,
            command,
            embeds::missing_database_table_embed_l10n(
                self.locale_for_command(command),
                missing_table.name(),
            ),
            true,
        )
        .await?;

        Ok(false)
    }

    pub(super) async fn cached_who_sell(
        &self,
        item_id: i64,
        group_threshold: i32,
        query_limit: u32,
    ) -> Result<Vec<MarketSellEntry>> {
        cached_data(
            "whosell",
            format!("item_id={item_id};group_threshold={group_threshold};limit={query_limit}"),
            self.state.config.cache.duration(MARKET_CACHE_TTL_SECONDS),
            &self.state.cache.who_sell,
            async {
                self.state
                    .database
                    .who_sell(item_id, group_threshold, query_limit)
                    .await
            },
        )
        .await
    }

    pub(super) async fn cached_who_buy(
        &self,
        item_id: i64,
        group_threshold: i32,
        query_limit: u32,
    ) -> Result<Vec<MarketBuyEntry>> {
        cached_data(
            "whobuy",
            format!("item_id={item_id};group_threshold={group_threshold};limit={query_limit}"),
            self.state.config.cache.duration(MARKET_CACHE_TTL_SECONDS),
            &self.state.cache.who_buy,
            async {
                self.state
                    .database
                    .who_buy(item_id, group_threshold, query_limit)
                    .await
            },
        )
        .await
    }

    pub(super) async fn cached_market_overview(
        &self,
        item_id: i64,
        group_threshold: i32,
    ) -> Result<MarketOverview> {
        cached_data(
            "market",
            format!("item_id={item_id};group_threshold={group_threshold}"),
            self.state.config.cache.duration(MARKET_CACHE_TTL_SECONDS),
            &self.state.cache.market,
            async {
                self.state
                    .database
                    .market_overview(item_id, group_threshold)
                    .await
            },
        )
        .await
    }

    pub(super) fn list_limits(&self, command: &CommandInteraction) -> (u32, u32) {
        let display_limit = self.requested_limit(command);
        let query_limit = display_limit.saturating_add(1);

        (display_limit, query_limit)
    }

    pub(super) fn requested_limit(&self, command: &CommandInteraction) -> u32 {
        self.state
            .config
            .display
            .clamp_limit(integer_option(command, "limit"))
    }
}

pub(super) async fn cached_data<T, F>(
    command_name: &'static str,
    key: String,
    ttl: Option<Duration>,
    cache: &TimedCache<String, T>,
    fetch: F,
) -> Result<T>
where
    T: Clone,
    F: Future<Output = Result<T>>,
{
    let Some(ttl) = ttl else {
        debug!(
            command = command_name,
            cache_state = "disabled",
            cache_key = %key,
            "Cache de commande désactivé."
        );
        return fetch.await;
    };

    if let Some(value) = cache.get(&key) {
        info!(
            command = command_name,
            cache_state = "hit",
            cache_key = %key,
            "Résultat trouvé dans le cache de commande."
        );
        return Ok(value);
    }

    debug!(
        command = command_name,
        cache_state = "miss",
        cache_key = %key,
        "Aucun résultat dans le cache de commande."
    );
    let value = fetch.await?;
    cache.insert(key, value.clone(), ttl);

    Ok(value)
}
