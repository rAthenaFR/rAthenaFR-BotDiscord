use crate::config::{AppConfig, DatabaseConfig};
use crate::discord::{create_client, deploy_commands};
use crate::infra::env_loader;
use crate::infra::observability::sanitize_database_host;
use crate::rathenafr::RAthenaFrDatabase;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

const DB_CONNECT_MAX_ATTEMPTS: u32 = 30;
const DB_CONNECT_RETRY_DELAY_SECONDS: u64 = 2;

pub async fn run() -> Result<()> {
    init_logging();
    env_loader::load_environment()?;

    if is_deploy_mode() {
        deploy().await
    } else {
        start_bot().await
    }
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rathenafr_discord_bot=info,info".into()),
        )
        .init();
}

fn is_deploy_mode() -> bool {
    std::env::args().any(|arg| arg == "--deploy")
}

async fn deploy() -> Result<()> {
    let config = AppConfig::from_env_for_deploy()?;
    log_display_name();
    log_cache_configuration(&config);
    deploy_commands(&config).await?;
    info!("Commandes slash Discord déployées avec succès.");
    Ok(())
}

async fn start_bot() -> Result<()> {
    let config = Arc::new(AppConfig::from_env_for_runtime()?);
    log_display_name();
    log_database_target(&config);
    log_cache_configuration(&config);

    let database = connect_database_with_retry(&config.database).await?;
    info!("Connexion à la base de données validée.");

    let mut client = create_client(config, Arc::new(database)).await?;
    info!("Client Discord créé. Démarrage de la connexion à la passerelle.");

    if let Err(why) = client.start().await {
        error!(error = %why, "Le client Discord s’est arrêté avec une erreur.");
    }

    Ok(())
}

/// Établit la connexion à la base en réessayant tant qu'elle n'est pas
/// joignable. Évite les redémarrages du conteneur quand la base (souvent sur
/// une autre stack Docker) démarre après le bot.
async fn connect_database_with_retry(config: &DatabaseConfig) -> Result<RAthenaFrDatabase> {
    let max_attempts = env_u32("RATHENAFR_DB_CONNECT_MAX_ATTEMPTS")
        .filter(|value| *value > 0)
        .unwrap_or(DB_CONNECT_MAX_ATTEMPTS);
    let delay = Duration::from_secs(
        env_u64("RATHENAFR_DB_CONNECT_RETRY_DELAY_SECONDS")
            .unwrap_or(DB_CONNECT_RETRY_DELAY_SECONDS),
    );

    let mut attempt = 1;
    loop {
        match try_connect_database(config).await {
            Ok(database) => return Ok(database),
            Err(error) if attempt < max_attempts => {
                warn!(
                    attempt,
                    max_attempts,
                    retry_in_seconds = delay.as_secs(),
                    error = %error,
                    "Base de données injoignable, nouvelle tentative de connexion."
                );
                tokio::time::sleep(delay).await;
                attempt += 1;
            }
            Err(error) => {
                return Err(error.context(format!(
                    "échec de connexion à la base après {max_attempts} tentative(s)"
                )));
            }
        }
    }
}

async fn try_connect_database(config: &DatabaseConfig) -> Result<RAthenaFrDatabase> {
    let database = RAthenaFrDatabase::connect(config).await?;
    database.ping().await?;
    Ok(database)
}

fn env_u32(key: &str) -> Option<u32> {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
}

fn env_u64(key: &str) -> Option<u64> {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
}

fn log_display_name() {
    let display_name = std::env::var("RATHENAFR_DISPLAY_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "rAthenaFR".to_string());

    info!(display_name = %display_name, "Nom d’affichage configuré.");
}

fn log_database_target(config: &AppConfig) {
    info!(
        database_host = %sanitize_database_host(&config.database.host),
        database_name = %config.database.name,
        "Base de données cible configurée."
    );
}

fn log_cache_configuration(config: &AppConfig) {
    info!(
        cache_enabled = config.cache.enabled,
        cache_ttl_seconds = ?config.cache.ttl_seconds,
        "Configuration du cache chargée."
    );
}
