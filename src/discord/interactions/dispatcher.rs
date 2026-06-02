use crate::cache::{BotCache, StatusCacheEntry, TimedCache};
use crate::config::{AppConfig, GameBridgeMode, StaffRole, TopZenyMode};
use crate::discord::embeds;
use crate::infra::observability::CommandTimer;
use crate::rathenafr::{
    check_services, BroadcastMode, DatabaseTable, GameBridge, MarketBuyEntry, MarketOverview,
    MarketSellEntry, RAthenaFrDatabase,
};
use anyhow::Result;
use serenity::all::{
    async_trait, ActivityData, ApplicationId, ChannelId, Client, CommandDataOption,
    CommandDataOptionValue, CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateMessage, EventHandler, GatewayIntents, Interaction,
    OnlineStatus, Ready,
};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

const STATUS_CACHE_TTL_SECONDS: u64 = 10;
const GUILDS_CACHE_TTL_SECONDS: u64 = 30;
const CASTLES_CACHE_TTL_SECONDS: u64 = 60;
const MARKET_CACHE_TTL_SECONDS: u64 = 20;

#[cfg(test)]
const CACHED_COMMAND_NAMES: &[&str] =
    &["status", "guilds", "castles", "whosell", "whobuy", "market"];

const BUYING_STORE_TABLES: &[DatabaseTable] =
    &[DatabaseTable::BuyingStores, DatabaseTable::BuyingStoreItems];
const CASTLE_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildCastle];
const GUILD_MEMBER_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildMember];
const INVENTORY_TABLES: &[DatabaseTable] = &[DatabaseTable::Inventory];
const ITEM_STORAGE_TABLES: &[DatabaseTable] = &[
    DatabaseTable::Inventory,
    DatabaseTable::CartInventory,
    DatabaseTable::Storage,
    DatabaseTable::GuildStorage,
];
const MARKET_TABLES: &[DatabaseTable] = &[
    DatabaseTable::Vendings,
    DatabaseTable::VendingItems,
    DatabaseTable::CartInventory,
    DatabaseTable::BuyingStores,
    DatabaseTable::BuyingStoreItems,
];
const QUEST_TABLES: &[DatabaseTable] = &[DatabaseTable::Quest];
const SELL_TABLES: &[DatabaseTable] = &[
    DatabaseTable::Vendings,
    DatabaseTable::VendingItems,
    DatabaseTable::CartInventory,
];

pub struct BotState {
    pub config: Arc<AppConfig>,
    pub database: Arc<RAthenaFrDatabase>,
    pub game_bridge: GameBridge,
    pub cache: BotCache,
}

pub async fn create_client(
    config: Arc<AppConfig>,
    database: Arc<RAthenaFrDatabase>,
) -> Result<Client> {
    let intents = GatewayIntents::empty();
    let handler = Handler {
        state: Arc::new(BotState {
            game_bridge: GameBridge::new(config.game_bridge.clone()),
            config,
            database,
            cache: BotCache::default(),
        }),
    };

    Ok(
        Client::builder(&handler.state.config.discord.token, intents)
            .application_id(ApplicationId::new(
                handler.state.config.discord.application_id,
            ))
            .event_handler(handler)
            .await?,
    )
}

struct Handler {
    state: Arc<BotState>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        let display_name = std::env::var("RATHENAFR_DISPLAY_NAME")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "rAthenaFR".to_string());
        let presence = format!("v{}", env!("CARGO_PKG_VERSION"));

        context.set_presence(Some(ActivityData::custom(&presence)), OnlineStatus::Online);

        info!(
            bot_username = %ready.user.name,
            bot_user_id = ready.user.id.get(),
            display_name = %display_name,
            presence = %presence,
            shard = ?ready.shard,
            "Shard Discord prêt."
        );
    }

    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let timer = CommandTimer::start();
            let result = self.handle_command(&context, &command).await;
            let duration_ms = timer.elapsed_ms();
            let command_name = command.data.name.as_str();
            let guild_id = command.guild_id.map(|guild_id| guild_id.get());
            let user_id = command.user.id.get();

            match result {
                Ok(()) => {
                    info!(
                        command = command_name,
                        guild_id = ?guild_id,
                        user_id = user_id,
                        duration_ms = duration_ms,
                        "Commande terminée avec succès."
                    );
                }
                Err(why) => {
                    error!(
                        error = %format!("{why:#}"),
                        command = command_name,
                        guild_id = ?guild_id,
                        user_id = user_id,
                        duration_ms = duration_ms,
                        "La commande a échoué."
                    );

                    let _ = command
                        .create_response(
                            &context.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .embed(embeds::command_error_embed(&why.to_string()))
                                    .ephemeral(true),
                            ),
                        )
                        .await;
                }
            }
        }
    }
}

impl Handler {
    async fn handle_command(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let command_path = command_path(command);
        if is_public_pack_root(&command.data.name)
            && !self.state.config.commands.public_pack_enabled
        {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::command_disabled_embed("pack public"),
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
                    embeds::command_disabled_embed("pack staff"),
                    true,
                )
                .await;
        }
        if !self.state.config.commands.command_enabled(&command_path) {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::command_disabled_embed(&command_path),
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
                self.respond_error(context, command, "Commande inconnue.")
                    .await
            }
        }
    }

    async fn handle_server(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        self.handle_status(context, command).await
    }

    async fn handle_online_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("count") {
            "count" => {
                let status = self
                    .state
                    .database
                    .database_status(self.state.config.display.public_character_group_threshold())
                    .await?;
                self.respond_embed(
                    context,
                    command,
                    embeds::text_embed(
                        "Joueurs connectés",
                        format!("`{}` joueur(s) connecté(s).", status.online_characters),
                    ),
                    false,
                )
                .await
            }
            "list" => {
                if !self.state.config.commands.online_list_public {
                    return self
                        .respond_error(
                            context,
                            command,
                            "La liste publique des joueurs connectés est désactivée.",
                        )
                        .await;
                }

                self.handle_online(context, command).await
            }
            "map" => {
                let (display_limit, query_limit) = self.list_limits(command);
                let entries = self
                    .state
                    .database
                    .map_stats(
                        self.state.config.display.public_character_group_threshold(),
                        true,
                        query_limit,
                    )
                    .await?;

                self.respond_embed(
                    context,
                    command,
                    embeds::map_stats_embed(&entries, true, display_limit),
                    false,
                )
                .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /online inconnue.")
                    .await
            }
        }
    }

    async fn handle_top_pack(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        match subcommand_name(command).unwrap_or("level") {
            "level" => self.handle_top(context, command).await,
            "job" => {
                let (display_limit, query_limit) = self.list_limits(command);
                let entries = self
                    .state
                    .database
                    .top_characters_by_job(
                        self.state.config.display.ranking_group_threshold(),
                        query_limit,
                    )
                    .await?;

                self.respond_embed(
                    context,
                    command,
                    embeds::ranking_embed(&entries, display_limit),
                    false,
                )
                .await
            }
            "guild" => self.handle_guilds(context, command).await,
            "zeny" => self.handle_top_zeny_configured(context, command).await,
            _ => {
                self.respond_error(context, command, "Sous-commande /top inconnue.")
                    .await
            }
        }
    }

    async fn handle_top_zeny_configured(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match self.state.config.commands.top_zeny_mode {
            TopZenyMode::Disabled => {
                self.respond_error(context, command, "Le classement zeny est désactivé.")
                    .await
            }
            TopZenyMode::Enabled => self.handle_topzeny(context, command).await,
            TopZenyMode::Anonymized => {
                let (display_limit, query_limit) = self.list_limits(command);
                let mut entries = self
                    .state
                    .database
                    .top_zeny(
                        self.state.config.display.ranking_group_threshold(),
                        query_limit,
                    )
                    .await?;

                for entry in &mut entries {
                    entry.name = format!("Personnage #{}", entry.rank);
                }

                self.respond_embed(
                    context,
                    command,
                    embeds::top_zeny_embed(&entries, display_limit),
                    false,
                )
                .await
            }
        }
    }

    async fn handle_guild_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "info" => self.handle_guild(context, command).await,
            "members" => self.handle_guildmembers(context, command).await,
            _ => {
                self.respond_error(context, command, "Sous-commande /guild inconnue.")
                    .await
            }
        }
    }

    async fn handle_castle_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "list" => self.handle_castles(context, command).await,
            "info" => self.handle_castle(context, command).await,
            _ => {
                self.respond_error(context, command, "Sous-commande /castle inconnue.")
                    .await
            }
        }
    }

    async fn handle_item_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "info" => {
                let Some(item) = string_option(command, "item") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : item.")
                        .await;
                };
                let lines = self
                    .state
                    .database
                    .item_detail_lines(item, &self.state.config.commands.item_table_name)
                    .await?;
                match lines {
                    Some(lines) => {
                        self.respond_lines(context, command, "Fiche item", lines, false)
                            .await
                    }
                    None => {
                        self.respond_error(context, command, "Aucun item n’a été trouvé.")
                            .await
                    }
                }
            }
            "search" => {
                let Some(text) = string_option(command, "text") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : text.")
                        .await;
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
                self.respond_lines_or_empty(
                    context,
                    command,
                    "Recherche items",
                    lines,
                    "Aucun item n’a été trouvé.",
                    false,
                )
                .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /item inconnue.")
                    .await
            }
        }
    }

    async fn handle_who_drops(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(item) = string_option(command, "item") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item.")
                .await;
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
                self.respond_lines(context, command, "Who drops", lines, false)
                    .await
            }
            None => {
                self.respond_error(context, command, "Aucun item n’a été trouvé.")
                    .await
            }
        }
    }

    async fn handle_mob_pack(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        match subcommand_name(command).unwrap_or("info") {
            "info" => {
                let Some(mob) = string_option(command, "mob") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : mob.")
                        .await;
                };
                let lines = self
                    .state
                    .database
                    .mob_detail_lines(mob, &self.state.config.commands.mob_table_name)
                    .await?;
                match lines {
                    Some(lines) => {
                        self.respond_lines(context, command, "Fiche monstre", lines, false)
                            .await
                    }
                    None => {
                        self.respond_error(context, command, "Aucun monstre n’a été trouvé.")
                            .await
                    }
                }
            }
            "drops" => {
                let Some(mob) = string_option(command, "mob") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : mob.")
                        .await;
                };
                let (_display_limit, query_limit) = self.list_limits(command);
                let lines = self
                    .state
                    .database
                    .mob_drop_lines(mob, &self.state.config.commands.mob_table_name, query_limit)
                    .await?;
                match lines {
                    Some(lines) => {
                        self.respond_lines(context, command, "Drops monstre", lines, false)
                            .await
                    }
                    None => {
                        self.respond_error(context, command, "Aucun monstre n’a été trouvé.")
                            .await
                    }
                }
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /mob inconnue.")
                    .await
            }
        }
    }

    async fn handle_mvp_pack(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (_display_limit, query_limit) = self.list_limits(command);
        match subcommand_name(command).unwrap_or("list") {
            "list" => {
                let lines = self
                    .state
                    .database
                    .mvp_list_lines(&self.state.config.commands.mob_table_name, query_limit)
                    .await?;
                self.respond_lines(context, command, "MVP", lines, false)
                    .await
            }
            "last" => {
                let lines = self
                    .state
                    .database
                    .recent_log_lines("mvplog", query_limit)
                    .await?;
                self.respond_lines(context, command, "Derniers MVP", lines, false)
                    .await
            }
            "top" => {
                let lines = self
                    .state
                    .database
                    .recent_log_lines("mvplog", query_limit)
                    .await?;
                self.respond_lines(context, command, "Top MVP", lines, false)
                    .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /mvp inconnue.")
                    .await
            }
        }
    }

    async fn handle_rank(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : name.")
                .await;
        };
        let lines = self
            .state
            .database
            .rank_summary_lines(
                name,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;

        match lines {
            Some(lines) => {
                self.respond_lines(context, command, "Rang personnage", lines, false)
                    .await
            }
            None => {
                self.respond_error(context, command, "Aucun personnage n’a été trouvé.")
                    .await
            }
        }
    }

    async fn handle_market_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(item_query) = string_option(command, "item") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item.")
                .await;
        };
        let Some(item_id) = self.resolve_item_id(item_query).await? else {
            return self
                .respond_error(context, command, "Aucun item n’a été trouvé.")
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
                self.respond_embed(context, command, embeds::market_embed(&overview), false)
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
                    embeds::who_sell_embed(item_id, &sellers, display_limit),
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
                    embeds::who_buy_embed(item_id, &buyers, display_limit),
                    false,
                )
                .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /market inconnue.")
                    .await
            }
        }
    }

    async fn handle_staff_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let subcommand = subcommand_name(command).unwrap_or("player");
        let required_role = match subcommand {
            "player" | "account" | "chars" => StaffRole::Helper,
            "loginlog" | "ip-accounts" | "multiaccount" | "banned" => StaffRole::Admin,
            _ => StaffRole::Gm,
        };
        if !self.has_staff_role(command, required_role) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
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
                self.respond_error(context, command, "Sous-commande /staff inconnue.")
                    .await
            }
        }
    }

    async fn handle_mod_pack(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self.has_staff_role(command, StaffRole::Moderator) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        match subcommand_name(command).unwrap_or("chatlog") {
            "chatlog" => {
                self.handle_character_log_command(context, command, "chatlog")
                    .await
            }
            "chat-search" => self.handle_chat_search(context, command).await,
            "report-context" => self.handle_report_context(context, command).await,
            "branchlog" => {
                self.handle_character_log_command(context, command, "branchlog")
                    .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /mod inconnue.")
                    .await
            }
        }
    }

    async fn handle_debug_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, self.state.config.commands.debug_min_role) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        match subcommand_name(command).unwrap_or("quest") {
            "quest" => self.handle_charquests(context, command).await,
            "char-vars" => {
                self.handle_variable_command(context, command, "char_reg_num")
                    .await
            }
            "acc-vars" => {
                self.handle_variable_command(context, command, "acc_reg_num")
                    .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /debug inconnue.")
                    .await
            }
        }
    }

    async fn handle_audit_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, self.state.config.commands.audit_min_role) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let (_display_limit, query_limit) = self.list_limits(command);
        match subcommand_name(command).unwrap_or("atcommands") {
            "atcommands" => {
                let Some(gm) = string_option(command, "gm") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : gm.")
                        .await;
                };
                let lines = self
                    .state
                    .database
                    .named_log_lines("atcommandlog", gm, query_limit)
                    .await?;
                self.respond_lines(context, command, "Audit atcommands", lines, true)
                    .await
            }
            "item-created" => {
                let lines = self
                    .state
                    .database
                    .recent_log_lines("picklog", query_limit)
                    .await?;
                self.respond_lines(context, command, "Audit des items créés", lines, true)
                    .await
            }
            "zeny-created" => {
                let lines = self
                    .state
                    .database
                    .recent_log_lines("zenylog", query_limit)
                    .await?;
                self.respond_lines(context, command, "Audit zeny", lines, true)
                    .await
            }
            "gm-activity" => {
                let Some(gm) = string_option(command, "gm") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : gm.")
                        .await;
                };
                let lines = self
                    .state
                    .database
                    .named_log_lines("atcommandlog", gm, query_limit)
                    .await?;
                self.respond_lines(context, command, "Activite GM", lines, true)
                    .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /audit inconnue.")
                    .await
            }
        }
    }

    async fn handle_db_pack(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self.has_staff_role(command, StaffRole::Owner) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        match subcommand_name(command).unwrap_or("health") {
            "health" => {
                let lines = self.state.database.release_health_lines().await?;
                self.respond_lines(context, command, "DB health", lines, true)
                    .await
            }
            "tables" => {
                let (_display_limit, query_limit) = self.list_limits(command);
                let lines = self
                    .state
                    .database
                    .detected_rathena_tables(query_limit)
                    .await?;
                self.respond_lines_or_empty(
                    context,
                    command,
                    "Tables rAthena détectées",
                    lines,
                    "Aucune table n’a été détectée.",
                    true,
                )
                .await
            }
            "count" => {
                let lines = self.state.database.useful_table_counts().await?;
                self.respond_lines_or_empty(
                    context,
                    command,
                    "Compteurs de tables",
                    lines,
                    "Aucune table utile n’a été détectée.",
                    true,
                )
                .await
            }
            "logs-size" => {
                let lines = self.state.database.log_table_sizes().await?;
                self.respond_lines(context, command, "Volume des logs", lines, true)
                    .await
            }
            "last-update" => {
                let (_display_limit, query_limit) = self.list_limits(command);
                let lines = self.state.database.sql_updates_lines(query_limit).await?;
                self.respond_lines(context, command, "sql_updates", lines, true)
                    .await
            }
            _ => {
                self.respond_error(context, command, "Sous-commande /db inconnue.")
                    .await
            }
        }
    }

    async fn handle_gmmsg_pack(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_role(command, self.state.config.commands.gmmsg_min_role) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let subcommand = subcommand_name(command).unwrap_or("server");
        let Some(message) = string_option(command, "message") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : message.")
                .await;
        };
        let message =
            match sanitize_gm_message(message, self.state.config.game_bridge.max_message_length) {
                Ok(message) => message,
                Err(message) => return self.respond_error(context, command, &message).await,
            };

        let result = match subcommand {
            "server" => {
                self.state
                    .game_bridge
                    .send_global_message(BroadcastMode::Broadcast, &message)
                    .await
            }
            "blue" => {
                self.state
                    .game_bridge
                    .send_global_message(BroadcastMode::KamiBlue, &message)
                    .await
            }
            "color" => {
                let Some(hex) = string_option(command, "hex") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : hex.")
                        .await;
                };
                let hex = match validate_hex_color(hex) {
                    Ok(hex) => hex,
                    Err(message) => return self.respond_error(context, command, message).await,
                };
                self.state
                    .game_bridge
                    .send_global_message(BroadcastMode::KamiColor(hex), &message)
                    .await
            }
            "map" => {
                let Some(map) = string_option(command, "map") else {
                    return self
                        .respond_error(context, command, "Option obligatoire manquante : map.")
                        .await;
                };
                self.state.game_bridge.send_map_message(map, &message).await
            }
            "test" => Ok(format!("mode test : {message}")),
            _ => Err(anyhow::anyhow!("Sous-commande /gmmsg inconnue.")),
        };

        let result_text = match result {
            Ok(details) => format!("Résultat : `{details}`"),
            Err(error) => format!("Résultat : `{}`", error),
        };
        self.log_staff_action(context, command, subcommand, &message, &result_text)
            .await;

        if self.state.config.game_bridge.mode == GameBridgeMode::Disabled && subcommand != "test" {
            return self
                .respond_error(
                    context,
                    command,
                    "Le bridge en jeu n’est pas configuré. Le message n’a pas été envoyé.",
                )
                .await;
        }

        self.respond_embed(
            context,
            command,
            embeds::success_message_embed(
                "Message GM",
                format!("Commande `{subcommand}` traitée.\n{result_text}"),
            ),
            true,
        )
        .await
    }

    async fn handle_staff_player(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let profile = self.state.database.find_player(i32::MAX, character).await?;
        let embed = match profile {
            Some(profile) => embeds::player_embed(&profile),
            None => embeds::player_not_found_embed(character),
        };
        self.respond_embed(context, command, embed, true).await
    }

    async fn handle_staff_account(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let account = self
            .state
            .database
            .account_status_by_character(character)
            .await?;
        let embed = match account {
            Some(account) => embeds::account_status_embed(&account),
            None => embeds::error_embed("Aucun compte n’est lié à ce personnage."),
        };
        self.respond_embed(context, command, embed, true).await
    }

    async fn handle_staff_chars(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(lookup) = string_option(command, "lookup") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : lookup.")
                .await;
        };
        let account_id = match lookup.trim().parse::<i64>() {
            Ok(value) if value > 0 => Some(value),
            _ => self.state.database.account_id_for_character(lookup).await?,
        };
        let Some(account_id) = account_id else {
            return self
                .respond_error(context, command, "Aucun compte n’a été trouvé.")
                .await;
        };
        let (display_limit, query_limit) = self.list_limits(command);
        let characters = self
            .state
            .database
            .account_characters(account_id, query_limit)
            .await?;
        self.respond_embed(
            context,
            command,
            embeds::account_characters_embed(account_id, &characters, display_limit),
            true,
        )
        .await
    }

    async fn handle_staff_cart(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .character_cart_lines(character, query_limit)
            .await?;
        self.respond_lines(context, command, "Cart personnage", lines, true)
            .await
    }

    async fn handle_staff_storage(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .character_storage_lines(character, query_limit)
            .await?;
        self.respond_lines(context, command, "Storage compte", lines, true)
            .await
    }

    async fn handle_staff_guildstorage(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(guild) = string_option(command, "guild") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : guild.")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .guild_storage_lines(guild, query_limit)
            .await?;
        self.respond_lines(context, command, "Storage guilde", lines, true)
            .await
    }

    async fn handle_staff_whohas(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(item_query) = string_option(command, "item") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item.")
                .await;
        };
        let Some(item_id) = self.resolve_item_id(item_query).await? else {
            return self
                .respond_error(context, command, "Aucun item n’a été trouvé.")
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
            embeds::item_owners_embed(item_id, &owners, display_limit),
            true,
        )
        .await
    }

    async fn handle_staff_zeny(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let profile = self.state.database.find_player(i32::MAX, character).await?;
        match profile {
            Some(profile) => {
                self.respond_embed(
                    context,
                    command,
                    embeds::text_embed(
                        "Zeny personnage",
                        format!("`{}` possède `{}` zeny.", profile.name, profile.zeny),
                    ),
                    true,
                )
                .await
            }
            None => {
                self.respond_error(context, command, "Aucun personnage n’a été trouvé.")
                    .await
            }
        }
    }

    async fn handle_character_log_command(
        &self,
        context: &Context,
        command: &CommandInteraction,
        table_name: &str,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .character_log_lines(table_name, character, query_limit)
            .await?;
        self.respond_lines(context, command, table_name, lines, true)
            .await
    }

    async fn handle_variable_command(
        &self,
        context: &Context,
        command: &CommandInteraction,
        table_name: &str,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .variable_lines(table_name, character, query_limit)
            .await?;
        self.respond_lines(context, command, table_name, lines, true)
            .await
    }

    async fn handle_chat_search(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(text) = string_option(command, "text") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : text.")
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let lines = self
            .state
            .database
            .named_log_lines("chatlog", text, query_limit)
            .await?;
        self.respond_lines(context, command, "Recherche chatlog", lines, true)
            .await
    }

    async fn handle_report_context(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
                .await;
        };
        let (_display_limit, query_limit) = self.list_limits(command);
        let mut lines = Vec::new();
        if let Some(profile) = self.state.database.find_player(i32::MAX, character).await? {
            lines.push(format!(
                "Position: `{}` - online `{}`",
                profile.map, profile.online
            ));
        }
        lines.extend(
            self.state
                .database
                .character_log_lines("chatlog", character, query_limit)
                .await?,
        );
        self.respond_lines(context, command, "Contexte signalement", lines, true)
            .await
    }

    async fn handle_status(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let cache_key = format!("group_threshold={group_threshold}");
        let cache_entry = cached_data(
            "status",
            cache_key,
            self.state.config.cache.duration(STATUS_CACHE_TTL_SECONDS),
            &self.state.cache.status,
            async {
                let status = self.state.database.database_status(group_threshold).await?;
                let endpoints = self.state.config.services.endpoints();
                let services = check_services(&endpoints).await;

                Ok(StatusCacheEntry { status, services })
            },
        )
        .await?;

        self.respond_embed(
            context,
            command,
            embeds::status_embed(&cache_entry.status, &cache_entry.services),
            false,
        )
        .await
    }

    async fn handle_online(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let characters = self
            .state
            .database
            .online_characters(
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::online_embed(&characters, display_limit),
            false,
        )
        .await
    }

    async fn handle_top(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self
            .state
            .database
            .top_characters(
                self.state.config.display.ranking_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::ranking_embed(&entries, display_limit),
            false,
        )
        .await
    }

    async fn handle_player(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : name.")
                .await;
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
            Some(profile) => embeds::player_embed(&profile),
            None => embeds::player_not_found_embed(name),
        };

        self.respond_embed(context, command, embed, false).await
    }

    async fn handle_guilds(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let guilds = cached_data(
            "guildes",
            format!("limit={query_limit}"),
            self.state.config.cache.duration(GUILDS_CACHE_TTL_SECONDS),
            &self.state.cache.guilds,
            async { self.state.database.top_guilds(query_limit).await },
        )
        .await?;

        self.respond_embed(
            context,
            command,
            embeds::guilds_embed(&guilds, display_limit),
            false,
        )
        .await
    }

    async fn handle_createaccount(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.state.config.account_commands.creation_enabled {
            return self
                .respond_embed(
                    context,
                    command,
                    embeds::account_creation_disabled_embed(),
                    true,
                )
                .await;
        }

        let Some(username) = string_option(command, "username") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : username.")
                .await;
        };
        let Some(password) = string_option(command, "password") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : password.")
                .await;
        };
        let Some(sex) = string_option(command, "sex") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : sex.")
                .await;
        };
        let Some(birthdate) = string_option(command, "birthdate") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : birthdate.",
                )
                .await;
        };

        let username = match validate_account_username(username) {
            Ok(username) => username,
            Err(message) => return self.respond_error(context, command, &message).await,
        };
        let password = match validate_account_password(password) {
            Ok(password) => password,
            Err(message) => return self.respond_error(context, command, &message).await,
        };
        let sex = match validate_account_sex(sex) {
            Ok(sex) => sex,
            Err(message) => return self.respond_error(context, command, &message).await,
        };

        let birthdate = match validate_account_birthdate(birthdate) {
            Ok(birthdate) => birthdate,
            Err(message) => return self.respond_error(context, command, &message).await,
        };

        let email = match validate_account_email(string_option(command, "email")) {
            Ok(email) => email,
            Err(message) => return self.respond_error(context, command, &message).await,
        };

        if self.state.database.account_userid_exists(&username).await? {
            return self
                .respond_error(
                    context,
                    command,
                    &format!("Le compte `{username}` existe déjà."),
                )
                .await;
        }

        let account = self
            .state
            .database
            .create_account(
                &username,
                &password,
                self.state.config.account_commands.password_mode,
                &sex,
                &birthdate,
                &email,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::account_created_embed(&account),
            true,
        )
        .await
    }

    async fn handle_topzeny(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self
            .state
            .database
            .top_zeny(
                self.state.config.display.ranking_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::top_zeny_embed(&entries, display_limit),
            false,
        )
        .await
    }

    async fn handle_guild(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : name.")
                .await;
        };

        let guild = self.state.database.find_guild(name).await?;
        let embed = match guild {
            Some(guild) => embeds::guild_detail_embed(&guild),
            None => embeds::guild_not_found_embed(name),
        };

        self.respond_embed(context, command, embed, false).await
    }

    async fn handle_guildmembers(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : name.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, GUILD_MEMBER_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let members = self
            .state
            .database
            .guild_members(
                name,
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::guild_members_embed(name, &members, display_limit),
            false,
        )
        .await
    }

    async fn handle_castles(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, CASTLE_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let castles = cached_data(
            "castles",
            format!("limit={query_limit}"),
            self.state.config.cache.duration(CASTLES_CACHE_TTL_SECONDS),
            &self.state.cache.castles,
            async { self.state.database.castles(query_limit).await },
        )
        .await?;

        self.respond_embed(
            context,
            command,
            embeds::castles_embed(&castles, display_limit),
            false,
        )
        .await
    }

    async fn handle_castle(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(castle_id) = non_negative_integer_option(command, "castle_id") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : castle_id.",
                )
                .await;
        };

        if !self
            .ensure_database_tables(context, command, CASTLE_TABLES)
            .await?
        {
            return Ok(());
        }

        let castle = self.state.database.castle_details(castle_id).await?;
        let embed = match castle {
            Some(castle) => embeds::castle_detail_embed(&castle),
            None => embeds::castle_not_found_embed(castle_id),
        };

        self.respond_embed(context, command, embed, false).await
    }

    async fn handle_charquests(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
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
            embeds::character_quests_embed(character, &quests, display_limit),
            true,
        )
        .await
    }

    async fn handle_charequipment(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
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
            embeds::character_equipment_embed(character, &items, display_limit),
            true,
        )
        .await
    }

    async fn handle_charinventory(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(character) = string_option(command, "character") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : character.",
                )
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
            embeds::character_inventory_embed(character, &items, display_limit),
            true,
        )
        .await
    }

    async fn handle_banlist(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self.state.database.ban_list(query_limit).await?;

        self.respond_embed(
            context,
            command,
            embeds::ban_list_embed(&entries, display_limit),
            true,
        )
        .await
    }

    fn has_staff_access(&self, command: &CommandInteraction) -> bool {
        self.has_staff_role(command, StaffRole::Helper)
    }

    fn has_staff_role(&self, command: &CommandInteraction, minimum_role: StaffRole) -> bool {
        let config = &self.state.config.discord;
        let Some(member) = command.member.as_ref() else {
            return false;
        };
        let member_role_ids = member
            .roles
            .iter()
            .map(|role| role.get())
            .collect::<Vec<_>>();

        let configured_roles = ConfiguredRoles {
            helper: &config.helper_role_ids,
            moderator: &config.moderator_role_ids,
            gm: &config.gm_role_ids,
            legacy_staff: &config.staff_role_ids,
            admin: &config.admin_role_ids,
            owner: &config.owner_role_ids,
        };

        has_configured_role(&member_role_ids, minimum_role, configured_roles)
    }

    async fn respond_embed(
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

    async fn respond_error(
        &self,
        context: &Context,
        command: &CommandInteraction,
        message: &str,
    ) -> Result<()> {
        self.respond_embed(context, command, embeds::error_embed(message), true)
            .await
    }

    async fn respond_lines(
        &self,
        context: &Context,
        command: &CommandInteraction,
        title: &str,
        lines: Vec<String>,
        ephemeral: bool,
    ) -> Result<()> {
        self.respond_lines_or_empty(context, command, title, lines, "Aucun resultat.", ephemeral)
            .await
    }

    async fn respond_lines_or_empty(
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

    async fn resolve_item_id(&self, item_query: &str) -> Result<Option<i64>> {
        Ok(self
            .state
            .database
            .search_items(item_query, 1)
            .await?
            .into_iter()
            .next()
            .map(|item| item.item_id))
    }

    async fn log_staff_action(
        &self,
        context: &Context,
        command: &CommandInteraction,
        action: &str,
        message: &str,
        result: &str,
    ) {
        let Some(channel_id) = self.state.config.discord.staff_log_channel_id else {
            info!(
                user_id = command.user.id.get(),
                action = action,
                message = message,
                result = result,
                "Action staff exécutée."
            );
            return;
        };

        let content = format!(
            "gmmsg utilisateur={} action={} message=`{}` résultat={}",
            command.user.id.get(),
            action,
            message.replace('@', "@\u{200B}"),
            result.replace('@', "@\u{200B}")
        );
        if let Err(error) = ChannelId::new(channel_id)
            .send_message(&context.http, CreateMessage::new().content(content))
            .await
        {
            error!(error = %error, "Impossible d’envoyer le log staff Discord.");
        }
    }

    async fn ensure_database_tables(
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
            embeds::missing_database_table_embed(missing_table.name()),
            true,
        )
        .await?;

        Ok(false)
    }

    async fn cached_who_sell(
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

    async fn cached_who_buy(
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

    async fn cached_market_overview(
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

    fn list_limits(&self, command: &CommandInteraction) -> (u32, u32) {
        let display_limit = self.requested_limit(command);
        let query_limit = display_limit.saturating_add(1);

        (display_limit, query_limit)
    }

    fn requested_limit(&self, command: &CommandInteraction) -> u32 {
        self.state
            .config
            .display
            .clamp_limit(integer_option(command, "limit"))
    }
}

fn command_path(command: &CommandInteraction) -> String {
    match subcommand_name(command) {
        Some(subcommand) => format!("{} {}", command.data.name, subcommand),
        None => command.data.name.clone(),
    }
}

fn is_public_pack_root(command_name: &str) -> bool {
    matches!(
        command_name,
        "server"
            | "online"
            | "player"
            | "guild"
            | "castle"
            | "item"
            | "who-drops"
            | "mob"
            | "mvp"
            | "top"
            | "rank"
            | "market"
    )
}

fn is_staff_pack_root(command_name: &str) -> bool {
    matches!(
        command_name,
        "staff" | "mod" | "debug" | "audit" | "db" | "gmmsg"
    )
}

fn subcommand_name(command: &CommandInteraction) -> Option<&str> {
    command
        .data
        .options
        .iter()
        .find_map(|option| match &option.value {
            CommandDataOptionValue::SubCommand(_) | CommandDataOptionValue::SubCommandGroup(_) => {
                Some(option.name.as_str())
            }
            _ => None,
        })
}

fn option_value<'a>(
    options: &'a [CommandDataOption],
    name: &str,
) -> Option<&'a CommandDataOptionValue> {
    for option in options {
        if option.name == name {
            return Some(&option.value);
        }

        match &option.value {
            CommandDataOptionValue::SubCommand(options)
            | CommandDataOptionValue::SubCommandGroup(options) => {
                if let Some(value) = option_value(options, name) {
                    return Some(value);
                }
            }
            _ => {}
        }
    }

    None
}

fn string_option<'a>(command: &'a CommandInteraction, name: &str) -> Option<&'a str> {
    option_value(&command.data.options, name).and_then(|value| match value {
        CommandDataOptionValue::String(value) => Some(value.as_str()),
        _ => None,
    })
}

fn non_negative_integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    integer_option(command, name).filter(|value| *value >= 0)
}

fn integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    option_value(&command.data.options, name).and_then(|value| match value {
        CommandDataOptionValue::Integer(value) => Some(*value),
        _ => None,
    })
}

struct ConfiguredRoles<'a> {
    helper: &'a [u64],
    moderator: &'a [u64],
    gm: &'a [u64],
    legacy_staff: &'a [u64],
    admin: &'a [u64],
    owner: &'a [u64],
}

fn has_configured_role(
    member_role_ids: &[u64],
    minimum_role: StaffRole,
    configured_roles: ConfiguredRoles<'_>,
) -> bool {
    let mut allowed_role_ids = Vec::new();

    if minimum_role <= StaffRole::Helper {
        allowed_role_ids.extend(configured_roles.helper.iter().copied());
        allowed_role_ids.extend(configured_roles.legacy_staff.iter().copied());
    }
    if minimum_role <= StaffRole::Moderator {
        allowed_role_ids.extend(configured_roles.moderator.iter().copied());
    }
    if minimum_role <= StaffRole::Gm {
        allowed_role_ids.extend(configured_roles.gm.iter().copied());
    }
    if minimum_role <= StaffRole::Admin {
        allowed_role_ids.extend(configured_roles.admin.iter().copied());
    }
    allowed_role_ids.extend(configured_roles.owner.iter().copied());

    !allowed_role_ids.is_empty()
        && member_role_ids
            .iter()
            .any(|role_id| allowed_role_ids.contains(role_id))
}

fn sanitize_gm_message(value: &str, max_length: usize) -> std::result::Result<String, String> {
    let sanitized = value
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>()
        .replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if sanitized.trim().is_empty() {
        return Err("Le message ne peut pas être vide.".to_string());
    }

    if sanitized.chars().count() > max_length {
        return Err(format!(
            "Le message dépasse la limite configurée de {max_length} caractères."
        ));
    }

    Ok(sanitized)
}

fn validate_hex_color(value: &str) -> std::result::Result<String, &'static str> {
    let value = value.trim().trim_start_matches('#');
    if value.len() == 6 && value.chars().all(|character| character.is_ascii_hexdigit()) {
        Ok(value.to_ascii_uppercase())
    } else {
        Err("La couleur doit être au format RRGGBB.")
    }
}

fn trim_discord_message(value: &str) -> String {
    const MAX_EMBED_DESCRIPTION: usize = 3900;
    if value.chars().count() <= MAX_EMBED_DESCRIPTION {
        return value.to_string();
    }

    value
        .chars()
        .take(MAX_EMBED_DESCRIPTION.saturating_sub(20))
        .collect::<String>()
        + "\n... sortie tronquée"
}

fn validate_account_username(value: &str) -> std::result::Result<String, String> {
    let trimmed = value.trim();

    if !(4..=23).contains(&trimmed.len()) {
        return Err("Le nom de compte doit contenir entre 4 et 23 caractères.".to_string());
    }

    if !trimmed
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(
            "Le nom de compte doit contenir uniquement lettres, chiffres ou `_`.".to_string(),
        );
    }

    Ok(trimmed.to_string())
}

fn validate_account_password(value: &str) -> std::result::Result<String, String> {
    if !(8..=32).contains(&value.len()) {
        return Err("Le mot de passe doit contenir entre 8 et 32 caractères.".to_string());
    }

    if value
        .chars()
        .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(
            "Le mot de passe ne doit pas contenir d’espace ou caractère de contrôle.".to_string(),
        );
    }

    Ok(value.to_string())
}

fn validate_account_sex(value: &str) -> std::result::Result<String, String> {
    match value.trim().to_ascii_uppercase().as_str() {
        "M" => Ok("M".to_string()),
        "F" => Ok("F".to_string()),
        _ => Err("Le sexe du compte doit être `M` ou `F`.".to_string()),
    }
}

fn validate_account_birthdate(value: &str) -> std::result::Result<String, String> {
    let birthdate = value.trim();
    let birthdate_is_valid = if birthdate.len() == 10 {
        let parts = birthdate.split('-').collect::<Vec<_>>();

        if parts.len() == 3 && parts[0].len() == 4 && parts[1].len() == 2 && parts[2].len() == 2 {
            match (
                parts[0].parse::<u16>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                (Ok(year), Ok(month), Ok(day)) => {
                    let leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                    let max_day = match month {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        2 if leap_year => 29,
                        2 => 28,
                        _ => 0,
                    };

                    year >= 1900 && max_day > 0 && day >= 1 && day <= max_day
                }
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    if !birthdate_is_valid {
        return Err("La date de naissance doit être au format `YYYY-MM-DD`.".to_string());
    }

    Ok(birthdate.to_string())
}

fn validate_account_email(value: Option<&str>) -> std::result::Result<String, String> {
    let email = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("a@a.com");

    if email.len() > 39 {
        return Err("L’email du compte doit contenir au maximum 39 caractères.".to_string());
    }

    if !email.contains('@') || email.chars().any(|character| character.is_control()) {
        return Err("L’email du compte est invalide.".to_string());
    }

    Ok(email.to_string())
}

async fn cached_data<T, F>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn staff_role_logic_requires_configured_matching_role() {
        assert!(!test_staff_role(&[10], &[], &[], &[]));
        assert!(!test_staff_role(&[10], &[20], &[], &[]));
        assert!(test_staff_role(&[10], &[10], &[], &[]));
        assert!(test_staff_role(&[30], &[], &[30], &[]));
        assert!(test_staff_role(&[40], &[], &[], &[40]));
    }

    fn test_staff_role(
        member_role_ids: &[u64],
        staff_role_ids: &[u64],
        admin_role_ids: &[u64],
        owner_role_ids: &[u64],
    ) -> bool {
        has_configured_role(
            member_role_ids,
            StaffRole::Helper,
            ConfiguredRoles {
                helper: staff_role_ids,
                moderator: &[],
                gm: &[],
                legacy_staff: staff_role_ids,
                admin: admin_role_ids,
                owner: owner_role_ids,
            },
        )
    }

    #[test]
    fn account_username_validation_is_strict() {
        assert_eq!(validate_account_username("User_123").unwrap(), "User_123");
        assert!(validate_account_username("abc").is_err());
        assert!(validate_account_username("invalid-name").is_err());
    }

    #[test]
    fn account_birthdate_validation_is_strict() {
        assert_eq!(
            validate_account_birthdate(" 2000-02-29 ").unwrap(),
            "2000-02-29"
        );
        assert!(validate_account_birthdate("1899-12-31").is_err());
        assert!(validate_account_birthdate("2001-02-29").is_err());
        assert!(validate_account_birthdate("2001/02/28").is_err());
    }

    #[test]
    fn account_email_defaults_and_validates() {
        assert_eq!(validate_account_email(None).unwrap(), "a@a.com");
        assert_eq!(
            validate_account_email(Some(" user@example.test ")).unwrap(),
            "user@example.test"
        );
        assert!(validate_account_email(Some("invalid")).is_err());
    }

    #[test]
    fn sensitive_staff_commands_are_not_cacheable() {
        let sensitive_commands = [
            "createaccount",
            "staff",
            "mod",
            "debug",
            "audit",
            "db",
            "gmmsg",
        ];

        for command_name in sensitive_commands {
            assert!(
                !CACHED_COMMAND_NAMES.contains(&command_name),
                "{command_name} must not be cacheable"
            );
        }
    }

    #[tokio::test]
    async fn cached_data_reuses_value_before_expiration() {
        let cache = TimedCache::<String, usize>::default();
        let calls = AtomicUsize::new(0);
        let key = "status".to_string();

        let first = cached_data(
            "status",
            key.clone(),
            Some(Duration::from_secs(1)),
            &cache,
            async { Ok(calls.fetch_add(1, Ordering::SeqCst) + 1) },
        )
        .await
        .expect("first value");

        let second = cached_data("status", key, Some(Duration::from_secs(1)), &cache, async {
            Ok(calls.fetch_add(1, Ordering::SeqCst) + 1)
        })
        .await
        .expect("cached value");

        assert_eq!(first, 1);
        assert_eq!(second, 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn cached_data_fetches_again_when_disabled() {
        let cache = TimedCache::<String, usize>::default();
        let calls = AtomicUsize::new(0);

        let first = cached_data("status", "key".to_string(), None, &cache, async {
            Ok(calls.fetch_add(1, Ordering::SeqCst) + 1)
        })
        .await
        .expect("first value");

        let second = cached_data("status", "key".to_string(), None, &cache, async {
            Ok(calls.fetch_add(1, Ordering::SeqCst) + 1)
        })
        .await
        .expect("second value");

        assert_eq!(first, 1);
        assert_eq!(second, 2);
        assert_eq!(cache.get(&"key".to_string()), None);
    }
}
