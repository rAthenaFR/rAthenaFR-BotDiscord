# Dépannage

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Le bot ne démarre pas

Vérifie :

```bash
cat .env
```

Les variables obligatoires doivent être renseignées et ne doivent pas rester à `replace_me`.

## Erreur `Access denied`

Cause probable : mauvais mot de passe SQL ou droits insuffisants.

À vérifier :

```env
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=...
```

Côté MariaDB :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
FLUSH PRIVILEGES;
```

## Connexion Docker impossible

Vérifie le réseau :

```bash
docker network ls
docker network inspect athena-network
```

Le conteneur MariaDB et le bot doivent partager `athena-network`.

Si l’erreur contient `failed to lookup address information`, le nom configuré dans `RATHENAFR_DB_HOST`, `RATHENAFR_LOGIN_HOST`, `RATHENAFR_CHAR_HOST` ou `RATHENAFR_MAP_HOST` n’est pas résolu depuis le conteneur du bot. Utilise un nom de service Docker présent sur le même réseau ou une adresse réseau joignable.

## Les commandes Discord ne changent pas

Redéploie les commandes slash :

```bash
cargo run -- --deploy
```

ou avec Docker :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

## Commandes staff refusées

Vérifie que l’utilisateur Discord possède un rôle dont l’ID est dans :

```env
RATHENAFR_STAFF_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

## Table manquante

Le schéma rAthena est probablement incomplet ou personnalisé. Vérifie que les fichiers SQL rAthena ont bien été importés dans la base ciblée.
