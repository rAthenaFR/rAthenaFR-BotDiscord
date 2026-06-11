# Déploiement

Docker Compose est le mode recommandé pour une instance durable. Le bot n’expose aucun port entrant.

## Préparer le serveur

Installe Docker et Docker Compose, clone le dépôt, puis :

```bash
cp .env.docker.example .env
docker network create rathena-network
```

Configure au minimum Discord et SQL dans `.env`.

> [!CAUTION]
> `.env` contient le token Discord et le mot de passe SQL. Limite ses permissions et ne le copie pas dans une image ou un dépôt.

## Réseau

Le fichier `docker-compose.yml` rejoint le réseau externe `rathena-network`.

Si MariaDB et rAthena sont dans Docker, connecte leurs services au même réseau et utilise leurs noms DNS :

```env
RATHENAFR_DB_HOST=rathena-db
RATHENAFR_LOGIN_HOST=rathena-login
RATHENAFR_CHAR_HOST=rathena-char
RATHENAFR_MAP_HOST=rathena-map
```

> [!WARNING]
> Ne publie pas le port MariaDB `3306` sur Internet. Utilise un réseau Docker ou privé.

## Construire et démarrer

```bash
docker compose build
docker compose up -d
docker compose logs -f rathenafr-discord-bot
```

Le conteneur :

- utilise l’utilisateur non-root `10001` ;
- possède un système de fichiers en lecture seule ;
- active `no-new-privileges` ;
- limite la rotation des logs Docker.

## Déployer le registre Discord

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

> [!IMPORTANT]
> Exécute cette commande après toute modification des commandes, sous-commandes, options ou descriptions slash.

## Mettre à jour

```bash
git pull
docker compose build --pull
docker compose run --rm rathenafr-discord-bot --deploy
docker compose up -d
docker compose logs --tail 100 rathenafr-discord-bot
```

Le redéploiement Discord peut être omis si la mise à jour ne touche pas au registre.

> [!TIP]
> Sous Windows, les scripts `docker-build.ps1`, `docker-deploy.ps1`, `docker-up.ps1` et `docker-logs.ps1` exécutent les mêmes opérations.

## Déploiement sans Docker

Compile :

```bash
cargo build --release
```

Le binaire se trouve dans `target/release/`. Exécute-le avec les variables d’environnement chargées par ton gestionnaire de services.

Un service `systemd` doit au minimum :

- utiliser un utilisateur système sans shell ;
- définir un répertoire de travail lisible ;
- charger les secrets depuis un fichier protégé ;
- redémarrer le processus en cas d’échec ;
- autoriser les connexions sortantes vers Discord et SQL.

## Checklist

- Les quatre commandes Cargo de validation passent.
- `.env` n’est pas suivi par Git.
- L’utilisateur SQL possède seulement les droits nécessaires.
- Le réseau Docker ou privé permet de joindre SQL et les services rAthena.
- Les rôles Discord sont configurés.
- Le registre slash a été redéployé si nécessaire.
- `/server` et `/db health` répondent sans exposer d’adresse sensible.
- Les fonctions d’écriture inutilisées restent désactivées.
