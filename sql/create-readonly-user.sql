-- rAthenaFR Discord Bot - utilisateur SQL en lecture seule
-- Exécuter ce fichier avec un utilisateur administrateur MariaDB/MySQL.
-- Remplacer le mot de passe avant d’exécuter ce fichier.
-- Base cible par défaut : ragnarok
-- Si ta base rAthenaFR porte un autre nom, remplace `ragnarok` ci-dessous avant exécution.
-- Ce script ne retire pas les droits deja accordes a un utilisateur existant.

CREATE USER IF NOT EXISTS 'rathenafr_bot'@'%' IDENTIFIED BY 'change_me_with_a_strong_password';

GRANT SELECT ON `ragnarok`.* TO 'rathenafr_bot'@'%';

FLUSH PRIVILEGES;
