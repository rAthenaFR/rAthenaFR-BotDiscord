-- rAthenaFR Discord Bot - permissions optionnelles pour gestion de comptes
-- Exécuter ce fichier avec un utilisateur administrateur MariaDB/MySQL.
-- Remplacer le mot de passe avant d’exécuter ce fichier.
-- Base cible par défaut : ragnarok
--
-- Ces droits sont nécessaires uniquement pour :
-- - /createaccount
-- - /accountmanage action:delete
--
-- La suppression directe est limitée côté bot aux comptes sans personnage.

CREATE USER IF NOT EXISTS 'rathenafr_bot'@'%' IDENTIFIED BY 'change_me_with_a_strong_password';

GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';
GRANT INSERT, DELETE ON `ragnarok`.`login` TO 'rathenafr_bot'@'%';

FLUSH PRIVILEGES;
