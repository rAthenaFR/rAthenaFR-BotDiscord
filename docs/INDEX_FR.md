# Documentation rAthenaFR Discord Bot

Cette documentation couvre l’installation, l’exploitation et le développement du bot.

> [!NOTE]
> Les fichiers `.env.example` et `.env.docker.example` restent les références exhaustives pour les variables disponibles.

## Installer et exploiter

- [Installation](INSTALLATION_FR.md) : prérequis, préparation SQL et premier lancement.
- [Configuration](CONFIGURATION_FR.md) : variables d’environnement et valeurs par défaut.
- [Commandes Discord](COMMANDS_FR.md) : commandes publiques, staff et permissions.
- [Base de données](DATABASE_FR.md) : scripts SQL, tables et droits minimaux.
- [Gestion des comptes](ACCOUNT_MANAGEMENT_FR.md) : création, modification, ban et désactivation forte.
- [Bridge GMMSG](GMMSG_BRIDGE_FR.md) : file SQL et script NPC rAthena.
- [Déploiement](DEPLOYMENT_FR.md) : Docker Compose, serveur distant et mise à jour.
- [Dépannage](TROUBLESHOOTING_FR.md) : problèmes fréquents et vérifications.

## Maintenir le projet

- [Architecture](ARCHITECTURE_FR.md) : organisation du code et flux d’une interaction.
- [Développement](DEVELOPMENT_FR.md) : workflow, tests, contribution et publication.
- [Sécurité](SECURITY_FR.md) : secrets, rôles, SQL et journalisation.
- [Scripts Windows](../scripts/README_FR.md) : raccourcis PowerShell et CMD.

## Projet

- [Crédits](CREDITS_FR.md) : attribution, licence et projet d’origine.

> [!TIP]
> Après une modification du nom, des options ou de la structure d’une commande slash, redéploie le registre avec `cargo run -- --deploy`.

> [!IMPORTANT]
> Un changement d’embed ou de traduction runtime demande seulement un redémarrage du bot. Un changement du registre Discord demande un redéploiement.
