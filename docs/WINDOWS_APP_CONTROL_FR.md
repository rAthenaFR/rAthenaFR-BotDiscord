# Windows App Control

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Cette page concerne surtout les postes Windows qui bloquent l’exécution d’un binaire Rust déjà présent dans `target`.

## Problème

Windows peut bloquer un binaire Rust déjà compilé dans `target`.

## Solution

Utilise un dossier de target séparé :

```powershell
$env:CARGO_TARGET_DIR="$env:LOCALAPPDATA\Athena\rathenafr-discord-bot\target"
cargo run
```

Les scripts PowerShell fournis utilisent déjà un dossier dédié dans `%LOCALAPPDATA%`.

Scripts concernés :

- `scripts\dev-run.ps1`
- `scripts\dev-deploy.ps1`
- `scripts\ci.ps1`
- `scripts\build-release.ps1`
- `scripts\clean-local-build.ps1`

> [!TIP]
> Si le problème persiste, supprime l’ancien dossier `target` du projet après avoir fermé les processus Rust en cours.
