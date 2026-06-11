-- rAthenaFR Discord Bot - schema du catalogue MVP
-- Executer avec un compte administrateur dans la base rAthena cible.
-- Executer ensuite rathenafr_mvp_data.sql pour installer le catalogue fourni.

CREATE TABLE IF NOT EXISTS `rathenafr_mvp_list` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `monster_id` INT UNSIGNED NOT NULL,
  `monster_name` VARCHAR(100) NOT NULL,
  `aegis_name` VARCHAR(100) DEFAULT NULL,
  `map_name` VARCHAR(50) DEFAULT NULL,
  `respawn_minutes` DECIMAL(8,2) DEFAULT NULL,
  `respawn_variance_minutes` DECIMAL(8,2) DEFAULT NULL,
  `enabled` TINYINT(1) NOT NULL DEFAULT 1,
  `source` VARCHAR(64) NOT NULL DEFAULT 'manual',
  `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uq_rathenafr_mvp_list` (`monster_id`, `map_name`, `respawn_minutes`),
  KEY `idx_rathenafr_mvp_enabled` (`enabled`),
  KEY `idx_rathenafr_mvp_monster_id` (`monster_id`),
  KEY `idx_rathenafr_mvp_map_name` (`map_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP PROCEDURE IF EXISTS `rathenafr_migrate_mvp_schema`;

DELIMITER //

CREATE PROCEDURE `rathenafr_migrate_mvp_schema`()
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'rathenafr_mvp_list'
      AND `column_name` = 'enabled'
  ) THEN
    ALTER TABLE `rathenafr_mvp_list`
      ADD COLUMN `enabled` TINYINT(1) NOT NULL DEFAULT 1 AFTER `respawn_variance_minutes`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`statistics`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'rathenafr_mvp_list'
      AND `index_name` = 'idx_rathenafr_mvp_enabled'
  ) THEN
    ALTER TABLE `rathenafr_mvp_list`
      ADD KEY `idx_rathenafr_mvp_enabled` (`enabled`);
  END IF;
END//

DELIMITER ;

CALL `rathenafr_migrate_mvp_schema`();

DROP PROCEDURE IF EXISTS `rathenafr_migrate_mvp_schema`;

CREATE OR REPLACE SQL SECURITY INVOKER VIEW `rathenafr_mvp_regular_spawn` AS
SELECT
  CAST(`monster_id` AS UNSIGNED) AS `monster_id`,
  NULLIF(TRIM(`monster_name`), '') AS `monster_name`,
  NULLIF(TRIM(`aegis_name`), '') AS `aegis_name`,
  NULLIF(TRIM(`map_name`), '') AS `map_name`,
  `respawn_minutes`,
  `respawn_variance_minutes`,
  `source`
FROM `rathenafr_mvp_list`
WHERE `enabled` = 1
  AND `source` = 'regular_spawn'
  AND `map_name` IS NOT NULL
  AND TRIM(`map_name`) <> ''
  AND COALESCE(`respawn_minutes`, 0) > 0;
