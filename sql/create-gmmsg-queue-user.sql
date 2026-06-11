-- rAthenaFR Discord Bot - permission optionnelle pour /gmmsg en mode sql_queue
-- Executer ce fichier avec un utilisateur administrateur MariaDB/MySQL.
-- Remplacer le mot de passe avant execution.
-- Base cible par defaut : ragnarok
-- Si ta base rAthenaFR porte un autre nom, remplace `ragnarok` ci-dessous avant execution.
-- Les droits accordes par ce script s'ajoutent aux droits deja presents.

CREATE USER IF NOT EXISTS 'rathenafr_bot'@'%' IDENTIFIED BY 'change_me_with_a_strong_password';

GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`discord_gmmsg_queue` TO 'rathenafr_bot'@'%';

FLUSH PRIVILEGES;
