# Base de données

Le bot cible MariaDB/MySQL et conserve `RAthenaFrDatabase` comme API d’accès SQL. La lecture seule est le mode normal.

## Scripts fournis

| Script | Rôle |
|---|---|
| `create-readonly-user.sql` | Crée l’utilisateur avec `SELECT` sur la base. |
| `create-account-management-user.sql` | Ajoute `INSERT` et `UPDATE` sur `login`. |
| `create-gmmsg-queue-user.sql` | Ajoute `INSERT` sur `discord_gmmsg_queue`. |
| `discord_gmmsg_queue.sql` | Crée ou migre la file GMMSG. |
| `rathenafr_item_search.sql` | Crée et rafraîchit la table de recherche d’items. |
| `rathenafr_mvp_regular_spawn.sql` | Crée la table support et la vue des MVP réguliers. |
| `sql_updates.sql` | Crée la table optionnelle lue par `/db last-update`. |

> [!WARNING]
> Les scripts contiennent un nom de base, un hôte SQL et un mot de passe d’exemple. Relis-les avant exécution.

## Droits minimaux

Mode normal :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Écritures optionnelles :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';
```

> [!CAUTION]
> N’accorde pas `DELETE`, `DROP`, `ALTER` ou `CREATE` à l’utilisateur d’exécution. Les migrations sont exécutées séparément avec un compte administrateur.

## Tables principales

Le bot détecte les tables avant les commandes qui en dépendent. Les principales familles sont :

| Fonction | Tables |
|---|---|
| Comptes et personnages | `login`, `char` |
| Guildes et WoE | `guild`, `guild_member`, `guild_position`, `guild_skill`, `guild_castle`, `guild_storage` |
| Inventaires | `inventory`, `cart_inventory`, `storage` |
| Quêtes et variables | `quest`, `char_reg_num`, `char_reg_str`, `acc_reg_num`, `acc_reg_str` |
| Items et monstres | `rathenafr_item_search`, `item_db`/`item_db_re`, `mob_db`/`mob_db_re`, `mob_skill_db` |
| Marché | `vendings`, `vending_items`, `buyingstores`, `buyingstore_items` |
| Logs | `mvplog`, `picklog`, `zenylog`, `loginlog`, `chatlog`, `atcommandlog`, `branchlog`, `charlog` |

Des tables rAthena supplémentaires sont détectées pour les diagnostics, notamment `party`, `pet`, `homunculus`, `mail`, `mail_attachments`, `skill` et `sql_updates`.

> [!NOTE]
> Une installation rAthena peut ne pas fournir tous les logs. Les commandes concernées restent disponibles mais retournent une erreur de table manquante ou une liste vide.

## Recherche d’items

`/item info` s’appuie sur `rathenafr_item_search`. Le script dédié consolide les données disponibles depuis `item_db` et `item_db_re`.

Après une mise à jour importante des tables d’items, réexécute :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_item_search.sql
```

## MVP réguliers

`/mvp list` lit la vue `rathenafr_mvp_regular_spawn`, construite depuis `rathenafr_mvp_list`.

> [!IMPORTANT]
> Le script SQL ne fournit pas les données de spawn. `rathenafr_mvp_list` doit être alimentée avec une source adaptée à la configuration de ton serveur.

`/mvp last` et `/mvp top` lisent `mvplog` lorsqu’elle existe.

## GMMSG

La file attend :

```text
discord_gmmsg_queue.message = VARBINARY(180)
```

Le bot insère uniquement les lignes `pending`. Le script NPC rAthena est responsable du traitement et des mises à jour `done`/`failed`.

Consulte [Bridge GMMSG](GMMSG_BRIDGE_FR.md).

## Diagnostic

Les commandes Owner suivantes aident à comparer le schéma attendu au schéma réel :

- `/db health`
- `/db tables`
- `/db count`
- `/db logs-size`
- `/db last-update`

> [!TIP]
> Commence par `/db health` après une migration rAthena ou l’activation d’une nouvelle fonctionnalité.
