use crate::config::{AccountCommandsConfig, StaffRole};
use crate::rathenafr::{AccountManageField, AccountStatus, DatabaseTable, RAthenaFrDatabase};
use anyhow::Result;

pub(super) const REQUIRED_TABLES: &[DatabaseTable] = &[DatabaseTable::Login, DatabaseTable::Char];

const FORBIDDEN_FIELDS: &[&str] = &[
    "account_id",
    "userid",
    "user_pass",
    "password",
    "hash",
    "email",
    "pincode",
    "last_ip",
    "lastlogin",
];

#[derive(Debug, Eq, PartialEq)]
pub(super) enum AccountLookup<'a> {
    AccountId(i64),
    Userid(&'a str),
}

pub(super) struct Options<'a> {
    pub account: Option<&'a str>,
    pub account_id: Option<i64>,
    pub field: Option<&'a str>,
    pub value: Option<&'a str>,
    pub confirm: Option<&'a str>,
    pub reason: Option<&'a str>,
    pub until: Option<i64>,
}

pub(super) struct PreparedEdit<'a> {
    pub lookup: &'a str,
    pub field: AccountManageField,
    pub value: String,
    pub reason: Option<String>,
}

pub(super) struct PreparedAccountAction<'a> {
    pub lookup: &'a str,
    pub until: Option<i64>,
    pub reason: Option<String>,
}

pub(super) struct PreparedDelete {
    pub account_id: i64,
    pub reason: Option<String>,
}

pub(super) fn required_role(config: &AccountCommandsConfig, action: &str) -> StaffRole {
    if action == "delete" {
        config.delete_min_role
    } else {
        config.manage_min_role
    }
}

pub(super) fn requested_account(options: &Options<'_>) -> Option<String> {
    options
        .account_id
        .map(|value| value.to_string())
        .or_else(|| {
            options
                .account
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
        })
}

pub(super) fn prepare_edit<'a>(
    options: &Options<'a>,
) -> std::result::Result<PreparedEdit<'a>, String> {
    let account = required_text(options.account, "account")?;
    let field_name = required_text(options.field, "field")?;
    let value = required_text(options.value, "value")?;
    let field = parse_field(field_name)?;
    let value = validate_value(field, value)?;

    Ok(PreparedEdit {
        lookup: account,
        field,
        value,
        reason: reason(options),
    })
}

pub(super) fn prepare_ban<'a>(
    options: &Options<'a>,
) -> std::result::Result<PreparedAccountAction<'a>, String> {
    Ok(PreparedAccountAction {
        lookup: required_text(options.account, "account")?,
        until: options.until,
        reason: reason(options),
    })
}

pub(super) fn prepare_unban<'a>(
    options: &Options<'a>,
) -> std::result::Result<PreparedAccountAction<'a>, String> {
    Ok(PreparedAccountAction {
        lookup: required_text(options.account, "account")?,
        until: None,
        reason: reason(options),
    })
}

pub(super) fn prepare_delete(
    config: &AccountCommandsConfig,
    options: &Options<'_>,
) -> std::result::Result<PreparedDelete, String> {
    let account_id = options
        .account_id
        .filter(|value| *value > 0)
        .ok_or_else(|| "Option obligatoire manquante : account_id.".to_string())?;
    let confirm = required_text(options.confirm, "confirm")?;

    validate_delete_request(config, confirm)?;

    Ok(PreparedDelete {
        account_id,
        reason: reason(options),
    })
}

pub(super) async fn resolve_account(
    database: &RAthenaFrDatabase,
    lookup: &str,
) -> Result<Option<AccountStatus>> {
    match parse_lookup(lookup) {
        Ok(AccountLookup::AccountId(account_id)) => database.account_status(account_id).await,
        Ok(AccountLookup::Userid(userid)) => database.account_status_by_userid(userid).await,
        Err(_) => Ok(None),
    }
}

pub(super) fn parse_lookup(value: &str) -> std::result::Result<AccountLookup<'_>, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("Le compte doit etre renseigne.".to_string());
    }

    match trimmed.parse::<i64>() {
        Ok(account_id) if account_id > 0 => Ok(AccountLookup::AccountId(account_id)),
        Ok(_) => Err("L'account_id doit etre strictement positif.".to_string()),
        Err(_) => Ok(AccountLookup::Userid(trimmed)),
    }
}

pub(super) fn parse_field(value: &str) -> std::result::Result<AccountManageField, String> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "group_id" => Ok(AccountManageField::GroupId),
        "state" => Ok(AccountManageField::State),
        "unban_time" => Ok(AccountManageField::UnbanTime),
        "expiration_time" => Ok(AccountManageField::ExpirationTime),
        "logincount" => Ok(AccountManageField::Logincount),
        "sex" => Ok(AccountManageField::Sex),
        field if FORBIDDEN_FIELDS.contains(&field) => Err(format!(
            "Le champ `{field}` ne peut jamais etre modifie par cette commande."
        )),
        field => Err(format!("Le champ `{field}` n'est pas autorise.")),
    }
}

pub(super) fn validate_value(
    field: AccountManageField,
    value: &str,
) -> std::result::Result<String, String> {
    if field == AccountManageField::Sex {
        return validate_account_sex(value);
    }

    let trimmed = value.trim();
    let parsed = trimmed
        .parse::<i64>()
        .map_err(|_| format!("La valeur de `{}` doit etre un entier.", field.name()))?;
    if parsed < 0 {
        return Err(format!(
            "La valeur de `{}` doit etre positive ou nulle.",
            field.name()
        ));
    }

    Ok(parsed.to_string())
}

pub(super) fn validate_delete_request(
    config: &AccountCommandsConfig,
    confirm: &str,
) -> std::result::Result<(), String> {
    if !config.delete_enabled {
        return Err(
            "La desactivation forte de compte est desactivee par configuration.".to_string(),
        );
    }
    if confirm != "SUPPRIMER" {
        return Err("Confirmation invalide. Renseigne `SUPPRIMER` exactement.".to_string());
    }

    Ok(())
}

pub(super) fn account_label(status: &AccountStatus) -> String {
    format!(
        "account_id `{}` / userid `{}`",
        status.account_id,
        sanitize_staff_text(&status.userid)
    )
}

pub(super) fn summary(status: &AccountStatus) -> String {
    format!(
        "account_id `{}` | userid `{}` | personnages `{}` | etat `{}` | group_id `{}` | unban_time `{}` | expiration_time `{}`",
        status.account_id,
        sanitize_staff_text(&status.userid),
        status.characters,
        status.state,
        status.group_id,
        status.unban_time,
        status.expiration_time
    )
}

pub(super) fn edit_result(field: AccountManageField, status: &AccountStatus) -> String {
    format!("Champ `{}` modifie. {}", field.name(), summary(status))
}

pub(super) fn ban_result(until: Option<i64>, status: &AccountStatus) -> String {
    match until {
        Some(value) if value > 0 => {
            format!(
                "Compte banni avec fin de ban `{value}`. {}",
                summary(status)
            )
        }
        _ => format!("Compte bloque. {}", summary(status)),
    }
}

pub(super) fn unban_result(status: &AccountStatus) -> String {
    format!("Compte debloque. {}", summary(status))
}

pub(super) fn delete_result(status: &AccountStatus) -> String {
    format!(
        "Desactivation forte appliquee sans suppression physique. {}",
        summary(status)
    )
}

pub(super) fn safe_sql_error(error: &anyhow::Error) -> String {
    let details = error.to_string();
    let lower = details.to_ascii_lowercase();

    if lower.contains("access denied") || lower.contains("permission") {
        "Permissions SQL insuffisantes pour modifier le compte.".to_string()
    } else if lower.contains("doesn't exist")
        || lower.contains("does not exist")
        || lower.contains("unknown table")
        || lower.contains("absente")
    {
        "Table ou colonne rAthena requise absente.".to_string()
    } else if lower.contains("aucun compte") {
        "Aucun compte n'a ete modifie.".to_string()
    } else {
        "Erreur SQL pendant l'action account-manage.".to_string()
    }
}

fn required_text<'a>(value: Option<&'a str>, name: &str) -> std::result::Result<&'a str, String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("Option obligatoire manquante : {name}."))
}

fn reason(options: &Options<'_>) -> Option<String> {
    options
        .reason
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn validate_account_sex(value: &str) -> std::result::Result<String, String> {
    match value.trim().to_ascii_uppercase().as_str() {
        "M" => Ok("M".to_string()),
        "F" => Ok("F".to_string()),
        _ => Err("Le sexe du compte doit être `M` ou `F`.".to_string()),
    }
}

fn sanitize_staff_text(value: &str) -> String {
    value
        .replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AccountPasswordMode;

    #[test]
    fn required_role_uses_delete_specific_role() {
        let config = test_config(StaffRole::Gm, StaffRole::Owner, false);

        assert_eq!(required_role(&config, "ban"), StaffRole::Gm);
        assert_eq!(required_role(&config, "unban"), StaffRole::Gm);
        assert_eq!(required_role(&config, "delete"), StaffRole::Owner);
    }

    #[test]
    fn delete_is_disabled_by_config_until_explicitly_enabled() {
        let disabled = test_config(StaffRole::Admin, StaffRole::Owner, false);
        let enabled = test_config(StaffRole::Admin, StaffRole::Owner, true);

        assert_eq!(
            validate_delete_request(&disabled, "SUPPRIMER").unwrap_err(),
            "La desactivation forte de compte est desactivee par configuration."
        );
        assert!(validate_delete_request(&enabled, "SUPPRIMER").is_ok());
    }

    #[test]
    fn delete_confirmation_is_strict() {
        let config = test_config(StaffRole::Admin, StaffRole::Owner, true);

        assert!(validate_delete_request(&config, "SUPPRIMER").is_ok());
        assert_eq!(
            validate_delete_request(&config, "supprimer").unwrap_err(),
            "Confirmation invalide. Renseigne `SUPPRIMER` exactement."
        );
        assert!(validate_delete_request(&config, " SUPPRIMER ").is_err());
    }

    #[test]
    fn ban_unban_lookup_accepts_account_id() {
        assert_eq!(
            parse_lookup(" 123 ").unwrap(),
            AccountLookup::AccountId(123)
        );
        assert_eq!(
            parse_lookup("ExactUserid").unwrap(),
            AccountLookup::Userid("ExactUserid")
        );
        assert!(parse_lookup("0").is_err());
    }

    #[test]
    fn rejects_forbidden_or_invalid_fields() {
        assert_eq!(
            parse_field("group_id").unwrap(),
            AccountManageField::GroupId
        );
        assert!(parse_field("user_pass").is_err());
        assert!(parse_field("email").is_err());
        assert!(parse_field("last_ip").is_err());
        assert!(validate_value(AccountManageField::State, "-1").is_err());
        assert_eq!(validate_value(AccountManageField::Sex, " f ").unwrap(), "F");
    }

    #[test]
    fn checks_login_and_char_tables() {
        assert!(REQUIRED_TABLES.contains(&DatabaseTable::Login));
        assert!(REQUIRED_TABLES.contains(&DatabaseTable::Char));
    }

    #[test]
    fn sql_errors_are_loggable_without_sql_details() {
        let error = anyhow::anyhow!(
            "error returned from database: 1146 (42S02): Table 'ragnarok.login' doesn't exist"
        );

        assert_eq!(
            safe_sql_error(&error),
            "Table ou colonne rAthena requise absente."
        );
    }

    fn test_config(
        manage_min_role: StaffRole,
        delete_min_role: StaffRole,
        delete_enabled: bool,
    ) -> AccountCommandsConfig {
        AccountCommandsConfig {
            creation_enabled: false,
            password_mode: AccountPasswordMode::Plain,
            manage_enabled: true,
            delete_enabled,
            manage_min_role,
            delete_min_role,
        }
    }
}
