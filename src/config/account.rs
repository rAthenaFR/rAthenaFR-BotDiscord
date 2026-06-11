use super::*;
use anyhow::{anyhow, Result};

impl AccountCommandsConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    pub(crate) fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        Ok(Self {
            creation_enabled: parse_bool_optional_from(
                lookup,
                "RATHENAFR_ACCOUNT_CREATION_ENABLED",
            )?
            .unwrap_or(false),
            password_mode: parse_account_password_mode(lookup)?,
            manage_enabled: parse_bool_optional_from(lookup, "RATHENAFR_ACCOUNT_MANAGE_ENABLED")?
                .unwrap_or(false),
            delete_enabled: parse_bool_optional_from(lookup, "RATHENAFR_ACCOUNT_DELETE_ENABLED")?
                .unwrap_or(false),
            manage_min_role: parse_staff_role(lookup, "RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE")?
                .unwrap_or(StaffRole::Admin),
            delete_min_role: parse_staff_role(lookup, "RATHENAFR_ACCOUNT_DELETE_MIN_ROLE")?
                .unwrap_or(StaffRole::Owner),
        })
    }
}

fn parse_account_password_mode<F>(lookup: &F) -> Result<AccountPasswordMode>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup_value(lookup, "RATHENAFR_ACCOUNT_PASSWORD_MODE") else {
        return Ok(AccountPasswordMode::Plain);
    };

    match value.to_ascii_lowercase().as_str() {
        "plain" => Ok(AccountPasswordMode::Plain),
        "md5" => Ok(AccountPasswordMode::Md5),
        _ => Err(anyhow!(
            "Valeur invalide pour la variable d’environnement : RATHENAFR_ACCOUNT_PASSWORD_MODE. Valeurs attendues : plain ou md5."
        )),
    }
}

pub(crate) fn parse_staff_role<F>(lookup: &F, name: &str) -> Result<Option<StaffRole>>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup_value(lookup, name) else {
        return Ok(None);
    };

    match value.to_ascii_lowercase().as_str() {
        "helper" => Ok(Some(StaffRole::Helper)),
        "moderator" | "mod" => Ok(Some(StaffRole::Moderator)),
        "gm" => Ok(Some(StaffRole::Gm)),
        "admin" => Ok(Some(StaffRole::Admin)),
        "owner" => Ok(Some(StaffRole::Owner)),
        _ => Err(anyhow!(
            "Valeur invalide pour {name}. Valeurs attendues : helper, moderator, gm, admin ou owner."
        )),
    }
}
