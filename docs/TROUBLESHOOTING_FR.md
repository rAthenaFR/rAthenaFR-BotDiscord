# Dépannage

## Les commandes Discord ne changent pas

Redéploie les commandes slash :

```bash
cargo run -- --deploy
```

ou avec Docker :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

> [!NOTE]
> **Registre Discord**
>
> Les anciennes commandes sont retirées du registre uniquement après redéploiement.

## Erreur SQL

Vérifie la connexion et les droits :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
```

Si `/createaccount` est activée :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

> [!CAUTION]
> **Droits inutiles**
>
> N’ajoute pas `UPDATE` ou `DELETE` pour cette version.

## Table manquante

Utilise `/db health` avec un rôle Owner. La commande liste les tables présentes, les tables manquantes, les tables optionnelles et les logs détectés.

## Commandes staff refusées

Vérifie les IDs de rôles :

```env
RATHENAFR_HELPER_ROLE_IDS=
RATHENAFR_MODERATOR_ROLE_IDS=
RATHENAFR_GM_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Les commandes Admin ne sont pas autorisées par un rôle GM seul.

## `/createaccount` refuse la création

Vérifie :

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=true
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
```

Puis vérifie que l’utilisateur SQL possède `INSERT` sur `login`.

## `/gmmsg` ne diffuse pas en jeu

Par défaut :

```env
RATHENAFR_GMMSG_MODE=disabled
```

Passe en `test` pour valider la commande sans envoi.

Passe en `sql_queue` pour envoyer les messages via la file SQL rAthena :

```env
RATHENAFR_GMMSG_MODE=sql_queue
RATHENAFR_GMMSG_ENCODING=windows1252
```

> [!IMPORTANT]
> Le mode `sql_queue` nécessite la table `discord_gmmsg_queue`, sa colonne `message` en `VARBINARY(180)` et un script NPC rAthena chargé depuis `npc/scripts_custom.conf`.

> [!TIP]
> Si les accents français s’affichent mal en jeu, vérifie que `discord_gmmsg_queue.message` est bien en `VARBINARY(180)` et que `RATHENAFR_GMMSG_ENCODING=windows1252`.