# rAthenaFR Discord Bot

![Rust](https://img.shields.io/badge/rust-2021-B7410E?style=plastic&logo=rust&logoColor=white)
![Discord](https://img.shields.io/badge/discord-bot-5865F2?style=plastic&logo=discord&logoColor=white)
![MariaDB](https://img.shields.io/badge/mariadb-compatible-003545?style=plastic&logo=mariadb&logoColor=white)
![Docker](https://img.shields.io/badge/docker-ready-2496ED?style=plastic&logo=docker&logoColor=white)

![CI](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/ci.yml?branch=master&label=ci&style=plastic&logo=githubactions&logoColor=white)
![Rust check](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/ci.yml?branch=master&label=rust%20check&style=plastic&logo=rust&logoColor=white)
![Docker build](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/docker.yml?branch=master&label=docker%20build&style=plastic&logo=docker&logoColor=white)

![last commit](https://img.shields.io/github/last-commit/rAthenaFR/rAthenaFR-BotDiscord/master?style=plastic&logo=github&logoColor=white)
[![issues](https://img.shields.io/github/issues/rAthenaFR/rAthenaFR-BotDiscord?label=issues&style=plastic&logo=github&logoColor=white)](https://github.com/rAthenaFR/rAthenaFR-BotDiscord/issues)
[![pull requests](https://img.shields.io/github/issues-pr/rAthenaFR/rAthenaFR-BotDiscord?style=plastic&logo=github&logoColor=white)](https://github.com/rAthenaFR/rAthenaFR-BotDiscord/pulls)
![code size](https://img.shields.io/github/languages/code-size/rAthenaFR/rAthenaFR-BotDiscord?style=plastic&logo=github&logoColor=white)
![license](https://img.shields.io/badge/license-GPL--3.0--only-A42E2B?style=plastic&logo=gnu&logoColor=white)

Bot Discord en Rust pour les communautés Ragnarok Online utilisant une base de données compatible rAthena.

Le bot expose des commandes slash en lecture seule pour consulter l’état du serveur, les personnages connectés, les guildes, les classements, les châteaux, les quêtes, le marché et certaines informations réservées au staff.

## Objectif

rAthenaFR est un compagnon Discord pour le projet Athena. Il ne remplace pas rAthena, FluxCP ou un panel d’administration. Il lit uniquement la base SQL et affiche des informations utiles dans Discord.

## Principes

- accès SQL en lecture seule ;
- aucune modification de compte, personnage, objet ou guilde ;
- commandes staff protégées par rôles Discord ;
- réponses staff éphémères quand Discord le permet ;
- configuration via `.env` ;
- exécution locale ou Docker ;
- compatible MariaDB/MySQL.

## Commandes principales

| Commande | Rôle |
|---|---|
| `/status` | État des services rAthena et compteurs SQL. |
| `/online` | Personnages connectés. |
| `/top` | Classement par niveau. |
| `/topzeny` | Classement zeny. |
| `/player` | Profil d’un personnage. |
| `/guilds` | Classement des guildes. |
| `/guild` | Détail d’une guilde. |
| `/guildmembers` | Membres d’une guilde. |
| `/classes` | Répartition par classes. |
| `/mapstats` | Répartition par cartes. |
| `/maponline` | Personnages connectés sur une carte. |
| `/party` | Détail d’un groupe. |
| `/partymembers` | Membres d’un groupe. |
| `/homunculus` | Homoncule d’un personnage. |
| `/pet` | Familier d’un personnage. |
| `/castles` | Liste des châteaux. |
| `/castle` | Détail d’un château. |
| `/whosell` | Boutiques vendant un objet. |
| `/whobuy` | Boutiques achetant un objet. |
| `/market` | Vue achat/vente d’un objet. |
| `/venders` | Boutiques de vente actives. |
| `/buyers` | Boutiques d’achat actives. |

Commandes staff : `/charquests`, `/charequipment`, `/charinventory`, `/itemcount`, `/itemowners`, `/accountoverview`, `/banlist`, `/accountchars`, `/accountstatus`.

## Installation rapide

```bash
cp .env.example .env
```

Renseigne au minimum :

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

Déployer les commandes slash :

```bash
cargo run -- --deploy
```

Lancer le bot :

```bash
cargo run
```

## Docker

```bash
cp .env.docker.example .env
docker compose up -d --build
```

Le `docker-compose.yml` utilise le réseau Docker externe `athena-network`. La base de données ne doit pas être exposée publiquement.

## Sécurité

Le bot doit utiliser un utilisateur SQL dédié avec uniquement `SELECT` :

```bash
mysql -u root -p < sql/create-readonly-user.sql
```

Change le mot de passe avant d’exécuter le script SQL.

## Documentation

- `docs/INSTALLATION_FR.md`
- `docs/CONFIGURATION_FR.md`
- `docs/COMMANDS_FR.md`
- `docs/DOCKER_FR.md`
- `docs/DATABASE_FR.md`
- `docs/SECURITY_FR.md`
- `docs/TROUBLESHOOTING_FR.md`
- `docs/ARCHITECTURE_FR.md`
- `docs/DEVELOPMENT_FR.md`
- `docs/CONTRIBUTOR_GUIDE_FR.md`
- `docs/RELEASE_FR.md`

## Licence

Ce projet est distribué sous licence GPL-3.0-only. Le projet rAthenaFR doit rester conforme aux lois et licences applicables autour de Ragnarok Online, rAthena, FluxCP et des contenus utilisés.
