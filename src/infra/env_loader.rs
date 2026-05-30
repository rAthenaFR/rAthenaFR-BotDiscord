use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

const ENV_FILE_NAME: &str = ".env";
const ENV_FILE_OVERRIDE: &str = "RATHENAFR_DISCORD_BOT_ENV";

/// Loads the bot environment from a .env file.
///
/// The loader intentionally checks multiple locations because users often run the bot
/// with `cargo run`, from a compiled binary, or from a Windows shortcut. This keeps the
/// runtime behavior predictable and avoids silent configuration failures.
pub fn load_environment() -> Result<()> {
    let candidates = env_file_candidates()?;

    for path in candidates {
        debug!(path = %path.display(), "Checking environment file candidate.");

        if path.is_file() {
            dotenvy::from_path_override(&path)
                .with_context(|| format!("Failed to load environment file: {}", path.display()))?;

            info!(path = %path.display(), "Environment file loaded.");
            return Ok(());
        }
    }

    if has_runtime_environment() {
        info!(
            "Aucun fichier .env trouvé. Utilisation des variables déjà fournies par l’environnement du processus."
        );
    } else {
        info!(
            "Aucun fichier .env trouvé. Poursuite avec l’environnement du processus uniquement. Les variables manquantes seront signalées explicitement pendant le chargement de la configuration."
        );
    }

    Ok(())
}

/// Returns the most useful paths that were checked for .env.
pub fn environment_hint() -> String {
    match env_file_candidates() {
        Ok(paths) => paths
            .into_iter()
            .map(|path| format!("  - {}", path.display()))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(_) => "  - .env".to_string(),
    }
}

fn env_file_candidates() -> Result<Vec<PathBuf>> {
    let mut candidates = Vec::new();

    if let Ok(override_path) = env::var(ENV_FILE_OVERRIDE) {
        let trimmed = override_path.trim();
        if !trimmed.is_empty() {
            candidates.push(PathBuf::from(trimmed));
        }
    }

    let current_dir = env::current_dir().context("Failed to read current working directory")?;
    push_directory_chain(&mut candidates, &current_dir);

    if let Ok(executable_path) = env::current_exe() {
        if let Some(executable_dir) = executable_path.parent() {
            push_directory_chain(&mut candidates, executable_dir);
        }
    }

    deduplicate_paths(candidates)
}

fn push_directory_chain(candidates: &mut Vec<PathBuf>, start: &Path) {
    let mut current = Some(start);

    while let Some(directory) = current {
        candidates.push(directory.join(ENV_FILE_NAME));
        current = directory.parent();
    }
}

fn has_runtime_environment() -> bool {
    env::var("DISCORD_TOKEN").is_ok()
        || env::var("RATHENAFR_DB_HOST").is_ok()
        || env::var("RATHENAFR_DISCORD_BOT_ENV").is_ok()
}

fn deduplicate_paths(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();

    for path in paths {
        let normalized = match path.canonicalize() {
            Ok(canonical) => canonical,
            Err(_) => path,
        };

        if !result.iter().any(|existing| existing == &normalized) {
            result.push(normalized);
        }
    }

    Ok(result)
}
