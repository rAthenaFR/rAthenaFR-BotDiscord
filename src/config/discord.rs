use super::*;
use anyhow::{Context, Result};

impl DiscordConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Ok(Self {
            token: required("DISCORD_TOKEN")?,
            application_id: parse_discord_application_id()?,
            guild_id: parse_required("DISCORD_GUILD_ID")?,
            helper_role_ids: parse_role_ids(&[
                "RATHENAFR_HELPER_ROLE_IDS",
                "RATHENAFR_STAFF_ROLE_IDS",
                "DISCORD_STAFF_ROLE_IDS",
            ])?,
            moderator_role_ids: parse_role_ids(&["RATHENAFR_MODERATOR_ROLE_IDS"])?,
            gm_role_ids: parse_role_ids(&["RATHENAFR_GM_ROLE_IDS"])?,
            staff_role_ids: parse_role_ids(&[
                "RATHENAFR_STAFF_ROLE_IDS",
                "DISCORD_STAFF_ROLE_IDS",
            ])?,
            admin_role_ids: parse_role_ids(&[
                "RATHENAFR_ADMIN_ROLE_IDS",
                "DISCORD_ADMIN_ROLE_IDS",
            ])?,
            owner_role_ids: parse_role_ids(&[
                "RATHENAFR_OWNER_ROLE_IDS",
                "DISCORD_OWNER_ROLE_IDS",
            ])?,
            staff_log_channel_id: parse_optional("RATHENAFR_STAFF_LOG_CHANNEL_ID")?,
        })
    }
}

fn parse_discord_application_id() -> Result<u64> {
    match optional("DISCORD_APPLICATION_ID") {
        Some(value) => value.parse::<u64>().with_context(|| {
            "Valeur invalide pour la variable d’environnement : DISCORD_APPLICATION_ID"
        }),
        None => parse_required("DISCORD_CLIENT_ID"),
    }
}

fn parse_role_ids(names: &[&str]) -> Result<Vec<u64>> {
    let value = names.iter().find_map(|name| optional(name));

    let Some(value) = value else {
        return Ok(Vec::new());
    };

    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<u64>()
                .with_context(|| format!("ID de rôle Discord invalide : {part}"))
        })
        .collect()
}
