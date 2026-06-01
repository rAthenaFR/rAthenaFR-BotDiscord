# Dépannage

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Commence toujours par les logs du bot : ils indiquent généralement si le problème vient de Discord, MariaDB ou du réseau.

## Le bot ne démarre pas

Vérifie :

```bash
cat .env
```

Les variables obligatoires doivent être renseignées et ne doivent pas rester à `replace_me`.

> [!WARNING]
> Ne colle pas le contenu réel de `.env` dans Discord ou dans un ticket public.

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

> [!TIP]
> Pour `/accountmanage`, vérifie aussi les droits `UPDATE` et `DELETE` décrits dans `ACCOUNT_MANAGEMENT_FR.md`.

## Connexion Docker impossible

Vérifie le réseau :

```bash
docker network ls
docker network inspect athena-network
```

Le conteneur MariaDB et le bot doivent partager `athena-network`.

Si l’erreur contient `failed to lookup address information`, le nom configuré dans `RATHENAFR_DB_HOST`, `RATHENAFR_LOGIN_HOST`, `RATHENAFR_CHAR_HOST` ou `RATHENAFR_MAP_HOST` n’est pas résolu depuis le conteneur du bot. Utilise un nom de service Docker présent sur le même réseau ou une adresse réseau joignable.

> [!CAUTION]
> Dans Docker, `127.0.0.1` désigne le conteneur du bot, pas la machine hôte.

## Les commandes Discord ne changent pas

Redéploie les commandes slash :

```bash
cargo run -- --deploy
```

ou avec Docker :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

> [!IMPORTANT]
> Un changement d’embed ne change pas la définition des commandes slash. Dans ce cas, redémarre seulement le bot.

## Commandes staff refusées

Vérifie que l’utilisateur Discord possède un rôle dont l’ID est dans :

```env
RATHENAFR_STAFF_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

## Table manquante

Le schéma rAthena est probablement incomplet ou personnalisé. Vérifie que les fichiers SQL rAthena ont bien été importés dans la base ciblée.

## `/createaccount` refuse la création

Vérifie :

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=true
```

Puis vérifie que l’utilisateur SQL possède `INSERT` sur la table `login`.

> [!WARNING]
> Si `RATHENAFR_ACCOUNT_PASSWORD_MODE` ne correspond pas au serveur login, les comptes créés peuvent être inutilisables.

## `/accountmanage` refuse l’édition

La commande exige un rôle présent dans `RATHENAFR_STAFF_ROLE_IDS`, `RATHENAFR_ADMIN_ROLE_IDS` ou `RATHENAFR_OWNER_ROLE_IDS`.

Vérifie aussi que l’utilisateur SQL possède `UPDATE` sur la table `login`, par exemple via `sql/create-account-management-user.sql`.

> [!TIP]
> L’action `edit` exige au moins un champ à modifier. Le mot de passe et les valeurs sensibles ne sont pas réaffichés.

## `/accountmanage` refuse la suppression

La commande exige un rôle présent dans `RATHENAFR_STAFF_ROLE_IDS`, `RATHENAFR_ADMIN_ROLE_IDS` ou `RATHENAFR_OWNER_ROLE_IDS`.

Pour supprimer un compte, la confirmation doit être exactement :

```text
DELETE-ALL-ID
```

La suppression est bloquée si un personnage du compte possède une guilde. Transfère ou dissous la guilde avant de supprimer le compte.

Vérifie aussi que l’utilisateur SQL possède `DELETE` sur les tables rAthena concernées, par exemple via `sql/create-account-management-user.sql`.

> [!CAUTION]
> La confirmation attendue est `DELETE-ALL-<account_id>`, par exemple `DELETE-ALL-2000001`.
