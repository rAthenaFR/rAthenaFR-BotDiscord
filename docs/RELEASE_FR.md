# Publication

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

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

## Liste de vérification

- Changelog mis à jour.
- Documentation française à jour.
- `.env.example` et `.env.docker.example` cohérents.
- Aucun secret dans le dépôt.
- Aucun ancien nom de projet dans les fichiers.
