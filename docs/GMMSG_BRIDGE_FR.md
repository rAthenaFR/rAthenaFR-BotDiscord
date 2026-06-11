# Bridge GMMSG

Le mode `sql_queue` relie Discord à rAthena par une file SQL :

```text
/gmmsg -> bot Discord -> discord_gmmsg_queue -> script NPC -> annonce en jeu
```

> [!NOTE]
> Le bot ne se connecte pas directement au map-server. Le mode `bridge` existe dans la configuration, mais aucune implémentation map-server n’est active.

## Modes disponibles

| Mode | Comportement |
|---|---|
| `disabled` | Commande déclarée, aucun envoi. |
| `test` | Permissions, validation et logs sans SQL. |
| `sql_queue` | Insertion d’une ligne `pending` dans la file SQL. |
| `bridge` | Réservé, retourne actuellement une erreur de bridge absent. |

Commence par :

```env
RATHENAFR_GMMSG_MODE=test
RATHENAFR_GMMSG_MIN_ROLE=gm
```

Puis active la file :

```env
RATHENAFR_GMMSG_MODE=sql_queue
RATHENAFR_GMMSG_ENCODING=windows1252
RATHENAFR_GMMSG_MAX_LENGTH=180
```

> [!TIP]
> Le mode `test` permet de valider les rôles et `RATHENAFR_STAFF_LOG_CHANNEL_ID` avant toute modification SQL.

## Installer la table

```bash
mariadb -u root -p ragnarok < sql/discord_gmmsg_queue.sql
mariadb -u root -p ragnarok < sql/create-gmmsg-queue-user.sql
```

Le premier script crée la table complète ou ajoute les colonnes et l’index manquants à une ancienne version. Il conserve les lignes existantes et peut être réexécuté.

> [!WARNING]
> La migration s’arrête si une ancienne ligne dépasse 180 octets. Archive ou corrige ces messages avant de relancer le script; ils ne sont jamais tronqués automatiquement.

Le schéma contient notamment :

```sql
`mode` ENUM('server', 'map', 'blue', 'color'),
`message` VARBINARY(180) NOT NULL,
`status` ENUM('pending', 'done', 'failed') NOT NULL DEFAULT 'pending'
```

Le bot a seulement besoin de :

```sql
GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';
```

> [!CAUTION]
> Ne donne pas au bot `UPDATE` ou `DELETE` sur la file. Le compte SQL utilisé par rAthena traite les lignes; le bot ne fait que les créer.

> [!NOTE]
> Le script de permissions ajoute `INSERT` aux droits déjà présents. Il ne révoque aucun ancien privilège.

## Encodage

`windows1252` est recommandé pour les clients Ragnarok Online historiques. Le bot encode le message avant insertion et refuse les caractères non représentables.

> [!WARNING]
> La colonne `message` doit rester en `VARBINARY(180)`. Un `VARCHAR` UTF-8 peut produire des accents illisibles en jeu.

La limite `RATHENAFR_GMMSG_MAX_LENGTH` est appliquée aux octets encodés. Les emojis sont refusés en Windows-1252.

## Script NPC

Crée un script, par exemple :

```text
npc/custom/rathenafr_discord_gmmsg.txt
```

Exemple minimal :

```text
-	script	rAthenaFR_DiscordGMMSG	-1,{

OnInit:
	initnpctimer;
	end;

OnTimer3000:
	stopnpctimer;

	.@count = query_sql(
		"SELECT `id`, `mode`, IFNULL(`map`, ''), `message`, IFNULL(`color`, '') FROM `discord_gmmsg_queue` WHERE `status` = 'pending' ORDER BY `id` ASC LIMIT 5",
		.@id, .@mode$, .@map$, .@message$, .@color$
	);

	for (.@i = 0; .@i < .@count; .@i++) {
		if (.@mode$[.@i] == "server") {
			announce "[Discord] " + .@message$[.@i], bc_all;
		}
		else if (.@mode$[.@i] == "blue") {
			announce "[Discord] " + .@message$[.@i], bc_all | bc_blue;
		}
		else if (.@mode$[.@i] == "color" && .@color$[.@i] != "") {
			announce "[Discord] " + .@message$[.@i], bc_all, axtoi(.@color$[.@i]);
		}
		else if (.@mode$[.@i] == "map" && .@map$[.@i] != "") {
			mapannounce .@map$[.@i], "[Discord] " + .@message$[.@i], bc_map;
		}
		else {
			query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'failed', `error` = 'Parametres invalides', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
			continue;
		}

		query_sql("UPDATE `discord_gmmsg_queue` SET `status` = 'done', `processed_at` = NOW() WHERE `id` = " + .@id[.@i]);
	}

	initnpctimer;
	end;
}
```

Charge-le dans `npc/scripts_custom.conf` :

```conf
npc: npc/custom/rathenafr_discord_gmmsg.txt
```

> [!IMPORTANT]
> L’API de script peut varier selon la version rAthena. Teste le script sur un serveur de développement et vérifie les logs du map-server avant la production.

## Installation rAthena sous Docker

Les noms de conteneurs dépendent de ton installation. Exemples :

```powershell
docker exec -it rathena-db mariadb -u root -p ragnarok
docker exec -it rathena-map sh -lc 'grep -n rathenafr_discord_gmmsg /rathena/npc/scripts_custom.conf'
docker restart rathena-map
docker logs rathena-map --tail 150
```

Adapte `rathena-db`, `rathena-map`, `/rathena` et `ragnarok` à ton environnement.

## Tester la file

Insertion ASCII :

```sql
INSERT INTO `discord_gmmsg_queue`
  (`mode`, `message`, `discord_user_id`, `discord_username`)
VALUES
  ('server', X'5465737420474D4D5347', '0', 'test');
```

Vérification :

```sql
SELECT `id`, `mode`, HEX(`message`), `status`, `error`, `processed_at`
FROM `discord_gmmsg_queue`
ORDER BY `id` DESC
LIMIT 5;
```

| État | Signification |
|---|---|
| `pending` | Le script NPC n’a pas encore traité la ligne. |
| `done` | L’annonce a été exécutée. |
| `failed` | Le script a refusé la ligne; consulter `error`. |

## Données sensibles

La file conserve l’ID et le nom Discord de l’auteur.

> [!CAUTION]
> N’envoie jamais de token, mot de passe, IP privée ou donnée personnelle avec `/gmmsg`. Le contenu peut être visible dans Discord, SQL, les logs staff et le jeu.
