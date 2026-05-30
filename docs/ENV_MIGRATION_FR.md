# Migration des variables d’environnement

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

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
