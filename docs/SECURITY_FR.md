# Sécurité

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!WARNING]
> Le bot manipule des données de compte rAthena. Les secrets, mots de passe et tokens ne doivent jamais apparaître dans les embeds, les logs ou Git.

## Principes

- Bot en lecture seule par défaut.
- Secrets dans `.env`, jamais dans Git.
- Utilisateur SQL dédié.
- Droits Discord staff configurés explicitement.
- Pas d’exposition publique de MariaDB.
- Conteneur Docker non-root.
- Écritures SQL désactivées par défaut ; les commandes de compte exigent des droits SQL dédiés.

## Données sensibles interdites dans les embeds

Ne pas afficher :

- `user_pass`
- `email`
- `last_ip`
- `pincode`
- `web_auth_token`
- tokens ou mots de passe

> [!IMPORTANT]
> Même dans une commande staff éphémère, ces champs restent interdits.

## Rôles staff

```env
RATHENAFR_STAFF_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Laisse vide pour refuser les commandes staff.

> [!TIP]
> Utilise des rôles Discord dédiés au bot plutôt que des rôles trop larges.

## Backups

Sauvegarde régulièrement la base rAthena, mais ne stocke pas les backups dans le dépôt du bot.

> [!CAUTION]
> Une suppression complète via `/accountmanage` doit toujours être précédée d’une sauvegarde vérifiable.

## Mise en ligne

- Active le secret scanning et la push protection sur GitHub.
- Active le private vulnerability reporting si le dépôt public appartient à une organisation.
- Garde `.env` uniquement sur le serveur.
- Protège `.env` avec des permissions restrictives, par exemple `chmod 600 .env`.
- N’expose pas MariaDB/MySQL publiquement.
- Fais communiquer le bot avec la base via réseau Docker, réseau privé, VPN ou tunnel privé.

## Commandes de compte

`/createaccount` est désactivée par défaut avec `RATHENAFR_ACCOUNT_CREATION_ENABLED=false`.

Si tu l’actives :

- le mot de passe transite par Discord ;
- la réponse du bot est éphémère ;
- le bot ne réaffiche jamais le mot de passe ;
- l’utilisateur SQL doit avoir `INSERT` sur `login`.
- `RATHENAFR_ACCOUNT_PASSWORD_MODE` doit correspondre au serveur login rAthena (`plain` ou `md5`).

`/accountmanage` est réservé aux rôles GM/staff configurés dans `RATHENAFR_STAFF_ROLE_IDS`, `RATHENAFR_ADMIN_ROLE_IDS` ou `RATHENAFR_OWNER_ROLE_IDS`. L’action `edit` modifie les champs de `login`, et l’action `delete` supprime le compte complet et ses données liées dans une transaction, avec confirmation exacte `DELETE-ALL-ID`.

La suppression est refusée si un personnage du compte possède une guilde. Transfère ou dissous la guilde avant de supprimer le compte.

> [!IMPORTANT]
> L’édition d’un mot de passe via `/accountmanage action:edit` ne réaffiche jamais le mot de passe dans la réponse Discord.

Voir aussi : [Gestion de comptes](ACCOUNT_MANAGEMENT_FR.md).
