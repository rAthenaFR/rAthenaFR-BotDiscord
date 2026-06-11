#![allow(dead_code, unused_imports)]

use super::{
    account_manage,
    staff_audit::{AccountManageAuditEntry, GmmsgAuditEntry, StaffAuditLogger},
};
use crate::cache::{BotCache, StatusCacheEntry, TimedCache};
use crate::config::{AppConfig, GameBridgeMode, StaffRole, TopZenyMode};
use crate::discord::embeds;
use crate::i18n::{translate, translate_with_args, BotLocale, I18nKey, TranslationArg};
use crate::infra::observability::CommandTimer;
use crate::rathenafr::{
    check_services, BroadcastMode, DatabaseTable, GameBridge, MarketBuyEntry, MarketOverview,
    MarketSellEntry, RAthenaFrDatabase,
};
use anyhow::Result;
use serenity::all::{
    async_trait, ActivityData, ApplicationId, ButtonStyle, Client, CommandDataOption,
    CommandDataOptionValue, CommandInteraction, ComponentInteraction, Context, CreateActionRow,
    CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
    GatewayIntents, Interaction, OnlineStatus, Ready,
};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

mod components;
mod public;
mod responses;
mod router;
mod routing;
mod staff;
mod validation;

#[cfg(test)]
mod tests;

pub(super) use components::*;
pub(super) use responses::*;
pub(super) use routing::*;
pub(super) use staff::*;
pub(super) use validation::*;

const STATUS_CACHE_TTL_SECONDS: u64 = 10;
const GUILDS_CACHE_TTL_SECONDS: u64 = 30;
const CASTLES_CACHE_TTL_SECONDS: u64 = 60;
const MARKET_CACHE_TTL_SECONDS: u64 = 20;
const MVP_LIST_COMPONENT_PREFIX: &str = "mvp_list:";
const MVP_LIST_FETCH_LIMIT: u32 = 500;
const MVP_LIST_PAGE_SIZE_LIMIT: usize = 10;
const MVP_LAST_DISPLAY_LIMIT: u32 = 10;

#[cfg(test)]
const CACHED_COMMAND_NAMES: &[&str] =
    &["status", "guilds", "castles", "whosell", "whobuy", "market"];

const BUYING_STORE_TABLES: &[DatabaseTable] =
    &[DatabaseTable::BuyingStores, DatabaseTable::BuyingStoreItems];
const CASTLE_TABLES: &[DatabaseTable] = &[DatabaseTable::GuildCastle];
const GUILD_TABLES: &[DatabaseTable] = &[
    DatabaseTable::Guild,
    DatabaseTable::GuildMember,
    DatabaseTable::Char,
    DatabaseTable::Login,
];
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
            game_bridge: GameBridge::new(config.game_bridge.clone(), Arc::clone(&database)),
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
        match interaction {
            Interaction::Command(command) => {
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
                                        .embed(embeds::command_error_embed_l10n(
                                            self.locale_for_command(&command),
                                            &why.to_string(),
                                        ))
                                        .ephemeral(true),
                                ),
                            )
                            .await;
                    }
                }
            }
            Interaction::Component(component) => {
                if !component
                    .data
                    .custom_id
                    .starts_with(MVP_LIST_COMPONENT_PREFIX)
                {
                    return;
                }

                let timer = CommandTimer::start();
                let result = self.handle_component(&context, &component).await;
                let duration_ms = timer.elapsed_ms();
                let custom_id = component.data.custom_id.as_str();
                let guild_id = component.guild_id.map(|guild_id| guild_id.get());
                let user_id = component.user.id.get();

                match result {
                    Ok(()) => {
                        info!(
                            component = custom_id,
                            guild_id = ?guild_id,
                            user_id = user_id,
                            duration_ms = duration_ms,
                            "Interaction composant terminée avec succès."
                        );
                    }
                    Err(why) => {
                        error!(
                            error = %format!("{why:#}"),
                            component = custom_id,
                            guild_id = ?guild_id,
                            user_id = user_id,
                            duration_ms = duration_ms,
                            "L'interaction composant a échoué."
                        );

                        let _ = component
                            .create_response(
                                &context.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .embed(embeds::command_error_embed_l10n(
                                            self.locale_for_component(&component),
                                            &why.to_string(),
                                        ))
                                        .ephemeral(true),
                                ),
                            )
                            .await;
                    }
                }
            }
            _ => {}
        }
    }
}
