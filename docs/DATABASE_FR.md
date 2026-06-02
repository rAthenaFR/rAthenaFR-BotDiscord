# Base de données

Le bot doit fonctionner avec un utilisateur SQL en lecture seule pour toutes les commandes publiques, staff, modération, debug, audit et DB.

## Permissions recommandées

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

> [!CAUTION]
> **Droits SQL interdits**
>
> Ne donne pas `UPDATE`, `DELETE`, `DROP`, `ALTER` ou `CREATE` au bot pour cette version.

## Exception createaccount

`/createaccount` est conservée. Si `RATHENAFR_ACCOUNT_CREATION_ENABLED=true`, elle peut écrire dans `login` comme avant.

Permission minimale additionnelle :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

Aucune autre commande de cette version n’est censée écrire en base.

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