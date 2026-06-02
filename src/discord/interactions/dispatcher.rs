use crate::cache::{BotCache, StatusCacheEntry, TimedCache};
use crate::config::{AppConfig, AssetConfig};
use crate::discord::embeds;
use crate::infra::observability::CommandTimer;
use crate::rathenafr::{
    check_services, AccountUpdateRequest, BuyingStoreEntry, ClassDistributionEntry, DatabaseTable,
    MapStatsEntry, MarketBuyEntry, MarketOverview, MarketSellEntry, RAthenaFrDatabase,
    SearchResults, VendingStoreEntry,
};
use anyhow::Result;
use serenity::all::{
    async_trait, ActivityData, ApplicationId, Client, CommandDataOptionValue, CommandInteraction,
    Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
    GatewayIntents, Interaction, OnlineStatus, Ready,
};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

const STATUS_CACHE_TTL_SECONDS: u64 = 10;
const GUILDS_CACHE_TTL_SECONDS: u64 = 30;
const CLASSES_CACHE_TTL_SECONDS: u64 = 60;
const MAP_STATS_CACHE_TTL_SECONDS: u64 = 60;
const CASTLES_CACHE_TTL_SECONDS: u64 = 60;
const MARKET_CACHE_TTL_SECONDS: u64 = 20;

#[cfg(test)]
const CACHED_COMMAND_NAMES: &[&str] = &[
    "status", "guilds", "classes", "mapstats", "castles", "whosell", "whobuy", "market", "venders",
    "buyers",
];

const BUYING_STORE_TABLES: &[DatabaseTable] =
    &[DatabaseTable::BuyingStores, DatabaseTable::BuyingStoreItems];
const CASTLE_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildCastle];
const GUILD_ALLIANCE_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildAlliance];
const GUILD_MEMBER_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildMember];
const GUILD_SKILL_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildSkill];
const HOMUNCULUS_TABLES: &[DatabaseTable] = &[DatabaseTable::Homunculus];
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
const PARTY_TABLES: &[DatabaseTable] = &[DatabaseTable::Party];
const PET_TABLES: &[DatabaseTable] = &[DatabaseTable::Pet];
const QUEST_TABLES: &[DatabaseTable] = &[DatabaseTable::Quest];
const SELL_TABLES: &[DatabaseTable] = &[
    DatabaseTable::Vendings,
    DatabaseTable::VendingItems,
    DatabaseTable::CartInventory,
];
const VENDING_STORE_TABLES: &[DatabaseTable] =
    &[DatabaseTable::Vendings, DatabaseTable::VendingItems];

pub struct BotState {
    pub config: Arc<AppConfig>,
    pub database: Arc<RAthenaFrDatabase>,
    pub cache: BotCache,
}

pub async fn create_client(
    config: Arc<AppConfig>,
    database: Arc<RAthenaFrDatabase>,
) -> Result<Client> {
    let intents = GatewayIntents::empty();
    let handler = Handler {
        state: Arc::new(BotState {
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
        match command.data.name.as_str() {
            "status" => self.handle_status(context, command).await,
            "online" => self.handle_online(context, command).await,
            "top" => self.handle_top(context, command).await,
            "player" => self.handle_player(context, command).await,
            "guilds" | "guildes" => self.handle_guilds(context, command).await,
            "search" => self.handle_search(context, command).await,
            "createaccount" => self.handle_createaccount(context, command).await,
            "topzeny" => self.handle_topzeny(context, command).await,
            "guild" => self.handle_guild(context, command).await,
            "guildmembers" => self.handle_guildmembers(context, command).await,
            "classes" => self.handle_classes(context, command).await,
            "mapstats" => self.handle_mapstats(context, command).await,
            "maponline" => self.handle_maponline(context, command).await,
            "party" => self.handle_party(context, command).await,
            "partymembers" => self.handle_partymembers(context, command).await,
            "homunculus" => self.handle_homunculus(context, command).await,
            "pet" => self.handle_pet(context, command).await,
            "zeny" => self.handle_zeny(context, command).await,
            "castles" => self.handle_castles(context, command).await,
            "castle" => self.handle_castle(context, command).await,
            "guildalliances" => self.handle_guildalliances(context, command).await,
            "guildskills" => self.handle_guildskills(context, command).await,
            "homunculustop" => self.handle_homunculustop(context, command).await,
            "pettop" => self.handle_pettop(context, command).await,
            "queststats" => self.handle_queststats(context, command).await,
            "whosell" => self.handle_whosell(context, command).await,
            "whobuy" => self.handle_whobuy(context, command).await,
            "market" => self.handle_market(context, command).await,
            "venders" => self.handle_venders(context, command).await,
            "buyers" => self.handle_buyers(context, command).await,
            "charquests" => self.handle_charquests(context, command).await,
            "charequipment" => self.handle_charequipment(context, command).await,
            "charinventory" => self.handle_charinventory(context, command).await,
            "itemcount" => self.handle_itemcount(context, command).await,
            "itemowners" => self.handle_itemowners(context, command).await,
            "accountlist" => self.handle_accountlist(context, command).await,
            "accountoverview" => self.handle_accountoverview(context, command).await,
            "accountmanage" => self.handle_accountmanage(context, command).await,
            "banlist" => self.handle_banlist(context, command).await,
            "accountchars" => self.handle_accountchars(context, command).await,
            "accountstatus" => self.handle_accountstatus(context, command).await,
            _ => {
                self.respond_error(context, command, "Commande inconnue.")
                    .await
            }
        }
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

    async fn handle_search(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(query) = string_option(command, "query") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : query.")
                .await;
        };
        let query = query.trim();
        if query.is_empty() {
            return self
                .respond_error(context, command, "La recherche ne peut pas être vide.")
                .await;
        }
        let category = match SearchCategory::from_option(string_option(command, "category")) {
            Ok(category) => category,
            Err(message) => return self.respond_error(context, command, message).await,
        };

        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let results = match category {
            SearchCategory::All => {
                self.state
                    .database
                    .search_all(group_threshold, query, query_limit)
                    .await?
            }
            SearchCategory::Players => SearchResults {
                characters: self
                    .state
                    .database
                    .search_characters(group_threshold, query, query_limit)
                    .await?,
                items: Vec::new(),
                monsters: Vec::new(),
            },
            SearchCategory::Items => SearchResults {
                characters: Vec::new(),
                items: self.state.database.search_items(query, query_limit).await?,
                monsters: Vec::new(),
            },
            SearchCategory::Monsters => SearchResults {
                characters: Vec::new(),
                items: Vec::new(),
                monsters: self
                    .state
                    .database
                    .search_monsters(query, query_limit)
                    .await?,
            },
        };

        warm_search_asset_cache(&self.state.config.assets, &results).await;

        self.respond_embeds(
            context,
            command,
            embeds::search_embeds(
                query,
                category.label(),
                &results,
                display_limit,
                &self.state.config.assets,
            ),
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

    async fn handle_classes(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let entries = self.cached_classes(group_threshold, query_limit).await?;

        self.respond_embed(
            context,
            command,
            embeds::classes_embed(&entries, display_limit),
            false,
        )
        .await
    }

    async fn handle_mapstats(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let (display_limit, query_limit) = self.list_limits(command);
        let online_only = bool_option(command, "online_only").unwrap_or(false);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let entries = self
            .cached_map_stats(group_threshold, online_only, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::map_stats_embed(&entries, online_only, display_limit),
            false,
        )
        .await
    }

    async fn handle_maponline(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(map) = string_option(command, "map") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : map.")
                .await;
        };

        let (display_limit, query_limit) = self.list_limits(command);
        let characters = self
            .state
            .database
            .map_online_characters(
                self.state.config.display.public_character_group_threshold(),
                map,
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::map_online_embed(map, &characters, display_limit),
            false,
        )
        .await
    }

    async fn handle_party(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(name) = string_option(command, "name") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : name.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, PARTY_TABLES)
            .await?
        {
            return Ok(());
        }

        let party = self
            .state
            .database
            .find_party(
                name,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;

        let embed = match party {
            Some(party) => embeds::party_embed(&party),
            None => embeds::party_not_found_embed(name),
        };

        self.respond_embed(context, command, embed, false).await
    }

    async fn handle_partymembers(
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
            .ensure_database_tables(context, command, PARTY_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let members = self
            .state
            .database
            .party_members(
                name,
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::party_members_embed(name, &members, display_limit),
            false,
        )
        .await
    }

    async fn handle_homunculus(
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

        if !self
            .ensure_database_tables(context, command, HOMUNCULUS_TABLES)
            .await?
        {
            return Ok(());
        }

        let homunculus = self
            .state
            .database
            .find_homunculus(
                character,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;

        let embed = match homunculus {
            Some(homunculus) => embeds::homunculus_embed(&homunculus),
            None => embeds::homunculus_not_found_embed(character),
        };

        self.respond_embed(context, command, embed, false).await
    }

    async fn handle_pet(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
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
            .ensure_database_tables(context, command, PET_TABLES)
            .await?
        {
            return Ok(());
        }

        let pet = self
            .state
            .database
            .find_pet(
                character,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;

        let embed = match pet {
            Some(pet) => embeds::pet_embed(&pet),
            None => embeds::pet_not_found_embed(character),
        };

        self.respond_embed(context, command, embed, false).await
    }

    async fn handle_zeny(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let summary = self
            .state
            .database
            .zeny_summary(self.state.config.display.ranking_group_threshold())
            .await?;

        self.respond_embed(context, command, embeds::zeny_embed(&summary), false)
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

    async fn handle_guildalliances(
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
            .ensure_database_tables(context, command, GUILD_ALLIANCE_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let alliances = self
            .state
            .database
            .guild_alliances(name, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::guild_alliances_embed(name, &alliances, display_limit),
            false,
        )
        .await
    }

    async fn handle_guildskills(
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
            .ensure_database_tables(context, command, GUILD_SKILL_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let skills = self.state.database.guild_skills(name, query_limit).await?;

        self.respond_embed(
            context,
            command,
            embeds::guild_skills_embed(name, &skills, display_limit),
            false,
        )
        .await
    }

    async fn handle_homunculustop(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, HOMUNCULUS_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self
            .state
            .database
            .top_homunculus(
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::homunculus_top_embed(&entries, display_limit),
            false,
        )
        .await
    }

    async fn handle_pettop(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, PET_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let entries = self
            .state
            .database
            .top_pets(
                self.state.config.display.public_character_group_threshold(),
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::pet_top_embed(&entries, display_limit),
            false,
        )
        .await
    }

    async fn handle_queststats(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        let Some(quest_id) = positive_integer_option(command, "quest_id") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : quest_id.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, QUEST_TABLES)
            .await?
        {
            return Ok(());
        }

        let stats = self
            .state
            .database
            .quest_stats(
                quest_id,
                self.state.config.display.public_character_group_threshold(),
            )
            .await?;

        self.respond_embed(context, command, embeds::quest_stats_embed(&stats), false)
            .await
    }

    async fn handle_whosell(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(item_id) = positive_integer_option(command, "item_id") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item_id.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, SELL_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let sellers = self
            .cached_who_sell(item_id, group_threshold, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::who_sell_embed(item_id, &sellers, display_limit),
            false,
        )
        .await
    }

    async fn handle_whobuy(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(item_id) = positive_integer_option(command, "item_id") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item_id.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, BUYING_STORE_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let buyers = self
            .cached_who_buy(item_id, group_threshold, query_limit)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::who_buy_embed(item_id, &buyers, display_limit),
            false,
        )
        .await
    }

    async fn handle_market(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        let Some(item_id) = positive_integer_option(command, "item_id") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item_id.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, MARKET_TABLES)
            .await?
        {
            return Ok(());
        }

        let group_threshold = self.state.config.display.public_character_group_threshold();
        let overview = self
            .cached_market_overview(item_id, group_threshold)
            .await?;

        self.respond_embed(context, command, embeds::market_embed(&overview), false)
            .await
    }

    async fn handle_venders(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, VENDING_STORE_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let stores = self.cached_venders(group_threshold, query_limit).await?;

        self.respond_embed(
            context,
            command,
            embeds::venders_embed(&stores, display_limit),
            false,
        )
        .await
    }

    async fn handle_buyers(&self, context: &Context, command: &CommandInteraction) -> Result<()> {
        if !self
            .ensure_database_tables(context, command, BUYING_STORE_TABLES)
            .await?
        {
            return Ok(());
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let group_threshold = self.state.config.display.public_character_group_threshold();
        let stores = self.cached_buyers(group_threshold, query_limit).await?;

        self.respond_embed(
            context,
            command,
            embeds::buyers_embed(&stores, display_limit),
            false,
        )
        .await
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

    async fn handle_itemcount(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(item_id) = positive_integer_option(command, "item_id") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item_id.")
                .await;
        };

        if !self
            .ensure_database_tables(context, command, ITEM_STORAGE_TABLES)
            .await?
        {
            return Ok(());
        }

        let summary = self.state.database.item_count(item_id).await?;

        self.respond_embed(context, command, embeds::item_count_embed(&summary), true)
            .await
    }

    async fn handle_itemowners(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(item_id) = positive_integer_option(command, "item_id") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : item_id.")
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

    async fn handle_accountlist(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let (display_limit, query_limit) = self.list_limits(command);
        let page = positive_integer_option(command, "page").unwrap_or(1);
        let page = u32::try_from(page).unwrap_or(u32::MAX);
        let accounts = self.state.database.account_list(query_limit, page).await?;

        self.respond_embed(
            context,
            command,
            embeds::account_list_embed(&accounts, display_limit),
            true,
        )
        .await
    }

    async fn handle_accountoverview(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(account_id) = positive_integer_option(command, "account_id") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : account_id.",
                )
                .await;
        };

        let (display_limit, query_limit) = self.list_limits(command);
        let account = self.state.database.account_status(account_id).await?;
        let characters = self
            .state
            .database
            .account_characters(account_id, query_limit)
            .await?;

        let embed = match account {
            Some(account) => embeds::account_overview_embed(&account, &characters, display_limit),
            None => embeds::account_not_found_embed(account_id),
        };

        self.respond_embed(context, command, embed, true).await
    }

    async fn handle_accountmanage(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(account_id) = positive_integer_option(command, "account_id") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : account_id.",
                )
                .await;
        };
        let Some(action) = string_option(command, "action") else {
            return self
                .respond_error(context, command, "Option obligatoire manquante : action.")
                .await;
        };
        if action.eq_ignore_ascii_case("edit") {
            let update = match account_update_request(command) {
                Ok(update) => update,
                Err(message) => return self.respond_error(context, command, &message).await,
            };

            if update.is_empty() {
                return self
                    .respond_error(
                        context,
                        command,
                        "Aucun champ à modifier. Renseigne au moins une option d’édition.",
                    )
                    .await;
            }

            let result = self
                .state
                .database
                .update_account(
                    account_id,
                    update,
                    self.state.config.account_commands.password_mode,
                )
                .await?;

            return self
                .respond_embed(
                    context,
                    command,
                    embeds::account_update_result_embed(&result),
                    true,
                )
                .await;
        }

        if !action.eq_ignore_ascii_case("delete") {
            return self
                .respond_error(context, command, "Actions supportées : edit, delete.")
                .await;
        }

        let Some(confirm) = string_option(command, "confirm") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire pour delete : confirm.",
                )
                .await;
        };

        let expected_confirmation = format!("DELETE-ALL-{account_id}");
        if confirm != expected_confirmation {
            return self
                .respond_error(
                    context,
                    command,
                    &format!("Confirmation attendue : `{expected_confirmation}`."),
                )
                .await;
        }

        let result = self
            .state
            .database
            .delete_account_completely(account_id)
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::account_delete_result_embed(&result),
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

    async fn handle_accountchars(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(account_id) = positive_integer_option(command, "account_id") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : account_id.",
                )
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

    async fn handle_accountstatus(
        &self,
        context: &Context,
        command: &CommandInteraction,
    ) -> Result<()> {
        if !self.has_staff_access(command) {
            return self
                .respond_embed(context, command, embeds::staff_only_embed(), true)
                .await;
        }

        let Some(account_id) = positive_integer_option(command, "account_id") else {
            return self
                .respond_error(
                    context,
                    command,
                    "Option obligatoire manquante : account_id.",
                )
                .await;
        };

        let account = self.state.database.account_status(account_id).await?;
        let embed = match account {
            Some(account) => embeds::account_status_embed(&account),
            None => embeds::account_not_found_embed(account_id),
        };

        self.respond_embed(context, command, embed, true).await
    }

    fn has_staff_access(&self, command: &CommandInteraction) -> bool {
        let config = &self.state.config.discord;
        let Some(member) = command.member.as_ref() else {
            return false;
        };
        let member_role_ids = member
            .roles
            .iter()
            .map(|role| role.get())
            .collect::<Vec<_>>();

        has_configured_staff_role(
            &member_role_ids,
            &config.staff_role_ids,
            &config.admin_role_ids,
            &config.owner_role_ids,
        )
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

    async fn respond_embeds(
        &self,
        context: &Context,
        command: &CommandInteraction,
        embeds: Vec<serenity::all::CreateEmbed>,
        ephemeral: bool,
    ) -> Result<()> {
        command
            .create_response(
                &context.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embeds(embeds)
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

    async fn cached_classes(
        &self,
        group_threshold: i32,
        query_limit: u32,
    ) -> Result<Vec<ClassDistributionEntry>> {
        cached_data(
            "classes",
            format!("group_threshold={group_threshold};limit={query_limit}"),
            self.state.config.cache.duration(CLASSES_CACHE_TTL_SECONDS),
            &self.state.cache.classes,
            async {
                self.state
                    .database
                    .class_distribution(group_threshold, query_limit)
                    .await
            },
        )
        .await
    }

    async fn cached_map_stats(
        &self,
        group_threshold: i32,
        online_only: bool,
        query_limit: u32,
    ) -> Result<Vec<MapStatsEntry>> {
        cached_data(
            "mapstats",
            format!(
                "group_threshold={group_threshold};online_only={online_only};limit={query_limit}"
            ),
            self.state
                .config
                .cache
                .duration(MAP_STATS_CACHE_TTL_SECONDS),
            &self.state.cache.map_stats,
            async {
                self.state
                    .database
                    .map_stats(group_threshold, online_only, query_limit)
                    .await
            },
        )
        .await
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

    async fn cached_venders(
        &self,
        group_threshold: i32,
        query_limit: u32,
    ) -> Result<Vec<VendingStoreEntry>> {
        cached_data(
            "venders",
            format!("group_threshold={group_threshold};limit={query_limit}"),
            self.state.config.cache.duration(MARKET_CACHE_TTL_SECONDS),
            &self.state.cache.venders,
            async {
                self.state
                    .database
                    .vending_stores(group_threshold, query_limit)
                    .await
            },
        )
        .await
    }

    async fn cached_buyers(
        &self,
        group_threshold: i32,
        query_limit: u32,
    ) -> Result<Vec<BuyingStoreEntry>> {
        cached_data(
            "buyers",
            format!("group_threshold={group_threshold};limit={query_limit}"),
            self.state.config.cache.duration(MARKET_CACHE_TTL_SECONDS),
            &self.state.cache.buyers,
            async {
                self.state
                    .database
                    .buying_stores(group_threshold, query_limit)
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SearchCategory {
    All,
    Players,
    Items,
    Monsters,
}

impl SearchCategory {
    fn from_option(value: Option<&str>) -> std::result::Result<Self, &'static str> {
        match value.unwrap_or("all") {
            "all" => Ok(Self::All),
            "players" => Ok(Self::Players),
            "items" => Ok(Self::Items),
            "monsters" => Ok(Self::Monsters),
            _ => Err("Catégorie supportée : all, players, items ou monsters."),
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::All => "toutes les catégories",
            Self::Players => "joueurs",
            Self::Items => "items",
            Self::Monsters => "monstres",
        }
    }
}

fn account_update_request(
    command: &CommandInteraction,
) -> std::result::Result<AccountUpdateRequest, String> {
    Ok(AccountUpdateRequest {
        userid: string_option(command, "username")
            .map(validate_account_username)
            .transpose()?,
        password: string_option(command, "password")
            .map(validate_account_password)
            .transpose()?,
        sex: string_option(command, "sex")
            .map(validate_account_sex)
            .transpose()?,
        birthdate: string_option(command, "birthdate")
            .map(validate_account_birthdate)
            .transpose()?,
        email: string_option(command, "email")
            .map(|value| validate_account_email(Some(value)))
            .transpose()?,
        group_id: optional_non_negative_i32_option(command, "group_id")?,
        state: optional_non_negative_i64_option(command, "state")?,
        unban_time: optional_non_negative_i64_option(command, "unban_time")?,
        expiration_time: optional_non_negative_i64_option(command, "expiration_time")?,
        character_slots: optional_non_negative_i32_option(command, "character_slots")?,
    })
}

fn string_option<'a>(command: &'a CommandInteraction, name: &str) -> Option<&'a str> {
    command
        .data
        .options
        .iter()
        .find(|option| option.name == name)
        .and_then(|option| match &option.value {
            CommandDataOptionValue::String(value) => Some(value.as_str()),
            _ => None,
        })
}

fn bool_option(command: &CommandInteraction, name: &str) -> Option<bool> {
    command
        .data
        .options
        .iter()
        .find(|option| option.name == name)
        .and_then(|option| match &option.value {
            CommandDataOptionValue::Boolean(value) => Some(*value),
            _ => None,
        })
}

fn positive_integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    integer_option(command, name).filter(|value| *value > 0)
}

fn non_negative_integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    integer_option(command, name).filter(|value| *value >= 0)
}

fn integer_option(command: &CommandInteraction, name: &str) -> Option<i64> {
    command
        .data
        .options
        .iter()
        .find(|option| option.name == name)
        .and_then(|option| match &option.value {
            CommandDataOptionValue::Integer(value) => Some(*value),
            _ => None,
        })
}

fn optional_non_negative_i32_option(
    command: &CommandInteraction,
    name: &str,
) -> std::result::Result<Option<i32>, String> {
    let Some(value) = integer_option(command, name) else {
        return Ok(None);
    };

    if value < 0 {
        return Err(format!("`{name}` doit être supérieur ou égal à 0."));
    }

    if value > i32::MAX as i64 {
        return Err(format!("`{name}` est trop grand."));
    }

    Ok(Some(value as i32))
}

fn optional_non_negative_i64_option(
    command: &CommandInteraction,
    name: &str,
) -> std::result::Result<Option<i64>, String> {
    let Some(value) = integer_option(command, name) else {
        return Ok(None);
    };

    if value < 0 {
        return Err(format!("`{name}` doit être supérieur ou égal à 0."));
    }

    Ok(Some(value))
}

fn has_configured_staff_role(
    member_role_ids: &[u64],
    staff_role_ids: &[u64],
    admin_role_ids: &[u64],
    owner_role_ids: &[u64],
) -> bool {
    let allowed_role_ids = staff_role_ids
        .iter()
        .chain(admin_role_ids.iter())
        .chain(owner_role_ids.iter())
        .copied()
        .collect::<Vec<_>>();

    !allowed_role_ids.is_empty()
        && member_role_ids
            .iter()
            .any(|role_id| allowed_role_ids.contains(role_id))
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

async fn warm_search_asset_cache(assets: &AssetConfig, results: &SearchResults) {
    let Some(url) = monster_cache_warmup_url(assets, results) else {
        return;
    };

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            debug!(error = %error, "Impossible de creer le client de prechargement FluxCP.");
            return;
        }
    };

    match client.get(&url).send().await {
        Ok(response) => {
            debug!(
                url = %url,
                status = response.status().as_u16(),
                "Prechargement FluxCP monstre termine."
            );
        }
        Err(error) => {
            debug!(
                url = %url,
                error = %error,
                "Prechargement FluxCP monstre ignore."
            );
        }
    }
}

fn monster_cache_warmup_url(assets: &AssetConfig, results: &SearchResults) -> Option<String> {
    if !results.items.is_empty() {
        return None;
    }

    let monster = results.monsters.first()?;
    let path = assets.monster_image_path.trim();

    if path.starts_with("http://") || path.starts_with("https://") {
        return None;
    }

    let base_url = assets.base_url.as_deref()?.trim_end_matches('/');
    if base_url.is_empty() {
        return None;
    }

    Some(format!(
        "{base_url}/?module=monster&action=view&id={}",
        monster.monster_id
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn staff_role_logic_requires_configured_matching_role() {
        assert!(!has_configured_staff_role(&[10], &[], &[], &[]));
        assert!(!has_configured_staff_role(&[10], &[20], &[], &[]));
        assert!(has_configured_staff_role(&[10], &[10], &[], &[]));
        assert!(has_configured_staff_role(&[30], &[], &[30], &[]));
        assert!(has_configured_staff_role(&[40], &[], &[], &[40]));
    }

    #[test]
    fn search_category_defaults_and_validates() {
        assert_eq!(
            SearchCategory::from_option(None).unwrap(),
            SearchCategory::All
        );
        assert_eq!(
            SearchCategory::from_option(Some("players")).unwrap(),
            SearchCategory::Players
        );
        assert_eq!(
            SearchCategory::from_option(Some("items")).unwrap(),
            SearchCategory::Items
        );
        assert_eq!(
            SearchCategory::from_option(Some("monsters")).unwrap(),
            SearchCategory::Monsters
        );
        assert!(SearchCategory::from_option(Some("guilds")).is_err());
    }

    #[test]
    fn monster_cache_warmup_uses_fluxcp_view_for_relative_monster_images() {
        let assets = AssetConfig {
            base_url: Some("https://panel.example.com".to_string()),
            item_icon_path: "https://cdn.example.com/items/{item_id}.png".to_string(),
            monster_image_path: "data/monsters/{monster_id}.png".to_string(),
            character_image_path: None,
        };
        let results = SearchResults {
            characters: Vec::new(),
            items: Vec::new(),
            monsters: vec![crate::rathenafr::MonsterSearchEntry {
                monster_id: 1039,
                sprite: "BAPHOMET".to_string(),
                display_name: "Baphomet".to_string(),
                level: 81,
                hp: 668000,
                source_table: "mob_db".to_string(),
            }],
        };

        assert_eq!(
            monster_cache_warmup_url(&assets, &results).as_deref(),
            Some("https://panel.example.com/?module=monster&action=view&id=1039")
        );
    }

    #[test]
    fn monster_cache_warmup_skips_absolute_monster_images_and_item_first_results() {
        let assets = AssetConfig {
            base_url: Some("https://panel.example.com".to_string()),
            item_icon_path: "https://cdn.example.com/items/{item_id}.png".to_string(),
            monster_image_path: "https://cdn.example.com/mobs/{monster_id}.png".to_string(),
            character_image_path: None,
        };
        let monster = crate::rathenafr::MonsterSearchEntry {
            monster_id: 1039,
            sprite: "BAPHOMET".to_string(),
            display_name: "Baphomet".to_string(),
            level: 81,
            hp: 668000,
            source_table: "mob_db".to_string(),
        };
        let item_first_results = SearchResults {
            characters: Vec::new(),
            items: vec![crate::rathenafr::ItemSearchEntry {
                item_id: 501,
                aegis_name: "Red_Potion".to_string(),
                display_name: "Red Potion".to_string(),
                item_type: "Healing".to_string(),
                source_table: "item_db".to_string(),
            }],
            monsters: vec![monster.clone()],
        };
        let absolute_monster_results = SearchResults {
            characters: Vec::new(),
            items: Vec::new(),
            monsters: vec![monster],
        };

        assert_eq!(monster_cache_warmup_url(&assets, &item_first_results), None);
        assert_eq!(
            monster_cache_warmup_url(&assets, &absolute_monster_results),
            None
        );
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
            "accountstatus",
            "accountchars",
            "accountoverview",
            "charquests",
            "charinventory",
            "charequipment",
            "itemcount",
            "itemowners",
            "accountlist",
            "accountmanage",
            "banlist",
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
