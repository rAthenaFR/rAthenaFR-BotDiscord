# Guide contributeur

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Objectif du projet

rAthenaFR Discord Bot lit une base compatible rAthena et affiche des informations utiles dans Discord sans modifier les données.

## Règles obligatoires

- SQL en lecture seule.
- Pas d’affichage de secrets ou données sensibles.
- Commandes staff protégées par rôle.
- Documentation française à jour.
- Aucun retour au nom historique du projet.

## Ajouter une commande publique

1. Définir la commande dans `src/discord/command_registry/public.rs`.
2. Ajouter les options dans `options.rs` si nécessaire.
3. Ajouter une méthode SQL dans `src/rathenafr/database.rs`.
4. Ajouter un modèle dans `src/rathenafr/models.rs` si nécessaire.
5. Ajouter un embed dans `src/discord/embeds/mod.rs`.
6. Ajouter le handler dans `dispatcher.rs`.
7. Déployer avec `cargo run -- --deploy`.

## Ajouter une commande staff

Même flux, mais dans `staff.rs`, avec vérification des rôles et réponse éphémère. N’affiche jamais `email`, `last_ip`, `user_pass`, tokens ou PINs.

## Revue avant PR

```bash
cargo fmt --all
cargo check
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```
