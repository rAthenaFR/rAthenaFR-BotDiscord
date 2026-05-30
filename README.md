# rAthenaFR Discord Bot

![Rust](https://img.shields.io/badge/rust-2021-B7410E?style=plastic&logo=rust&logoColor=white)
![Discord](https://img.shields.io/badge/discord-bot-5865F2?style=plastic&logo=discord&logoColor=white)
![MariaDB](https://img.shields.io/badge/mariadb-compatible-003545?style=plastic&logo=mariadb&logoColor=white)
![Docker](https://img.shields.io/badge/docker-ready-2496ED?style=plastic&logo=docker&logoColor=white)
![license](https://img.shields.io/badge/license-GPL--3.0--only-A42E2B?style=plastic&logo=gnu&logoColor=white)
![code size](https://img.shields.io/github/languages/code-size/rAthenaFR/rAthenaFR-BotDiscord?style=plastic&logo=github&logoColor=white)

![CI](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/ci.yml?branch=master&label=ci&style=plastic&logo=githubactions&logoColor=white)
![Docker build](https://img.shields.io/github/actions/workflow/status/rAthenaFR/rAthenaFR-BotDiscord/docker.yml?branch=master&label=docker%20build&style=plastic&logo=docker&logoColor=white)

Bot Discord en Rust pour les communautés Ragnarok Online utilisant une base de données compatible rAthena.

Le bot expose des commandes slash en lecture seule pour consulter l’état du serveur, les personnages connectés, les guildes, les classements, les châteaux, les quêtes, le marché et certaines informations réservées au staff.

## Objectif

rAthenaFR Discord Bot est un compagnon Discord pour le projet rAthena. Il ne remplace pas rAthena, FluxCP ou un panel d’administration. Il lit uniquement la base SQL et affiche des informations utiles dans Discord.

## Principes

- accès SQL en lecture seule ;
- aucune modification de compte, personnage, objet ou guilde ;
- commandes staff protégées par rôles Discord ;
- réponses staff éphémères quand Discord le permet ;
- configuration via `.env` ;
- exécution locale ou Docker ;
- compatible MariaDB/MySQL.

## Documentation

| Document | Description |
|---|---|
| [`docs/CONTRIBUTOR_GUIDE_FR.md`](docs/CONTRIBUTOR_GUIDE_FR.md) | Cartographie des contributeurs, flux de projet et processus de gestion des changements sécurisés.|
| [`docs/INSTALLATION_FR.md`](docs/INSTALLATION_FR.md) | Installation et première configuration. |
| [`docs/DEPLOYMENT_FR.md`](docs/DEPLOYMENT_FR.md) | Mise en ligne sur serveur distant ou VPS. |
| [`docs/CONFIGURATION_FR.md`](docs/CONFIGURATION_FR.md) | Variables d'environnement et configuration d'exécution. |
| [`docs/COMMANDS_FR.md`](docs/COMMANDS_FR.md) | Référence complète des commandes. |
| [`docs/DATABASE_FR.md`](docs/DATABASE_FR.md) | Tables de base de données et étendue des requêtes. |
| [`docs/DOCKER_FR.md`](docs/DOCKER_FR.md) | Déploiement Docker. |
| [`docs/SECURITY_FR.md`](docs/SECURITY_FR.md) | Modèle de sécurité et autorisations recommandées. |
| [`docs/TROUBLESHOOTING_FR.md`](docs/TROUBLESHOOTING_FR.md) | Problèmes courants et correctifs. |
| [`docs/DEVELOPMENT_FR.md`](docs/DEVELOPMENT_FR.md) | Flux de travail de développement. |
| [`docs/ARCHITECTURE_FR.md`](docs/ARCHITECTURE_FR.md) | Architecture interne. |
| [`docs/ENV_MIGRATION_FR.md`](docs/ENV_MIGRATION_FR.md) | Migration des anciennes variables d’environnement. |
| [`docs/WINDOWS_APP_CONTROL_FR.md`](docs/WINDOWS_APP_CONTROL_FR.md) | Contournement Windows App Control pour le build local. |
| [`docs/RELEASE_FR.md`](docs/RELEASE_FR.md) | processus de libération. |

## Licence

Ce projet est distribué sous licence GPL-3.0-only. Le projet rAthenaFR doit rester conforme aux lois et licences applicables autour de Ragnarok Online, rAthena, FluxCP et des contenus utilisés.
