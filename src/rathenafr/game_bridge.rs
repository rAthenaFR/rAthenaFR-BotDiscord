use crate::config::{GameBridgeConfig, GameBridgeMode};
use crate::rathenafr::database::RAthenaFrDatabase;
use anyhow::{anyhow, Result};
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BroadcastMode {
    Broadcast,
    KamiBlue,
    KamiColor(String),
}

#[derive(Clone)]
pub struct GameBridge {
    config: GameBridgeConfig,
    sql_queue: SqlQueueGameBridge,
}

#[derive(Clone)]
pub struct SqlQueueGameBridge {
    database: Arc<RAthenaFrDatabase>,
}

impl GameBridge {
    pub fn new(config: GameBridgeConfig, database: Arc<RAthenaFrDatabase>) -> Self {
        Self {
            config,
            sql_queue: SqlQueueGameBridge::new(database),
        }
    }

    pub async fn send_global_message(
        &self,
        mode: BroadcastMode,
        message: &str,
        discord_user_id: u64,
        discord_username: &str,
    ) -> Result<String> {
        match self.config.mode {
            GameBridgeMode::Disabled => Err(anyhow!("Le bridge en jeu n’est pas configuré.")),
            GameBridgeMode::Test => Ok(format!("mode test: {mode:?}: {message}")),
            GameBridgeMode::SqlQueue => match mode {
                BroadcastMode::Broadcast => {
                    self.sql_queue
                        .enqueue("server", None, None, message, discord_user_id, discord_username)
                        .await
                }
                BroadcastMode::KamiBlue => {
                    self.sql_queue
                        .enqueue("blue", None, None, message, discord_user_id, discord_username)
                        .await
                }
                BroadcastMode::KamiColor(color) => {
                    self.sql_queue
                        .enqueue(
                            "color",
                            None,
                            Some(color.as_str()),
                            message,
                            discord_user_id,
                            discord_username,
                        )
                        .await
                }
            },
            GameBridgeMode::Bridge => Err(anyhow!(
                "Le bridge en jeu n’est pas configuré : aucune implémentation map-server n’est active."
            )),
        }
    }

    pub async fn send_map_message(
        &self,
        map: &str,
        message: &str,
        discord_user_id: u64,
        discord_username: &str,
    ) -> Result<String> {
        match self.config.mode {
            GameBridgeMode::Disabled => Err(anyhow!("Le bridge en jeu n’est pas configuré.")),
            GameBridgeMode::Test => Ok("mode test : broadcast map non envoyé".to_string()),
            GameBridgeMode::SqlQueue => {
                self.sql_queue
                    .enqueue(
                        "map",
                        Some(map),
                        None,
                        message,
                        discord_user_id,
                        discord_username,
                    )
                    .await
            }
            GameBridgeMode::Bridge => Err(anyhow!(
                "Le broadcast map n’est pas supporté par le bridge actuel."
            )),
        }
    }
}

impl SqlQueueGameBridge {
    pub fn new(database: Arc<RAthenaFrDatabase>) -> Self {
        Self { database }
    }

    async fn enqueue(
        &self,
        mode: &str,
        map: Option<&str>,
        color: Option<&str>,
        message: &str,
        discord_user_id: u64,
        discord_username: &str,
    ) -> Result<String> {
        self.database
            .enqueue_discord_gmmsg(mode, map, color, message, discord_user_id, discord_username)
            .await?;

        Ok("Message ajouté à la file d’envoi en jeu.".to_string())
    }
}
