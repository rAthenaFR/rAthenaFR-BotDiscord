# Docker

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Démarrage

```bash
cp .env.docker.example .env
docker compose up -d --build
```

## Services

Le compose fournit un service unique :

```yaml
rathenafr-discord-bot
```

Il utilise le réseau externe `athena-network`, prévu pour communiquer avec les conteneurs rAthena existants.

## Bonnes pratiques

- Ne publie pas MariaDB sur Internet.
- Garde `.env` hors Git.
- Utilise un utilisateur SQL en lecture seule.
- Laisse `read_only: true` et `no-new-privileges:true`.
- Consulte les logs avec :

```bash
docker compose logs -f rathenafr-discord-bot
```

## Déployer les commandes via Docker

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

## Serveur distant

Sur un serveur online, le conteneur n’a pas besoin de port public. Il doit seulement pouvoir sortir vers Discord et joindre MariaDB/MySQL ainsi que les services rAthena configurés.

Si le réseau externe n’existe pas encore :

```bash
docker network create athena-network
```

Si MariaDB n’est pas dans Docker, configure `RATHENAFR_DB_HOST` avec une IP privée, un DNS privé ou un tunnel réseau. Ne publie pas le port `3306` sur Internet.

Voir aussi :

```text
docs/DEPLOYMENT_FR.md
```
