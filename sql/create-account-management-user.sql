-- rAthenaFR Discord Bot - permissions optionnelles pour /createaccount et /staff account-manage
-- Executer ce fichier avec un utilisateur administrateur MariaDB/MySQL.
-- Remplacer le mot de passe avant execution.
-- Base cible par defaut : ragnarok
--
-- /createaccount utilise INSERT sur login.
-- /staff account-manage utilise UPDATE cible sur login si la commande est activee.
-- Les droits accordes par ce script s'ajoutent aux droits deja presents.

CREATE USER IF NOT EXISTS 'rathenafr_bot'@'%' IDENTIFIED BY 'change_me_with_a_strong_password';

GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';
GRANT UPDATE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';

FLUSH PRIVILEGES;
