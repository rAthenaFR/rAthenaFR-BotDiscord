use anyhow::{anyhow, Context, Result};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub discord: DiscordConfig,
    pub database: DatabaseConfig,
    pub services: ServicesConfig,
    pub display: DisplayConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct DiscordConfig {
    pub token: String,
    pub application_id: u64,
    pub guild_id: u64,
    pub staff_role_ids: Vec<u64>,
    pub admin_role_ids: Vec<u64>,
    pub owner_role_ids: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
    pub acquire_timeout_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct ServiceEndpointConfig {
    pub name: &'static str,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct ServicesConfig {
    pub login: ServiceEndpointConfig,
    pub char_server: ServiceEndpointConfig,
    pub map: ServiceEndpointConfig,
}

#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub hide_gm_characters: bool,
    pub hide_gm_from_top: bool,
    pub hide_gm_group_from_ranking: i32,
    pub default_limit: u32,
    pub max_limit: u32,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: Option<u64>,
}

impl AppConfig {
    pub fn from_env_for_deploy() -> Result<Self> {
        Ok(Self {
            discord: DiscordConfig::from_env()?,
            database: DatabaseConfig::placeholder(),
            services: ServicesConfig::from_env()?,
            display: DisplayConfig::from_env()?,
            cache: CacheConfig::from_env()?,
        })
    }

    pub fn from_env_for_runtime() -> Result<Self> {
        Ok(Self {
            discord: DiscordConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            services: ServicesConfig::from_env()?,
            display: DisplayConfig::from_env()?,
            cache: CacheConfig::from_env()?,
        })
    }
}

impl DiscordConfig {
    fn from_env() -> Result<Self> {
        Ok(Self {
            token: required("DISCORD_TOKEN")?,
            application_id: parse_discord_application_id()?,
            guild_id: parse_required("DISCORD_GUILD_ID")?,
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
        })
    }
}

impl DatabaseConfig {
    fn from_env() -> Result<Self> {
        Ok(Self {
            host: required("RATHENAFR_DB_HOST")?,
            port: parse_required("RATHENAFR_DB_PORT")?,
            name: required("RATHENAFR_DB_NAME")?,
            user: required("RATHENAFR_DB_USER")?,
            password: required("RATHENAFR_DB_PASSWORD")?,
            max_connections: parse_optional("RATHENAFR_DB_MAX_CONNECTIONS")?.unwrap_or(5),
            acquire_timeout_seconds: parse_optional("RATHENAFR_DB_ACQUIRE_TIMEOUT_SECONDS")?
                .unwrap_or(5),
        })
    }

    fn placeholder() -> Self {
        Self {
            host: String::new(),
            port: 3306,
            name: String::new(),
            user: String::new(),
            password: String::new(),
            max_connections: 1,
            acquire_timeout_seconds: 5,
        }
    }

    pub fn connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            urlencoding::encode(&self.user),
            urlencoding::encode(&self.password),
            self.host,
            self.port,
            self.name
        )
    }
}

impl ServicesConfig {
    fn from_env() -> Result<Self> {
        let default_host =
            optional("RATHENAFR_SERVER_HOST").unwrap_or_else(|| "127.0.0.1".to_string());

        Ok(Self {
            login: ServiceEndpointConfig {
                name: "Serveur login",
                host: optional("RATHENAFR_LOGIN_HOST").unwrap_or_else(|| default_host.clone()),
                port: parse_optional("RATHENAFR_LOGIN_PORT")?.unwrap_or(6900),
            },
            char_server: ServiceEndpointConfig {
                name: "Serveur char",
                host: optional("RATHENAFR_CHAR_HOST").unwrap_or_else(|| default_host.clone()),
                port: parse_optional("RATHENAFR_CHAR_PORT")?.unwrap_or(6121),
            },
            map: ServiceEndpointConfig {
                name: "Serveur map",
                host: optional("RATHENAFR_MAP_HOST").unwrap_or(default_host),
                port: parse_optional("RATHENAFR_MAP_PORT")?.unwrap_or(5121),
            },
        })
    }

    pub fn endpoints(&self) -> [&ServiceEndpointConfig; 3] {
        [&self.login, &self.char_server, &self.map]
    }
}

impl DisplayConfig {
    fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        let default_limit = parse_optional_from(lookup, "RATHENAFR_DEFAULT_LIMIT")?.unwrap_or(10);
        let max_limit = parse_optional_from(lookup, "RATHENAFR_MAX_LIMIT")?.unwrap_or(25);

        Ok(Self {
            hide_gm_characters: parse_bool_optional_from(lookup, "RATHENAFR_HIDE_GM_CHARACTERS")?
                .unwrap_or(false),
            hide_gm_from_top: parse_bool_optional_from(lookup, "RATHENAFR_HIDE_GM_FROM_TOP")?
                .unwrap_or(true),
            hide_gm_group_from_ranking: parse_optional_from(
                lookup,
                "RATHENAFR_HIDE_GM_GROUP_FROM_RANKING",
            )?
            .unwrap_or(60),
            default_limit,
            max_limit: max_limit.max(default_limit),
        })
    }

    pub fn clamp_limit(&self, requested: Option<i64>) -> u32 {
        match requested {
            Some(value) if value > 0 => (value as u32).min(self.max_limit),
            _ => self.default_limit,
        }
    }

    pub fn public_character_group_threshold(&self) -> i32 {
        if self.hide_gm_characters {
            self.hide_gm_group_from_ranking
        } else {
            i32::MAX
        }
    }

    pub fn ranking_group_threshold(&self) -> i32 {
        if self.hide_gm_from_top {
            self.hide_gm_group_from_ranking
        } else {
            i32::MAX
        }
    }
}

impl CacheConfig {
    fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        Ok(Self {
            enabled: parse_bool_optional_from(lookup, "RATHENAFR_CACHE_ENABLED")?.unwrap_or(true),
            ttl_seconds: parse_optional_from(lookup, "RATHENAFR_CACHE_TTL_SECONDS")?,
        })
    }

    pub fn duration(&self, default_seconds: u64) -> Option<Duration> {
        if !self.enabled {
            return None;
        }

        let seconds = self.ttl_seconds.unwrap_or(default_seconds);
        if seconds == 0 {
            None
        } else {
            Some(Duration::from_secs(seconds))
        }
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

fn required(name: &str) -> Result<String> {
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

fn optional(name: &str) -> Option<String> {
    lookup_value(&|name| env::var(name).ok(), name)
}

fn lookup_value<F>(lookup: &F, name: &str) -> Option<String>
where
    F: Fn(&str) -> Option<String>,
{
    lookup(name)
        .map(|value| clean_env_value(&value))
        .filter(|value| !value.is_empty() && value != "replace_me")
}

fn clean_env_value(value: &str) -> String {
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

fn parse_required<T>(name: &str) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    required(name)?
        .parse::<T>()
        .with_context(|| format!("Valeur invalide pour la variable d’environnement : {name}"))
}

fn parse_optional<T>(name: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    parse_optional_from(&optional, name)
}

fn parse_optional_from<T, F>(lookup: &F, name: &str) -> Result<Option<T>>
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

fn parse_bool_optional_from<F>(lookup: &F, name: &str) -> Result<Option<bool>>
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn lookup(values: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let values = values
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect::<HashMap<_, _>>();

        move |name| values.get(name).cloned()
    }

    #[test]
    fn display_config_uses_defaults() {
        let config = DisplayConfig::from_lookup(&lookup(&[])).expect("display config");

        assert!(!config.hide_gm_characters);
        assert!(config.hide_gm_from_top);
        assert_eq!(config.hide_gm_group_from_ranking, 60);
        assert_eq!(config.default_limit, 10);
        assert_eq!(config.max_limit, 25);
        assert_eq!(config.clamp_limit(None), 10);
    }

    #[test]
    fn display_config_clamps_limits_and_raises_maximum() {
        let config = DisplayConfig::from_lookup(&lookup(&[
            ("RATHENAFR_DEFAULT_LIMIT", "30"),
            ("RATHENAFR_MAX_LIMIT", "10"),
        ]))
        .expect("display config");

        assert_eq!(config.default_limit, 30);
        assert_eq!(config.max_limit, 30);
        assert_eq!(config.clamp_limit(Some(5)), 5);
        assert_eq!(config.clamp_limit(Some(500)), 30);
        assert_eq!(config.clamp_limit(Some(0)), 30);
    }

    #[test]
    fn cache_config_is_enabled_by_default() {
        let config = CacheConfig::from_lookup(&lookup(&[])).expect("cache config");

        assert!(config.enabled);
        assert_eq!(config.ttl_seconds, None);
        assert_eq!(config.duration(10), Some(Duration::from_secs(10)));
    }

    #[test]
    fn cache_config_can_be_disabled() {
        let config = CacheConfig::from_lookup(&lookup(&[("RATHENAFR_CACHE_ENABLED", "false")]))
            .expect("cache config");

        assert!(!config.enabled);
        assert_eq!(config.duration(10), None);
    }

    #[test]
    fn cache_config_uses_global_ttl_override() {
        let config = CacheConfig::from_lookup(&lookup(&[
            ("RATHENAFR_CACHE_ENABLED", "true"),
            ("RATHENAFR_CACHE_TTL_SECONDS", "45"),
        ]))
        .expect("cache config");

        assert!(config.enabled);
        assert_eq!(config.ttl_seconds, Some(45));
        assert_eq!(config.duration(10), Some(Duration::from_secs(45)));
    }

    #[test]
    fn cache_config_zero_ttl_disables_storage_duration() {
        let config = CacheConfig::from_lookup(&lookup(&[("RATHENAFR_CACHE_TTL_SECONDS", "0")]))
            .expect("cache config");

        assert_eq!(config.duration(10), None);
    }
}
