$ErrorActionPreference = "Stop"

$ProjectRoot = Resolve-Path "$PSScriptRoot\.."
Set-Location $ProjectRoot

# Utilise un dossier target dédié hors du dépôt. Cela évite de réutiliser un
# exécutable que Windows Application Control pourrait déjà avoir bloqué.
$env:CARGO_TARGET_DIR = Join-Path $env:LOCALAPPDATA "Athena\rathenafr-discord-bot\target"

cargo run -- --deploy
