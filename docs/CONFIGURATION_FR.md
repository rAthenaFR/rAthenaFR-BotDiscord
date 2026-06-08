# Configuration

Le projet conserve une configuration par variables d’environnement, chargées depuis l’environnement du processus ou depuis un fichier `.env`. Les fichiers `.env.example` et `.env.docker.example` sont des modèles à copier puis adapter.

## Discord

```env
DISCORD_TOKEN=replace_me
DISCORD_CLIENT_ID=replace_me
DISCORD_GUILD_ID=replace_me
DISCORD_APPLICATION_ID=
```

`DISCORD_APPLICATION_ID` est optionnel. S’il est absent, le bot utilise `DISCORD_CLIENT_ID`.

## Rôles staff

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
RATHENAFR_STAFF_LOG_CHANNEL_ID=
```

Les valeurs sont des IDs Discord séparés par des virgules. Les anciens alias `RATHENAFR_STAFF_ROLE_IDS`, `DISCORD_STAFF_ROLE_IDS`, `DISCORD_ADMIN_ROLE_IDS` et `DISCORD_OWNER_ROLE_IDS` restent acceptés pour compatibilité.

> [!TIP]
> **Rôles dédiés**
>
> Utilise des rôles Discord dédiés au bot plutôt que des rôles trop larges.
> Cela permet de retirer un accès sans modifier l’organisation globale du serveur Discord.

## Packs et commandes

```env
RATHENAFR_PUBLIC_PACK_ENABLED=true
RATHENAFR_STAFF_PACK_ENABLED=true
RATHENAFR_DISABLED_COMMANDS=
RATHENAFR_ONLINE_LIST_PUBLIC=false
RATHENAFR_TOP_ZENY_MODE=enabled
RATHENAFR_DEFAULT_LIMIT=10
RATHENAFR_MAX_LIMIT=25
```

`RATHENAFR_DISABLED_COMMANDS` accepte des chemins séparés par des virgules, par exemple `staff inventory,top zeny`.

`RATHENAFR_TOP_ZENY_MODE` accepte :

- `enabled`
- `anonymized`
- `disabled`

## Tables rAthena

```env
RATHENAFR_ITEM_DB_TABLE=item_db
RATHENAFR_MOB_DB_TABLE=mob_db
```

Les tables item/mob acceptent `item_db` ou `item_db_re`, et `mob_db` ou `mob_db_re`.

## Rates EXP et drops

Les tables SQL rAthena contiennent les valeurs de base. Pour que `/mob info` et
`/mob drops` appliquent les rates du map-server, recopie les paramètres actifs
de `conf/import/battle_conf.txt` dans les variables
`RATHENAFR_BATTLE_*` fournies dans `.env.example`.

```env
RATHENAFR_BATTLE_RATES_CONFIGURED=true
RATHENAFR_BATTLE_BASE_EXP_RATE=1500
RATHENAFR_BATTLE_JOB_EXP_RATE=1500
RATHENAFR_BATTLE_MVP_EXP_RATE=1000
RATHENAFR_BATTLE_ITEM_RATE_COMMON=500
RATHENAFR_BATTLE_ITEM_RATE_COMMON_BOSS=300
RATHENAFR_BATTLE_ITEM_RATE_MVP=200
```

Laisse `RATHENAFR_BATTLE_RATES_CONFIGURED=false` tant que les valeurs ne sont
pas synchronisées avec le map-server. Dans ce cas, le bot masque les
pourcentages ajustés au lieu d’afficher les taux SQL bruts comme taux serveur.

Si `db/import/mob_item_ratio.yml` contient des overrides, configure
`RATHENAFR_BATTLE_ITEM_RATIO_OVERRIDES=true`. Les taux sont alors masqués, car
ces overrides ne peuvent pas être déduits de la base SQL.

Les taux affichés restent antérieurs aux modificateurs propres au joueur :
pénalité de niveau Renewal, VIP, LUK, équipement, effets temporaires et partage
d’EXP. `/mvp last` affiche pour sa part l’EXP réellement attribuée et enregistrée
dans `mvplog`.

## Configuration de `/gmmsg`

```env
RATHENAFR_GMMSG_MODE=disabled
RATHENAFR_GMMSG_ENCODING=windows1252
RATHENAFR_GMMSG_MAX_LENGTH=180
RATHENAFR_GMMSG_MIN_ROLE=gm
RATHENAFR_DEBUG_MIN_ROLE=gm
RATHENAFR_AUDIT_MIN_ROLE=admin
```

Modes disponibles pour `/gmmsg` :

- `disabled` : aucun envoi en jeu.
- `test` : réponse et log uniquement.
- `sql_queue` : insertion dans `discord_gmmsg_queue`, puis traitement par un script NPC rAthena.

Encodages disponibles :

- `windows1252` : valeur recommandée pour le client Ragnarok Online et les accents français.
- `utf8` : réservé aux installations qui le supportent explicitement côté NPC/client.

> [!IMPORTANT]
> Le mode `sql_queue` nécessite la table `discord_gmmsg_queue`, la colonne `message` en `VARBINARY(180)` et un script NPC rAthena chargé côté serveur.

> [!WARNING]
> Les emojis et les caractères hors Windows-1252 sont refusés avec `RATHENAFR_GMMSG_ENCODING=windows1252`.

`/gmmsg color` valide strictement `RRGGBB`. Les messages sont nettoyés et les mentions `@everyone`/`@here` sont neutralisées dans les logs Discord.

Pour le détail complet, consulte [Bridge GMMSG SQL Queue](GMMSG_BRIDGE_FR.md).

## Création de compte

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=false
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
RATHENAFR_ACCOUNT_MANAGE_ENABLED=false
RATHENAFR_ACCOUNT_DELETE_ENABLED=false
RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE=admin
RATHENAFR_ACCOUNT_DELETE_MIN_ROLE=owner
```

`/createaccount` est conservée et déclarée. Elle refuse la création tant que `RATHENAFR_ACCOUNT_CREATION_ENABLED=false`.

`/staff account-manage` est déclarée mais refuse toute action tant que
`RATHENAFR_ACCOUNT_MANAGE_ENABLED=false`. Les sous-commandes `edit`, `ban` et
`unban` utilisent `RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE`, avec `admin` par défaut.

`delete` utilise `RATHENAFR_ACCOUNT_DELETE_MIN_ROLE`, avec `owner` par défaut,
et reste refusée tant que `RATHENAFR_ACCOUNT_DELETE_ENABLED=false`.
La commande demande aussi `confirm="SUPPRIMER"` exactement.

> [!CAUTION]
> **Écriture SQL**
>
> `/createaccount` et `/staff account-manage` peuvent écrire en base uniquement
> si elles sont activées explicitement. `account-manage delete` applique une
> désactivation forte et ne supprime pas physiquement la ligne `login`.

## Base de données et services

```env
RATHENAFR_DB_HOST=127.0.0.1
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=replace_me
RATHENAFR_DB_MAX_CONNECTIONS=5
RATHENAFR_DB_ACQUIRE_TIMEOUT_SECONDS=5
RATHENAFR_SERVER_HOST=127.0.0.1
RATHENAFR_LOGIN_PORT=6900
RATHENAFR_CHAR_PORT=6121
RATHENAFR_MAP_PORT=5121
```

Les commandes SQL de cette version sont en lecture seule, sauf `/createaccount`
et `/staff account-manage` si elles sont activées explicitement.

## Cache et logs runtime

```env
RATHENAFR_CACHE_ENABLED=true
RATHENAFR_CACHE_TTL_SECONDS=
RUST_LOG=rathenafr_discord_bot=info,info
```
