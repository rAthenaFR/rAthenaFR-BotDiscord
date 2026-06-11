# Contribution

Merci d’aider à améliorer rAthenaFR Discord Bot.

Commence par lire `docs/DEVELOPMENT_FR.md`. Le guide explique le workflow, la structure des commandes, l’i18n, les tests et la publication.

Lis aussi `CODE_OF_CONDUCT.md` avant de participer aux issues, pull requests, reviews ou discussions.

## Avant de commencer

Le projet est en lecture seule par défaut. Les nouvelles écritures doivent être désactivées par défaut, protégées, auditées et explicitement documentées.

Règles principales :

- garder le SQL en lecture seule par défaut ;
- protéger les données privées des comptes ;
- réserver les commandes sensibles au staff ;
- utiliser `.env` ou les variables d’environnement pour la configuration ;
- mettre la documentation à jour si le comportement change.

## Vérifications de développement

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Ou :

```bash
make ci
```

## Règles pour les commandes

Les commandes publiques doivent rester sûres pour les joueurs. Les commandes staff doivent vérifier les rôles, répondre en éphémère quand possible et éviter les champs sensibles.

Quand une commande ou une option Discord change, redéploie les commandes :

```bash
cargo run -- --deploy
```

## Documentation

À mettre à jour quand le comportement change :

- `README.md`
- `docs/COMMANDS_FR.md`
- `docs/CONFIGURATION_FR.md`
- `docs/DATABASE_FR.md`
- `docs/DEVELOPMENT_FR.md`
- `.env.example`
- `.env.docker.example`
- `CHANGELOG.md`
