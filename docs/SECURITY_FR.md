# Sécurité

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Principes

- Bot en lecture seule par défaut.
- Secrets dans `.env`, jamais dans Git.
- Utilisateur SQL dédié.
- Droits Discord staff configurés explicitement.
- Pas d’exposition publique de MariaDB.
- Conteneur Docker non-root.
- Écritures SQL limitées à `login` uniquement si les commandes de compte sont activées.

## Données sensibles interdites dans les embeds

Ne pas afficher :

- `user_pass`
- `email`
- `last_ip`
- `pincode`
- `web_auth_token`
- tokens ou mots de passe

## Rôles staff

```env
RATHENAFR_STAFF_ROLE_IDS=
RATHENAFR_ADMIN_ROLE_IDS=
RATHENAFR_OWNER_ROLE_IDS=
```

Laisse vide pour refuser les commandes staff.

## Backups

Sauvegarde régulièrement la base rAthena, mais ne stocke pas les backups dans le dépôt du bot.

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

`/accountmanage` est réservé aux rôles `RATHENAFR_OWNER_ROLE_IDS`. L’action `delete` supprime uniquement les comptes sans personnage ni stockage et exige une confirmation exacte `DELETE-ID`.
