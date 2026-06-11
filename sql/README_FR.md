# Scripts SQL

Le dossier contient uniquement les extensions nécessaires au bot. Le schéma
rAthena natif (`login`, `char`, `guild`, inventaires et logs) doit être installé
depuis rAthena.

> [!WARNING]
> Exécute les scripts de structure avec un compte administrateur, après
> sauvegarde. Modifie le nom de base et le mot de passe dans les scripts
> `create-*.sql` avant leur utilisation.

## Installation recommandée

Dans la base rAthena cible :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_item_search.sql
mariadb -u root -p ragnarok < sql/rathenafr_mvp_regular_spawn.sql
mariadb -u root -p ragnarok < sql/rathenafr_mvp_data.sql
mariadb -u root -p ragnarok < sql/discord_gmmsg_queue.sql
mariadb -u root -p ragnarok < sql/sql_updates.sql
```

Puis accorde les droits correspondant aux fonctionnalités activées :

```bash
mariadb -u root -p ragnarok < sql/create-readonly-user.sql
mariadb -u root -p ragnarok < sql/create-account-management-user.sql
mariadb -u root -p ragnarok < sql/create-gmmsg-queue-user.sql
```

> [!IMPORTANT]
> Les scripts de droits sont cumulatifs. `create-readonly-user.sql` ne retire
> pas les droits déjà accordés à un utilisateur existant.

## Objets installés

| Script | Objets |
|---|---|
| `rathenafr_item_search.sql` | Table `rathenafr_item_list` et vue `rathenafr_item_search`. |
| `rathenafr_mvp_regular_spawn.sql` | Table `rathenafr_mvp_list` et vue `rathenafr_mvp_regular_spawn`. |
| `rathenafr_mvp_data.sql` | Catalogue de 202 MVP, dont 61 spawns réguliers. |
| `discord_gmmsg_queue.sql` | Table et migration complète de `discord_gmmsg_queue`. |
| `sql_updates.sql` | Table de compatibilité utilisée par `/db last-update`. |

## Items

`rathenafr_item_search.sql` accepte trois situations :

- installation fraîche ;
- ancienne table matérialisée `rathenafr_item_search` ;
- nouvelle vue `rathenafr_item_search` déjà reliée à `rathenafr_item_list`.

Les lignes existantes sont conservées. Les données trouvées dans `item_db` puis
`item_db_re` sont ajoutées ou actualisées, Renewal ayant la priorité en cas
d’identifiant identique.

> [!CAUTION]
> Le script ne supprime pas automatiquement les items absents des tables
> sources. Cette règle évite d’effacer un catalogue importé depuis les fichiers
> YAML ou des items personnalisés.

## MVP

Exécute toujours le schéma avant les données :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_mvp_regular_spawn.sql
mariadb -u root -p ragnarok < sql/rathenafr_mvp_data.sql
```

Le fichier de données remplace uniquement les lignes portant les sources
`regular_spawn` et `mob_db_mvp_no_regular_spawn`. Les lignes `manual` et les
sources personnalisées sont conservées.

> [!NOTE]
> Les temps de réapparition doivent être adaptés si la configuration de ton
> serveur modifie les spawns officiels.

## GMMSG

`discord_gmmsg_queue.sql` ajoute toutes les colonnes et l’index manquants sur une
ancienne table. Les lignes existantes sont conservées.

> [!CAUTION]
> La migration s’arrête si un ancien message dépasse 180 octets. Corrige ou
> archive ces lignes avant de relancer le script afin d’éviter une troncature.

## Vérification

```bash
mariadb -u rathenafr_bot -p ragnarok < sql/verify-installation.sql
```

Le vérificateur affiche les objets et colonnes attendus sans modifier la base.
