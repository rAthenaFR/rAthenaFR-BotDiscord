# Configuration

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Variables Discord

```env
DISCORD_TOKEN=replace_me
DISCORD_CLIENT_ID=replace_me
DISCORD_GUILD_ID=replace_me
```

`DISCORD_GUILD_ID` permet de déployer les commandes slash sur un serveur Discord précis.

`DISCORD_APPLICATION_ID` est optionnel. Si la variable est absente, le bot utilise `DISCORD_CLIENT_ID`.

## Rôles Discord staff

```env
RATHENAFR_STAFF_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Les valeurs sont des IDs de rôles Discord séparés par des virgules. Laisse vide pour refuser les commandes staff.

Les anciens alias `DISCORD_STAFF_ROLE_IDS`, `DISCORD_ADMIN_ROLE_IDS` et `DISCORD_OWNER_ROLE_IDS` restent acceptés si les variables `RATHENAFR_*` correspondantes sont absentes.

## Nom visible

```env
RATHENAFR_DISPLAY_NAME=rAthenaFR
```

Ce nom est utilisé dans le footer des embeds et dans les logs. Les titres et descriptions des commandes affichent `rAthena` par défaut.

## Base de données

```env
RATHENAFR_DB_HOST=127.0.0.1
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=replace_me
RATHENAFR_DB_MAX_CONNECTIONS=5
RATHENAFR_DB_ACQUIRE_TIMEOUT_SECONDS=5
```

Utilise un compte SQL dédié avec uniquement `SELECT`.

## Services rAthena

```env
RATHENAFR_SERVER_HOST=127.0.0.1
RATHENAFR_LOGIN_PORT=6900
RATHENAFR_CHAR_PORT=6121
RATHENAFR_MAP_PORT=5121
```

Des overrides existent : `RATHENAFR_LOGIN_HOST`, `RATHENAFR_CHAR_HOST`, `RATHENAFR_MAP_HOST`.

Sur un serveur distant, ces hôtes doivent être joignables depuis le conteneur du bot. Utilise un réseau Docker partagé, une IP privée, un DNS privé ou un VPN.

## Visibilité et limites

```env
RATHENAFR_HIDE_GM_CHARACTERS=false
RATHENAFR_HIDE_GM_FROM_TOP=true
RATHENAFR_HIDE_GM_GROUP_FROM_RANKING=60
RATHENAFR_DEFAULT_LIMIT=10
RATHENAFR_MAX_LIMIT=25
```

## Cache

```env
RATHENAFR_CACHE_ENABLED=true
RATHENAFR_CACHE_TTL_SECONDS=
```

Laisse `RATHENAFR_CACHE_TTL_SECONDS` vide pour utiliser les valeurs par défaut par commande.

## Logs

```env
RUST_LOG=rathenafr_discord_bot=info,info
```

Si `RUST_LOG` est absent, le bot utilise cette valeur par défaut.
