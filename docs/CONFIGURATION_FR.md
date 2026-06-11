# Configuration

Le bot charge les variables du processus et un fichier `.env`. Le chemin peut être remplacé en définissant `RATHENAFR_DISCORD_BOT_ENV` dans l’environnement du processus avant le lancement.

> [!NOTE]
> `.env.example` décrit le développement local et `.env.docker.example` une installation Docker. Ces modèles sont plus exhaustifs que cette page.

## Discord

| Variable | Obligatoire | Défaut | Usage |
|---|---:|---|---|
| `DISCORD_TOKEN` | oui | aucun | Token du bot. |
| `DISCORD_CLIENT_ID` | oui | aucun | ID client, utilisé aussi comme application ID par défaut. |
| `DISCORD_APPLICATION_ID` | non | `DISCORD_CLIENT_ID` | ID d’application explicite. |
| `DISCORD_GUILD_ID` | oui | aucun | Serveur où le registre slash est déployé. |
| `RATHENAFR_DISPLAY_NAME` | non | `rAthenaFR` | Nom affiché dans les embeds et les logs. |
| `RATHENAFR_STAFF_LOG_CHANNEL_ID` | non | vide | Salon des journaux staff. |

## Base et services rAthena

```env
RATHENAFR_DB_HOST=127.0.0.1
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=...
RATHENAFR_DB_MAX_CONNECTIONS=5
RATHENAFR_DB_ACQUIRE_TIMEOUT_SECONDS=5
```

Les checks TCP de `/server` utilisent :

```env
RATHENAFR_SERVER_HOST=127.0.0.1
RATHENAFR_LOGIN_PORT=6900
RATHENAFR_CHAR_PORT=6121
RATHENAFR_MAP_PORT=5121
```

`RATHENAFR_LOGIN_HOST`, `RATHENAFR_CHAR_HOST` et `RATHENAFR_MAP_HOST` permettent de remplacer l’hôte service par service.

## Rôles staff

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Les IDs sont séparés par des virgules. La hiérarchie est cumulative : Owner satisfait tous les niveaux, Admin satisfait Admin et les niveaux inférieurs, etc.

`RATHENAFR_STAFF_ROLE_IDS` reste un alias historique du niveau Helper. Les anciens alias `DISCORD_STAFF_ROLE_IDS`, `DISCORD_ADMIN_ROLE_IDS` et `DISCORD_OWNER_ROLE_IDS` restent lus pour compatibilité.

> [!TIP]
> Laisse un niveau vide pour qu’aucun rôle de ce niveau ne donne accès. Configure des rôles dédiés au bot.

## Packs et commandes

```env
RATHENAFR_PUBLIC_PACK_ENABLED=true
RATHENAFR_STAFF_PACK_ENABLED=true
RATHENAFR_DISABLED_COMMANDS=
RATHENAFR_ONLINE_LIST_PUBLIC=false
RATHENAFR_TOP_ZENY_MODE=enabled
```

- `RATHENAFR_DISABLED_COMMANDS` accepte des chemins séparés par des virgules, par exemple `staff inventory,top zeny`.
- `RATHENAFR_TOP_ZENY_MODE` accepte `enabled`, `anonymized` ou `disabled`.
- `/createaccount` reste enregistré même si le pack public est désactivé, mais son exécution dépend de sa propre option.

## Affichage et cache

```env
RATHENAFR_DEFAULT_LIMIT=10
RATHENAFR_MAX_LIMIT=25
RATHENAFR_HIDE_GM_CHARACTERS=false
RATHENAFR_HIDE_GM_FROM_TOP=true
RATHENAFR_HIDE_GM_GROUP_FROM_RANKING=60
RATHENAFR_CACHE_ENABLED=true
RATHENAFR_CACHE_TTL_SECONDS=
```

Une valeur TTL vide utilise les durées internes par commande. `0` désactive le stockage en cache.

## Comptes

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=false
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
RATHENAFR_ACCOUNT_MANAGE_ENABLED=false
RATHENAFR_ACCOUNT_DELETE_ENABLED=false
RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE=admin
RATHENAFR_ACCOUNT_DELETE_MIN_ROLE=owner
```

Les rôles configurables acceptent `helper`, `moderator`, `mod`, `gm`, `admin` ou `owner`.

> [!CAUTION]
> Les fonctionnalités de compte écrivent dans `login`. Elles sont désactivées par défaut et demandent des droits SQL supplémentaires.

Consulte [Gestion des comptes](ACCOUNT_MANAGEMENT_FR.md).

## Tables et rates

```env
RATHENAFR_ITEM_DB_TABLE=item_db
RATHENAFR_MOB_DB_TABLE=mob_db
RATHENAFR_BATTLE_RATES_CONFIGURED=false
```

Les valeurs acceptées sont `item_db`/`item_db_re` et `mob_db`/`mob_db_re`.

Les variables `RATHENAFR_BATTLE_*` reprennent les rates et bornes de drop du `battle_conf`. Laisse `RATHENAFR_BATTLE_RATES_CONFIGURED=false` tant que toutes les valeurs ne correspondent pas au map-server réellement chargé.

> [!IMPORTANT]
> Si `mob_item_ratio.yml` contient des overrides, active `RATHENAFR_BATTLE_ITEM_RATIO_OVERRIDES=true`. Le bot masquera les taux qu’il ne peut pas calculer correctement.

## GMMSG

```env
RATHENAFR_GMMSG_MODE=disabled
RATHENAFR_GMMSG_ENCODING=windows1252
RATHENAFR_GMMSG_MAX_LENGTH=180
RATHENAFR_GMMSG_MIN_ROLE=gm
RATHENAFR_DEBUG_MIN_ROLE=gm
RATHENAFR_AUDIT_MIN_ROLE=admin
```

`RATHENAFR_GMMSG_MODE` accepte `disabled`, `test`, `sql_queue` ou `bridge`. Le mode `bridge` est réservé mais ne possède actuellement aucune implémentation map-server active.

> [!WARNING]
> En `windows1252`, les emojis et les caractères non représentables sont refusés. La limite porte sur les octets encodés.

Consulte [Bridge GMMSG](GMMSG_BRIDGE_FR.md).

## Langues et logs

Les réponses suivent la locale Discord de l’utilisateur pour `fr-FR`, `en-US`, `es-ES`, `de-DE`, `ja-JP`, `ko-KR` et `zh-CN`. Les variantes régionales reconnues sont normalisées, notamment `fr`, `en-GB`, `es-419`, `de`, `ja`, `ko`, `zh-TW` et `zh-HK`. Le fallback est fixé à `fr-FR`.

```env
RUST_LOG=rathenafr_discord_bot=info,info
```

Les variables `RATHENAFR_DEFAULT_LOCALE` et `RATHENAFR_SUPPORTED_LOCALES` présentes dans certains anciens fichiers d’environnement ne pilotent pas le runtime actuel.
