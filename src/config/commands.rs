use super::account::parse_staff_role;
use super::*;
use anyhow::{anyhow, Result};

impl CommandConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    pub(crate) fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        Ok(Self {
            public_pack_enabled: parse_bool_optional_from(lookup, "RATHENAFR_PUBLIC_PACK_ENABLED")?
                .unwrap_or(true),
            staff_pack_enabled: parse_bool_optional_from(lookup, "RATHENAFR_STAFF_PACK_ENABLED")?
                .unwrap_or(true),
            online_list_public: parse_bool_optional_from(lookup, "RATHENAFR_ONLINE_LIST_PUBLIC")?
                .unwrap_or(false),
            top_zeny_mode: parse_top_zeny_mode(lookup)?,
            disabled_commands: parse_csv_strings(lookup, "RATHENAFR_DISABLED_COMMANDS"),
            item_table_name: parse_table_choice(
                lookup,
                "RATHENAFR_ITEM_DB_TABLE",
                "item_db",
                &["item_db", "item_db_re"],
            )?,
            mob_table_name: parse_table_choice(
                lookup,
                "RATHENAFR_MOB_DB_TABLE",
                "mob_db",
                &["mob_db", "mob_db_re"],
            )?,
            gmmsg_min_role: parse_staff_role(lookup, "RATHENAFR_GMMSG_MIN_ROLE")?
                .unwrap_or(StaffRole::Gm),
            debug_min_role: parse_staff_role(lookup, "RATHENAFR_DEBUG_MIN_ROLE")?
                .unwrap_or(StaffRole::Gm),
            audit_min_role: parse_staff_role(lookup, "RATHENAFR_AUDIT_MIN_ROLE")?
                .unwrap_or(StaffRole::Admin),
        })
    }

    pub fn command_enabled(&self, command_path: &str) -> bool {
        !self
            .disabled_commands
            .iter()
            .any(|disabled| command_path_matches_disabled(command_path, disabled))
    }
}

fn command_path_matches_disabled(command_path: &str, disabled_path: &str) -> bool {
    if disabled_path.eq_ignore_ascii_case(command_path) {
        return true;
    }

    let disabled_path = disabled_path.trim();
    let disabled_parts = disabled_path.split_whitespace().count();
    disabled_parts >= 2
        && command_path.len() > disabled_path.len()
        && command_path
            .get(..disabled_path.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(disabled_path))
        && command_path[disabled_path.len()..].starts_with(' ')
}

fn parse_top_zeny_mode<F>(lookup: &F) -> Result<TopZenyMode>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup_value(lookup, "RATHENAFR_TOP_ZENY_MODE") else {
        return Ok(TopZenyMode::Enabled);
    };

    match value.to_ascii_lowercase().as_str() {
        "enabled" | "on" | "true" => Ok(TopZenyMode::Enabled),
        "anonymized" | "anonymous" | "anon" => Ok(TopZenyMode::Anonymized),
        "disabled" | "off" | "false" => Ok(TopZenyMode::Disabled),
        _ => Err(anyhow!(
            "Valeur invalide pour RATHENAFR_TOP_ZENY_MODE. Valeurs attendues : enabled, anonymized ou disabled."
        )),
    }
}

fn parse_table_choice<F>(
    lookup: &F,
    name: &str,
    default_value: &str,
    allowed_values: &[&str],
) -> Result<String>
where
    F: Fn(&str) -> Option<String>,
{
    let value = lookup_value(lookup, name).unwrap_or_else(|| default_value.to_string());

    if allowed_values
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(&value))
    {
        return Ok(value);
    }

    Err(anyhow!(
        "Valeur invalide pour {name}. Valeurs attendues : {}.",
        allowed_values.join(", ")
    ))
}

fn parse_csv_strings<F>(lookup: &F, name: &str) -> Vec<String>
where
    F: Fn(&str) -> Option<String>,
{
    lookup_value(lookup, name)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(|part| part.to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default()
}
