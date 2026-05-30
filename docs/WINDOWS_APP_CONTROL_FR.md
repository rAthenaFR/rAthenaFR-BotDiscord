# Windows App Control

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Problème

Windows peut bloquer un binaire Rust déjà compilé dans `target`.

## Solution

Utilise un dossier de target séparé :

```powershell
$env:CARGO_TARGET_DIR="$env:LOCALAPPDATA\Athena\rathenafr-discord-bot\target"
cargo run
```

Les scripts PowerShell fournis utilisent déjà un dossier dédié dans `%LOCALAPPDATA%`.
