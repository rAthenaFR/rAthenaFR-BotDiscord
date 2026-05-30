# Sécurité

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

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

Sauvegarde régulièrement la base Athena, mais ne stocke pas les backups dans le dépôt du bot.
