# Commandes Discord

Cette page décrit les packs de commandes publics et staff de la première version de release.

> [!NOTE]
> **Sous-commandes Discord**
>
> Les commandes écrites avec un espace sont des sous-commandes Discord.
> Par exemple, `/guild members` correspond à la commande racine `/guild` avec la sous-commande `members`.

## Commandes publiques

| Commande | Description |
|---|---|
| `/server` | Résumé serveur : comptes, personnages, guildes, joueurs en ligne et services. |
| `/online count` | Nombre de joueurs connectés. |
| `/online list` | Liste des joueurs connectés si `RATHENAFR_ONLINE_LIST_PUBLIC=true`. |
| `/online map` | Répartition des joueurs connectés par map. |
| `/player name:` | Profil public d’un personnage. |
| `/guild info name:` | Informations publiques d’une guilde. |
| `/guild members name:` | Membres publics d’une guilde. |
| `/castle list` | Châteaux et propriétaires. |
| `/castle info castle_id:` | Détail d’un château. |
| `/item info item:` | Fiche item par nom ou ID. |
| `/item search text:` | Recherche d’items par nom partiel. |
| `/who-drops item:` | Monstres qui drop un item. |
| `/mob info mob:` | Fiche monstre par nom ou ID. |
| `/mob drops mob:` | Drops d’un monstre. |
| `/mvp list` | Panneau paginé des MVP réguliers depuis `rathenafr_mvp_regular_spawn`. |
| `/mvp last` | Dernières lignes de `mvplog` si la table est disponible. |
| `/mvp top` | Vue compacte depuis `mvplog` si la table est disponible. |
| `/top level` | Classement par base level. |
| `/top job` | Classement par job level. |
| `/top guild` | Classement des guildes. |
| `/top zeny` | Classement zeny selon `RATHENAFR_TOP_ZENY_MODE`. |
| `/rank name:` | Positions publiques d’un personnage. |
| `/market info item:` | Résumé achat/vente si les tables market existent. |
| `/market sell item:` | Ventes actives. |
| `/market buy item:` | Buying stores actifs. |
| `/createaccount username: password: sex: birthdate: email:` | Commande conservée ; création de compte si elle est activée. |

## Commandes staff

Les commandes staff répondent en éphémère lorsqu’elles affichent des données sensibles. Les rôles sont configurables avec :

- `RATHENAFR_HELPER_ROLE_IDS`
- `RATHENAFR_MODERATOR_ROLE_IDS`
- `RATHENAFR_GM_ROLE_IDS`
- `RATHENAFR_ADMIN_ROLE_IDS`
- `RATHENAFR_OWNER_ROLE_IDS`

| Commande | Rôle minimum | Description |
|---|---:|---|
| `/staff player character:` | Helper | Fiche complète d’un personnage. |
| `/staff account character:` | Helper | Compte lié au personnage, sans password, hash ni e-mail privé. |
| `/staff chars lookup:` | Helper | Personnages d’un compte ou du compte lié à un personnage. |
| `/staff inventory character:` | GM | Inventaire. |
| `/staff equipment character:` | GM | Équipement porté. |
| `/staff cart character:` | GM | Cart. |
| `/staff storage character:` | GM | Storage du compte. |
| `/staff guildstorage guild:` | GM | Coffre de guilde. |
| `/staff whohas item:` | GM | Propriétaires d’un item. |
| `/staff item-search item:` | GM | Recherche d’un item dans les conteneurs. |
| `/staff zeny character:` | GM | Zeny d’un personnage. |
| `/staff zenylog character:` | GM | Logs zeny si disponibles. |
| `/staff picklog character:` | GM | Logs items si disponibles. |
| `/staff trade-log character:` | GM | Vue depuis les logs items si disponibles. |
| `/staff mvp-log character:` | GM | Logs MVP du joueur si disponibles. |
| `/staff loginlog character:` | Admin | Logs de connexion. |
| `/staff ip-accounts character:` | Admin | Contexte `loginlog` avec IP masquées. |
| `/staff multiaccount character:` | Admin | Contexte multi-compte depuis `loginlog`. |
| `/staff banned` | Admin | Comptes bannis ou bloqués. |
| `/staff account-manage edit account: field: value: reason:` | Admin par défaut | Modifie uniquement un champ sûr de `login`, si la commande est activée. |
| `/staff account-manage ban account: until: reason:` | Admin par défaut | Bloque un compte par `account_id` ou `userid` exact. |
| `/staff account-manage unban account: reason:` | Admin par défaut | Débloque un compte par `account_id` ou `userid` exact. |
| `/staff account-manage delete account_id: confirm: reason:` | Owner par défaut | Désactivation forte sans suppression physique ; `confirm` doit être `SUPPRIMER`. |

`/staff account-manage` est désactivée par défaut avec
`RATHENAFR_ACCOUNT_MANAGE_ENABLED=false`. `delete` a sa propre configuration
`RATHENAFR_ACCOUNT_DELETE_ENABLED=false` et reste une désactivation forte du
compte, pas une suppression SQL physique.

## Modération, debug, audit et base de données

| Commande | Rôle minimum | Description |
|---|---:|---|
| `/mod chatlog character:` | Moderator | Messages récents si `chatlog` existe. |
| `/mod chat-search text:` | Moderator | Recherche compacte dans `chatlog`. |
| `/mod report-context character:` | Moderator | Position et logs récents. |
| `/mod branchlog character:` | Moderator | Dead Branch/Bloody Branch si `branchlog` existe. |
| `/debug quest character:` | GM par défaut | Quêtes du personnage. |
| `/debug char-vars character:` | GM par défaut | Variables personnage. |
| `/debug acc-vars character:` | GM par défaut | Variables compte. |
| `/audit atcommands gm:` | Admin par défaut | Commandes GM utilisées. |
| `/audit item-created` | Admin par défaut | Vue depuis `picklog`. |
| `/audit zeny-created` | Admin par défaut | Vue depuis `zenylog`. |
| `/audit gm-activity gm:` | Admin par défaut | Activité GM depuis `atcommandlog`. |
| `/db health` | Owner | Tables présentes, tables manquantes et logs actifs. |
| `/db tables` | Owner | Tables détectées. |
| `/db count` | Owner | Nombre de lignes par table utile. |
| `/db logs-size` | Owner | Volume des logs SQL. |
| `/db last-update` | Owner | Dernières entrées `sql_updates` si la table est disponible. |

## Commande `/gmmsg`

| Commande | Description |
|---|---|
| `/gmmsg server message:` | Message global serveur via GameBridge. |
| `/gmmsg map map: message:` | Message map si le bridge le supporte. |
| `/gmmsg blue message:` | Annonce bleue si elle est supportée. |
| `/gmmsg color hex: message:` | Annonce couleur. `hex` doit être au format `RRGGBB`. |
| `/gmmsg test message:` | Test et log uniquement, sans envoi en jeu. |

> [!IMPORTANT]
> `/gmmsg` utilise le mode configuré par `RATHENAFR_GMMSG_MODE`.
>
> - `disabled` : aucun envoi en jeu.
> - `test` : validation des permissions et logs staff, sans SQL.
> - `sql_queue` : insertion dans `discord_gmmsg_queue`, puis traitement par un script NPC rAthena.
>
> Consulte [Bridge GMMSG SQL Queue](GMMSG_BRIDGE_FR.md) pour le schéma `VARBINARY(180)`, l’encodage Windows-1252 et l’installation du script NPC.

## Anciennes commandes retirées

Les anciens packs ont été retirés du registre Discord : `/status`, `/guilds`, `/guildmembers`, `/search`, `/topzeny`, `/classes`, `/mapstats`, `/maponline`, `/party`, `/partymembers`, `/homunculus`, `/pet`, `/zeny`, `/castles`, `/guildalliances`, `/guildskills`, `/homunculustop`, `/pettop`, `/queststats`, `/whosell`, `/whobuy`, `/venders`, `/buyers`, `/charquests`, `/charequipment`, `/charinventory`, `/itemcount`, `/itemowners`, `/accountlist`, `/accountoverview`, `/accountmanage`, `/banlist`, `/accountchars` et `/accountstatus`.

> [!IMPORTANT]
> **Redéploiement Discord**
>
> Redéploie les commandes après mise à jour pour retirer les anciennes commandes côté Discord :
>
> ```bash
> cargo run -- --deploy
> ```
