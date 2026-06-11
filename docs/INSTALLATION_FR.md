# Installation

## Prérequis

- Rust stable avec les composants `rustfmt` et `clippy`.
- MariaDB ou MySQL avec une base rAthena existante.
- Une application Discord avec un bot, un token et les IDs application/serveur.
- Git et, pour l’installation conteneurisée, Docker Compose.

> [!NOTE]
> Le bot ne crée pas la base rAthena. Il se connecte à une installation existante.

## Préparer Discord

Dans le portail Discord Developer :

1. crée ou sélectionne une application ;
2. ajoute un bot ;
3. récupère le token ;
4. invite le bot avec les scopes `bot` et `applications.commands` ;
5. récupère l’ID de l’application et l’ID du serveur Discord cible.

Le bot utilise des interactions slash et ne demande aucun intent privilégié.

## Installation locale

Clone le dépôt, puis crée le fichier d’environnement :

```bash
cp .env.example .env
```

Sous PowerShell :

```powershell
Copy-Item .env.example .env
```

Renseigne au minimum :

```env
DISCORD_TOKEN=...
DISCORD_CLIENT_ID=...
DISCORD_GUILD_ID=...
RATHENAFR_DB_HOST=127.0.0.1
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=...
```

> [!WARNING]
> Ne laisse aucune valeur d’exemple et ne commit jamais le fichier `.env`.

## Préparer SQL

Édite le nom de base, l’hôte SQL et le mot de passe dans le script avant exécution :

```bash
mariadb -u root -p ragnarok < sql/create-readonly-user.sql
```

Les scripts optionnels dépendent des fonctionnalités activées :

```bash
mariadb -u root -p ragnarok < sql/rathenafr_item_search.sql
mariadb -u root -p ragnarok < sql/rathenafr_mvp_regular_spawn.sql
mariadb -u root -p ragnarok < sql/sql_updates.sql
mariadb -u root -p ragnarok < sql/discord_gmmsg_queue.sql
mariadb -u root -p ragnarok < sql/create-gmmsg-queue-user.sql
mariadb -u root -p ragnarok < sql/create-account-management-user.sql
```

> [!IMPORTANT]
> `rathenafr_mvp_regular_spawn.sql` crée la table support et la vue de `/mvp list`, mais ne peuple pas `rathenafr_mvp_list`.

Consulte [Base de données](DATABASE_FR.md) avant d’accorder des droits d’écriture.

## Valider et lancer

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Déploie ensuite les commandes dans le serveur Discord configuré :

```bash
cargo run -- --deploy
```

Puis démarre le bot :

```bash
cargo run
```

> [!TIP]
> Sous Windows, `scripts\dev-deploy.ps1` et `scripts\dev-run.ps1` utilisent un dossier Cargo externe pour éviter certains blocages Windows App Control.

## Installation Docker

```bash
cp .env.docker.example .env
docker network create rathena-network
docker compose up -d --build
```

Le réseau `rathena-network` est externe. La commande de création peut signaler qu’il existe déjà, ce qui est sans conséquence.

Déploie les commandes slash depuis l’image :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

> [!CAUTION]
> Dans un conteneur, `127.0.0.1` désigne le conteneur du bot. Utilise le nom du service MariaDB, une IP privée ou un DNS accessible depuis `rathena-network`.

Pour une installation durable sur serveur, continue avec [Déploiement](DEPLOYMENT_FR.md).
