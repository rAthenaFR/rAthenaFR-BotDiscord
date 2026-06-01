# Architecture

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Le bot est majoritairement en lecture seule. Les seules écritures prévues sont les commandes de compte explicitement documentées.

## Structure principale

```text
src/
  app/                 Démarrage, déploiement et cycle de vie du bot.
  cache.rs             Cache mémoire court.
  config/              Lecture et validation de la configuration.
  discord/             Commandes, interactions et embeds Discord.
  infra/               Chargement .env et observabilité.
  rathenafr/           Accès SQL, modèles et checks services.
```

## Flux d’une commande

1. Discord reçoit une commande slash.
2. `dispatcher.rs` route vers le handler adapté.
3. Le handler valide les options et les rôles si nécessaire.
4. `database.rs` exécute la requête SQL adaptée.
5. `embeds/mod.rs` construit la réponse française.
6. Discord reçoit un embed public ou éphémère.

> [!TIP]
> Le rendu Discord est centralisé dans `src/discord/embeds/mod.rs`. Pour améliorer l’affichage de toutes les commandes, commence par ce fichier plutôt que par chaque handler.

## Séparation des responsabilités

- `command_registry` : définition des commandes Discord.
- `interactions` : logique de routage.
- `embeds` : affichage utilisateur.
- `rathenafr/database.rs` : requêtes SQL.
- `rathenafr/models.rs` : structures partagées.
- `rathenafr/status.rs` : checks TCP login/char/map.

> [!IMPORTANT]
> Les commandes staff doivent répondre en éphémère et ne doivent jamais afficher de secrets ou de données sensibles.

## Ajout propre d’une commande

1. Ajouter la définition dans `src/discord/command_registry`.
2. Ajouter la méthode SQL en lecture seule dans `src/rathenafr/database.rs`.
3. Ajouter ou réutiliser un modèle dans `src/rathenafr/models.rs`.
4. Ajouter l’embed français dans `src/discord/embeds/mod.rs`.
5. Router dans `src/discord/interactions/dispatcher.rs`.
6. Déployer les commandes Discord.
