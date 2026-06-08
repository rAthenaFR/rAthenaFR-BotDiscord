-- rAthenaFR Discord Bot - table de compatibilite pour /db last-update
-- Executer ce fichier dans la base rAthena avec un utilisateur administrateur.

CREATE TABLE IF NOT EXISTS `sql_updates` (
  `revision` VARCHAR(255) NOT NULL,
  `applied` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`revision`),
  KEY `idx_sql_updates_applied` (`applied`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

INSERT IGNORE INTO `sql_updates` (`revision`)
VALUES ('rathenafr-discord-bootstrap');
