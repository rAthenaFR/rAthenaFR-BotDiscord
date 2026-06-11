# Base de données

Le bot cible MariaDB/MySQL et conserve `RAthenaFrDatabase` comme API d’accès SQL. La lecture seule est le mode normal.

## Scripts fournis

| Script | Rôle |
|---|---|
| `create-account-management-user.sql` | Ajoute `INSERT` et `UPDATE` sur `login`. |
| `create-gmmsg-queue-user.sql` | Ajoute `INSERT` sur `discord_gmmsg_queue`. |
| `create-readonly-user.sql` | Crée l’utilisateur avec `SELECT` sur la base. |
| `discord_gmmsg_queue.sql` | Crée ou migre la file GMMSG. |
| `rathenafr_item_search.sql` | Crée le catalogue d’items et sa vue de recherche, puis importe les tables rAthena disponibles. |
| `rathenafr_mvp_data.sql` | Installe le catalogue MVP fourni avec le projet. |
| `rathenafr_mvp_regular_spawn.sql` | Crée la table support et la vue des MVP réguliers. |
| `sql_updates.sql` | Crée la table optionnelle lue par `/db last-update`. |
| `verify-installation.sql` | Vérifie les objets et colonnes SQL attendus sans modifier la base. |

> [!WARNING]
> Les scripts contiennent un nom de base, un hôte SQL et un mot de passe d’exemple. Relis-les avant exécution.

L’ordre d’installation et le détail de chaque script sont aussi disponibles dans [`sql/README_FR.md`](../sql/README_FR.md).

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

> [!IMPORTANT]
> Les scripts de permissions sont cumulatifs : ils ajoutent les droits demandés sans retirer les droits existants. `CREATE USER IF NOT EXISTS` ne remplace pas non plus le mot de passe d’un utilisateur déjà créé.

## Tables principales

Le bot détecte les tables avant les commandes qui en dépendent. Les principales familles sont :

| Fonction | Tables |
|---|---|
| Comptes et personnages | `login`, `char` |
| Guildes et WoE | `guild`, `guild_member`, `guild_position`, `guild_skill`, `guild_castle`, `guild_storage` |
| Inventaires | `inventory`, `cart_inventory`, `storage` |
| Quêtes et variables | `quest`, `char_reg_num`, `char_reg_str`, `acc_reg_num`, `acc_reg_str` |
| Items et monstres | `rathenafr_item_list`, `rathenafr_item_search`, `item_db`/`item_db_re`, `mob_db`/`mob_db_re`, `mob_skill_db` |
| Marché | `vendings`, `vending_items`, `buyingstores`, `buyingstore_items` |
| Logs | `mvplog`, `picklog`, `zenylog`, `loginlog`, `chatlog`, `atcommandlog`, `branchlog`, `charlog` |

Des tables rAthena supplémentaires sont détectées pour les diagnostics, notamment `party`, `pet`, `homunculus`, `mail`, `mail_attachments`, `skill` et `sql_updates`.

> [!NOTE]
> Une installation rAthena peut ne pas fournir tous les logs. Les commandes concernées restent disponibles mais retournent une erreur de table manquante ou une liste vide.

## Recherche d’items

`/item info` s’appuie sur la vue `rathenafr_item_search`. Les données sont conservées dans la table `rathenafr_item_list`.

Le script dédié :

- crée la table et la vue si elles n’existent pas ;
- migre l’ancien objet `rathenafr_item_search` lorsqu’il s’agit encore d’une table ;
- importe ou actualise les données disponibles depuis `item_db` et `item_db_re` ;
- conserve les lignes personnalisées ou importées depuis les fichiers YAML ;
- expose uniquement les lignes actives avec `enabled = 1`.

Après une mise à jour importante des tables d’items, réexécute :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_item_search.sql
```

> [!NOTE]
> Le script met à jour les objets trouvés, mais ne désactive pas automatiquement les anciennes lignes absentes des tables rAthena. Utilise la colonne `enabled` pour retirer explicitement une entrée personnalisée.

## MVP réguliers

`/mvp list` lit la vue `rathenafr_mvp_regular_spawn`, construite depuis `rathenafr_mvp_list`.

Installe d’abord le schéma, puis le catalogue fourni :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_mvp_regular_spawn.sql
mariadb -u root -p ragnarok < sql/rathenafr_mvp_data.sql
```

Le catalogue contient les MVP connus et les spawns réguliers utilisés par `/mvp list`. Le script de données remplace seulement les sources gérées par le projet et conserve les lignes issues d’une source personnalisée.

> [!IMPORTANT]
> Les cartes et délais fournis correspondent au catalogue de référence du projet. Adapte ou désactive les lignes concernées si ton serveur modifie les spawns MVP.

`/mvp top` lit `mvplog`. `/mvp last` utilise aussi `rathenafr_mvp_list` et `rathenafr_item_search` pour résoudre les noms du MVP et de sa récompense.

## GMMSG

La file attend :

```text
discord_gmmsg_queue.message = VARBINARY(180)
```

Le bot insère uniquement les lignes `pending`. Le script NPC rAthena est responsable du traitement et des mises à jour `done`/`failed`.

`discord_gmmsg_queue.sql` peut être réexécuté : il ajoute les colonnes et l’index manquants sans supprimer les lignes existantes.

> [!WARNING]
> La migration s’arrête volontairement si une ancienne ligne contient un message de plus de 180 octets. Corrige ou archive ces lignes avant de relancer le script afin d’éviter une troncature silencieuse.

Consulte [Bridge GMMSG](GMMSG_BRIDGE_FR.md).

## Diagnostic

Avant de lancer le bot, vérifie les objets installés :

```bash
mariadb -u root -p ragnarok < sql/verify-installation.sql
```

Les commandes Owner suivantes aident à comparer le schéma attendu au schéma réel :

- `/db health`
- `/db tables`
- `/db count`
- `/db logs-size`
- `/db last-update`

> [!TIP]
> Commence par `/db health` après une migration rAthena ou l’activation d’une nouvelle fonctionnalité.
