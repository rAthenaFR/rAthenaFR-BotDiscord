# Développement

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Préparer l’environnement

```bash
cp .env.example .env
cargo check
```

## Commandes utiles

```bash
cargo fmt --all
cargo check
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Ou :

```bash
make ci
```

## Lancer localement

```bash
cargo run
```

Déployer les commandes :

```bash
cargo run -- --deploy
```

## Règles de développement

- Ne pas ajouter de requêtes d’écriture SQL.
- Ne pas afficher de données sensibles.
- Garder les textes visibles en français.
- Garder `rAthena` dans l’affichage des commandes et `Bot Discord rAthenaFR` dans le footer par défaut.
- Utiliser les variables `RATHENAFR_*`.
