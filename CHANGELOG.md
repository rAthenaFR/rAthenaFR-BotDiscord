# Changelog

## 0.2.1

### Ajouté

- Ajout du bridge SQL queue pour `/gmmsg` via `discord_gmmsg_queue`.
- Ajout du mode `RATHENAFR_GMMSG_MODE=sql_queue`.
- Ajout de `RATHENAFR_GMMSG_ENCODING=windows1252` pour stocker les messages en octets compatibles client Ragnarok Online.
- Ajout du schéma attendu avec `discord_gmmsg_queue.message` en `VARBINARY(180)`.

### Modifié

- Mise à jour de la documentation `/gmmsg`, configuration, base de données, sécurité et installation.
- Mise à jour des exemples `.env.example` et `.env.docker.example`.

### Corrigé

- Correction de la CI Clippy en supprimant le code mort des anciennes commandes retirées.
- Refus des caractères non compatibles Windows-1252, notamment les emojis, pour éviter les messages illisibles en jeu.

## 0.2.0

### Modifié

- Remplacement des anciens packs public/staff par les packs essentiels de première release.
- Retrait du registre Discord des commandes hors scope, dont les anciennes commandes de compte dangereuses.
- Conservation de `/createaccount` sans renommage.

### Ajouté

- Nouveau pack public : `/server`, `/online`, `/player`, `/guild`, `/castle`, `/item`, `/who-drops`, `/mob`, `/mvp`, `/top`, `/rank`, `/market`.
- Nouveau pack staff sous `/staff`, `/mod`, `/debug`, `/audit` et `/db`.
- `/gmmsg` avec abstraction GameBridge, modes `disabled`, `test` et `bridge`, et validation des messages.
- `/db health` et diagnostics de tables/logs rAthena.
- Configuration des rôles Helper, Moderator, GM, Admin et Owner.
- Configuration des packs, commandes désactivables, top zeny, tables item/mob et limites `/gmmsg`.

### Sécurité

- Les nouvelles commandes SQL sont en lecture seule.
- `/createaccount` reste la seule commande autorisée à écrire en base si elle est activée.
- Les logs staff sensibles répondent en éphémère et masquent les IP complètes.

## 0.1.0

Version française complète pour le projet rAthena.

### Modifié

- Renommage complet du projet en `rAthenaFR`.
- Renommage du crate Rust en `rathenafr-discord-bot`.
- Renommage du module interne `src/rathenafr`.
- Renommage des variables d’environnement en `RATHENAFR_*`.
- Renommage de l’utilisateur SQL d’exemple en `rathenafr_bot`.
- Traduction française des descriptions de commandes Discord.
- Traduction française des embeds, erreurs et messages visibles.
- Traduction française de la documentation principale.

### Sécurité

- Conservation du modèle SQL en lecture seule.
- Conservation du principe non-root dans Docker.
- Conservation des commandes staff protégées par rôles Discord.

### Compatibilité

Les noms des commandes slash restent inchangés pour éviter de casser les habitudes Discord existantes. Après mise à jour, redéploie les commandes :

```bash
cargo run -- --deploy
```
