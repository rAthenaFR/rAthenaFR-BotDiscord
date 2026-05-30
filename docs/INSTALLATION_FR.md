# Installation

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

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

## Mise en ligne

Pour un VPS, un serveur dédié ou une machine distante, utilise la procédure dédiée :

```text
docs/DEPLOYMENT_FR.md
```

Le principe recommandé reste Docker, avec une base MariaDB/MySQL joignable par réseau privé ou réseau Docker, jamais exposée publiquement.
