-- rAthenaFR Discord Bot - verification SQL en lecture seule
-- Executer dans la base rAthena cible avec l'utilisateur du bot ou un
-- administrateur. Ce script ne modifie aucune donnee.

SELECT
  DATABASE() AS `database_name`,
  VERSION() AS `database_version`,
  CURRENT_USER() AS `sql_user`;

SELECT
  expected.`object_name`,
  expected.`expected_type`,
  COALESCE(actual.`table_type`, 'MISSING') AS `actual_type`
FROM (
  SELECT 'discord_gmmsg_queue' AS `object_name`, 'BASE TABLE' AS `expected_type`
  UNION ALL SELECT 'rathenafr_item_list', 'BASE TABLE'
  UNION ALL SELECT 'rathenafr_item_search', 'VIEW'
  UNION ALL SELECT 'rathenafr_mvp_list', 'BASE TABLE'
  UNION ALL SELECT 'rathenafr_mvp_regular_spawn', 'VIEW'
  UNION ALL SELECT 'sql_updates', 'BASE TABLE'
) expected
LEFT JOIN `information_schema`.`tables` actual
  ON actual.`table_schema` = DATABASE()
 AND actual.`table_name` = expected.`object_name`
ORDER BY expected.`object_name`;

SELECT
  expected.`table_name`,
  expected.`column_name`,
  CASE WHEN actual.`column_name` IS NULL THEN 'MISSING' ELSE actual.`column_type` END AS `status`
FROM (
  SELECT 'discord_gmmsg_queue' AS `table_name`, 'mode' AS `column_name`
  UNION ALL SELECT 'discord_gmmsg_queue', 'message'
  UNION ALL SELECT 'discord_gmmsg_queue', 'status'
  UNION ALL SELECT 'discord_gmmsg_queue', 'processed_at'
  UNION ALL SELECT 'rathenafr_item_list', 'item_id'
  UNION ALL SELECT 'rathenafr_item_list', 'enabled'
  UNION ALL SELECT 'rathenafr_item_search', 'item_name'
  UNION ALL SELECT 'rathenafr_item_search', 'aegis_name'
  UNION ALL SELECT 'rathenafr_mvp_list', 'monster_id'
  UNION ALL SELECT 'rathenafr_mvp_list', 'enabled'
  UNION ALL SELECT 'rathenafr_mvp_regular_spawn', 'respawn_minutes'
  UNION ALL SELECT 'sql_updates', 'revision'
) expected
LEFT JOIN `information_schema`.`columns` actual
  ON actual.`table_schema` = DATABASE()
 AND actual.`table_name` = expected.`table_name`
 AND actual.`column_name` = expected.`column_name`
ORDER BY expected.`table_name`, expected.`column_name`;
