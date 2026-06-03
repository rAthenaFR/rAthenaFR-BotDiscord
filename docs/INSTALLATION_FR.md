# Installation

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Pour une installation sur serveur, consulte plutôt `DEPLOYMENT_FR.md`.

## Prérequis

- Rust stable avec Cargo.
- MariaDB ou MySQL accessible par le bot.
- Une base compatible rAthena déjà importée.
- Un bot Discord avec token, client ID et guild ID.

## Installation locale

```bash
cp .env.example .env
```

Renseigne les variables obligatoires :

```env
DISCORD_TOKEN=replace_me
DISCORD_CLIENT_ID=replace_me
DISCORD_GUILD_ID=replace_me
RATHENAFR_DB_HOST=127.0.0.1
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=replace_me
```

> [!WARNING]
> Remplace toutes les valeurs `replace_me` avant de lancer le bot.

## Préparation SQL

Crée l’utilisateur SQL du bot avec le script adapté :

```bash
mariadb -u root -p ragnarok < sql/create-readonly-user.sql
```

Installe les scripts optionnels selon les fonctionnalités activées :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_item_search.sql
mariadb -u root -p ragnarok < sql/rathenafr_mvp_regular_spawn.sql
mariadb -u root -p ragnarok < sql/discord_gmmsg_queue.sql
mariadb -u root -p ragnarok < sql/create-gmmsg-queue-user.sql
mariadb -u root -p ragnarok < sql/create-account-management-user.sql
```

> [!IMPORTANT]
> `sql/rathenafr_item_search.sql` crée et rafraîchit la table utilisée par `/item info` depuis `item_db` et/ou `item_db_re`.

> [!IMPORTANT]
> `sql/rathenafr_mvp_regular_spawn.sql` crée la table support et la vue de `/mvp list`, mais ne peuple pas `rathenafr_mvp_list`. Importe ensuite les données MVP Athena validées pour ton serveur.

Déploie les commandes slash :

```bash
cargo run -- --deploy
```

Lance le bot :

```bash
cargo run
```

## Installation Docker

```bash
cp .env.docker.example .env
docker compose up -d --build
```

Le service attend le réseau Docker externe `athena-network`.

> [!TIP]
> Crée le réseau avec `docker network create athena-network` s’il n’existe pas encore.

## Mise en ligne

Pour un VPS, un serveur dédié ou une machine distante, utilise la procédure dédiée :

```text
docs/DEPLOYMENT_FR.md
```

Le principe recommandé reste Docker, avec une base MariaDB/MySQL joignable par réseau privé ou réseau Docker, jamais exposée publiquement.

> [!IMPORTANT]
> Les commandes de compte nécessitent des permissions SQL supplémentaires. Consulte `ACCOUNT_MANAGEMENT_FR.md` avant de les activer.

## Option `/gmmsg` en jeu

Pour envoyer `/gmmsg` en jeu, configure `RATHENAFR_GMMSG_MODE=sql_queue`, installe la table `discord_gmmsg_queue`, puis charge le script NPC rAthena correspondant dans `npc/scripts_custom.conf`.

> [!IMPORTANT]
> La colonne `discord_gmmsg_queue.message` doit être en `VARBINARY(180)` pour conserver les octets Windows-1252 nécessaires aux accents français côté client Ragnarok Online.

Consulte [Bridge GMMSG SQL Queue](GMMSG_BRIDGE_FR.md) pour la procédure complète.
