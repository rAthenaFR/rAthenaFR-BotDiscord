# Index de la documentation

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Cet index est le point d’entrée recommandé pour naviguer dans `docs/`. Le `README.md` racine reste volontairement séparé.

## Démarrage

- [Installation](INSTALLATION_FR.md) : installation locale et premier lancement.
- [Configuration](CONFIGURATION_FR.md) : variables d’environnement prises en charge.
- [Docker](DOCKER_FR.md) : exécution avec Docker Compose.
- [Mise en ligne](DEPLOYMENT_FR.md) : déploiement sur serveur ou VPS.

## Utilisation

- [Commandes Discord](COMMANDS_FR.md) : liste des commandes publiques et staff.
- [Bridge GMMSG SQL Queue](GMMSG_BRIDGE_FR.md) : envoi `/gmmsg`, file SQL, encodage Windows-1252 et script NPC rAthena.
- [Gestion de comptes](ACCOUNT_MANAGEMENT_FR.md) : liste GM, création, édition et suppression complète de comptes.
- [Base de données](DATABASE_FR.md) : tables utilisées et permissions SQL.
- [Dépannage](TROUBLESHOOTING_FR.md) : erreurs courantes et corrections.

## Maintenance

- [Développement](DEVELOPMENT_FR.md) : commandes locales et règles de contribution.
- [Guide contributeur](CONTRIBUTOR_GUIDE_FR.md) : ajout propre d’une commande.
- [Architecture](ARCHITECTURE_FR.md) : organisation du code.
- [Sécurité](SECURITY_FR.md) : règles de sécurité, secrets et permissions.
- [Publication](RELEASE_FR.md) : checklist de release.

## Annexes

- [Migration des variables d’environnement](ENV_MIGRATION_FR.md).
- [Windows App Control](WINDOWS_APP_CONTROL_FR.md).

> [!TIP]
> Après tout changement de nom, description ou option de commande slash, redéploie les commandes Discord avec `cargo run -- --deploy` ou l’équivalent Docker.
