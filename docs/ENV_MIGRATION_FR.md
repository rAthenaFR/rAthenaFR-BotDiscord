# Migration des variables d’environnement

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Les variables `RATHENAFR_*` sont les noms recommandés pour toutes les options propres au bot.

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
RATHENAFR_ACCOUNT_CREATION_ENABLED=false
RATHENAFR_ACCOUNT_PASSWORD_MODE=plain
```

## Fichiers à mettre à jour

- `.env`
- `.env.example`
- `.env.docker.example`
- variables d’environnement Docker/CI
- secrets de déploiement

Après migration, redéploie les commandes Discord si les descriptions ont changé.

Les anciens alias `DISCORD_STAFF_ROLE_IDS`, `DISCORD_ADMIN_ROLE_IDS` et `DISCORD_OWNER_ROLE_IDS` restent acceptés, mais les variables `RATHENAFR_*` sont les noms recommandés.

> [!WARNING]
> Ne conserve pas deux valeurs divergentes pour un même rôle. Si `RATHENAFR_STAFF_ROLE_IDS` existe, utilise-le comme source de vérité.
