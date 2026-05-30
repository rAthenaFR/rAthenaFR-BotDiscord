use crate::cache::{BotCache, StatusCacheEntry, TimedCache};
use crate::config::AppConfig;
use crate::discord::embeds;
use crate::infra::observability::CommandTimer;
use crate::rathenafr::{
    check_services, BuyingStoreEntry, ClassDistributionEntry, DatabaseTable, MapStatsEntry,
    MarketBuyEntry, MarketOverview, MarketSellEntry, RAthenaFrDatabase, VendingStoreEntry,
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
    "status", "guildes", "classes", "mapstats", "castles", "whosell", "whobuy", "market",
    "venders", "buyers",
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
                        error = %why,
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
            "guildes" => self.handle_guilds(context, command).await,
            "search" => self.handle_search(context, command).await,
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
            "accountoverview" => self.handle_accountoverview(context, command).await,
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

        let (display_limit, query_limit) = self.list_limits(command);
        let characters = self
            .state
            .database
            .search_characters(
                self.state.config.display.public_character_group_threshold(),
                query,
                query_limit,
            )
            .await?;

        self.respond_embed(
            context,
            command,
            embeds::search_embed(query, &characters, display_limit),
            false,
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
        assert!(!has_configured_staff_role(&[10], &[], &[], &[]));
        assert!(!has_configured_staff_role(&[10], &[20], &[], &[]));
        assert!(has_configured_staff_role(&[10], &[10], &[], &[]));
        assert!(has_configured_staff_role(&[30], &[], &[30], &[]));
        assert!(has_configured_staff_role(&[40], &[], &[], &[40]));
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
