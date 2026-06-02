use crate::config::{GameBridgeConfig, GameBridgeMode};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BroadcastMode {
    Broadcast,
    Kami,
    KamiBlue,
    KamiColor(String),
}

#[derive(Debug, Clone)]
pub struct GameBridge {
    config: GameBridgeConfig,
}

impl GameBridge {
    pub fn new(config: GameBridgeConfig) -> Self {
        Self { config }
    }

    pub fn is_available(&self) -> bool {
        matches!(
            self.config.mode,
            GameBridgeMode::Bridge | GameBridgeMode::Test
        )
    }

    pub async fn send_global_message(&self, mode: BroadcastMode, message: &str) -> Result<String> {
        match self.config.mode {
            GameBridgeMode::Disabled => Err(anyhow!("Le bridge en jeu n’est pas configuré.")),
            GameBridgeMode::Test => Ok(format!("mode test: {mode:?}: {message}")),
            GameBridgeMode::Bridge => Err(anyhow!(
                "Le bridge en jeu n’est pas configuré : aucune implémentation map-server n’est active."
            )),
        }
    }

    pub async fn send_map_message(&self, _map: &str, _message: &str) -> Result<String> {
        match self.config.mode {
            GameBridgeMode::Disabled => Err(anyhow!("Le bridge en jeu n’est pas configuré.")),
            GameBridgeMode::Test => Ok("mode test : broadcast map non envoyé".to_string()),
            GameBridgeMode::Bridge => Err(anyhow!(
                "Le broadcast map n’est pas supporté par le bridge actuel."
            )),
        }
    }
}
