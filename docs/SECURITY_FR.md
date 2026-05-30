# Sécurité

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Principes

- Bot en lecture seule.
- Secrets dans `.env`, jamais dans Git.
- Utilisateur SQL dédié.
- Droits Discord staff configurés explicitement.
- Pas d’exposition publique de MariaDB.
- Conteneur Docker non-root.

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
