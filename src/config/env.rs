use anyhow::{anyhow, Context, Result};
use std::env;

pub(crate) fn required(name: &str) -> Result<String> {
    let value = env::var(name).with_context(|| {
        format!(
            "Variable d’environnement manquante : {name}\n\nLe bot n’a pas trouvé cette valeur obligatoire dans l’environnement du processus ni dans un fichier .env.\nEmplacements .env vérifiés :\n{}\n\nCrée un fichier .env à partir de .env.example, renseigne les valeurs, puis relance la commande.",
            crate::infra::env_loader::environment_hint()
        )
    })?;
    let trimmed = clean_env_value(&value);

    if trimmed.is_empty() || trimmed == "replace_me" {
        return Err(anyhow!(
            "Variable d’environnement invalide ou non remplacée : {name}\n\nRemplace la valeur dans ton fichier .env. La valeur ne peut pas être vide ni rester à replace_me."
        ));
    }

    Ok(trimmed)
}

pub(crate) fn optional(name: &str) -> Option<String> {
    lookup_value(&|name| env::var(name).ok(), name)
}

pub(crate) fn lookup_value<F>(lookup: &F, name: &str) -> Option<String>
where
    F: Fn(&str) -> Option<String>,
{
    lookup(name)
        .map(|value| clean_env_value(&value))
        .filter(|value| !value.is_empty() && value != "replace_me")
}

pub(crate) fn clean_env_value(value: &str) -> String {
    let trimmed = value.trim();

    if trimmed.len() >= 2 {
        let starts_with_double_quote = trimmed.starts_with('"') && trimmed.ends_with('"');
        let starts_with_single_quote = trimmed.starts_with('\'') && trimmed.ends_with('\'');

        if starts_with_double_quote || starts_with_single_quote {
            return trimmed[1..trimmed.len() - 1].trim().to_string();
        }
    }

    trimmed.to_string()
}

pub(crate) fn parse_required<T>(name: &str) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    required(name)?
        .parse::<T>()
        .with_context(|| format!("Valeur invalide pour la variable d’environnement : {name}"))
}

pub(crate) fn parse_optional<T>(name: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    parse_optional_from(&optional, name)
}

pub(crate) fn parse_optional_from<T, F>(lookup: &F, name: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    F: Fn(&str) -> Option<String>,
{
    match lookup_value(lookup, name) {
        Some(value) => Ok(Some(value.parse::<T>().with_context(|| {
            format!("Valeur invalide pour la variable d’environnement : {name}")
        })?)),
        None => Ok(None),
    }
}

pub(crate) fn parse_bool_optional_from<F>(lookup: &F, name: &str) -> Result<Option<bool>>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = lookup_value(lookup, name) else {
        return Ok(None);
    };

    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "y" | "on" => Ok(Some(true)),
        "0" | "false" | "no" | "n" | "off" => Ok(Some(false)),
        _ => Err(anyhow!(
            "Valeur invalide pour la variable d’environnement : {name}. Valeur attendue : true ou false."
        )),
    }
}
