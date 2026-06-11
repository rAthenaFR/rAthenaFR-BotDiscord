mod account;
mod cache;
mod commands;
mod database;
mod discord;
mod display;
mod env;
mod game_bridge;
mod rates;
mod services;
mod types;

pub use types::*;

use anyhow::Result;
pub(crate) use env::*;

impl AppConfig {
    pub fn from_env_for_deploy() -> Result<Self> {
        Ok(Self {
            discord: DiscordConfig::from_env()?,
            database: DatabaseConfig::placeholder(),
            services: ServicesConfig::from_env()?,
            display: DisplayConfig::from_env()?,
            cache: CacheConfig::from_env()?,
            server_rates: ServerRatesConfig::from_env()?,
            account_commands: AccountCommandsConfig::from_env()?,
            commands: CommandConfig::from_env()?,
            game_bridge: GameBridgeConfig::from_env()?,
        })
    }

    pub fn from_env_for_runtime() -> Result<Self> {
        Ok(Self {
            discord: DiscordConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            services: ServicesConfig::from_env()?,
            display: DisplayConfig::from_env()?,
            cache: CacheConfig::from_env()?,
            server_rates: ServerRatesConfig::from_env()?,
            account_commands: AccountCommandsConfig::from_env()?,
            commands: CommandConfig::from_env()?,
            game_bridge: GameBridgeConfig::from_env()?,
        })
    }
}

#[cfg(test)]
mod tests;
