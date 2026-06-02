# Configuration

Le projet conserve une configuration par variables d’environnement, chargées depuis l’environnement du processus ou depuis les fichiers `.env`, `.env.example` et `.env.docker.example`.

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

!!! tip "Rôles dédiés"
    Utilise des rôles Discord dédiés au bot plutôt que des rôles trop larges. Cela permet de retirer un accès sans modifier l’organisation globale du serveur Discord.

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

## Tables optionnelles

```env
RATHENAFR_ITEM_DB_TABLE=item_db
RATHENAFR_MOB_DB_TABLE=mob_db
RATHENAFR_OPTIONAL_VENDING_ENABLED=true
RATHENAFR_OPTIONAL_BUYINGSTORE_ENABLED=true
RATHENAFR_OPTIONAL_LOGS_ENABLED=true
```

Les tables item/mob acceptent `item_db` ou `item_db_re`, et `mob_db` ou `mob_db_re`.

## Configuration de `/gmmsg`

```env
RATHENAFR_GMMSG_MODE=disabled
RATHENAFR_GMMSG_MAX_LENGTH=180
RATHENAFR_GMMSG_MIN_ROLE=gm
RATHENAFR_DEBUG_MIN_ROLE=gm
RATHENAFR_AUDIT_MIN_ROLE=admin
```

Modes disponibles pour `/gmmsg` :

- `disabled` : aucun envoi en jeu.
- `test` : réponse et log uniquement.
- `bridge` : utilisation de GameBridge.

!!! warning "Transport GameBridge"
    Aucun transport map-server concret n’est actif par défaut. Le mode `bridge` suppose qu’une implémentation GameBridge opérationnelle est disponible.

`/gmmsg color` valide strictement `RRGGBB`. Les messages sont nettoyés et les mentions `@everyone`/`@here` sont neutralisées dans les logs Discord.

## Création de compte

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=false
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
```

`/createaccount` est conservée et déclarée. Elle refuse la création tant que `RATHENAFR_ACCOUNT_CREATION_ENABLED=false`.

!!! danger "Écriture SQL"
    `/createaccount` est la seule commande conservée qui peut écrire en base. Elle nécessite `INSERT` sur `login` uniquement si elle est activée.

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

Les commandes SQL de cette version sont en lecture seule, sauf `/createaccount` si elle est activée.

## Cache et logs runtime

```env
RATHENAFR_CACHE_ENABLED=true
RATHENAFR_CACHE_TTL_SECONDS=
RUST_LOG=rathenafr_discord_bot=info,info
```
