# Développement

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Pour une vue globale des fichiers, consulte `ARCHITECTURE_FR.md`.

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

> [!TIP]
> Lance `cargo run -- --deploy` uniquement après une modification de commande slash. Un changement d’embed ne demande qu’un redémarrage.

## Règles de développement

- Ne pas ajouter de requêtes d’écriture SQL hors fonctionnalités de compte explicitement documentées.
- Ne pas afficher de données sensibles.
- Garder les textes visibles en français.
- Garder `rAthena` dans l’affichage des commandes et `Bot Discord rAthenaFR` dans le footer par défaut.
- Utiliser les variables `RATHENAFR_*`.

> [!WARNING]
> Ne touche pas aux droits SQL ou aux commandes de compte sans mettre à jour `DATABASE_FR.md`, `SECURITY_FR.md` et `ACCOUNT_MANAGEMENT_FR.md`.
