# Mise en ligne

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!WARNING]
> Ne déploie jamais le bot avec un fichier `.env` incomplet, un token Discord de test ou une base MariaDB exposée publiquement.

## Objectif

Cette page décrit un déploiement hors poste local, par exemple sur un VPS, un serveur dédié ou la même machine Docker que rAthena.

Le bot n’a pas besoin de port entrant public. Il doit seulement pouvoir sortir vers Discord et joindre la base MariaDB/MySQL ainsi que les ports login, char et map si tu veux que `/status` vérifie les services.

## Architecture recommandée

- Bot Discord dans un conteneur Docker.
- Base MariaDB/MySQL non exposée à Internet.
- Accès SQL en lecture seule par défaut avec l’utilisateur `rathenafr_bot`.
- Communication avec la base via réseau Docker, IP privée, VPN ou tunnel privé.
- Fichier `.env` présent uniquement sur le serveur.

> [!NOTE]
> Le bot n’a pas besoin d’un port HTTP public. Il initie lui-même la connexion à Discord.

## Préparer le serveur

Installe au minimum Git, Docker et Docker Compose.

```bash
git clone https://github.com/rAthenaFR/rAthenaFR-BotDiscord.git
cd rAthenaFR-BotDiscord
cp .env.docker.example .env
```

Garde le fichier `.env` lisible uniquement par l’utilisateur qui administre le bot :

```bash
chmod 600 .env
```

> [!IMPORTANT]
> Ne stocke pas `.env` dans Git ni dans une image Docker.

## Configurer Discord

Renseigne :

```env
DISCORD_TOKEN=replace_me
DISCORD_CLIENT_ID=replace_me
DISCORD_GUILD_ID=replace_me
```

`DISCORD_APPLICATION_ID` est optionnel. Si la variable est absente, le bot utilise `DISCORD_CLIENT_ID`.

Le bot utilise des commandes slash et des interactions Discord. Aucun intent privilégié n’est nécessaire dans le code actuel.

Si tu veux autoriser les commandes staff, ajoute les IDs de rôles Discord :

```env
RATHENAFR_STAFF_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

La création publique de compte reste désactivée tant que tu ne définis pas :

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=true
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
```

Cette option nécessite des droits SQL supplémentaires sur `login`. Ne l’active pas avec l’utilisateur SQL strictement lecture seule.

Utilise `RATHENAFR_ACCOUNT_PASSWORD_MODE=md5` uniquement si ton serveur login rAthena attend des mots de passe MD5 dans `login.user_pass`.

> [!CAUTION]
> Si tu veux aussi utiliser `/accountmanage`, applique les permissions décrites dans `docs/ACCOUNT_MANAGEMENT_FR.md` et vérifie tes sauvegardes.

## Configurer la base distante

Si la base est dans Docker sur le même hôte, utilise le nom du conteneur ou du service :

```env
RATHENAFR_DB_HOST=rathena-db
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=replace_me
```

Si la base est sur un autre serveur, utilise une IP privée ou un nom DNS privé. N’expose pas MariaDB publiquement.

> [!WARNING]
> Un port MariaDB ouvert sur Internet est une erreur de déploiement. Utilise un réseau privé, Docker, VPN ou tunnel privé.

Crée ou vérifie l’utilisateur SQL en lecture seule :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
FLUSH PRIVILEGES;
```

Remplace `%` par l’hôte exact du bot si ton infrastructure le permet.

## Configurer le réseau Docker

Le `docker-compose.yml` attend un réseau externe nommé `athena-network`.

Si rAthena et MariaDB tournent déjà sur ce réseau, garde ce nom. Sinon, crée-le avant le démarrage :

```bash
docker network create athena-network
```

Les conteneurs que le bot doit joindre doivent être connectés au même réseau Docker, ou bien les variables `RATHENAFR_DB_HOST`, `RATHENAFR_LOGIN_HOST`, `RATHENAFR_CHAR_HOST` et `RATHENAFR_MAP_HOST` doivent pointer vers des adresses joignables depuis le conteneur du bot.

## Déployer les commandes

Déploie les commandes slash sur le serveur Discord configuré :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

Refais cette commande après tout changement de nom, description ou option de commande.

> [!TIP]
> Les changements de rendu des embeds nécessitent seulement un rebuild/redémarrage du bot.

## Démarrer le bot

```bash
docker compose up -d --build
```

Vérifie les logs :

```bash
docker compose logs -f rathenafr-discord-bot
```

Le démarrage est correct quand les logs indiquent une connexion validée à la base de données et un shard Discord prêt.

## Mettre à jour

```bash
git pull
docker compose build
docker compose run --rm rathenafr-discord-bot --deploy
docker compose up -d
```

Si seules les commandes Discord ont changé, le `--deploy` est nécessaire. Si seul le code d’exécution change, le rebuild et le redémarrage suffisent.

## Checklist online

- `.env` présent sur le serveur et absent de Git.
- `DISCORD_TOKEN` réel et non expiré.
- `DISCORD_GUILD_ID` correct.
- Base SQL joignable depuis le conteneur.
- Utilisateur SQL limité à `SELECT`.
- MariaDB non exposée publiquement.
- Réseau Docker ou réseau privé configuré.
- `docker compose run --rm rathenafr-discord-bot --deploy` exécuté au moins une fois.
- `docker compose up -d --build` exécuté.
