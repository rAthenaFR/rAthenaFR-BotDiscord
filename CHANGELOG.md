# Changelog

## 0.2.4

### Ajouté

- Ajout d’une couche i18n dans `src/i18n/` avec locales typées, clés `I18nKey`, chargement des catalogues FTL et traduction avec variables.
- Ajout des catalogues `fr-FR`, `en-US`, `es-ES`, `de-DE`, `ja-JP`, `ko-KR` et `zh-CN`, avec `fr-FR` comme fallback.
- Localisation des descriptions de commandes, sous-commandes et options slash, des réponses runtime, des embeds et des boutons de pagination MVP.
- Ajout de variantes d’embeds localisées pour les commandes publiques et staff, les erreurs, les listes, les comptes, le marché, les monstres, les MVP et les logs staff.
- Ajout de tests i18n vérifiant la présence des clés dans les sept catalogues, la cohérence des variables FTL et la normalisation des locales Discord.
- Ajout d’un catalogue SQL reproductible de 202 MVP, dont 61 spawns réguliers utilisés par `/mvp list`.
- Ajout de `sql/verify-installation.sql` et `sql/README_FR.md` pour installer et contrôler les objets SQL du bot.

### Modifié

- Découpage du dispatcher Discord en modules de routage, réponses, validations, composants et handlers publics/staff par domaine sous `src/discord/interactions/dispatcher/`.
- Découpage des embeds Discord par domaine sous `src/discord/embeds/`, avec helpers communs de formatage, limitation et assainissement.
- Découpage de `src/rathenafr/database.rs` en repositories internes spécialisés sous `src/rathenafr/database/`.
- Découpage de la configuration sous `src/config/` et des modèles rAthenaFR sous `src/rathenafr/models/`.
- Conservation de `RAthenaFrDatabase` comme API publique stable et limitation des helpers internes à leur module.
- Conservation des noms de commandes slash, du fallback français et du comportement des permissions staff.
- Mise à jour de la documentation d’installation, de base de données, GMMSG, sécurité et dépannage avec l’ordre SQL reproductible.

### Corrigé

- Correction des conflits de modules Rust entre les anciens fichiers monolithiques et leurs nouveaux dossiers `dispatcher/` et `models/`.
- Correction des imports et réexports cassés après le déplacement des modules de configuration, modèles, interactions et embeds.
- Correction de la locale utilisée lors de la création initiale du panneau MVP.
- Correction des attributs Clippy dupliqués, des signatures utilisant `&String`, des visibilités de tests et de plusieurs simplifications sans changement métier.
- Correction de `rathenafr_item_search.sql` : le catalogue est désormais stocké dans `rathenafr_item_list`, exposé par une vue stable et migré depuis l’ancien format sans supprimer les entrées personnalisées.
- Correction de `discord_gmmsg_queue.sql` afin de compléter toutes les colonnes et l’index manquants sans supprimer les lignes existantes ni tronquer silencieusement les messages.
- Correction de `rathenafr_mvp_regular_spawn.sql` afin d’aligner le schéma et la vue sur les requêtes utilisées par le bot.
- Validation finale réussie avec `cargo fmt --all`, `cargo check --workspace`, Clippy avec `-D warnings` et 87 tests.

### Nettoyage

- Suppression des anciens modules de compatibilité ou placeholders non référencés :
  - `src/discord/commands/` ;
  - `src/discord/interactions/components/` ;
  - `src/discord/interactions/public/` ;
  - `src/discord/interactions/staff/` ;
  - `src/discord/interactions/router.rs` ;
  - `src/discord/ui/embeds/` ;
  - `src/rathenafr/repositories/` ;
  - `src/rathenafr/services/`.
- Suppression des anciens monolithes `src/discord/interactions/dispatcher.rs` et `src/rathenafr/models.rs`, remplacés par leurs structures modulaires.

### Compatibilité

- Les scripts SQL migrent les anciens objets de compatibilité sans modifier le schéma natif rAthena.
- Aucun changement des permissions staff ou des noms de commandes slash publiques.
- Les appels existants à `RAthenaFrDatabase` restent compatibles.

## 0.2.3

### Ajouté

- Ajout d’un panneau paginé pour `/mvp list` avec boutons `Début`, `Précédent`, `Suivant` et `Fin`.
- Ajout de timers Discord dynamiques à `/mvp list`, calculés depuis le dernier kill SQL par MVP et par carte.
- Ajout d’un embed détaillé à `/mvp last` avec joueur, carte, date Discord, EXP MVP et récompense.
- Ajout du script `sql/rathenafr_mvp_regular_spawn.sql` pour créer `rathenafr_mvp_list` et la vue `rathenafr_mvp_regular_spawn`.
- Ajout du script `sql/rathenafr_item_search.sql` pour créer et rafraîchir la source SQL utilisée par `/item info`.
- Ajout du script `sql/sql_updates.sql` pour les installations rAthena qui ne fournissent plus cette table.
- Ajout du script `sql/create-gmmsg-queue-user.sql` pour les droits SQL du mode `/gmmsg` `sql_queue`.
- Ajout des variables d’exemple `RATHENAFR_ACCOUNT_MANAGE_ENABLED`, `RATHENAFR_ACCOUNT_DELETE_ENABLED`, `RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE` et `RATHENAFR_ACCOUNT_DELETE_MIN_ROLE`.
- Ajout des scripts Windows `ci`, `docker-build`, `docker-up`, `docker-deploy`, `docker-logs`, `docker-down` et de la documentation `scripts/README_FR.md`.

### Modifié

- `/mvp list` combine désormais la vue `rathenafr_mvp_regular_spawn` avec `mvplog` et affiche les états d’attente, de fenêtre ouverte ou de disponibilité probable.
- `/mvp last` résout désormais les noms des MVP et des récompenses depuis les tables de recherche rAthenaFR.
- `/mob info`, `/mob drops` et `/who-drops` prennent désormais en charge les tables Renewal et les références d’objets par AegisName.
- `/mob info` et `/mob drops` appliquent désormais les rates EXP/drop configurés du map-server et distinguent les valeurs serveur des modificateurs propres au joueur.
- `/mob drops` utilise désormais un embed structuré limité à 10 objets, avec noms, identifiants, AegisName et taux serveur lisibles sur mobile.
- `/db health` signale désormais aussi la présence de `rathenafr_item_search` dans les tables optionnelles.
- Mise à jour de `sql/create-account-management-user.sql` avec le droit `UPDATE` sur `login` requis par `/staff account-manage`.
- Alignement de `.env.example`, `.env.docker.example` et `docs/CONFIGURATION_FR.md` avec les variables réellement lues par le bot.
- Mise à jour de la documentation SQL, installation, sécurité, GMMSG et commandes pour refléter les scripts fournis.
- Alignement des wrappers `.cmd` sur les scripts PowerShell afin d’utiliser la racine du dépôt et le même target Cargo dédié.

### Corrigé

- Correction de l’affichage incomplet de `/mvp list` en récupérant tous les MVP réguliers disponibles dans la vue SQL.
- Correction de la jointure carte de `/mvp list` lorsque `mvplog` et la vue MVP utilisent des collations différentes.
- Correction des doublons possibles de `/mvp last` lorsqu’un même MVP possède plusieurs cartes.
- Correction de la détection des colonnes `dropN_item` et `mvpdropN_item` des schémas rAthena Renewal.
- Les taux SQL bruts ne sont plus présentés comme taux de drop effectifs lorsque les rates serveur ne sont pas configurés dans le bot.
- Correction de `/item info`, qui lit désormais `rathenafr_item_search` et accepte ID, nom d’objet ou AegisName.
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
