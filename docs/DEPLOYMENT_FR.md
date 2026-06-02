# Mise en ligne

## Préparation

Le bot n’a pas besoin de port entrant public. Il doit joindre Discord, MariaDB/MySQL et, pour les checks serveur de `/server`, les ports login/char/map si tu les configures.

> [!TIP]
> **Fichier d’environnement Docker**
>
> ```bash
> cp .env.docker.example .env
> ```

> [!CAUTION]
> **Secrets**
>
> Ne commit jamais `.env`.
> Ce fichier contient le token Discord et les identifiants SQL.

## Discord

```env
DISCORD_TOKEN=replace_me
DISCORD_CLIENT_ID=replace_me
DISCORD_GUILD_ID=replace_me
```

Ajoute les rôles staff selon tes besoins :

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

## SQL

Lecture seule :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Option `/createaccount` :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

## Déployer les commandes

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

Refais cette commande après tout changement de commande slash. Elle retire aussi les anciennes commandes hors scope du serveur Discord.

> [!IMPORTANT]
> **Après la refonte**
>
> Le redéploiement est nécessaire pour que Discord remplace réellement les anciens registres par les nouveaux packs publics et staff.

## Démarrer

```bash
docker compose up -d --build
docker compose logs -f rathenafr-discord-bot
```

## Checklist

- `.env` est absent de Git.
- `DISCORD_TOKEN` est valide.
- `DISCORD_GUILD_ID` correspond au serveur Discord cible.
- La base SQL est joignable.
- L’utilisateur SQL est limité à `SELECT`, avec `INSERT` sur `login` seulement si `/createaccount` est activée.
- Les rôles staff sont configurés.
- `cargo run -- --deploy` ou l’équivalent Docker a été exécuté après la refonte des commandes.