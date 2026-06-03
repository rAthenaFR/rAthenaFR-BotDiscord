-- rAthenaFR Discord Bot - support SQL pour /mvp list
-- Executer ce fichier dans la base rAthena avec un utilisateur administrateur.
-- Le script cree la table support et la vue lue par le bot.
-- Il ne peuple pas la table : importe les donnees MVP Athena dans rathenafr_mvp_list.

CREATE TABLE IF NOT EXISTS `rathenafr_mvp_list` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `monster_id` INT UNSIGNED NOT NULL,
  `monster_name` VARCHAR(100) DEFAULT NULL,
  `aegis_name` VARCHAR(100) DEFAULT NULL,
  `map_name` VARCHAR(32) DEFAULT NULL,
  `respawn_minutes` DECIMAL(10,2) DEFAULT NULL,
  `respawn_variance_minutes` DECIMAL(10,2) DEFAULT NULL,
  `source` VARCHAR(100) NOT NULL DEFAULT 'manual',
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uniq_rathenafr_mvp_list_spawn` (`monster_id`, `map_name`, `source`),
  KEY `idx_rathenafr_mvp_list_monster` (`monster_id`),
  KEY `idx_rathenafr_mvp_list_name_map` (`monster_name`, `map_name`),
  KEY `idx_rathenafr_mvp_list_respawn` (`respawn_minutes`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE OR REPLACE VIEW `rathenafr_mvp_regular_spawn` AS
SELECT
  CAST(`monster_id` AS UNSIGNED) AS `monster_id`,
  NULLIF(TRIM(`monster_name`), '') AS `monster_name`,
  NULLIF(TRIM(`aegis_name`), '') AS `aegis_name`,
  NULLIF(TRIM(`map_name`), '') AS `map_name`,
  `respawn_minutes`,
  `respawn_variance_minutes`,
  `source`
FROM `rathenafr_mvp_list`
WHERE `map_name` IS NOT NULL
  AND TRIM(`map_name`) <> ''
  AND COALESCE(`respawn_minutes`, 0) > 0;
