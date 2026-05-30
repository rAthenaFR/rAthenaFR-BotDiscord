# Contribution

Merci d’aider à améliorer rAthenaFR Discord Bot.

Commence par lire `docs/CONTRIBUTOR_GUIDE_FR.md`. Le guide explique les modes d’exécution, la structure des modules, le flux des commandes et les règles de sécurité.

Lis aussi `CODE_OF_CONDUCT.md` avant de participer aux issues, pull requests, reviews ou discussions.

## Avant de commencer

Le projet est volontairement en lecture seule. Les nouvelles commandes ne doivent pas écrire dans la base rAthenaFR, sauf si une couche d’écriture séparée, auditée et explicitement documentée est ajoutée plus tard.

Règles principales :

- garder le SQL en lecture seule ;
- protéger les données privées des comptes ;
- réserver les commandes sensibles au staff ;
- utiliser `.env` ou les variables d’environnement pour la configuration ;
- mettre la documentation à jour si le comportement change.

## Vérifications de développement

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo check --all-features
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
- `docs/CONTRIBUTOR_GUIDE_FR.md`
- `.env.example`
- `.env.docker.example`
- `CHANGELOG.md`
