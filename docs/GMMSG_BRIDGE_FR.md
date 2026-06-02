# Bridge GMMSG SQL Queue

Cette page décrit le fonctionnement de `/gmmsg` quand le bot Discord envoie les messages en jeu via la file SQL rAthena `discord_gmmsg_queue`.

> [!NOTE]
> Cette page documente le bridge GMMSG. Le fichier `README.md` racine n’est pas modifié par cette phase.

## Fonctionnement

`/gmmsg` est une commande staff qui prépare un message Discord, le valide, puis le transmet selon `RATHENAFR_GMMSG_MODE`.

Modes disponibles :

- `disabled` : la commande reste déclarée, mais aucun message n’est envoyé en jeu.
- `test` : le bot valide les permissions, nettoie le message et journalise l’action sans dépendre de SQL.
- `sql_queue` : le bot insère une ligne `pending` dans `discord_gmmsg_queue`; un script NPC rAthena lit ensuite la file et annonce le message en jeu.

Mappings SQL :

| Commande | `mode` | `map` | `color` |
|---|---|---|---|
| `/gmmsg server message:` | `server` | `NULL` | `NULL` |
| `/gmmsg map map: message:` | `map` | nom de map | `NULL` |
| `/gmmsg blue message:` | `blue` | `NULL` | `NULL` |
| `/gmmsg color hex: message:` | `color` | `NULL` | valeur `RRGGBB` validée |

La réponse Discord attendue après insertion est :

```text
Message ajouté à la file d’envoi en jeu.
```

Si la table manque, le bot répond :

```text
La table `discord_gmmsg_queue` est absente. Exécutez le script SQL d’installation du bridge GMMSG.
```

> [!IMPORTANT]
> Le mode `sql_queue` ne parle pas directement au map-server. Il écrit uniquement dans la base rAthena avec des requêtes préparées. Le script NPC côté rAthena est responsable de lire les messages `pending`, de les annoncer en jeu, puis de passer leur statut à `done`.

## Configuration

Configuration minimale recommandée :

```env
RATHENAFR_GMMSG_MODE=sql_queue
RATHENAFR_GMMSG_ENCODING=windows1252
RATHENAFR_GMMSG_MAX_LENGTH=180
RATHENAFR_GMMSG_MIN_ROLE=gm
```

Pour un test sans SQL :

```env
RATHENAFR_GMMSG_MODE=test
```

Pour désactiver l’envoi :

```env
RATHENAFR_GMMSG_MODE=disabled
```

> [!TIP]
> Utilise `test` pour vérifier les rôles Discord et les logs staff avant de brancher la file SQL.

## Encodage Windows-1252

Le client Ragnarok Online historique n’affiche pas toujours correctement les accents français si le texte est stocké en UTF-8. La solution validée consiste à stocker les octets Windows-1252 dans `discord_gmmsg_queue.message`, dont le type SQL attendu est `VARBINARY(180)`.

Caractères français attendus :

```text
é è à ç ù ê î ô û É Ç À
```

Le bot convertit le message Discord UTF-8 en Windows-1252 avant insertion SQL lorsque `RATHENAFR_GMMSG_ENCODING=windows1252`.

> [!WARNING]
> Les emojis et les caractères hors Windows-1252 sont refusés. Le bot répond : `Le message contient des caractères non compatibles avec l’encodage Windows-1252 utilisé par le client en jeu.`

> [!CAUTION]
> Ne remplace pas `VARBINARY(180)` par `VARCHAR(180)` si ton client en jeu attend des octets Windows-1252. Un `VARCHAR` encodé en UTF-8 peut réintroduire les accents mal affichés.

## Table SQL

Le script SQL d’installation attendu est `sql/discord_gmmsg_queue.sql`.

Schéma attendu :

```sql
CREATE TABLE IF NOT EXISTS `discord_gmmsg_queue` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `mode` ENUM('server', 'map', 'blue', 'color') NOT NULL,
  `map` VARCHAR(32) DEFAULT NULL,
  `message` VARBINARY(180) NOT NULL,
  `color` VARCHAR(16) DEFAULT NULL,
  `discord_user_id` VARCHAR(32) DEFAULT NULL,
  `discord_username` VARCHAR(100) DEFAULT NULL,
  `status` ENUM('pending', 'done', 'failed') NOT NULL DEFAULT 'pending',
  `error` VARCHAR(255) DEFAULT NULL,
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `processed_at` TIMESTAMP NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `idx_status_id` (`status`, `id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

Migration utile si la table existait déjà avec `message` en texte :

```sql
ALTER TABLE `discord_gmmsg_queue`
  MODIFY `message` VARBINARY(180) NOT NULL;
```

Permission SQL minimale additionnelle pour le bot en mode `sql_queue` :

```sql
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';
```

> [!IMPORTANT]
> Le bot doit pouvoir lire l’existence de la table et insérer dans `discord_gmmsg_queue`. Il ne doit pas avoir besoin de `UPDATE`, `DELETE`, `DROP` ou `ALTER` en production.

## Script NPC rAthena

Le serveur rAthena doit charger un script NPC custom qui :

1. lit les lignes `status='pending'` dans `discord_gmmsg_queue`;
2. récupère les octets de `message` sans les convertir en UTF-8;
3. annonce le message en jeu selon `mode`, `map` et `color`;
4. marque la ligne en `status='done'`;
5. marque la ligne en `status='failed'` et renseigne `error` si l’annonce échoue.

Activation habituelle dans `npc/scripts_custom.conf` :

```conf
npc: npc/custom/discord_gmmsg_queue.txt
```

> [!WARNING]
> Sans script NPC chargé côté rAthena, le bot peut ajouter les messages à la file SQL, mais rien ne les annoncera en jeu.

## Données sensibles

La table stocke `discord_user_id` et `discord_username` pour tracer l’origine d’un message GM.

> [!CAUTION]
> Ne stocke pas de mots de passe, tokens, informations personnelles, IP complètes ou secrets dans les messages `/gmmsg`. Les messages peuvent être visibles en jeu, en base SQL et dans les logs staff.