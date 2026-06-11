# Développement

## Environnement

```bash
cp .env.example .env
cargo check --workspace
```

Le toolchain stable installe `rustfmt` et `clippy` via `rust-toolchain.toml`.

> [!NOTE]
> L’état du code, des scripts SQL, de `.env.example` et des catalogues `locales/` prime sur une ancienne description documentaire.

## Validation obligatoire

Exécute dans cet ordre :

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Raccourcis :

```bash
make ci
```

```powershell
.\scripts\ci.ps1
```

## Lancer et déployer

```bash
cargo run
cargo run -- --deploy
```

Sous Windows :

```powershell
.\scripts\dev-run.ps1
.\scripts\dev-deploy.ps1
```

> [!TIP]
> Redéploie uniquement quand le registre slash change. Les modifications de handlers, SQL, embeds ou traductions runtime demandent un redémarrage.

## Ajouter ou modifier une commande

1. Définis la commande dans `src/discord/command_registry/public.rs` ou `staff.rs`.
2. Ajoute les clés de description dans les quatre catalogues `locales/*.ftl`.
3. Route la commande dans `src/discord/interactions/dispatcher/router.rs`.
4. Place le handler dans `dispatcher/public/` ou `dispatcher/staff/`.
5. Ajoute ou réutilise une méthode de `RAthenaFrDatabase` dans `src/rathenafr/database/`.
6. Ajoute les modèles dans `src/rathenafr/models/`.
7. Construit la réponse dans `src/discord/embeds/`.
8. Ajoute des tests proportionnés au risque.
9. Mets à jour la documentation et redéploie le registre.

> [!IMPORTANT]
> Les handlers staff sensibles doivent répondre en éphémère, vérifier un niveau de rôle et rester hors du cache public.

## Internationalisation

- `fr-FR` est le fallback.
- Les catalogues pris en charge sont `fr-FR`, `en-US`, `es-ES` et `de-DE`.
- Utilise `I18nKey` pour les textes récurrents ou sensibles.
- Les helpers raw `tr_raw`, `ts` et `tsa` restent réservés aux clés de catalogue non encore typées.
- Une clé avec variables doit fournir les mêmes variables dans les quatre catalogues.

Les tests de `src/i18n/tests.rs` vérifient la couverture et les placeholders.

> [!WARNING]
> Ne réintroduis pas une réponse Discord en français en dur lorsqu’une clé i18n est raisonnable. Les logs techniques et les contextes SQL internes peuvent rester en français.

## SQL et sécurité

- Préfère les requêtes préparées SQLx.
- Ne concatène jamais une valeur utilisateur dans une requête.
- Vérifie l’existence des tables optionnelles.
- N’ajoute aucune écriture sans option d’activation, rôle, audit, droits minimaux et documentation.
- Ne journalise pas les tokens, mots de passe, hashes, PIN, e-mails privés ou IP complètes.

## Publication

Avant une release :

1. exécute la validation complète ;
2. mets à jour `CHANGELOG.md` et la version du package si nécessaire ;
3. vérifie `.env.example` et `.env.docker.example` ;
4. construis avec `cargo build --release` et `docker compose build` ;
5. vérifie les liens Markdown et l’absence de secrets ;
6. teste le déploiement des commandes sur un serveur Discord de développement.

Sous Windows :

```powershell
.\scripts\build-release.ps1
```

Le binaire est copié dans `dist/`.

## Revue de contribution

- Le diff reste limité au besoin.
- Les commandes publiques et permissions existantes ne changent pas sans justification.
- Le schéma SQL n’est pas modifié implicitement par le bot.
- Les nouvelles variables sont documentées et ajoutées aux deux exemples `.env`.
- Les erreurs visibles sont localisées.
- Les tests couvrent les nouveaux chemins.

> [!CAUTION]
> Ne supprime pas une compatibilité publique ou un script SQL sans vérifier ses références dans le code, la documentation et les procédures de déploiement.
