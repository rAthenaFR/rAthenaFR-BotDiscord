# Migration depuis une ancienne version Go

Documentation française de rAthenaFR Discord Bot pour le projet Athena.

## Principes conservés

- Accès SQL en lecture seule.
- Commandes Discord orientées communauté.
- Pas de modification de données serveur.

## Différences

La version Rust privilégie une architecture plus stricte : configuration validée, cache mémoire court, embeds centralisés et séparation claire entre Discord et SQL.

## Commandes non reprises

Les commandes qui ne lisent pas de données natives fiables dans la base rAthena ne sont pas incluses par défaut.
