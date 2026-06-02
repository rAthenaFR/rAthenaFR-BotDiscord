use crate::config::{GameBridgeConfig, GameBridgeEncoding, GameBridgeMode};
use crate::rathenafr::database::RAthenaFrDatabase;
use anyhow::{anyhow, bail, Result};
use encoding_rs::WINDOWS_1252;
use std::sync::Arc;

const WINDOWS_1252_INCOMPATIBLE_MESSAGE: &str =
    "Le message contient des caractères non compatibles avec l’encodage Windows-1252 utilisé par le client en jeu.";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BroadcastMode {
    Broadcast,
    KamiBlue,
    KamiColor(String),
}

#[derive(Clone)]
pub struct GameBridge {
    config: GameBridgeConfig,
    sql_queue: Option<SqlQueueGameBridge>,
}

#[derive(Clone)]
pub struct SqlQueueGameBridge {
    database: Arc<RAthenaFrDatabase>,
    encoding: GameBridgeEncoding,
    max_message_length: usize,
}

impl GameBridge {
    pub fn new(config: GameBridgeConfig, database: Arc<RAthenaFrDatabase>) -> Self {
        Self {
            sql_queue: Some(SqlQueueGameBridge::new(
                database,
                config.encoding,
                config.max_message_length,
            )),
            config,
        }
    }

    #[cfg(test)]
    fn new_for_tests(config: GameBridgeConfig) -> Self {
        Self {
            config,
            sql_queue: None,
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
                    self.sql_queue()?
                        .enqueue("server", None, None, message, discord_user_id, discord_username)
                        .await
                }
                BroadcastMode::KamiBlue => {
                    self.sql_queue()?
                        .enqueue("blue", None, None, message, discord_user_id, discord_username)
                        .await
                }
                BroadcastMode::KamiColor(color) => {
                    self.sql_queue()?
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
                self.sql_queue()?
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

    fn sql_queue(&self) -> Result<&SqlQueueGameBridge> {
        self.sql_queue
            .as_ref()
            .ok_or_else(|| anyhow!("Le bridge SQL GMMSG n’est pas initialisé."))
    }
}

impl SqlQueueGameBridge {
    pub fn new(
        database: Arc<RAthenaFrDatabase>,
        encoding: GameBridgeEncoding,
        max_message_length: usize,
    ) -> Self {
        Self {
            database,
            encoding,
            max_message_length,
        }
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
        let encoded_message =
            encode_gmmsg_message(message, self.encoding, self.max_message_length)?;

        self.database
            .enqueue_discord_gmmsg(
                mode,
                map,
                color,
                &encoded_message,
                discord_user_id,
                discord_username,
            )
            .await?;

        Ok("Message ajouté à la file d’envoi en jeu.".to_string())
    }
}

fn encode_gmmsg_message(
    message: &str,
    encoding: GameBridgeEncoding,
    max_bytes: usize,
) -> Result<Vec<u8>> {
    let bytes = match encoding {
        GameBridgeEncoding::Windows1252 => {
            let (encoded, _, had_errors) = WINDOWS_1252.encode(message);
            if had_errors {
                bail!(WINDOWS_1252_INCOMPATIBLE_MESSAGE);
            }
            encoded.into_owned()
        }
        GameBridgeEncoding::Utf8 => message.as_bytes().to_vec(),
    };

    if bytes.len() > max_bytes {
        bail!("Le message dépasse la limite configurée de {max_bytes} octets.");
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(mode: GameBridgeMode) -> GameBridgeConfig {
        GameBridgeConfig {
            mode,
            max_message_length: 180,
            encoding: GameBridgeEncoding::Windows1252,
        }
    }

    #[test]
    fn windows1252_encoding_accepts_french_accents() {
        let encoded = encode_gmmsg_message(
            "é è à ç ù ê î ô û É Ç À",
            GameBridgeEncoding::Windows1252,
            180,
        )
        .expect("windows-1252 accents");

        assert_eq!(
            encoded,
            vec![
                0xE9, 0x20, 0xE8, 0x20, 0xE0, 0x20, 0xE7, 0x20, 0xF9, 0x20, 0xEA, 0x20, 0xEE, 0x20,
                0xF4, 0x20, 0xFB, 0x20, 0xC9, 0x20, 0xC7, 0x20, 0xC0,
            ]
        );
    }

    #[test]
    fn windows1252_encoding_rejects_emoji() {
        let error = encode_gmmsg_message("Bonjour 🙂", GameBridgeEncoding::Windows1252, 180)
            .expect_err("emoji rejected");

        assert_eq!(error.to_string(), WINDOWS_1252_INCOMPATIBLE_MESSAGE);
    }

    #[test]
    fn utf8_encoding_keeps_utf8_bytes() {
        let encoded =
            encode_gmmsg_message("é", GameBridgeEncoding::Utf8, 180).expect("utf-8 message");

        assert_eq!(encoded, "é".as_bytes());
    }

    #[tokio::test]
    async fn test_mode_does_not_require_sql_queue() {
        let bridge = GameBridge::new_for_tests(config(GameBridgeMode::Test));

        let result = bridge
            .send_global_message(BroadcastMode::Broadcast, "é", 1, "gm")
            .await
            .expect("test mode");

        assert!(result.contains("mode test"));
    }

    #[tokio::test]
    async fn disabled_mode_stays_disabled_without_sql_queue() {
        let bridge = GameBridge::new_for_tests(config(GameBridgeMode::Disabled));

        let error = bridge
            .send_global_message(BroadcastMode::Broadcast, "é", 1, "gm")
            .await
            .expect_err("disabled mode");

        assert!(error.to_string().contains("n’est pas configuré"));
    }
}
