# Changelog

## 0.2.3

### Ajouté

- Ajout d’un panneau paginé pour `/mvp list` avec boutons `Début`, `Précédent`, `Suivant` et `Fin`.
- Ajout du script `sql/rathenafr_mvp_regular_spawn.sql` pour créer `rathenafr_mvp_list` et la vue `rathenafr_mvp_regular_spawn`.
- Ajout du script `sql/create-gmmsg-queue-user.sql` pour les droits SQL du mode `/gmmsg` `sql_queue`.
- Ajout des variables d’exemple `RATHENAFR_ACCOUNT_MANAGE_ENABLED`, `RATHENAFR_ACCOUNT_DELETE_ENABLED`, `RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE` et `RATHENAFR_ACCOUNT_DELETE_MIN_ROLE`.

### Modifié

- `/mvp list` lit désormais directement la vue SQL `rathenafr_mvp_regular_spawn`.
- Mise à jour de `sql/create-account-management-user.sql` avec le droit `UPDATE` sur `login` requis par `/staff account-manage`.
- Alignement de `.env.example`, `.env.docker.example` et `docs/CONFIGURATION_FR.md` avec les variables réellement lues par le bot.
- Mise à jour de la documentation SQL, installation, sécurité, GMMSG et commandes pour refléter les scripts fournis.

### Corrigé

- Correction de l’affichage incomplet de `/mvp list` en récupérant tous les MVP réguliers disponibles dans la vue SQL.
- Correction des `custom_id` des boutons de pagination MVP pour éviter les doublons refusés par Discord.
- Retrait des variables d’exemple non utilisées liées aux assets et aux anciennes options de tables optionnelles.

## 0.2.2

### Ajouté

- Ajout de `/staff account-manage` avec `edit`, `ban`, `unban` et `delete` en désactivation forte.
- Ajout des variables `RATHENAFR_ACCOUNT_MANAGE_ENABLED`, `RATHENAFR_ACCOUNT_DELETE_ENABLED`, `RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE` et `RATHENAFR_ACCOUNT_DELETE_MIN_ROLE`.
- Ajout de la documentation staff de gestion de compte dans `docs/`.

### Sécurité

- `account-manage` est désactivée par défaut, répond en éphémère et journalise les actions dans le salon staff-log configuré.
- `delete` exige `confirm="SUPPRIMER"` et ne supprime pas physiquement la ligne `login`.
- Les secrets de compte restent exclus des réponses et des logs.

### Corrigé

- Correction du comptage des membres connectés de guilde via `guild_member` et `char`.`online`.
- Correction des commandes `/mvp list`, `/mvp last` et `/mvp top` pour éviter les réponses vides et respecter la table mob configurée.

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
