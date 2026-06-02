# rAthenaFR Discord Bot

![Rust](https://img.shields.io/badge/rust-2021-B7410E?style=plastic&logo=rust&logoColor=white)
![Discord](https://img.shields.io/badge/discord-bot-5865F2?style=plastic&logo=discord&logoColor=white)
![MariaDB](https://img.shields.io/badge/mariadb-compatible-003545?style=plastic&logo=mariadb&logoColor=white)
![Docker](https://img.shields.io/badge/docker-ready-2496ED?style=plastic&logo=docker&logoColor=white)
![license](https://img.shields.io/badge/license-GPL--3.0--only-A42E2B?style=plastic&logo=gnu&logoColor=white)
![code size](https://img.shields.io/github/languages/code-size/rAthenaFR/rAthenaFR-BotDiscord?style=plastic&logo=github&logoColor=white)

![CI](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/ci.yml?branch=master&label=ci&style=plastic&logo=githubactions&logoColor=white)
![Docker build](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/docker.yml?branch=master&label=docker%20build&style=plastic&logo=docker&logoColor=white)

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/rAthenaFR/rAthenaFR-BotDiscord)

Bot Discord en Rust pour les communautés Ragnarok Online utilisant une base de données compatible rAthena.

Le bot expose des commandes slash en lecture seule pour consulter l’état du serveur, les personnages connectés, les guildes, les classements, les châteaux, les quêtes, le marché et certaines informations réservées au staff.

## Objectif

rAthenaFR Discord Bot est un compagnon Discord pour le projet rAthena. Il ne remplace pas rAthena, FluxCP ou un panel d’administration. Il lit uniquement la base SQL et affiche des informations utiles dans Discord.

## Principes

- accès SQL en lecture seule par défaut ;
- aucune modification de personnage, objet ou guilde ;
- création/suppression de compte uniquement si les commandes dédiées sont activées ;
- commandes staff protégées par rôles Discord ;
- réponses staff éphémères quand Discord le permet ;
- configuration via `.env` ;
- exécution locale ou Docker ;
- compatible MariaDB/MySQL.

> [!WARNING]
> Les commandes Discord de **rAthenaFR** sont actuellement en phase de prototype.
> Certaines commandes peuvent être modifiées, renommées ou supprimées dans de futures versions.

## Documentation

> [!NOTE]
> Le point d’entrée recommandé est [`docs/INDEX_FR.md`](docs/INDEX_FR.md).

| Document | Description |
|---|---|
| [`docs/ACCOUNT_MANAGEMENT_FR.md`](docs/ACCOUNT_MANAGEMENT_FR.md) | Liste GM, création, édition et suppression complète de comptes. |
| [`docs/ARCHITECTURE_FR.md`](docs/ARCHITECTURE_FR.md) | Architecture interne. |
| [`docs/COMMANDS_FR.md`](docs/COMMANDS_FR.md) | Référence complète des commandes. |
| [`docs/CONFIGURATION_FR.md`](docs/CONFIGURATION_FR.md) | Variables d'environnement et configuration d'exécution. |
| [`docs/CONTRIBUTOR_GUIDE_FR.md`](docs/CONTRIBUTOR_GUIDE_FR.md) | Guide pour ajouter ou modifier des commandes. |
| [`docs/DATABASE_FR.md`](docs/DATABASE_FR.md) | Tables utilisées et permissions SQL. |
| [`docs/DEPLOYMENT_FR.md`](docs/DEPLOYMENT_FR.md) | Mise en ligne sur serveur distant ou VPS. |
| [`docs/DEVELOPMENT_FR.md`](docs/DEVELOPMENT_FR.md) | Flux de travail de développement. |
| [`docs/DOCKER_FR.md`](docs/DOCKER_FR.md) | Exécution avec Docker Compose. |
| [`docs/ENV_MIGRATION_FR.md`](docs/ENV_MIGRATION_FR.md) | Migration des anciennes variables d’environnement. |
| [`docs/INSTALLATION_FR.md`](docs/INSTALLATION_FR.md) | Installation et première configuration. |
| [`docs/RELEASE_FR.md`](docs/RELEASE_FR.md) | Checklist de publication. |
| [`docs/SECURITY_FR.md`](docs/SECURITY_FR.md) | Sécurité, secrets et autorisations recommandées. |
| [`docs/TROUBLESHOOTING_FR.md`](docs/TROUBLESHOOTING_FR.md) | Problèmes courants et correctifs. |
| [`docs/WINDOWS_APP_CONTROL_FR.md`](docs/WINDOWS_APP_CONTROL_FR.md) | Contournement Windows App Control pour le build local. |

## Licence

Ce projet est distribué sous licence GPL-3.0-only. Le projet rAthenaFR doit rester conforme aux lois et licences applicables autour de Ragnarok Online, rAthena, FluxCP et des contenus utilisés.
