# Base de données

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!IMPORTANT]
> Par défaut, le bot doit fonctionner avec un utilisateur SQL en lecture seule.

## Accès SQL

Le bot lit les tables natives rAthena. Par défaut, il ne doit disposer d’aucun droit d’écriture.

Créer l’utilisateur en lecture seule :

```bash
mysql -u root -p < sql/create-readonly-user.sql
```

Avant exécution, remplace le mot de passe dans le fichier SQL.

> [!WARNING]
> N’utilise pas un compte SQL administrateur pour le bot.

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

> [!NOTE]
> Certaines tables sont optionnelles selon la version ou la configuration rAthena. Les commandes concernées signalent les tables manquantes quand elles sont nécessaires.

## Permissions recommandées

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Ne donne pas `INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER` ou `CREATE` au bot pour les commandes de consultation.

> [!TIP]
> Cette configuration suffit pour toutes les commandes publiques de lecture et pour les commandes staff de consultation.

## Permissions optionnelles pour les comptes

Les commandes `/createaccount` et `/accountmanage` changent volontairement le modèle historique en lecture seule.

> [!CAUTION]
> N’applique ces droits que si tu assumes la création et la suppression de comptes depuis Discord.

Si tu les utilises, applique les droits nécessaires aux tables de compte et de personnages :

```bash
mysql -u root -p < sql/create-account-management-user.sql
```

Ce script ajoute :

```sql
GRANT INSERT, UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT DELETE ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Ne donne pas `DROP`, `ALTER` ou `CREATE` au bot. Le droit `UPDATE` est requis pour que `/accountmanage action:edit` modifie `login`, et le droit `DELETE` est requis pour que `/accountmanage action:delete` nettoie les tables rAthena liées au compte et aux personnages dans une transaction.

La suppression via `/accountmanage` est bloquée par le bot si un personnage du compte possède une guilde, afin d’éviter de supprimer ou d’orpheliner des données de guilde appartenant aussi à d’autres joueurs.

Voir aussi : [Gestion de comptes](ACCOUNT_MANAGEMENT_FR.md).
