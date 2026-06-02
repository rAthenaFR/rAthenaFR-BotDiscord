# Gestion des comptes

Cette version conserve `/createaccount` et ajoute une commande staff sensible sous
`/staff account-manage`. La gestion staff est désactivée par défaut et doit être
activée explicitement par configuration.

## Commandes disponibles

| Commande | Écriture SQL | Accès Discord |
|---|---:|---|
| `/createaccount username: password: sex: birthdate: email:` | `INSERT` dans `login` si la commande est activée | Publique, désactivée par défaut |
| `/staff account-manage edit account: field: value: reason:` | `UPDATE` ciblé dans `login` | Staff, Admin par défaut |
| `/staff account-manage ban account: until: reason:` | `UPDATE` ciblé dans `login` | Staff, Admin par défaut |
| `/staff account-manage unban account: reason:` | `UPDATE` ciblé dans `login` | Staff, Admin par défaut |
| `/staff account-manage delete account_id: confirm: reason:` | Désactivation forte dans `login`, sans suppression physique | Staff, Owner par défaut, désactivée par défaut |

`createaccount` n’a pas été renommée. Son comportement reste celui du projet existant :

- `RATHENAFR_ACCOUNT_CREATION_ENABLED=false` refuse la création.
- `RATHENAFR_ACCOUNT_PASSWORD_MODE=plain` ou `md5` contrôle le mode de mot de passe.
- Le mot de passe n’est jamais réaffiché par le bot.
- L’e-mail reste une donnée sensible et ne doit pas être exposé en public, en dehors de la réponse de création prévue.

> [!WARNING]
> **Données sensibles**
>
> Même si `/createaccount` est publique, le mot de passe transite par Discord au moment de la commande.
> Active cette fonctionnalité uniquement si ce flux est accepté pour ton serveur.

## Configuration staff

```env
RATHENAFR_ACCOUNT_MANAGE_ENABLED=false
RATHENAFR_ACCOUNT_DELETE_ENABLED=false
RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE=admin
RATHENAFR_ACCOUNT_DELETE_MIN_ROLE=owner
```

`RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE` protège `edit`, `ban` et `unban`.
`RATHENAFR_ACCOUNT_DELETE_MIN_ROLE` protège `delete`.

Les rôles acceptés sont `helper`, `moderator`, `gm`, `admin` et `owner`.
Par défaut, `delete` demande Owner et reste désactivée même si la commande
globale de gestion est activée.

## Champs modifiables

`edit` accepte uniquement ces colonnes de `login`, si elles existent dans le
schéma ciblé :

- `group_id`
- `state`
- `unban_time`
- `expiration_time`
- `logincount`
- `sex`

Les champs suivants sont toujours refusés : `account_id`, `userid`,
`user_pass`, `password`, `hash`, `email`, `pincode`, `last_ip` et `lastlogin`.

Le bot ne réaffiche pas de secret de compte. Les réponses et logs n'incluent
pas `user_pass`, hash, PIN, IP privée ou e-mail privé.

## Ban et unban

`ban` identifie le compte par `account_id` exact ou `userid` exact. Si `state`
existe, le compte est passé en état bloqué. Si `unban_time` existe, l'option
`until` permet de définir un timestamp UNIX de fin de ban.

`unban` remet `state` à `0` et `unban_time` à `0` quand ces colonnes existent.

Chaque action réussie est journalisée dans `RATHENAFR_STAFF_LOG_CHANNEL_ID` si
le salon est configuré.

## Delete

`delete` est volontairement conservée comme désactivation forte :

- elle exige `account_id` exact ;
- elle exige `confirm="SUPPRIMER"` exactement ;
- elle ne supprime pas physiquement la ligne `login` ;
- elle applique `state = 5`, `group_id = 0`, `expiration_time = 0` et remet
  `unban_time = 0` si la colonne existe.

Cette approche évite de casser les dépendances rAthena liées à `char`,
`storage`, guild ownership, historiques et logs. Une suppression physique d'un
compte rAthena peut rompre l'intégrité des données et doit être réalisée hors
bot avec une procédure SQL dédiée et vérifiée.

## Permissions SQL

Sans création ni gestion staff, `SELECT` suffit.

Pour activer `/createaccount`, ajoute uniquement :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

Pour activer `/staff account-manage`, ajoute les droits minimaux nécessaires :

```sql
GRANT SELECT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT SELECT ON `ragnarok`.`char` TO 'rathenafr_bot'@'%';
GRANT UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```
