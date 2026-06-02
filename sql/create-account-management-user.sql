-- rAthenaFR Discord Bot - permission optionnelle pour /createaccount
-- Executer ce fichier avec un utilisateur administrateur MariaDB/MySQL.
-- Remplacer le mot de passe avant execution.
-- Base cible par defaut : ragnarok
--
-- Cette release conserve uniquement /createaccount comme commande SQL d'ecriture.

CREATE USER IF NOT EXISTS 'rathenafr_bot'@'%' IDENTIFIED BY 'change_me_with_a_strong_password';

GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
GRANT INSERT ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';

FLUSH PRIVILEGES;
