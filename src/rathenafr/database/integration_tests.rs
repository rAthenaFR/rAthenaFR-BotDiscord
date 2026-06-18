//! Tests d'intégration exécutant de vraies requêtes SQL contre une base
//! MariaDB/MySQL jetable.
//!
//! Ils ne s'exécutent que si `RATHENAFR_TEST_DATABASE_URL` est défini et que le
//! nom de la base contient « test » (garde-fou anti-production : ces tests
//! créent et suppriment des tables). Sans cette variable, ils sont ignorés en
//! silence pour ne pas casser `cargo test` ni la CI sans base.
//!
//! Exemple local :
//! ```sh
//! docker run --rm -d --name rathenafr-it -e MARIADB_ROOT_PASSWORD=root \
//!   -e MARIADB_DATABASE=rathena_test -p 3310:3306 mariadb:11
//! RATHENAFR_TEST_DATABASE_URL="mysql://root:root@127.0.0.1:3310/rathena_test" \
//!   cargo test --test-threads=1 integration
//! ```

use super::*;
use sqlx::mysql::MySqlPoolOptions;

async fn connect_test_db() -> Option<RAthenaFrDatabase> {
    let url = std::env::var("RATHENAFR_TEST_DATABASE_URL").ok()?;

    let database_name = url
        .rsplit('/')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("");
    if !database_name.to_ascii_lowercase().contains("test") {
        eprintln!(
            "RATHENAFR_TEST_DATABASE_URL ignoré : le nom de base « {database_name} » ne contient pas « test » (garde-fou anti-production)."
        );
        return None;
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("connexion à la base de test");

    Some(RAthenaFrDatabase { pool })
}

async fn exec(database: &RAthenaFrDatabase, sql: &str) {
    sqlx::query(sql)
        .execute(&database.pool)
        .await
        .unwrap_or_else(|error| panic!("échec SQL « {sql} » : {error}"));
}

#[tokio::test]
async fn renewal_item_search_returns_modern_schema_rows() {
    let Some(database) = connect_test_db().await else {
        return;
    };

    exec(&database, "DROP TABLE IF EXISTS `item_db`").await;
    exec(&database, "DROP TABLE IF EXISTS `item_db_re`").await;
    exec(
        &database,
        "CREATE TABLE `item_db_re` (\
            `id` INT UNSIGNED NOT NULL PRIMARY KEY,\
            `name_aegis` VARCHAR(64) NOT NULL,\
            `name_english` VARCHAR(128) NOT NULL,\
            `type` VARCHAR(32) NULL)",
    )
    .await;
    exec(
        &database,
        "INSERT INTO `item_db_re` (`id`, `name_aegis`, `name_english`, `type`) VALUES \
            (501, 'Red_Potion', 'Red Potion', 'Healing'),\
            (1201, 'Knife', 'Knife', 'Weapon')",
    )
    .await;

    let by_name = database.search_items("Red", 10).await.unwrap();
    assert_eq!(by_name.len(), 1);
    assert_eq!(by_name[0].item_id, 501);
    assert_eq!(by_name[0].aegis_name, "Red_Potion");
    assert_eq!(by_name[0].display_name, "Red Potion");
    assert_eq!(by_name[0].item_type, "Healing");

    let by_id = database.search_items("1201", 10).await.unwrap();
    assert_eq!(by_id.len(), 1);
    assert_eq!(by_id[0].item_id, 1201);
    assert_eq!(by_id[0].aegis_name, "Knife");

    exec(&database, "DROP TABLE IF EXISTS `item_db_re`").await;
}

#[tokio::test]
async fn renewal_monster_search_exposes_aegis_name() {
    let Some(database) = connect_test_db().await else {
        return;
    };

    // mob_db_re moderne : pas de colonne `sprite`, l'aegis est `name_aegis`.
    exec(&database, "DROP TABLE IF EXISTS `mob_db`").await;
    exec(&database, "DROP TABLE IF EXISTS `mob_db_re`").await;
    exec(
        &database,
        "CREATE TABLE `mob_db_re` (\
            `id` INT UNSIGNED NOT NULL PRIMARY KEY,\
            `name_aegis` VARCHAR(64) NOT NULL,\
            `name_english` VARCHAR(128) NOT NULL,\
            `level` INT NOT NULL,\
            `hp` INT NOT NULL)",
    )
    .await;
    exec(
        &database,
        "INSERT INTO `mob_db_re` (`id`, `name_aegis`, `name_english`, `level`, `hp`) VALUES \
            (1002, 'PORING', 'Poring', 1, 55)",
    )
    .await;

    let results = database.search_monsters("Poring", 10).await.unwrap();
    assert_eq!(results.len(), 1);
    let poring = &results[0];
    assert_eq!(poring.monster_id, 1002);
    assert_eq!(poring.display_name, "Poring");
    // Vérifie la correction renewal : l'aegis remonte bien en l'absence de `sprite`.
    assert_eq!(poring.sprite, "PORING");
    assert_eq!(poring.level, 1);
    assert_eq!(poring.hp, 55);

    // Recherche par nom aegis (nouveau candidat de recherche).
    let by_aegis = database.search_monsters("PORING", 10).await.unwrap();
    assert_eq!(by_aegis.len(), 1);
    assert_eq!(by_aegis[0].monster_id, 1002);

    exec(&database, "DROP TABLE IF EXISTS `mob_db_re`").await;
}

#[tokio::test]
async fn database_status_counts_accounts_characters_and_guilds() {
    let Some(database) = connect_test_db().await else {
        return;
    };

    exec(&database, "DROP TABLE IF EXISTS `char`").await;
    exec(&database, "DROP TABLE IF EXISTS `login`").await;
    exec(&database, "DROP TABLE IF EXISTS `guild`").await;
    exec(
        &database,
        "CREATE TABLE `login` (\
            `account_id` INT UNSIGNED NOT NULL PRIMARY KEY,\
            `userid` VARCHAR(32) NOT NULL,\
            `group_id` INT NOT NULL DEFAULT 0)",
    )
    .await;
    exec(
        &database,
        "CREATE TABLE `char` (\
            `char_id` INT UNSIGNED NOT NULL PRIMARY KEY,\
            `account_id` INT UNSIGNED NOT NULL,\
            `name` VARCHAR(32) NOT NULL,\
            `online` TINYINT NOT NULL DEFAULT 0)",
    )
    .await;
    exec(
        &database,
        "CREATE TABLE `guild` (\
            `guild_id` INT UNSIGNED NOT NULL PRIMARY KEY,\
            `name` VARCHAR(32) NOT NULL)",
    )
    .await;
    // Compte joueur (group_id 0) + compte GM (group_id 99).
    exec(
        &database,
        "INSERT INTO `login` (`account_id`, `userid`, `group_id`) VALUES \
            (1, 'player', 0), (2, 'gm', 99)",
    )
    .await;
    exec(
        &database,
        "INSERT INTO `char` (`char_id`, `account_id`, `name`, `online`) VALUES \
            (10, 1, 'Hero', 1), (11, 2, 'GameMaster', 1)",
    )
    .await;
    exec(
        &database,
        "INSERT INTO `guild` (`guild_id`, `name`) VALUES (1, 'Alliance')",
    )
    .await;

    // Seuil de groupe à 10 : le GM (group_id 99) est exclu des comptes/persos.
    let status = database.database_status(10).await.unwrap();
    assert_eq!(status.accounts, 1);
    assert_eq!(status.characters, 1);
    assert_eq!(status.online_characters, 1);
    assert_eq!(status.guilds, 1);
    assert!(!status.database_engine.is_empty());

    exec(&database, "DROP TABLE IF EXISTS `char`").await;
    exec(&database, "DROP TABLE IF EXISTS `login`").await;
    exec(&database, "DROP TABLE IF EXISTS `guild`").await;
}
