# Configuration serveur rAthena pour GMMSG

Ce document explique comment configurer le serveur **rAthena** pour utiliser la commande Discord `/gmmsg` avec le mode `sql_queue`.

Le fonctionnement est le suivant :

```txt
Discord /gmmsg
→ rAthenaFR-BotDiscord
→ table SQL discord_gmmsg_queue
→ script NPC rAthena
→ announce / mapannounce en jeu
```

> [!WARNING]
> Cette configuration concerne uniquement le mode `sql_queue` de `/gmmsg`.
>
> Si `RATHENAFR_GMMSG_MODE=disabled`, aucun message n’est envoyé en jeu.
>
> Si `RATHENAFR_GMMSG_MODE=test`, la commande est simulée côté Discord et ne dépend pas de rAthena.

---

## 1. Chemins rapides

| Élément | Installation classique | Docker rAthena |
|---|---|---|
| Script NPC GMMSG | `npc/custom/rathenafr_discord_gmmsg.txt` | `/rathena/npc/custom/rathenafr_discord_gmmsg.txt` |
| Chargement des scripts custom | `npc/scripts_custom.conf` | `/rathena/npc/scripts_custom.conf` |
| Ligne à ajouter | `npc: npc/custom/rathenafr_discord_gmmsg.txt` | identique |
| Conteneur map-server | non applicable | `rathena-map` |
| Conteneur MariaDB | non applicable | `rathena-db` |
| Base SQL utilisée | selon votre configuration | `ragnarok` dans l’exemple Docker |

> [!NOTE]
> Les noms `rathena-map`, `rathena-db` et `ragnarok` correspondent à l’environnement Docker utilisé par rAthenaFR.
>
> Adaptez-les si vos conteneurs ou votre base portent un autre nom.

---

## 2. Créer la table SQL

La table doit être créée dans la base principale rAthena, celle qui contient les tables comme `login`, `char`, `guild`, `inventory`, etc.

Dans l’environnement Docker rAthenaFR, la base s’appelle généralement :

```txt
ragnarok
```

Créer le fichier SQL suivant côté dépôt du bot :

```txt
sql/discord_gmmsg_queue.sql
```

Contenu du fichier :

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

> [!WARNING]
> La colonne `message` doit être en `VARBINARY(180)`.
>
> Ne la remplacez pas par `VARCHAR(180)`.
>
> Ce choix permet de conserver les octets Windows-1252 envoyés par le bot, ce qui évite les textes cassés du type `franÃ§ais` en jeu.

### Si la table existe déjà

Si la table existe déjà mais que `message` n’est pas en `VARBINARY`, exécutez :

```sql
ALTER TABLE `discord_gmmsg_queue`
  MODIFY `message` VARBINARY(180) NOT NULL;
```

---

## 3. Exécuter le SQL avec Docker

Depuis PowerShell :

```powershell
docker exec -it rathena-db mariadb -u root -p ragnarok
```

Puis collez le schéma SQL.

Vérifiez ensuite :

```sql
SHOW TABLES LIKE 'discord_gmmsg_queue';
```

Résultat attendu :

```txt
discord_gmmsg_queue
```

> [!TIP]
> Si la base ne s’appelle pas `ragnarok`, listez les bases disponibles :
>
> ```powershell
> docker exec -it rathena-db mariadb -u root -p -e "SHOW DATABASES;"
> ```

---

## 4. Permissions SQL du bot

L’utilisateur SQL du bot doit avoir :

- `SELECT` sur les tables nécessaires à la lecture rAthena ;
- `INSERT` sur `discord_gmmsg_queue` pour `/gmmsg` ;
- `INSERT` sur `login` uniquement si `createaccount` est activée.

Exemple avec l’utilisateur `rathenafr_bot` :

```sql
CREATE USER IF NOT EXISTS 'rathenafr_bot'@'%' IDENTIFIED BY 'MOT_DE_PASSE_BOT';
CREATE USER IF NOT EXISTS 'rathenafr_bot'@'localhost' IDENTIFIED BY 'MOT_DE_PASSE_BOT';

ALTER USER 'rathenafr_bot'@'%' IDENTIFIED BY 'MOT_DE_PASSE_BOT';
ALTER USER 'rathenafr_bot'@'localhost' IDENTIFIED BY 'MOT_DE_PASSE_BOT';

GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'localhost';

GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'localhost';

FLUSH PRIVILEGES;
```

Si `createaccount` est activée :

```sql
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'localhost';

FLUSH PRIVILEGES;
```

> [!NOTE]
> L’utilisateur `'rathenafr_bot'@'%'` est utilisé quand le bot se connecte depuis son propre conteneur Docker.
>
> L’utilisateur `'rathenafr_bot'@'localhost'` est utile pour les tests exécutés avec `docker exec` directement dans le conteneur MariaDB.

### Tester les droits SQL

```powershell
docker exec -it rathena-db mariadb -u rathenafr_bot --password="MOT_DE_PASSE_BOT" ragnarok -e "SELECT 1;"
```

Tester l’insertion dans la file GMMSG :

```powershell
docker exec -it rathena-db mariadb -u rathenafr_bot --password="MOT_DE_PASSE_BOT" ragnarok -e "INSERT INTO discord_gmmsg_queue (mode, message, discord_user_id, discord_username) VALUES ('server', X'5465737420474D4D5347', '0', 'test');"
```

---

## 5. Créer le script NPC rAthena

Créer le fichier suivant :

```txt
npc/custom/rathenafr_discord_gmmsg.txt
```

En Docker, le chemin dans le conteneur est généralement :

```txt
/rathena/npc/custom/rathenafr_discord_gmmsg.txt
```

Contenu du script :

```txt
-	script	rAthenaFR_DiscordGMMSG	-1,{

OnInit:
	initnpctimer;
	end;

OnTimer3000:
	stopnpctimer;

	.@count = query_sql(
		"SELECT `id`, `mode`, IFNULL(`map`, ''), `message`, IFNULL(`color`, '') FROM `discord_gmmsg_queue` WHERE `status` = 'pending' ORDER BY `id` ASC LIMIT 5",
		.@id,
		.@mode$,
		.@map$,
		.@message$,
		.@color$
	);

	for (.@i = 0; .@i < .@count; .@i++) {
		if (.@mode$[.@i] == "server") {
			announce "[Discord] " + .@message$[.@i], bc_all;
			query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'done', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
		}
		else if (.@mode$[.@i] == "blue") {
			announce "[Discord] " + .@message$[.@i], bc_all | bc_blue;
			query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'done', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
		}
		else if (.@mode$[.@i] == "color") {
			if (.@color$[.@i] == "") {
				query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'failed', `error` = 'Couleur absente', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
			}
			else {
				announce "[Discord] " + .@message$[.@i], bc_all, axtoi(.@color$[.@i]);
				query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'done', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
			}
		}
		else if (.@mode$[.@i] == "map") {
			if (.@map$[.@i] == "") {
				query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'failed', `error` = 'Map absente', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
			}
			else {
				mapannounce .@map$[.@i], "[Discord] " + .@message$[.@i], bc_map;
				query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'done', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
			}
		}
		else {
			query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'failed', `error` = 'Mode inconnu', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
		}
	}

	initnpctimer;
	end;
}
```

> [!WARNING]
> Le script traite jusqu’à 5 messages toutes les 3 secondes.
>
> Vous pouvez ajuster `OnTimer3000` et `LIMIT 5` si nécessaire, mais évitez une fréquence trop agressive.

---

## 6. Activer le script dans rAthena

Ouvrir :

```txt
npc/scripts_custom.conf
```

En Docker :

```txt
/rathena/npc/scripts_custom.conf
```

Ajouter cette ligne :

```txt
npc: npc/custom/rathenafr_discord_gmmsg.txt
```

> [!CAUTION]
> La ligne ne doit pas être commentée.
>
> Mauvais :
>
> ```txt
> //npc: npc/custom/rathenafr_discord_gmmsg.txt
> ```
>
> Bon :
>
> ```txt
> npc: npc/custom/rathenafr_discord_gmmsg.txt
> ```

---

## 7. Commandes utiles avec Docker

### Vérifier que le script NPC existe

```powershell
docker exec -it rathena-map sh -lc 'find / -name rathenafr_discord_gmmsg.txt 2>/dev/null'
```

Résultat attendu :

```txt
/rathena/npc/custom/rathenafr_discord_gmmsg.txt
```

### Vérifier `scripts_custom.conf`

```powershell
docker exec -it rathena-map sh -lc 'find / -name scripts_custom.conf 2>/dev/null'
```

Résultat attendu :

```txt
/rathena/npc/scripts_custom.conf
```

### Vérifier que le script est activé

```powershell
docker exec -it rathena-map sh -lc 'grep -n rathenafr_discord_gmmsg /rathena/npc/scripts_custom.conf'
```

Résultat attendu :

```txt
npc: npc/custom/rathenafr_discord_gmmsg.txt
```

Si le résultat commence par `//npc:`, le script est encore commenté.

### Décommenter automatiquement la ligne

```powershell
docker exec -it rathena-map sh -lc 'sed -i "s#^//npc: npc/custom/rathenafr_discord_gmmsg.txt#npc: npc/custom/rathenafr_discord_gmmsg.txt#" /rathena/npc/scripts_custom.conf'
```

### Ajouter la ligne si elle n’existe pas

```powershell
docker exec -it rathena-map sh -lc 'printf "\nnpc: npc/custom/rathenafr_discord_gmmsg.txt\n" >> /rathena/npc/scripts_custom.conf'
```

### Créer le dossier custom si nécessaire

```powershell
docker exec -it rathena-map sh -lc 'mkdir -p /rathena/npc/custom'
```

### Copier le script NPC depuis Windows vers le conteneur

Depuis un dossier `tools/docker` :

```powershell
docker cp ..\..\npc\custom\rathenafr_discord_gmmsg.txt rathena-map:/rathena/npc/custom/rathenafr_discord_gmmsg.txt
```

Depuis la racine rAthena :

```powershell
docker cp .\npc\custom\rathenafr_discord_gmmsg.txt rathena-map:/rathena/npc/custom/rathenafr_discord_gmmsg.txt
```

---

## 8. Redémarrer ou recharger les scripts

Avec Docker :

```powershell
docker restart rathena-map
```

Vérifier les logs :

```powershell
docker logs rathena-map --tail 150
```

Filtrer les erreurs utiles en PowerShell :

```powershell
docker logs rathena-map 2>&1 | Select-String "gmmsg|discord|script|error|query"
```

> [!TIP]
> Si vous êtes connecté en GM et que votre configuration le permet, vous pouvez aussi utiliser `@reloadscript`.
>
> En cas de doute, redémarrez simplement le conteneur `rathena-map`.

---

## 9. Configurer le bot

Dans le fichier `.env` utilisé par Docker Compose :

```env
RATHENAFR_GMMSG_MODE=sql_queue
RATHENAFR_GMMSG_ENCODING=windows1252
RATHENAFR_GMMSG_MAX_LENGTH=180
```

Vérifiez aussi la connexion SQL :

```env
RATHENAFR_DB_HOST=rathena-db
RATHENAFR_DB_PORT=3306
RATHENAFR_DB_NAME=ragnarok
RATHENAFR_DB_USER=rathenafr_bot
RATHENAFR_DB_PASSWORD=MOT_DE_PASSE_BOT
```

> [!WARNING]
> Le mot de passe `RATHENAFR_DB_PASSWORD` doit être identique à celui défini dans MariaDB pour `rathenafr_bot`.

---

## 10. Tester manuellement la file GMMSG

### Test simple

```powershell
docker exec -it rathena-db mariadb -u root -p ragnarok -e "INSERT INTO discord_gmmsg_queue (mode, message, discord_user_id, discord_username) VALUES ('server', X'5465737420474D4D5347', '0', 'test');"
```

Le message correspond à :

```txt
Test GMMSG
```

### Test des accents français

Ce test insère directement les octets Windows-1252 :

```powershell
docker exec -it rathena-db mariadb -u root -p ragnarok -e "INSERT INTO discord_gmmsg_queue (mode, message, discord_user_id, discord_username) VALUES ('server', X'5465737420616363656E7473203A20E920E820E020E720F920EA20EE20F420FB20C920C720C0', '0', 'test');"
```

Le message attendu en jeu est :

```txt
Test accents : é è à ç ù ê î ô û É Ç À
```

### Vérifier le traitement

```powershell
docker exec -it rathena-db mariadb -u root -p ragnarok -e "SELECT id, mode, HEX(message), status, error, processed_at FROM discord_gmmsg_queue ORDER BY id DESC LIMIT 5;"
```

Résultat attendu :

```txt
status = done
error = NULL
processed_at = rempli
```

---

## 11. États possibles

| Statut | Signification |
|---|---|
| `pending` | Message en attente de traitement par le script NPC |
| `done` | Message annoncé en jeu avec succès |
| `failed` | Message non traité, voir la colonne `error` |

---

## 12. Dépannage

| Problème | Cause probable | Solution |
|---|---|---|
| Le message reste en `pending` | Le script NPC n’est pas chargé | Vérifier `npc/scripts_custom.conf` et redémarrer `rathena-map` |
| La ligne du script commence par `//npc:` | Le script est commenté | Retirer `//` devant la ligne |
| Le message passe en `failed` | Erreur traitée par le script NPC | Lire la colonne `error` |
| Le message affiche `franÃ§ais` | Encodage incorrect | Vérifier `VARBINARY(180)` et `RATHENAFR_GMMSG_ENCODING=windows1252` |
| `Access denied for user 'rathenafr_bot'` | Mauvais mot de passe ou GRANT manquant | Vérifier `ALTER USER`, `GRANT` et `.env` |
| Table absente | SQL non exécuté | Exécuter `sql/discord_gmmsg_queue.sql` |
| Rien ne s’affiche en jeu mais le statut est `done` | Annonce masquée ou map incorrecte | Tester d’abord avec `mode='server'` |
| `/gmmsg` répond que le mode est désactivé | Configuration bot en `disabled` | Mettre `RATHENAFR_GMMSG_MODE=sql_queue` |

---

## 13. Résumé de configuration

Côté rAthena :

```txt
npc/custom/rathenafr_discord_gmmsg.txt
npc/scripts_custom.conf
```

Ligne à activer :

```txt
npc: npc/custom/rathenafr_discord_gmmsg.txt
```

Côté SQL :

```txt
discord_gmmsg_queue.message = VARBINARY(180)
```

Côté bot :

```env
RATHENAFR_GMMSG_MODE=sql_queue
RATHENAFR_GMMSG_ENCODING=windows1252
RATHENAFR_GMMSG_MAX_LENGTH=180
```

> [!NOTE]
> Une fois cette configuration en place, `/gmmsg` ajoute les messages à la file SQL.
>
> Le serveur rAthena les lit ensuite via le script NPC et les diffuse en jeu.