use super::*;
use anyhow::{anyhow, Result};

impl GameBridgeConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    pub(crate) fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        Ok(Self {
            mode: parse_game_bridge_mode(lookup)?,
            max_message_length: parse_optional_from(lookup, "RATHENAFR_GMMSG_MAX_LENGTH")?
                .unwrap_or(180),
            encoding: parse_game_bridge_encoding(lookup)?,
        })
    }
}

fn parse_game_bridge_mode<F>(lookup: &F) -> Result<GameBridgeMode>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup_value(lookup, "RATHENAFR_GMMSG_MODE") else {
        return Ok(GameBridgeMode::Disabled);
    };

    match value.to_ascii_lowercase().as_str() {
        "disabled" | "off" | "false" => Ok(GameBridgeMode::Disabled),
        "test" | "log" => Ok(GameBridgeMode::Test),
        "sql_queue" => Ok(GameBridgeMode::SqlQueue),
        "bridge" | "enabled" | "on" | "true" => Ok(GameBridgeMode::Bridge),
        _ => Err(anyhow!(
            "Valeur invalide pour RATHENAFR_GMMSG_MODE. Valeurs attendues : disabled, test, sql_queue ou bridge."
        )),
    }
}

fn parse_game_bridge_encoding<F>(lookup: &F) -> Result<GameBridgeEncoding>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup_value(lookup, "RATHENAFR_GMMSG_ENCODING") else {
        return Ok(GameBridgeEncoding::Windows1252);
    };

    match value.to_ascii_lowercase().as_str() {
        "windows1252" | "windows-1252" | "cp1252" => Ok(GameBridgeEncoding::Windows1252),
        "utf8" | "utf-8" => Ok(GameBridgeEncoding::Utf8),
        _ => Err(anyhow!(
            "Valeur invalide pour RATHENAFR_GMMSG_ENCODING. Valeurs attendues : windows1252 ou utf8."
        )),
    }
}
