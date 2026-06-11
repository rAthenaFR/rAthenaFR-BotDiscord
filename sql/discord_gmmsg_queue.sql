-- rAthenaFR Discord Bot - file SQL pour /gmmsg
-- Executer avec un compte administrateur dans la base rAthena cible.
-- Le script cree la table ou complete une ancienne installation sans supprimer
-- les messages existants.

CREATE TABLE IF NOT EXISTS `discord_gmmsg_queue` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `mode` ENUM('server', 'map', 'blue', 'color') NOT NULL DEFAULT 'server',
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP PROCEDURE IF EXISTS `rathenafr_migrate_gmmsg_queue`;

DELIMITER //

CREATE PROCEDURE `rathenafr_migrate_gmmsg_queue`()
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'mode'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `mode` ENUM('server', 'map', 'blue', 'color')
        NOT NULL DEFAULT 'server' AFTER `id`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'map'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `map` VARCHAR(32) DEFAULT NULL AFTER `mode`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'message'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `message` VARBINARY(180) NOT NULL AFTER `map`;
  ELSE
    IF EXISTS (
      SELECT 1
      FROM `discord_gmmsg_queue`
      WHERE OCTET_LENGTH(`message`) > 180
      LIMIT 1
    ) THEN
      SIGNAL SQLSTATE '45000'
        SET MESSAGE_TEXT = 'GMMSG migration refused: an existing message exceeds 180 bytes.';
    END IF;

    ALTER TABLE `discord_gmmsg_queue`
      MODIFY COLUMN `message` VARBINARY(180) NOT NULL;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'color'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `color` VARCHAR(16) DEFAULT NULL AFTER `message`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'discord_user_id'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `discord_user_id` VARCHAR(32) DEFAULT NULL AFTER `color`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'discord_username'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `discord_username` VARCHAR(100) DEFAULT NULL AFTER `discord_user_id`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'status'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `status` ENUM('pending', 'done', 'failed')
        NOT NULL DEFAULT 'pending' AFTER `discord_username`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'error'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `error` VARCHAR(255) DEFAULT NULL AFTER `status`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'created_at'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP AFTER `error`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `column_name` = 'processed_at'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD COLUMN `processed_at` TIMESTAMP NULL DEFAULT NULL AFTER `created_at`;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`statistics`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'discord_gmmsg_queue'
      AND `index_name` = 'idx_status_id'
  ) THEN
    ALTER TABLE `discord_gmmsg_queue`
      ADD KEY `idx_status_id` (`status`, `id`);
  END IF;
END//

DELIMITER ;

CALL `rathenafr_migrate_gmmsg_queue`();

DROP PROCEDURE IF EXISTS `rathenafr_migrate_gmmsg_queue`;
