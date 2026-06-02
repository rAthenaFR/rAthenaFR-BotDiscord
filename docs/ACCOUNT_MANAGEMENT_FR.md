# Gestion des comptes

Cette version conserve uniquement `/createaccount` pour la gestion des comptes.

## Commande conservée

| Commande | Écriture SQL | Accès Discord |
|---|---:|---|
| `/createaccount username: password: sex: birthdate: email:` | `INSERT` dans `login` si la commande est activée | Publique, désactivée par défaut |

`createaccount` n’a pas été renommée. Son comportement reste celui du projet existant :

- `RATHENAFR_ACCOUNT_CREATION_ENABLED=false` refuse la création.
- `RATHENAFR_ACCOUNT_PASSWORD_MODE=plain` ou `md5` contrôle le mode de mot de passe.
- Le mot de passe n’est jamais réaffiché par le bot.
- L’e-mail reste une donnée sensible et ne doit pas être exposé en public, en dehors de la réponse de création prévue.

!!! danger "Données sensibles"
    Même si `/createaccount` est publique, le mot de passe transite par Discord au moment de la commande. Active cette fonctionnalité uniquement si ce flux est accepté pour ton serveur.

## Permissions SQL

Sans création de compte, `SELECT` suffit.

Pour activer `/createaccount`, ajoute uniquement :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
```

!!! warning "Droits SQL"
    Ne donne pas `UPDATE` ou `DELETE` pour cette version. Les anciennes commandes de modification ou de suppression de compte ne sont plus dans le registre Discord.
