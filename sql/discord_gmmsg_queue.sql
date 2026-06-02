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

ALTER TABLE `discord_gmmsg_queue`
  MODIFY `message` VARBINARY(180) NOT NULL;
