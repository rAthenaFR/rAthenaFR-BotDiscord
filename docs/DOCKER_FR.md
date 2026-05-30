# Docker

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

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

Il utilise le réseau externe `athena-network`, prévu pour communiquer avec les conteneurs Athena existants.

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
