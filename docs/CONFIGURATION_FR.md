# Configuration

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

## Variables Discord

```env
DISCORD_TOKEN=replace_me
DISCORD_CLIENT_ID=replace_me
DISCORD_GUILD_ID=replace_me
```

`DISCORD_GUILD_ID` permet de déployer les commandes slash sur un serveur Discord précis.

## Nom visible

```env
RATHENAFR_DISPLAY_NAME=rAthenaFR
```

Ce nom est utilisé dans les titres et footers des embeds.

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
