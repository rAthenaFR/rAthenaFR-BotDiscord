use crate::config::AppConfig;
use crate::discord::{create_client, deploy_commands};
use crate::infra::env_loader;
use crate::infra::observability::sanitize_database_host;
use crate::rathenafr::RAthenaFrDatabase;
use anyhow::Result;
use std::sync::Arc;
use tracing::{error, info};

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

    let database = RAthenaFrDatabase::connect(&config.database).await?;
    database.ping().await?;
    info!("Connexion à la base de données validée.");

    let mut client = create_client(config, Arc::new(database)).await?;
    info!("Client Discord créé. Démarrage de la connexion à la passerelle.");

    if let Err(why) = client.start().await {
        error!(error = %why, "Le client Discord s’est arrêté avec une erreur.");
    }

    Ok(())
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
