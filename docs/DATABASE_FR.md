# Base de données

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Accès SQL

Le bot lit les tables natives rAthena. Par défaut, il ne doit disposer d’aucun droit d’écriture.

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
- `item_db` / `item_db_re` si disponibles
- `mob_db` / `mob_db_re` si disponibles

## Permissions recommandées

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Ne donne pas `INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER` ou `CREATE` au bot pour les commandes de consultation.

## Permissions optionnelles pour les comptes

Les commandes `/createaccount` et `/accountmanage` changent volontairement le modèle historique en lecture seule.

Si tu les utilises, applique uniquement les droits nécessaires sur `login` :

```bash
mysql -u root -p < sql/create-account-management-user.sql
```

Ce script ajoute :

```sql
GRANT INSERT, DELETE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

Ne donne pas de droits d’écriture sur les tables de personnages, inventaires, guildes ou stockages.

La suppression via `/accountmanage` est bloquée par le bot si le compte possède encore un personnage ou du stockage.
