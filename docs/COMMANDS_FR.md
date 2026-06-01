# Commandes Discord

Documentation française de rAthenaFR Discord Bot pour le projet rAthena.

> [!NOTE]
> Les réponses publiques utilisent des embeds. Les commandes staff répondent en éphémère quand elles exposent des informations sensibles.

## Commandes publiques

| Commande | Description |
|---|---|
| `/status` | État des services et compteurs SQL. |
| `/online` | Personnages connectés. |
| `/top` | Classement par niveau. |
| `/player name:` | Profil d’un personnage. |
| `/guilds` | Classement des guildes. |
| `/search query: category:` | Recherche par catégorie : tout, joueurs, items ou monstres. |
| `/createaccount username: password: sex: birthdate: email:` | Création de compte si activée. |
| `/topzeny` | Classement zeny. |
| `/guild name:` | Détail d’une guilde. |
| `/guildmembers name:` | Membres d’une guilde. |
| `/classes` | Répartition par classes. |
| `/mapstats online_only:` | Répartition par cartes. |
| `/maponline map:` | Personnages connectés sur une carte. |
| `/party name:` | Détail d’un groupe. |
| `/partymembers name:` | Membres d’un groupe. |
| `/homunculus character:` | Homoncule d’un personnage. |
| `/pet character:` | Familier d’un personnage. |
| `/zeny` | Statistiques zeny visibles. |
| `/castles` | Liste des châteaux. |
| `/castle castle_id:` | Détail d’un château. |
| `/guildalliances name:` | Alliances et oppositions d’une guilde. |
| `/guildskills name:` | Compétences d’une guilde. |
| `/homunculustop` | Classement des homoncules. |
| `/pettop` | Classement des familiers. |
| `/queststats quest_id:` | Statistiques d’une quête. |
| `/whosell item_id:` | Vendeurs d’un objet. |
| `/whobuy item_id:` | Acheteurs d’un objet. |
| `/market item_id:` | Résumé du marché. |
| `/venders` | Boutiques de vente actives. |
| `/buyers` | Boutiques d’achat actives. |

La recherche d’objets et de monstres utilise les tables SQL `item_db`, `item_db_re`, `mob_db` et `mob_db_re` quand elles existent dans la base cible.

> [!TIP]
> `/search` accepte `category` avec les choix `Tout`, `Joueurs`, `Items` et `Monstres`. Choisir une catégorie évite de requêter les autres sources.

## Commandes staff

Ces commandes exigent un rôle présent dans `RATHENAFR_STAFF_ROLE_IDS`, `RATHENAFR_ADMIN_ROLE_IDS` ou `RATHENAFR_OWNER_ROLE_IDS`.

> [!WARNING]
> `/accountmanage` peut éditer ou supprimer un compte. L’action `delete` supprime le compte complet et exige une confirmation exacte `DELETE-ALL-ID`. Consulte `ACCOUNT_MANAGEMENT_FR.md` avant usage.

| Commande | Description |
|---|---|
| `/charquests` | Quêtes d’un personnage. |
| `/charequipment` | Équipement d’un personnage. |
| `/charinventory` | Inventaire d’un personnage. |
| `/itemcount` | Comptage global d’un objet. |
| `/itemowners` | Propriétaires visibles d’un objet. |
| `/accountoverview` | Résumé sûr d’un compte. |
| `/accountmanage action:edit ...` | Édition des champs autorisés d’un compte par un GM/staff. |
| `/accountmanage action:delete confirm:DELETE-ALL-ID` | Suppression complète d’un compte par un GM/staff. |
| `/banlist` | Comptes bloqués ou bannis. |
| `/accountchars` | Personnages d’un compte. |
| `/accountstatus` | Statut sûr d’un compte. |

## Déploiement des commandes

Après tout changement de nom, description ou option :

```bash
cargo run -- --deploy
```

Avec Docker :

```bash
docker compose run --rm rathenafr-discord-bot --deploy
```

> [!IMPORTANT]
> Modifier uniquement le rendu des embeds ne nécessite pas de redéploiement des commandes slash. Un rebuild/redémarrage du bot suffit.
