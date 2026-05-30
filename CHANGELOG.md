# Changelog

## 0.1.0

Version française complète pour le projet rAthena.

### Modifié

- Renommage complet du projet en `rAthenaFR`.
- Renommage du crate Rust en `rathenafr-discord-bot`.
- Renommage du module interne `src/rathenafr`.
- Renommage des variables d’environnement en `RATHENAFR_*`.
- Renommage de l’utilisateur SQL d’exemple en `rathenafr_bot`.
- Traduction française des descriptions de commandes Discord.
- Traduction française des embeds, erreurs et messages visibles.
- Traduction française de la documentation principale.

### Sécurité

- Conservation du modèle SQL en lecture seule.
- Conservation du principe non-root dans Docker.
- Conservation des commandes staff protégées par rôles Discord.

### Compatibilité

Les noms des commandes slash restent inchangés pour éviter de casser les habitudes Discord existantes. Après mise à jour, redéploie les commandes :

```bash
cargo run -- --deploy
```
