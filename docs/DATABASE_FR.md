# Base de données

Le bot doit fonctionner avec un utilisateur SQL en lecture seule par défaut. Les écritures SQL sont limitées aux fonctionnalités explicitement activées : `/createaccount`, `/staff account-manage` et `/gmmsg` en mode `sql_queue`.

## Scripts fournis

Le dossier `sql/` contient les scripts d’installation et de droits à exécuter manuellement avec un utilisateur administrateur MariaDB/MySQL :

| Script | Usage |
|---|---|
| `sql/create-readonly-user.sql` | Crée l’utilisateur bot avec `SELECT` sur la base rAthena. |
| `sql/create-account-management-user.sql` | Ajoute les droits nécessaires à `/createaccount` et `/staff account-manage`. |
| `sql/create-gmmsg-queue-user.sql` | Ajoute le droit `INSERT` sur `discord_gmmsg_queue` pour `/gmmsg` en mode `sql_queue`. |
| `sql/discord_gmmsg_queue.sql` | Crée ou met à jour la table `discord_gmmsg_queue`. |
| `sql/rathenafr_mvp_regular_spawn.sql` | Crée `rathenafr_mvp_list` et la vue `rathenafr_mvp_regular_spawn` utilisée par `/mvp list`. |

## Permissions recommandées

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

> [!CAUTION]
> **Droits SQL sensibles**
>
> Ne donne pas `DELETE`, `DROP`, `ALTER` ou `CREATE` à l’utilisateur d’exécution du bot.
> `UPDATE` sur `login` est nécessaire uniquement si `/staff account-manage` est activée.

## Exception createaccount

`/createaccount` est conservée. Si `RATHENAFR_ACCOUNT_CREATION_ENABLED=true`, elle peut écrire dans `login` comme avant.

Permission minimale additionnelle :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

## Exception account-management staff

Si `RATHENAFR_ACCOUNT_MANAGE_ENABLED=true`, `/staff account-manage` modifie uniquement des champs ciblés de `login`.

Permissions minimales additionnelles :

```sql
GRANT SELECT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT SELECT ON `ragnarok`.`char` TO 'rathenafr_bot'@'%';
GRANT UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

## Exception GMMSG SQL Queue

Si `RATHENAFR_GMMSG_MODE=sql_queue`, `/gmmsg` ajoute des messages dans la file SQL `discord_gmmsg_queue`.

Permission minimale additionnelle :

```sql
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';
```

La table attend une colonne `message` en `VARBINARY(180)` afin de stocker les octets Windows-1252 affichés correctement par le client Ragnarok Online.

> [!IMPORTANT]
> Le bot insère uniquement une ligne `pending`. Le script NPC rAthena doit lire la file, annoncer le message en jeu et passer le statut à `done`.

> [!WARNING]
> Ne donne pas `UPDATE`, `DELETE`, `DROP`, `ALTER` ou `CREATE` au bot pour gérer la file GMMSG. Ces droits relèvent de l’installation ou de la maintenance SQL, pas de l’exécution normale.

## MVP réguliers

`/mvp list` lit la vue `rathenafr_mvp_regular_spawn`. Le script `sql/rathenafr_mvp_regular_spawn.sql` crée la table support `rathenafr_mvp_list` et la vue filtrée sur les spawns réguliers.

La table `rathenafr_mvp_list` doit ensuite être peuplée par l’import MVP Athena validé pour le serveur. La vue expose les colonnes attendues par le bot :

- `monster_id`
- `monster_name`
- `aegis_name`
- `map_name`
- `respawn_minutes`
- `respawn_variance_minutes`
- `source`

## Tables principales

Les commandes lisent les tables seulement si elles existent :

- `login`
- `char`
- `guild`
- `guild_member`
- `guild_position`
- `guild_skill`
- `guild_castle`
- `guild_storage`
- `party`
- `inventory`
- `cart_inventory`
- `storage`
- `mail`
- `mail_attachments`
- `skill`
- `quest`
- `pet`
- `homunculus`
- `char_reg_num`
- `char_reg_str`
- `acc_reg_num`
- `acc_reg_str`

## Items, monstres, marché et logs

Tables item/mob configurables :

- `item_db` ou `item_db_re`
- `mob_db` ou `mob_db_re`

Tables optionnelles :

- `rathenafr_mvp_list`
- `rathenafr_mvp_regular_spawn`
- `mob_skill_db`
- `mvplog`
- `picklog`
- `zenylog`
- `loginlog`
- `chatlog`
- `atcommandlog`
- `branchlog`
- `charlog`
- `vendings`
- `vending_items`
- `buyingstores`
- `buyingstore_items`
- `sql_updates`

> [!NOTE]
> **Tables absentes**
>
> Certaines installations rAthena ne possèdent pas toutes ces tables.
> `/db health` affiche les tables présentes, les tables manquantes et les logs détectés.
