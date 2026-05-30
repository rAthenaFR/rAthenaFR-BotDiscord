# Base de données

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

## Accès SQL

Le bot lit les tables natives rAthena. Il ne doit jamais disposer de droits d’écriture.

Créer l’utilisateur en lecture seule :

```bash
mysql -u root -p < sql/create-readonly-user.sql
```

Avant exécution, remplace le mot de passe dans le fichier SQL.

## Tables utilisées

Selon les commandes, le bot peut lire notamment :

- `login`
- `char`
- `guild`
- `guild_castle`
- `guild_alliance`
- `guild_skill`
- `party`
- `homunculus`
- `pet`
- `quest`
- `inventory`
- `cart_inventory`
- `storage`
- `guild_storage`
- `vendings`
- `vending_items`
- `buyingstores`
- `buyingstore_items`

## Permissions recommandées

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Ne donne pas `INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER` ou `CREATE` au bot.
