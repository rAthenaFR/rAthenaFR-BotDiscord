# Commandes Discord

Les commandes sont enregistrées dans le serveur défini par `DISCORD_GUILD_ID`. Leurs descriptions et la majorité des réponses sont localisées selon la langue Discord de l’utilisateur.

> [!NOTE]
> Une notation comme `/guild members` désigne la commande `/guild` avec la sous-commande `members`.

## Commandes publiques

| Commande | Description |
|---|---|
| `/server` | État SQL, compteurs rAthena et disponibilité TCP login/char/map. |
| `/online count` | Nombre de personnages connectés. |
| `/online list [limit]` | Liste publique si `RATHENAFR_ONLINE_LIST_PUBLIC=true`. |
| `/online map [limit]` | Répartition des connectés par map. |
| `/player name:` | Profil public d’un personnage. |
| `/guild info name:` | Informations d’une guilde. |
| `/guild members name: [limit]` | Membres visibles d’une guilde. |
| `/castle list [limit]` | Châteaux et propriétaires. |
| `/castle info castle_id:` | Détail d’un château. |
| `/item info item:` | Fiche d’un item par ID, nom ou AegisName. |
| `/item search text: [limit]` | Recherche partielle d’items. |
| `/who-drops item: [limit]` | Monstres qui donnent l’item. |
| `/mob info mob:` | Fiche d’un monstre. |
| `/mob drops mob: [limit]` | Drops d’un monstre. |
| `/mvp list [limit]` | Panneau paginé des MVP réguliers. |
| `/mvp last [limit]` | Derniers kills présents dans `mvplog`. |
| `/mvp top [limit]` | Classement compact depuis `mvplog`. |
| `/top level [limit]` | Classement par base level. |
| `/top job [limit]` | Classement par job level. |
| `/top guild [limit]` | Classement des guildes. |
| `/top zeny [limit]` | Classement zeny selon la configuration. |
| `/rank name:` | Position d’un personnage dans les classements. |
| `/market info item:` | Résumé ventes et achats. |
| `/market sell item: [limit]` | Boutiques de vente actives. |
| `/market buy item: [limit]` | Buying stores actifs. |
| `/createaccount username: password: sex: birthdate: [email]` | Création de compte, désactivée par défaut. |

> [!IMPORTANT]
> `/createaccount` est indépendante du pack public. Elle reste déclarée lorsque `RATHENAFR_PUBLIC_PACK_ENABLED=false`, mais refuse toute création tant que sa propre option est désactivée.

## Hiérarchie staff

La hiérarchie est `Helper < Moderator < GM < Admin < Owner`. Un rôle supérieur satisfait un niveau inférieur.

| Commande | Niveau minimum par défaut | Description |
|---|---:|---|
| `/staff player character:` | Helper | Profil complet d’un personnage. |
| `/staff account character:` | Helper | Statut du compte lié, sans secret. |
| `/staff chars lookup:` | Helper | Personnages d’un compte ou d’un personnage. |
| `/staff inventory character: [limit]` | GM | Inventaire. |
| `/staff equipment character: [limit]` | GM | Équipement porté. |
| `/staff cart character: [limit]` | GM | Chariot. |
| `/staff storage character: [limit]` | GM | Storage du compte. |
| `/staff guildstorage guild: [limit]` | GM | Storage de guilde. |
| `/staff whohas item: [limit]` | GM | Propriétaires d’un item. |
| `/staff item-search item: [limit]` | GM | Même recherche ciblée dans les conteneurs. |
| `/staff zeny character:` | GM | Zeny du personnage. |
| `/staff zenylog character: [limit]` | GM | Historique zeny si disponible. |
| `/staff picklog character: [limit]` | GM | Historique objets si disponible. |
| `/staff trade-log character: [limit]` | GM | Vue des échanges depuis `picklog`. |
| `/staff mvp-log character: [limit]` | GM | Historique MVP du personnage. |
| `/staff loginlog character: [limit]` | Admin | Historique de connexion. |
| `/staff ip-accounts character: [limit]` | Admin | Contexte de comptes avec IP masquées. |
| `/staff multiaccount character: [limit]` | Admin | Contexte multi-compte. |
| `/staff banned [limit]` | Admin | Comptes bloqués ou bannis. |

Les sous-commandes `/staff account-manage` utilisent les niveaux configurés dans `RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE` et `RATHENAFR_ACCOUNT_DELETE_MIN_ROLE`.

## Modération, audit et diagnostic

| Commande | Niveau minimum | Description |
|---|---:|---|
| `/mod chatlog character: [limit]` | Moderator | Messages récents du personnage. |
| `/mod chat-search text: [limit]` | Moderator | Recherche dans `chatlog`. |
| `/mod report-context character: [limit]` | Moderator | Position et contexte de chat. |
| `/mod branchlog character: [limit]` | Moderator | Historique Dead/Bloody Branch. |
| `/debug quest character: [limit]` | Configurable, GM | Quêtes du personnage. |
| `/debug char-vars character: [limit]` | Configurable, GM | Variables personnage. |
| `/debug acc-vars character: [limit]` | Configurable, GM | Variables compte. |
| `/audit atcommands gm: [limit]` | Configurable, Admin | Commandes utilisées par un GM. |
| `/audit item-created [limit]` | Configurable, Admin | Lignes récentes de `picklog`. |
| `/audit zeny-created [limit]` | Configurable, Admin | Lignes récentes de `zenylog`. |
| `/audit gm-activity gm: [limit]` | Configurable, Admin | Activité depuis `atcommandlog`. |
| `/db health` | Owner | État des tables importantes. |
| `/db tables [limit]` | Owner | Tables rAthena détectées. |
| `/db count` | Owner | Compteurs de tables utiles. |
| `/db logs-size` | Owner | Taille des tables de logs. |
| `/db last-update [limit]` | Owner | Dernières entrées de `sql_updates`. |

Les réponses sensibles sont éphémères.

## GMMSG

| Commande | Description |
|---|---|
| `/gmmsg server message:` | Annonce globale. |
| `/gmmsg map map: message:` | Annonce sur une map. |
| `/gmmsg blue message:` | Annonce bleue. |
| `/gmmsg color hex: message:` | Annonce couleur, format `RRGGBB`. |
| `/gmmsg test message:` | Validation et journalisation sans envoi. |

Le niveau minimum est configurable avec `RATHENAFR_GMMSG_MIN_ROLE`.

> [!WARNING]
> Les commandes basées sur des tables optionnelles répondent proprement si la table manque, mais ne peuvent pas produire de données.

## Activer, désactiver et redéployer

Les packs sont contrôlés par :

```env
RATHENAFR_PUBLIC_PACK_ENABLED=true
RATHENAFR_STAFF_PACK_ENABLED=true
RATHENAFR_DISABLED_COMMANDS=staff inventory,top zeny
```

Après une modification du registre :

```bash
cargo run -- --deploy
```

> [!TIP]
> Un simple changement de texte runtime, d’embed ou de requête ne demande pas de redéploiement Discord.
