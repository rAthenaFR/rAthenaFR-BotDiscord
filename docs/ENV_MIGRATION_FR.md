# Migration des variables d’environnement

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

## Nouveau préfixe

Toutes les variables propres au bot utilisent maintenant le préfixe :

```text
RATHENAFR_
```

Exemples :

```env
RATHENAFR_DISPLAY_NAME=rAthenaFR
RATHENAFR_DB_HOST=127.0.0.1
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_STAFF_ROLE_IDS=
```

## Fichiers à mettre à jour

- `.env`
- `.env.example`
- `.env.docker.example`
- variables d’environnement Docker/CI
- secrets de déploiement

Après migration, redéploie les commandes Discord si les descriptions ont changé.

Les anciens alias `DISCORD_STAFF_ROLE_IDS`, `DISCORD_ADMIN_ROLE_IDS` et `DISCORD_OWNER_ROLE_IDS` restent acceptés, mais les variables `RATHENAFR_*` sont les noms recommandés.
