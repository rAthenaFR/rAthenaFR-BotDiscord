# Gestion des comptes

Le projet fournit `/createaccount` et le groupe staff `/staff account-manage`. Toutes ces écritures sont désactivées par défaut.

> [!WARNING]
> Un mot de passe fourni à `/createaccount` transite par Discord. N’active cette commande que si ce risque est accepté et documenté pour ta communauté.

## Création publique

```env
RATHENAFR_ACCOUNT_CREATION_ENABLED=false
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
```

`RATHENAFR_ACCOUNT_PASSWORD_MODE` accepte `plain` ou `md5` selon le schéma rAthena ciblé.

La commande :

```text
/createaccount username: password: sex: birthdate: [email]
```

valide les longueurs et formats, insère le compte dans `login` et ne réaffiche jamais le mot de passe.

> [!CAUTION]
> Vérifie le mode de mot de passe attendu par ton serveur avant d’activer la commande. Un mauvais mode peut créer des comptes inutilisables.

## Gestion staff

```env
RATHENAFR_ACCOUNT_MANAGE_ENABLED=false
RATHENAFR_ACCOUNT_DELETE_ENABLED=false
RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE=admin
RATHENAFR_ACCOUNT_DELETE_MIN_ROLE=owner
```

| Commande | Effet |
|---|---|
| `/staff account-manage edit account: field: value: [reason]` | Met à jour un champ autorisé. |
| `/staff account-manage ban account: [until] [reason]` | Bloque le compte et définit éventuellement une fin UNIX. |
| `/staff account-manage unban account: [reason]` | Remet `state` et `unban_time` à zéro. |
| `/staff account-manage delete account_id: confirm: [reason]` | Applique une désactivation forte. |

`edit`, `ban` et `unban` utilisent `RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE`. `delete` utilise `RATHENAFR_ACCOUNT_DELETE_MIN_ROLE` et sa propre option d’activation.

## Champs modifiables

`edit` autorise uniquement :

- `group_id`
- `state`
- `unban_time`
- `expiration_time`
- `logincount`
- `sex`

Les identifiants, mots de passe, hashes, PIN, e-mail, dernière IP et dernier login ne peuvent pas être modifiés par cette commande.

> [!IMPORTANT]
> Les recherches de compte sont exactes : `account_id` positif ou `userid` exact. Le bot n’effectue pas de recherche partielle avant une écriture.

## Désactivation forte

`delete` :

- exige un `account_id` exact ;
- exige `confirm:SUPPRIMER` ;
- ne supprime aucune ligne SQL ;
- applique un état bloqué et retire les privilèges du compte ;
- conserve les relations avec les personnages, guildes, storages et logs.

Une suppression physique doit rester une opération administrative externe, préparée avec sauvegarde et vérification des dépendances.

## Droits SQL

Création uniquement :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

Gestion staff :

```sql
GRANT UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

Le droit `SELECT` normal reste nécessaire pour `login`, `char` et les autres lectures.

Le script `sql/create-account-management-user.sql` accorde le jeu combiné `SELECT`, `INSERT login` et `UPDATE login`.

> [!TIP]
> Pour le principe du moindre privilège, accorde manuellement seulement `INSERT` ou `UPDATE` si une seule des deux fonctionnalités est activée.

## Audit

Les actions et refus sont envoyés dans `RATHENAFR_STAFF_LOG_CHANNEL_ID` lorsqu’il est configuré. Les réponses sont éphémères et les secrets ne sont pas inclus.
