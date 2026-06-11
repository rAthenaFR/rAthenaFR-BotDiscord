-- rAthenaFR Discord Bot - catalogue SQL pour /item info
-- Executer avec un compte administrateur dans la base rAthena cible.
--
-- Objets geres :
--   rathenafr_item_list   : table materialisee et extensible ;
--   rathenafr_item_search : vue stable lue par le bot.
--
-- Le script :
--   1. migre l'ancienne table rathenafr_item_search si elle existe ;
--   2. conserve les lignes deja importees dans rathenafr_item_list ;
--   3. ajoute ou actualise les items disponibles dans item_db/item_db_re ;
--   4. recree la vue de recherche.

CREATE TABLE IF NOT EXISTS `rathenafr_item_list` (
  `item_id` INT UNSIGNED NOT NULL,
  `aegis_name` VARCHAR(128) NOT NULL,
  `item_name` VARCHAR(255) NOT NULL,
  `item_type` VARCHAR(64) DEFAULT NULL,
  `item_subtype` VARCHAR(64) DEFAULT NULL,
  `buy` BIGINT DEFAULT NULL,
  `sell` BIGINT DEFAULT NULL,
  `weight` INT DEFAULT NULL,
  `attack` INT DEFAULT NULL,
  `magic_attack` INT DEFAULT NULL,
  `defense` INT DEFAULT NULL,
  `item_range` INT DEFAULT NULL,
  `slots` INT DEFAULT NULL,
  `weapon_level` INT DEFAULT NULL,
  `armor_level` INT DEFAULT NULL,
  `equip_level_min` INT DEFAULT NULL,
  `equip_level_max` INT DEFAULT NULL,
  `refineable` TINYINT(1) NOT NULL DEFAULT 0,
  `gradable` TINYINT(1) NOT NULL DEFAULT 0,
  `view_id` INT DEFAULT NULL,
  `source_file` VARCHAR(128) DEFAULT NULL,
  `enabled` TINYINT(1) NOT NULL DEFAULT 1,
  `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`item_id`),
  KEY `idx_rathenafr_item_list_name` (`item_name`),
  KEY `idx_rathenafr_item_list_aegis` (`aegis_name`),
  KEY `idx_rathenafr_item_list_type` (`item_type`),
  FULLTEXT KEY `ft_rathenafr_item_list_search` (`item_name`, `aegis_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP PROCEDURE IF EXISTS `rathenafr_prepare_item_catalog`;

DELIMITER //

CREATE PROCEDURE `rathenafr_prepare_item_catalog`()
BEGIN
  DECLARE search_object_type VARCHAR(32) DEFAULT NULL;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'rathenafr_item_list'
      AND `column_name` = 'source_file'
  ) THEN
    ALTER TABLE `rathenafr_item_list`
      ADD COLUMN `source_file` VARCHAR(128) DEFAULT NULL;
  END IF;

  IF NOT EXISTS (
    SELECT 1
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = 'rathenafr_item_list'
      AND `column_name` = 'enabled'
  ) THEN
    ALTER TABLE `rathenafr_item_list`
      ADD COLUMN `enabled` TINYINT(1) NOT NULL DEFAULT 1;
  END IF;

  SELECT `table_type`
    INTO search_object_type
  FROM `information_schema`.`tables`
  WHERE `table_schema` = DATABASE()
    AND `table_name` = 'rathenafr_item_search'
  LIMIT 1;

  IF search_object_type = 'BASE TABLE' THEN
    INSERT INTO `rathenafr_item_list` (
      `item_id`,
      `item_name`,
      `aegis_name`,
      `item_type`,
      `item_subtype`,
      `slots`,
      `buy`,
      `sell`,
      `weight`,
      `attack`,
      `magic_attack`,
      `defense`,
      `equip_level_min`,
      `source_file`,
      `enabled`
    )
    SELECT
      `item_id`,
      `item_name`,
      `aegis_name`,
      `item_type`,
      `item_subtype`,
      `slots`,
      `buy`,
      `sell`,
      `weight`,
      `attack`,
      `magic_attack`,
      `defense`,
      `equip_level_min`,
      'legacy_rathenafr_item_search',
      1
    FROM `rathenafr_item_search`
    ON DUPLICATE KEY UPDATE
      `item_name` = VALUES(`item_name`),
      `aegis_name` = VALUES(`aegis_name`),
      `item_type` = VALUES(`item_type`),
      `item_subtype` = VALUES(`item_subtype`),
      `slots` = VALUES(`slots`),
      `buy` = VALUES(`buy`),
      `sell` = VALUES(`sell`),
      `weight` = VALUES(`weight`),
      `attack` = VALUES(`attack`),
      `magic_attack` = VALUES(`magic_attack`),
      `defense` = VALUES(`defense`),
      `equip_level_min` = VALUES(`equip_level_min`),
      `enabled` = 1;

    DROP TABLE `rathenafr_item_search`;
  ELSEIF search_object_type = 'VIEW' THEN
    DROP VIEW `rathenafr_item_search`;
  END IF;
END//

DELIMITER ;

CALL `rathenafr_prepare_item_catalog`();

DROP PROCEDURE IF EXISTS `rathenafr_prepare_item_catalog`;
DROP PROCEDURE IF EXISTS `rathenafr_refresh_item_catalog`;

DELIMITER //

CREATE PROCEDURE `rathenafr_refresh_item_catalog`()
BEGIN
  DECLARE done BOOL DEFAULT FALSE;
  DECLARE source_table VARCHAR(64);
  DECLARE source_ref VARCHAR(140);
  DECLARE empty_string VARCHAR(8);
  DECLARE unknown_item VARCHAR(64);
  DECLARE unknown_aegis VARCHAR(64);
  DECLARE id_col VARCHAR(64);
  DECLARE item_name_col VARCHAR(64);
  DECLARE aegis_name_col VARCHAR(64);
  DECLARE item_type_col VARCHAR(64);
  DECLARE item_subtype_col VARCHAR(64);
  DECLARE slots_col VARCHAR(64);
  DECLARE buy_col VARCHAR(64);
  DECLARE sell_col VARCHAR(64);
  DECLARE weight_col VARCHAR(64);
  DECLARE attack_col VARCHAR(64);
  DECLARE magic_attack_col VARCHAR(64);
  DECLARE defense_col VARCHAR(64);
  DECLARE range_col VARCHAR(64);
  DECLARE weapon_level_col VARCHAR(64);
  DECLARE armor_level_col VARCHAR(64);
  DECLARE equip_level_min_col VARCHAR(64);
  DECLARE equip_level_max_col VARCHAR(64);
  DECLARE refineable_col VARCHAR(64);
  DECLARE gradable_col VARCHAR(64);
  DECLARE view_col VARCHAR(64);
  DECLARE id_expr TEXT;
  DECLARE item_name_expr TEXT;
  DECLARE aegis_name_expr TEXT;
  DECLARE item_type_expr TEXT;
  DECLARE item_subtype_expr TEXT;
  DECLARE slots_expr TEXT;
  DECLARE buy_expr TEXT;
  DECLARE sell_expr TEXT;
  DECLARE weight_expr TEXT;
  DECLARE attack_expr TEXT;
  DECLARE magic_attack_expr TEXT;
  DECLARE defense_expr TEXT;
  DECLARE range_expr TEXT;
  DECLARE weapon_level_expr TEXT;
  DECLARE armor_level_expr TEXT;
  DECLARE equip_level_min_expr TEXT;
  DECLARE equip_level_max_expr TEXT;
  DECLARE refineable_expr TEXT;
  DECLARE gradable_expr TEXT;
  DECLARE view_expr TEXT;
  DECLARE refresh_sql LONGTEXT;

  DECLARE source_cursor CURSOR FOR
    SELECT `table_name`
    FROM `information_schema`.`tables`
    WHERE `table_schema` = DATABASE()
      AND `table_type` = 'BASE TABLE'
      AND `table_name` IN ('item_db', 'item_db_re')
    ORDER BY FIELD(`table_name`, 'item_db', 'item_db_re');

  DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

  SET empty_string = CONCAT(CHAR(39), CHAR(39));
  SET unknown_item = CONCAT(CHAR(39), 'Objet inconnu', CHAR(39));
  SET unknown_aegis = CONCAT(CHAR(39), 'Unknown_Item', CHAR(39));

  OPEN source_cursor;

  read_loop: LOOP
    FETCH source_cursor INTO source_table;
    IF done THEN
      LEAVE read_loop;
    END IF;

    SET source_ref = CONCAT('`', REPLACE(source_table, '`', '``'), '`');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'id', 'item_id')), ',', 1)
      INTO id_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('id', 'item_id');

    IF id_col IS NULL THEN
      ITERATE read_loop;
    END IF;

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'name_english', 'name_japanese', 'name', 'name_aegis', 'aegis_name')), ',', 1)
      INTO item_name_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('name_english', 'name_japanese', 'name', 'name_aegis', 'aegis_name');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'name_aegis', 'aegis_name', 'name_japanese', 'name_english', 'name')), ',', 1)
      INTO aegis_name_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('name_aegis', 'aegis_name', 'name_japanese', 'name_english', 'name');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'type', 'item_type')), ',', 1)
      INTO item_type_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('type', 'item_type');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'subtype', 'item_subtype')), ',', 1)
      INTO item_subtype_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('subtype', 'item_subtype');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'slots', 'slot')), ',', 1)
      INTO slots_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('slots', 'slot');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'price_buy', 'buy')), ',', 1)
      INTO buy_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('price_buy', 'buy');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'price_sell', 'sell')), ',', 1)
      INTO sell_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('price_sell', 'sell');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'weight')), ',', 1)
      INTO weight_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('weight');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'attack', 'atk')), ',', 1)
      INTO attack_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('attack', 'atk');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'magic_attack', 'matk')), ',', 1)
      INTO magic_attack_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('magic_attack', 'matk');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'defense', 'def')), ',', 1)
      INTO defense_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('defense', 'def');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'range', 'item_range')), ',', 1)
      INTO range_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('range', 'item_range');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'weapon_level')), ',', 1)
      INTO weapon_level_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('weapon_level');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'armor_level')), ',', 1)
      INTO armor_level_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('armor_level');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'equip_level_min', 'elv', 'equip_level')), ',', 1)
      INTO equip_level_min_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('equip_level_min', 'elv', 'equip_level');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'equip_level_max')), ',', 1)
      INTO equip_level_max_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('equip_level_max');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'refineable')), ',', 1)
      INTO refineable_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('refineable');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'gradable')), ',', 1)
      INTO gradable_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('gradable');

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'view', 'view_id')), ',', 1)
      INTO view_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('view', 'view_id');

    SET id_expr = CONCAT('CAST(`', REPLACE(id_col, '`', '``'), '` AS UNSIGNED)');
    SET item_name_expr = IF(
      item_name_col IS NULL,
      unknown_item,
      CONCAT('COALESCE(NULLIF(TRIM(CAST(`', REPLACE(item_name_col, '`', '``'), '` AS CHAR)), ', empty_string, '), ', unknown_item, ')')
    );
    SET aegis_name_expr = IF(
      aegis_name_col IS NULL,
      unknown_aegis,
      CONCAT('COALESCE(NULLIF(TRIM(CAST(`', REPLACE(aegis_name_col, '`', '``'), '` AS CHAR)), ', empty_string, '), ', unknown_aegis, ')')
    );
    SET item_type_expr = IF(item_type_col IS NULL, 'NULL', CONCAT('NULLIF(TRIM(CAST(`', REPLACE(item_type_col, '`', '``'), '` AS CHAR)), ', empty_string, ')'));
    SET item_subtype_expr = IF(item_subtype_col IS NULL, 'NULL', CONCAT('NULLIF(TRIM(CAST(`', REPLACE(item_subtype_col, '`', '``'), '` AS CHAR)), ', empty_string, ')'));
    SET slots_expr = IF(slots_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(slots_col, '`', '``'), '` AS SIGNED)'));
    SET buy_expr = IF(buy_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(buy_col, '`', '``'), '` AS SIGNED)'));
    SET sell_expr = IF(sell_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(sell_col, '`', '``'), '` AS SIGNED)'));
    SET weight_expr = IF(weight_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(weight_col, '`', '``'), '` AS SIGNED)'));
    SET attack_expr = IF(attack_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(attack_col, '`', '``'), '` AS SIGNED)'));
    SET magic_attack_expr = IF(magic_attack_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(magic_attack_col, '`', '``'), '` AS SIGNED)'));
    SET defense_expr = IF(defense_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(defense_col, '`', '``'), '` AS SIGNED)'));
    SET range_expr = IF(range_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(range_col, '`', '``'), '` AS SIGNED)'));
    SET weapon_level_expr = IF(weapon_level_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(weapon_level_col, '`', '``'), '` AS SIGNED)'));
    SET armor_level_expr = IF(armor_level_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(armor_level_col, '`', '``'), '` AS SIGNED)'));
    SET equip_level_min_expr = IF(equip_level_min_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(equip_level_min_col, '`', '``'), '` AS SIGNED)'));
    SET equip_level_max_expr = IF(equip_level_max_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(equip_level_max_col, '`', '``'), '` AS SIGNED)'));
    SET refineable_expr = IF(refineable_col IS NULL, '0', CONCAT('CAST(COALESCE(`', REPLACE(refineable_col, '`', '``'), '`, 0) AS UNSIGNED)'));
    SET gradable_expr = IF(gradable_col IS NULL, '0', CONCAT('CAST(COALESCE(`', REPLACE(gradable_col, '`', '``'), '`, 0) AS UNSIGNED)'));
    SET view_expr = IF(view_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(view_col, '`', '``'), '` AS SIGNED)'));

    SET refresh_sql = CONCAT(
      'INSERT INTO `rathenafr_item_list` ',
      '(`item_id`, `item_name`, `aegis_name`, `item_type`, `item_subtype`, `slots`, `buy`, `sell`, `weight`, `attack`, `magic_attack`, `defense`, `item_range`, `weapon_level`, `armor_level`, `equip_level_min`, `equip_level_max`, `refineable`, `gradable`, `view_id`, `source_file`, `enabled`) ',
      'SELECT ', id_expr, ', ', item_name_expr, ', ', aegis_name_expr, ', ', item_type_expr, ', ', item_subtype_expr, ', ',
      slots_expr, ', ', buy_expr, ', ', sell_expr, ', ', weight_expr, ', ', attack_expr, ', ', magic_attack_expr, ', ',
      defense_expr, ', ', range_expr, ', ', weapon_level_expr, ', ', armor_level_expr, ', ', equip_level_min_expr, ', ',
      equip_level_max_expr, ', ', refineable_expr, ', ', gradable_expr, ', ', view_expr, ', ',
      QUOTE(source_table), ', 1 ',
      'FROM ', source_ref, ' ',
      'WHERE ', id_expr, ' IS NOT NULL ',
      'ON DUPLICATE KEY UPDATE ',
      '`item_name` = VALUES(`item_name`), ',
      '`aegis_name` = VALUES(`aegis_name`), ',
      '`item_type` = VALUES(`item_type`), ',
      '`item_subtype` = VALUES(`item_subtype`), ',
      '`slots` = VALUES(`slots`), ',
      '`buy` = VALUES(`buy`), ',
      '`sell` = VALUES(`sell`), ',
      '`weight` = VALUES(`weight`), ',
      '`attack` = VALUES(`attack`), ',
      '`magic_attack` = VALUES(`magic_attack`), ',
      '`defense` = VALUES(`defense`), ',
      '`item_range` = VALUES(`item_range`), ',
      '`weapon_level` = VALUES(`weapon_level`), ',
      '`armor_level` = VALUES(`armor_level`), ',
      '`equip_level_min` = VALUES(`equip_level_min`), ',
      '`equip_level_max` = VALUES(`equip_level_max`), ',
      '`refineable` = VALUES(`refineable`), ',
      '`gradable` = VALUES(`gradable`), ',
      '`view_id` = VALUES(`view_id`), ',
      '`source_file` = VALUES(`source_file`), ',
      '`enabled` = 1'
    );

    SET @rathenafr_item_catalog_refresh_sql = refresh_sql;
    PREPARE refresh_statement FROM @rathenafr_item_catalog_refresh_sql;
    EXECUTE refresh_statement;
    DEALLOCATE PREPARE refresh_statement;
  END LOOP;

  CLOSE source_cursor;
END//

DELIMITER ;

CALL `rathenafr_refresh_item_catalog`();

DROP PROCEDURE IF EXISTS `rathenafr_refresh_item_catalog`;

CREATE OR REPLACE SQL SECURITY INVOKER VIEW `rathenafr_item_search` AS
SELECT
  `item_id`,
  `aegis_name`,
  `item_name`,
  `item_type`,
  `item_subtype`,
  `buy`,
  `sell`,
  `weight`,
  `attack`,
  `magic_attack`,
  `defense`,
  `item_range`,
  `slots`,
  `weapon_level`,
  `armor_level`,
  `equip_level_min`,
  `equip_level_max`,
  `refineable`,
  `gradable`,
  `view_id`
FROM `rathenafr_item_list`
WHERE `enabled` = 1;
