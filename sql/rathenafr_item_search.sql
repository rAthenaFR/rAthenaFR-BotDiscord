-- rAthenaFR Discord Bot - support SQL pour /item info.
-- Executer ce fichier dans la base rAthena avec un utilisateur administrateur.
-- Le script cree la table lue par le bot et la rafraichit depuis item_db/item_db_re.

CREATE TABLE IF NOT EXISTS `rathenafr_item_search` (
  `item_id` INT UNSIGNED NOT NULL,
  `item_name` VARCHAR(100) NOT NULL,
  `aegis_name` VARCHAR(100) NOT NULL,
  `item_type` VARCHAR(50) DEFAULT NULL,
  `item_subtype` VARCHAR(50) DEFAULT NULL,
  `slots` SMALLINT DEFAULT NULL,
  `buy` INT DEFAULT NULL,
  `sell` INT DEFAULT NULL,
  `weight` INT DEFAULT NULL,
  `attack` INT DEFAULT NULL,
  `magic_attack` INT DEFAULT NULL,
  `defense` INT DEFAULT NULL,
  `equip_level_min` INT DEFAULT NULL,
  PRIMARY KEY (`item_id`),
  KEY `idx_rathenafr_item_search_name` (`item_name`),
  KEY `idx_rathenafr_item_search_aegis` (`aegis_name`),
  KEY `idx_rathenafr_item_search_type` (`item_type`, `item_subtype`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

DROP PROCEDURE IF EXISTS `rathenafr_refresh_item_search`;

DELIMITER //

CREATE PROCEDURE `rathenafr_refresh_item_search`()
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
  DECLARE equip_level_min_col VARCHAR(64);
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
  DECLARE equip_level_min_expr TEXT;
  DECLARE refresh_sql LONGTEXT;

  DECLARE source_cursor CURSOR FOR
    SELECT `table_name`
    FROM `information_schema`.`tables`
    WHERE `table_schema` = DATABASE()
      AND `table_name` IN ('item_db', 'item_db_re')
    ORDER BY FIELD(`table_name`, 'item_db', 'item_db_re');

  DECLARE CONTINUE HANDLER FOR NOT FOUND SET done = TRUE;

  SET empty_string = CONCAT(CHAR(39), CHAR(39));
  SET unknown_item = CONCAT(CHAR(39), 'Objet inconnu', CHAR(39));
  SET unknown_aegis = CONCAT(CHAR(39), 'Unknown_Item', CHAR(39));

  TRUNCATE TABLE `rathenafr_item_search`;

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

    SELECT SUBSTRING_INDEX(GROUP_CONCAT(`column_name` ORDER BY FIELD(`column_name`, 'equip_level_min', 'elv', 'equip_level')), ',', 1)
      INTO equip_level_min_col
    FROM `information_schema`.`columns`
    WHERE `table_schema` = DATABASE()
      AND `table_name` = source_table
      AND `column_name` IN ('equip_level_min', 'elv', 'equip_level');

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
    SET equip_level_min_expr = IF(equip_level_min_col IS NULL, 'NULL', CONCAT('CAST(`', REPLACE(equip_level_min_col, '`', '``'), '` AS SIGNED)'));

    SET refresh_sql = CONCAT(
      'INSERT INTO `rathenafr_item_search` ',
      '(`item_id`, `item_name`, `aegis_name`, `item_type`, `item_subtype`, `slots`, `buy`, `sell`, `weight`, `attack`, `magic_attack`, `defense`, `equip_level_min`) ',
      'SELECT ', id_expr, ', ', item_name_expr, ', ', aegis_name_expr, ', ', item_type_expr, ', ', item_subtype_expr, ', ',
      slots_expr, ', ', buy_expr, ', ', sell_expr, ', ', weight_expr, ', ', attack_expr, ', ', magic_attack_expr, ', ',
      defense_expr, ', ', equip_level_min_expr,
      ' FROM ', source_ref,
      ' WHERE ', id_expr, ' IS NOT NULL ',
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
      '`equip_level_min` = VALUES(`equip_level_min`)'
    );

    SET @rathenafr_item_search_refresh_sql = refresh_sql;
    PREPARE refresh_statement FROM @rathenafr_item_search_refresh_sql;
    EXECUTE refresh_statement;
    DEALLOCATE PREPARE refresh_statement;
  END LOOP;

  CLOSE source_cursor;
END//

DELIMITER ;

CALL `rathenafr_refresh_item_search`();

DROP PROCEDURE IF EXISTS `rathenafr_refresh_item_search`;
