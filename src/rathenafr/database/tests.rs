use super::*;

#[test]
fn available_columns_resolve_candidates_case_insensitively() {
    let columns = AvailableColumns {
        names: vec![
            "id".to_string(),
            "name_english".to_string(),
            "LV".to_string(),
        ],
    };

    assert_eq!(columns.first(&["ID"]).as_deref(), Some("id"));
    assert_eq!(columns.first(&["level", "lv"]).as_deref(), Some("LV"));
    assert_eq!(columns.all(&["name_english", "NAME_ENGLISH"]).len(), 1);
}

#[test]
fn quote_identifier_escapes_backticks() {
    assert_eq!(quote_identifier("safe_name"), "`safe_name`");
    assert_eq!(quote_identifier("bad`name"), "`bad``name`");
}

#[test]
fn guild_queries_count_online_members_from_joined_char_table() {
    for sql in [TOP_GUILDS_SQL, FIND_GUILD_SQL] {
        assert!(sql.contains("LEFT JOIN `guild_member` gm ON gm.guild_id = g.guild_id"));
        assert!(sql.contains("LEFT JOIN `char` ON `char`.`char_id` = gm.char_id"));
        assert!(sql.contains("`char`.`online` > 0"));
        assert!(sql.contains("gm.char_id"));
        assert!(!sql.contains("connect_member"));
        assert!(!sql.contains("c.guild_id"));
    }
}

#[test]
fn monster_aegis_name_is_detected_on_modern_renewal_schema() {
    // mob_db_re moderne (rAthena 2020+) : pas de colonne `sprite`, l'identifiant
    // aegis est `name_aegis` et l'affichage `name_english`.
    let columns = AvailableColumns {
        names: vec![
            "id".to_string(),
            "name_aegis".to_string(),
            "name_english".to_string(),
            "level".to_string(),
            "hp".to_string(),
        ],
    };

    assert_eq!(
        columns.first(MONSTER_SPRITE_COLUMN_CANDIDATES).as_deref(),
        Some("name_aegis")
    );
    assert_eq!(
        columns.first(MONSTER_DISPLAY_COLUMN_CANDIDATES).as_deref(),
        Some("name_english")
    );
    assert!(columns
        .all(MONSTER_DISPLAY_COLUMN_CANDIDATES)
        .iter()
        .any(|name| name == "name_aegis"));
}

#[test]
fn mvp_detection_prefers_configured_mvp_exp_columns() {
    let columns = AvailableColumns {
        names: vec![
            "id".to_string(),
            "MEXP".to_string(),
            "mvp_exp".to_string(),
            "mvpdrop1_item".to_string(),
        ],
    };

    assert_eq!(
        columns.first(MOB_MVP_EXP_COLUMNS).as_deref(),
        Some("mvp_exp")
    );
    assert_eq!(mvp_drop_id_columns(&columns), vec!["mvpdrop1_item"]);
}

#[test]
fn drop_columns_detect_current_rathena_renewal_schema() {
    let columns = AvailableColumns {
        names: vec![
            "drop1_item".to_string(),
            "drop1_rate".to_string(),
            "mvpdrop1_item".to_string(),
            "mvpdrop1_rate".to_string(),
        ],
    };

    assert_eq!(
        drop_column_pairs(&columns),
        vec![
            (
                "drop1_item".to_string(),
                Some("drop1_rate".to_string()),
                "Drop 1".to_string(),
            ),
            (
                "mvpdrop1_item".to_string(),
                Some("mvpdrop1_rate".to_string()),
                "MVP drop 1".to_string(),
            ),
        ]
    );
}

#[test]
fn drop_item_match_supports_aegis_names_and_legacy_ids() {
    let condition = drop_item_match_condition("drop1_item");

    assert!(condition.contains("BINARY CAST(`drop1_item` AS CHAR) = BINARY ?"));
    assert!(condition.contains("CAST(`drop1_item` AS SIGNED) = ?"));
}

fn configured_server_rates() -> ServerRatesConfig {
    let mut rates = ServerRatesConfig {
        configured: true,
        base_exp_rate: 1500,
        job_exp_rate: 1500,
        mvp_exp_rate: 1000,
        ..ServerRatesConfig::default()
    };
    rates.item_rate_common.normal = 500;
    rates.item_rate_common.boss = 300;
    rates.item_rate_equip.normal = 400;
    rates.item_rate_equip.boss = 200;
    rates.item_rate_card.normal = 300;
    rates.item_rate_mvp = 200;
    rates
}

#[test]
fn drop_rates_apply_server_category_and_caps() {
    let rates = configured_server_rates();

    assert_eq!(
        server_drop_rate(Some(7_000), "Etc", MonsterRateKind::Normal, false, &rates,),
        Some(100.0)
    );
    assert_eq!(
        server_drop_rate(Some(400), "Weapon", MonsterRateKind::Boss, false, &rates,),
        Some(8.0)
    );
    assert_eq!(
        server_drop_rate(Some(6_000), "Etc", MonsterRateKind::Mvp, true, &rates,),
        Some(100.0)
    );
}

#[test]
fn drop_rates_do_not_expose_raw_sql_when_not_configured() {
    let value = server_drop_rate(
        Some(7_000),
        "Etc",
        MonsterRateKind::Normal,
        false,
        &ServerRatesConfig::default(),
    );

    assert_eq!(value, None);
}

#[test]
fn drop_rates_are_hidden_when_item_category_is_unknown() {
    let value = server_drop_rate(
        Some(500),
        "",
        MonsterRateKind::Normal,
        false,
        &configured_server_rates(),
    );

    assert_eq!(value, None);
}

#[test]
fn exp_rates_match_rathena_percentage_scale() {
    assert_eq!(apply_exp_rate(100, 1500), 1_500);
    assert_eq!(apply_exp_rate(214_272, 1000), 2_142_720);
}

#[test]
fn mvp_log_columns_detect_common_rathena_schema() {
    let columns = AvailableColumns {
        names: vec![
            "mvp_date".to_string(),
            "kill_char_id".to_string(),
            "monster_id".to_string(),
            "mvpexp".to_string(),
            "map".to_string(),
            "prize".to_string(),
        ],
    };
    let detected = mvp_log_columns(&columns);

    assert_eq!(detected.date.as_deref(), Some("mvp_date"));
    assert_eq!(detected.killer_id.as_deref(), Some("kill_char_id"));
}

#[test]
fn mvp_timer_formats_waiting_state_with_discord_timestamps() {
    let timer = MvpTimerRow {
        monster_id: 1511,
        monster_name: "Amon Ra".to_string(),
        map_name: "moc_pryd06".to_string(),
        respawn_minutes: 60,
        respawn_variance_minutes: 10,
        last_kill_ts: Some(1_700_000_000),
        earliest_spawn_ts: Some(1_700_003_600),
        latest_spawn_ts: Some(1_700_004_200),
        spawn_state: "waiting".to_string(),
    };

    let line = format_mvp_timer_line(&timer);

    assert!(line.contains("Dernier kill : <t:1700000000:R>"));
    assert!(line.contains("Respawn au plus tôt : <t:1700003600:R>"));
    assert!(line.contains("Respawn au plus tard : <t:1700004200:R>"));
    assert!(line.contains("Statut : En attente"));
    assert!(!line.contains("`<t:"));
    assert!(!line.contains("60 min"));
}

#[test]
fn mvp_timer_formats_window_and_available_states() {
    let mut timer = MvpTimerRow {
        monster_id: 1511,
        monster_name: "Amon Ra".to_string(),
        map_name: "moc_pryd06".to_string(),
        respawn_minutes: 60,
        respawn_variance_minutes: 10,
        last_kill_ts: Some(1_700_000_000),
        earliest_spawn_ts: Some(1_700_003_600),
        latest_spawn_ts: Some(1_700_004_200),
        spawn_state: "window".to_string(),
    };

    let window = format_mvp_timer_line(&timer);
    assert!(window.contains("Fenêtre ouverte depuis : <t:1700003600:R>"));
    assert!(window.contains("Respawn maximum : <t:1700004200:R>"));
    assert!(window.contains("Statut : Fenêtre de respawn ouverte"));

    timer.spawn_state = "available".to_string();
    let available = format_mvp_timer_line(&timer);
    assert!(available.contains("Respawn maximum dépassé depuis : <t:1700004200:R>"));
    assert!(available.contains("Statut : Disponible probable"));
}

#[test]
fn mvp_timer_formats_unknown_state_with_static_respawn() {
    let timer = MvpTimerRow {
        monster_id: 1511,
        monster_name: "Amon Ra".to_string(),
        map_name: "moc_pryd06".to_string(),
        respawn_minutes: 60,
        respawn_variance_minutes: 10,
        last_kill_ts: None,
        earliest_spawn_ts: None,
        latest_spawn_ts: None,
        spawn_state: "unknown".to_string(),
    };

    let line = format_mvp_timer_line(&timer);

    assert!(line.contains("Dernier kill : Aucun log connu"));
    assert!(line.contains("Respawn : 60 min ± 10 min"));
    assert!(line.contains("Statut : Timer inconnu, MVP peut être disponible"));
}

#[test]
fn mvp_killer_expression_uses_backticked_char_table() {
    let columns = MvpLogColumns {
        killer_id: Some("kill_char_id".to_string()),
        killer_name: None,
        date: None,
    };
    let expression = mvp_killer_name_expression(&columns, true);

    assert!(expression.contains("`char`.`name`"));
    assert!(expression.contains("ml.`kill_char_id`"));
}

#[test]
fn parse_search_id_only_accepts_positive_numeric_queries() {
    assert_eq!(parse_search_id("501"), Some(501));
    assert_eq!(parse_search_id("  000501  "), Some(501));
    assert_eq!(parse_search_id("Poring"), None);
    assert_eq!(parse_search_id("501a"), None);
    assert_eq!(parse_search_id("0"), None);
}
