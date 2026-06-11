# Dépannage

## Le bot ne démarre pas

Vérifie d’abord :

```bash
cargo check --workspace
cargo run
```

Les variables obligatoires sont `DISCORD_TOKEN`, `DISCORD_CLIENT_ID` ou `DISCORD_APPLICATION_ID`, `DISCORD_GUILD_ID` et les variables `RATHENAFR_DB_*`.

> [!NOTE]
> Le mode `--deploy` ne se connecte pas à SQL, mais charge tout de même la configuration Discord et les options générales.

## Le fichier `.env` n’est pas trouvé

Le bot cherche le fichier selon son environnement. Pour imposer un chemin, définis avant le lancement :

```powershell
$env:RATHENAFR_DISCORD_BOT_ENV="C:\chemin\vers\.env"
cargo run
```

Dans Docker Compose, le chemin est `/app/.env`.

## Les commandes Discord sont absentes ou anciennes

```bash
cargo run -- --deploy
```

ou :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

Vérifie `DISCORD_GUILD_ID`, le token, l’ID d’application et les options `RATHENAFR_PUBLIC_PACK_ENABLED`/`RATHENAFR_STAFF_PACK_ENABLED`.

> [!TIP]
> Les commandes de guilde sont mises à jour rapidement. Un mauvais `DISCORD_GUILD_ID` est une cause fréquente de registre vide.

## Connexion SQL refusée

Vérifie l’hôte depuis le même environnement que le bot :

```bash
mariadb -h HOST -P 3306 -u rathenafr_bot -p ragnarok -e "SELECT 1;"
```

Dans Docker, `127.0.0.1` pointe vers le conteneur du bot.

> [!CAUTION]
> Ne corrige pas une erreur d’accès en accordant tous les privilèges. Applique les droits minimaux décrits dans [Base de données](DATABASE_FR.md).

## Une table manque

Utilise `/db health` avec un rôle Owner. Selon la fonctionnalité :

- réexécute `rathenafr_item_search.sql` pour recréer `rathenafr_item_list` et la vue `rathenafr_item_search` ;
- exécute `rathenafr_mvp_regular_spawn.sql`, puis `rathenafr_mvp_data.sql` pour `/mvp list` ;
- exécute `discord_gmmsg_queue.sql` pour créer ou compléter la file `/gmmsg` ;
- active les logs rAthena nécessaires aux commandes staff ;
- installe `sql_updates.sql` seulement si cette table de compatibilité est souhaitée.

Contrôle ensuite tous les objets fournis par le projet :

```bash
mariadb -u root -p ragnarok < sql/verify-installation.sql
```

> [!NOTE]
> Le vérificateur signale les objets attendus par les scripts du bot. Il ne valide pas toutes les tables natives de rAthena.

## Une commande staff est refusée

Vérifie :

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Les IDs doivent être numériques, séparés par des virgules. Le membre doit exécuter la commande dans le serveur configuré afin que Discord fournisse ses rôles.

## `/createaccount` ou `account-manage` est refusée

Vérifie les options d’activation et les droits :

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=true
RATHENAFR_ACCOUNT_MANAGE_ENABLED=true
RATHENAFR_ACCOUNT_DELETE_ENABLED=false
```

`/createaccount` demande `INSERT login`; `account-manage` demande `UPDATE login`. Consulte [Gestion des comptes](ACCOUNT_MANAGEMENT_FR.md).

## `/gmmsg` reste en attente

Si la ligne reste `pending` :

1. vérifie que le script NPC est chargé ;
2. consulte les logs du map-server ;
3. vérifie `npc/scripts_custom.conf` ;
4. teste une annonce `server` simple ;
5. vérifie que le compte SQL de rAthena peut mettre la file à jour.

Si les accents sont incorrects, contrôle `VARBINARY(180)` et `RATHENAFR_GMMSG_ENCODING=windows1252`.

> [!IMPORTANT]
> Un statut `done` signifie que le script NPC a marqué la ligne comme traitée. Il ne garantit pas que le client affiche l’annonce si les flags ou la map sont incorrects.

Si la migration SQL refuse de modifier la colonne `message`, recherche d’abord les anciennes lignes de plus de 180 octets :

```sql
SELECT `id`, OCTET_LENGTH(`message`) AS `message_bytes`
FROM `discord_gmmsg_queue`
WHERE OCTET_LENGTH(`message`) > 180;
```

> [!WARNING]
> N’augmente pas automatiquement la limite et ne tronque pas les messages en production. Archive ou corrige les lignes concernées, puis relance `discord_gmmsg_queue.sql`.

## Docker ne démarre pas

Si le réseau manque :

```bash
docker network create rathena-network
```

Puis :

```bash
docker compose config
docker compose up -d --build
docker compose logs --tail 200 rathenafr-discord-bot
```

## Windows bloque le binaire

Utilise le dossier Cargo externe employé par les scripts :

```powershell
$env:CARGO_TARGET_DIR="$env:LOCALAPPDATA\Athena\rathenafr-discord-bot\target"
cargo run
```

Ferme les processus Rust avant de nettoyer les anciens dossiers de build. Le script `scripts\clean-local-build.ps1` automatise ce nettoyage.

> [!WARNING]
> Ce script supprime `target`, `dist` et le target externe du projet. Ne l’exécute pas pendant un build.
