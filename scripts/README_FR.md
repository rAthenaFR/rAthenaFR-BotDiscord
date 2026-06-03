# Scripts Windows

Ces scripts sont des raccourcis pour les commandes locales les plus fréquentes du projet.
Ils se placent automatiquement à la racine du dépôt avant d’exécuter Cargo ou Docker Compose.

## Développement

| Script | Action |
|---|---|
| `dev-run.ps1` / `dev-run.cmd` | Lance le bot avec `cargo run`. |
| `dev-deploy.ps1` / `dev-deploy.cmd` | Déploie les commandes Discord avec `cargo run -- --deploy`. |
| `ci.ps1` / `ci.cmd` | Exécute `fmt --check`, `check`, `clippy` et les tests. |
| `build-release.ps1` / `build-release.cmd` | Compile en release et copie le binaire dans `dist/`. |
| `clean-local-build.ps1` / `clean-local-build.cmd` | Supprime `target/`, `dist/` et le dossier target local dédié. |

Les scripts Cargo utilisent `LOCALAPPDATA\Athena\rathenafr-discord-bot\target` pour éviter les blocages Windows App Control sur l’ancien binaire du dossier `target`.

## Docker

| Script | Action |
|---|---|
| `docker-build.ps1` / `docker-build.cmd` | Construit l’image Docker Compose. |
| `docker-up.ps1` / `docker-up.cmd` | Démarre le service avec build. |
| `docker-run.ps1` / `docker-run.cmd` | Alias historique de `docker-up`. |
| `docker-deploy.ps1` / `docker-deploy.cmd` | Déploie les commandes Discord depuis Docker. |
| `docker-logs.ps1` / `docker-logs.cmd` | Suit les logs du service bot. |
| `docker-down.ps1` / `docker-down.cmd` | Arrête la stack Docker Compose. |

`docker-deploy` n’est nécessaire qu’après un changement de structure des commandes slash.
