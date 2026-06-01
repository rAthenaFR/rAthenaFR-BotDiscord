# Publication

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!IMPORTANT]
> Une release ne doit pas partir si la documentation ne reflète pas les commandes slash et les permissions SQL actuelles.

## Préparer une release

```bash
cargo fmt --all
cargo check
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

## Build local

```bash
cargo build --release
```

Le binaire est produit dans :

```text
target/release/rathenafr-discord-bot
```

## Docker

```bash
docker compose build
```

> [!TIP]
> Après une modification d’option ou description de commande slash, exécute aussi le déploiement des commandes Discord sur l’environnement cible.

## Liste de vérification

- Changelog mis à jour.
- Documentation française à jour.
- `.env.example` et `.env.docker.example` cohérents.
- Aucun secret dans le dépôt.
- Aucun ancien nom de projet dans les fichiers.
- `docs/INDEX_FR.md` et les pages liées pointent vers les nouvelles fonctionnalités.

> [!WARNING]
> Ne modifie pas le `README.md` dans une release documentaire si la demande cible uniquement le dossier `docs/`.
