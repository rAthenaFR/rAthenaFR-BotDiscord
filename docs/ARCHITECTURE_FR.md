# Architecture

Le projet est un binaire Rust asynchrone basé sur Serenity, SQLx et Tokio.

## Organisation

```text
src/
├─ app/                    démarrage, mode --deploy et cycle de vie
├─ cache.rs                cache mémoire court
├─ config/                 lecture et validation de l’environnement
├─ discord/
│  ├─ command_registry/    définition des commandes slash
│  ├─ embeds/              rendu Discord par domaine
│  ├─ interactions/
│  │  └─ dispatcher/       routage, handlers, réponses et validations
│  └─ ui/                  utilitaires d’affichage préparés
├─ i18n/                   locales, clés, loader et traduction
├─ infra/                  chargement .env et observabilité
└─ rathenafr/
   ├─ database.rs          API RAthenaFrDatabase et helpers partagés
   ├─ database/            implémentations SQL par domaine
   ├─ models/              modèles de données
   ├─ game_bridge.rs       abstraction GMMSG
   └─ status.rs            checks TCP des services rAthena
```

> [!NOTE]
> `RAthenaFrDatabase` reste l’API publique stable. Son implémentation est répartie dans les sous-modules `database/`.

## Démarrage

`src/main.rs` appelle `app::run()` :

1. initialisation de `tracing` ;
2. chargement de l’environnement ;
3. lecture de `--deploy` ;
4. déploiement du registre ou connexion SQL ;
5. création du client Serenity ;
6. démarrage de la passerelle Discord.

Le mode `--deploy` ne crée pas de pool SQL.

## Flux d’une interaction

1. Serenity reçoit la commande ou le composant.
2. `dispatcher/router.rs` applique les packs et commandes désactivées.
3. Le handler lit les options et vérifie les rôles.
4. La couche `RAthenaFrDatabase` exécute une requête préparée.
5. `discord/embeds` construit la réponse localisée.
6. Serenity renvoie une réponse publique ou éphémère.

> [!TIP]
> Pour modifier un rendu, commence par le fichier du domaine dans `src/discord/embeds/`. Pour modifier la disponibilité d’une commande, commence par le registre et le routeur.

## Commandes et handlers

Le registre contient seulement la structure Discord : noms, descriptions, groupes et options.

Le dispatcher contient :

- `router.rs` : sélection du handler ;
- `routing.rs` : extraction des options et chemins ;
- `responses.rs` : réponses, limites, cache et tables requises ;
- `validation.rs` : validation comptes et GMMSG ;
- `components.rs` : pagination MVP ;
- `public/` : handlers publics ;
- `staff/` : permissions et handlers sensibles.

## i18n

Les catalogues sont intégrés au binaire avec `include_str!`. `BotLocale::from_discord` normalise la locale et utilise `fr-FR` en fallback.

Les descriptions slash sont localisées au déploiement. Les réponses runtime utilisent la locale de l’interaction.

> [!IMPORTANT]
> Une nouvelle clé doit exister dans les quatre catalogues avec les mêmes variables.

## Base de données

Les modules sont séparés par domaine : connexion, comptes, personnages, marché, items, mobs, MVP, classements, serveur, staff et GMMSG.

Les tables optionnelles sont détectées avant utilisation. Les identifiants dynamiques sont limités à des listes autorisées et les valeurs utilisateur sont liées avec SQLx.

## Cache

Le cache concerne uniquement certaines commandes publiques en lecture seule. Les commandes staff sensibles ne sont pas cacheables.

## Limites de responsabilité

- Le bot n’administre pas le schéma rAthena.
- Les scripts `sql/` sont exécutés manuellement.
- Le bridge GMMSG ne parle pas directement au map-server.
- Les permissions Discord sont vérifiées dans les handlers, pas seulement dans l’interface.
- Les secrets restent dans l’environnement.

> [!WARNING]
> Ne déplace pas une vérification de permission uniquement dans le registre Discord : un utilisateur peut toujours appeler une interaction existante tant que le serveur ne l’a pas redéployée.
