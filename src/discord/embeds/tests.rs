use super::*;

#[test]
fn limited_list_respects_requested_limit() {
    let items = vec![1, 2, 3];

    let list = limited_list(&items, 2, |_index, item| format!("Ligne {item}"));

    assert_eq!(list.value, "Ligne 1\n\nLigne 2");
    assert_eq!(list.displayed_count, 2);
    assert_eq!(list.available_count, 3);
    assert_eq!(
        list_summary(&list, "lignes"),
        "2 affiché(s) sur au moins 3 lignes. Masqué par la limite d’affichage configurée."
    );
}

#[test]
fn limited_list_summary_uses_exact_total_when_all_rows_fit() {
    let items = vec![1, 2];

    let list = limited_list(&items, 5, |_index, item| format!("Ligne {item}"));

    assert_eq!(list.value, "Ligne 1\n\nLigne 2");
    assert_eq!(list_summary(&list, "lignes"), "2 affiché(s) sur 2 lignes.");
}

#[test]
fn limited_list_formats_details_as_bullets() {
    let items = vec![1];

    let list = limited_list(&items, 1, |_index, _item| {
        "**Alice** — Base `99` — Carte `prontera`".to_string()
    });

    assert_eq!(list.value, "**Alice**\n• Base `99`\n• Carte `prontera`");
}

#[test]
fn limited_list_reports_discord_field_truncation() {
    let items = vec![1, 2];
    let long_text = "x".repeat(EMBED_FIELD_VALUE_LIMIT);

    let list = limited_list(&items, 5, |_index, _item| long_text.clone());

    assert_eq!(list.displayed_count, 1);
    assert_eq!(list.available_count, 2);
    assert!(list_summary(&list, "lignes").contains("les limites de champ des embeds Discord"));
}

#[test]
fn format_number_fr_uses_spaces() {
    assert_eq!(format_number_fr(2_142_720), "2 142 720");
    assert_eq!(format_number_fr(-12_345), "-12 345");
}

#[test]
fn drop_rates_use_discord_friendly_percentages() {
    assert_eq!(format_drop_rate(100.0), "100%");
    assert_eq!(format_drop_rate(7.5), "7.50%");
    assert_eq!(format_drop_rate(0.03), "0.03%");
}

#[test]
fn mob_drop_field_uses_french_labels_and_fallbacks() {
    let complete = MonsterDropEntry {
        item_id: Some(999),
        item_name: "Steel".to_string(),
        aegis_name: Some("Steel".to_string()),
        server_rate: Some(7.5),
    };
    let unavailable = MonsterDropEntry {
        item_id: None,
        item_name: "Non disponible".to_string(),
        aegis_name: None,
        server_rate: None,
    };

    assert_eq!(
        mob_drop_field_value(&complete),
        "ID : 999\nAegisName : Steel\nTaux serveur : 7.50%"
    );
    assert_eq!(
        mob_drop_field_value(&unavailable),
        "ID : Non disponible\nAegisName : Non disponible\nTaux serveur : Non disponible"
    );
}

#[test]
fn mvp_kill_field_uses_discord_timestamps_and_french_fallbacks() {
    let entry = MvpKillEntry {
        mvp_date: Some("2026-06-03 16:25:02".to_string()),
        mvp_timestamp: Some(1_717_424_702),
        killer_id: 42,
        killer_name: "GhostPunishR".to_string(),
        monster_id: 1272,
        monster_name: "Doppelganger".to_string(),
        monster_aegis_name: Some("DOPPELGANGER".to_string()),
        map: "gl_chyard".to_string(),
        mvp_exp: Some(2_142_720),
        prize_id: 607,
        prize_name: "Yggdrasil Berry".to_string(),
        prize_aegis_name: Some("Yggdrasilberry".to_string()),
    };

    let value = mvp_kill_field_value(&entry);

    assert!(value.contains("Joueur : GhostPunishR"));
    assert!(value.contains("Carte : `gl_chyard`"));
    assert!(value.contains("Date : <t:1717424702:F> (<t:1717424702:R>)"));
    assert!(value.contains("EXP MVP attribuée : 2 142 720"));
    assert!(value.contains("Récompense : Yggdrasil Berry"));
}

#[test]
fn mvp_kill_field_marks_missing_exp_as_unavailable() {
    let entry = MvpKillEntry {
        mvp_date: None,
        mvp_timestamp: None,
        killer_id: 42,
        killer_name: "Personnage #42".to_string(),
        monster_id: 1272,
        monster_name: "MVP #1272".to_string(),
        monster_aegis_name: None,
        map: "Carte inconnue".to_string(),
        mvp_exp: Some(0),
        prize_id: 0,
        prize_name: "Item #0".to_string(),
        prize_aegis_name: None,
    };

    let value = mvp_kill_field_value(&entry);

    assert!(value.contains("Date : Non disponible"));
    assert!(value.contains("EXP MVP attribuée : Non disponible"));
    assert!(value.contains("Récompense : Item #0"));
}

#[test]
fn gmmsg_log_mentions_are_neutralized() {
    assert_eq!(
        sanitize_embed_mentions("@everyone @here test"),
        "@\u{200B}everyone @\u{200B}here test"
    );
}

#[test]
fn gmmsg_log_message_is_truncated_cleanly() {
    let message = "a".repeat(GMMSG_LOG_MESSAGE_LIMIT + 20);
    let truncated = truncate_embed_field(&message, GMMSG_LOG_MESSAGE_LIMIT);

    assert_eq!(truncated.chars().count(), GMMSG_LOG_MESSAGE_LIMIT);
    assert!(truncated.ends_with('…'));
}
