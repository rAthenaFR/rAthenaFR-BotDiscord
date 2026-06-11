use super::*;
use std::collections::HashMap;
use std::time::Duration;

fn lookup(values: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
    let values = values
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect::<HashMap<_, _>>();

    move |name| values.get(name).cloned()
}

#[test]
fn display_config_uses_defaults() {
    let config = DisplayConfig::from_lookup(&lookup(&[])).expect("display config");

    assert!(!config.hide_gm_characters);
    assert!(config.hide_gm_from_top);
    assert_eq!(config.hide_gm_group_from_ranking, 60);
    assert_eq!(config.default_limit, 10);
    assert_eq!(config.max_limit, 25);
    assert_eq!(config.clamp_limit(None), 10);
}

#[test]
fn display_config_clamps_limits_and_raises_maximum() {
    let config = DisplayConfig::from_lookup(&lookup(&[
        ("RATHENAFR_DEFAULT_LIMIT", "30"),
        ("RATHENAFR_MAX_LIMIT", "10"),
    ]))
    .expect("display config");

    assert_eq!(config.default_limit, 30);
    assert_eq!(config.max_limit, 30);
    assert_eq!(config.clamp_limit(Some(5)), 5);
    assert_eq!(config.clamp_limit(Some(500)), 30);
    assert_eq!(config.clamp_limit(Some(0)), 30);
}

#[test]
fn cache_config_is_enabled_by_default() {
    let config = CacheConfig::from_lookup(&lookup(&[])).expect("cache config");

    assert!(config.enabled);
    assert_eq!(config.ttl_seconds, None);
    assert_eq!(config.duration(10), Some(Duration::from_secs(10)));
}

#[test]
fn cache_config_can_be_disabled() {
    let config = CacheConfig::from_lookup(&lookup(&[("RATHENAFR_CACHE_ENABLED", "false")]))
        .expect("cache config");

    assert!(!config.enabled);
    assert_eq!(config.duration(10), None);
}

#[test]
fn cache_config_uses_global_ttl_override() {
    let config = CacheConfig::from_lookup(&lookup(&[
        ("RATHENAFR_CACHE_ENABLED", "true"),
        ("RATHENAFR_CACHE_TTL_SECONDS", "45"),
    ]))
    .expect("cache config");

    assert!(config.enabled);
    assert_eq!(config.ttl_seconds, Some(45));
    assert_eq!(config.duration(10), Some(Duration::from_secs(45)));
}

#[test]
fn cache_config_zero_ttl_disables_storage_duration() {
    let config = CacheConfig::from_lookup(&lookup(&[("RATHENAFR_CACHE_TTL_SECONDS", "0")]))
        .expect("cache config");

    assert_eq!(config.duration(10), None);
}

#[test]
fn server_rates_are_disabled_and_standard_by_default() {
    let config = ServerRatesConfig::from_lookup(&lookup(&[])).expect("rates config");

    assert!(!config.configured);
    assert_eq!(config.base_exp_rate, 100);
    assert_eq!(config.item_rate_common.normal, 100);
    assert_eq!(config.item_rate_common.boss, 100);
    assert_eq!(config.item_rate_common.mvp, 100);
    assert_eq!(config.item_drop_common.min, 1);
    assert_eq!(config.item_drop_common.max, 10_000);
}

#[test]
fn server_rates_read_rathena_overrides() {
    let config = ServerRatesConfig::from_lookup(&lookup(&[
        ("RATHENAFR_BATTLE_RATES_CONFIGURED", "true"),
        ("RATHENAFR_BATTLE_BASE_EXP_RATE", "1500"),
        ("RATHENAFR_BATTLE_ITEM_RATE_COMMON", "500"),
        ("RATHENAFR_BATTLE_ITEM_RATE_COMMON_BOSS", "300"),
        ("RATHENAFR_BATTLE_ITEM_RATE_COMMON_MVP", "100"),
        ("RATHENAFR_BATTLE_ITEM_DROP_COMMON_MAX", "7500"),
    ]))
    .expect("rates config");

    assert!(config.configured);
    assert_eq!(config.base_exp_rate, 1500);
    assert_eq!(config.item_rate_common.normal, 500);
    assert_eq!(config.item_rate_common.boss, 300);
    assert_eq!(config.item_rate_common.mvp, 100);
    assert_eq!(config.item_drop_common.max, 7500);
}

#[test]
fn server_rates_reject_invalid_drop_bounds() {
    let result = ServerRatesConfig::from_lookup(&lookup(&[
        ("RATHENAFR_BATTLE_ITEM_DROP_CARD_MIN", "500"),
        ("RATHENAFR_BATTLE_ITEM_DROP_CARD_MAX", "100"),
    ]));

    assert!(result.is_err());
}

#[test]
fn account_creation_is_disabled_by_default() {
    let config = AccountCommandsConfig::from_lookup(&lookup(&[])).expect("account config");

    assert!(!config.creation_enabled);
    assert_eq!(config.password_mode, AccountPasswordMode::Plain);
    assert!(!config.manage_enabled);
    assert!(!config.delete_enabled);
    assert_eq!(config.manage_min_role, StaffRole::Admin);
    assert_eq!(config.delete_min_role, StaffRole::Owner);
}

#[test]
fn account_creation_can_be_enabled() {
    let config = AccountCommandsConfig::from_lookup(&lookup(&[(
        "RATHENAFR_ACCOUNT_CREATION_ENABLED",
        "true",
    )]))
    .expect("account config");

    assert!(config.creation_enabled);
}

#[test]
fn account_password_mode_can_use_md5() {
    let config =
        AccountCommandsConfig::from_lookup(&lookup(&[("RATHENAFR_ACCOUNT_PASSWORD_MODE", "md5")]))
            .expect("account config");

    assert_eq!(config.password_mode, AccountPasswordMode::Md5);
}

#[test]
fn account_manage_config_can_be_enabled_and_relaxed() {
    let config = AccountCommandsConfig::from_lookup(&lookup(&[
        ("RATHENAFR_ACCOUNT_MANAGE_ENABLED", "true"),
        ("RATHENAFR_ACCOUNT_DELETE_ENABLED", "true"),
        ("RATHENAFR_ACCOUNT_MANAGE_MIN_ROLE", "gm"),
        ("RATHENAFR_ACCOUNT_DELETE_MIN_ROLE", "admin"),
    ]))
    .expect("account config");

    assert!(config.manage_enabled);
    assert!(config.delete_enabled);
    assert_eq!(config.manage_min_role, StaffRole::Gm);
    assert_eq!(config.delete_min_role, StaffRole::Admin);
}

#[test]
fn command_config_uses_release_defaults() {
    let config = CommandConfig::from_lookup(&lookup(&[])).expect("command config");

    assert!(config.public_pack_enabled);
    assert!(config.staff_pack_enabled);
    assert!(!config.online_list_public);
    assert_eq!(config.top_zeny_mode, TopZenyMode::Enabled);
    assert_eq!(config.item_table_name, "item_db");
    assert_eq!(config.mob_table_name, "mob_db");
    assert_eq!(config.gmmsg_min_role, StaffRole::Gm);
    assert_eq!(config.audit_min_role, StaffRole::Admin);
    assert!(config.command_enabled("staff inventory"));
}

#[test]
fn disabled_commands_are_case_insensitive() {
    let config = CommandConfig::from_lookup(&lookup(&[(
        "RATHENAFR_DISABLED_COMMANDS",
        "staff inventory,top zeny",
    )]))
    .expect("command config");

    assert!(!config.command_enabled("STAFF INVENTORY"));
    assert!(!config.command_enabled("top zeny"));
    assert!(config.command_enabled("staff player"));
}

#[test]
fn disabled_commands_can_target_nested_subcommands() {
    let config = CommandConfig::from_lookup(&lookup(&[(
        "RATHENAFR_DISABLED_COMMANDS",
        "staff account-manage delete,gmmsg server",
    )]))
    .expect("command config");

    assert!(!config.command_enabled("staff account-manage delete"));
    assert!(config.command_enabled("staff account-manage ban"));
    assert!(!config.command_enabled("gmmsg server"));
    assert!(config.command_enabled("gmmsg map"));
}

#[test]
fn disabled_commands_can_disable_nested_groups() {
    let config = CommandConfig::from_lookup(&lookup(&[(
        "RATHENAFR_DISABLED_COMMANDS",
        "staff account-manage",
    )]))
    .expect("command config");

    assert!(!config.command_enabled("staff account-manage delete"));
    assert!(!config.command_enabled("staff account-manage edit"));
    assert!(config.command_enabled("staff inventory"));
    assert!(config.command_enabled("staff"));
}

#[test]
fn game_bridge_is_disabled_by_default() {
    let config = GameBridgeConfig::from_lookup(&lookup(&[])).expect("bridge config");

    assert_eq!(config.mode, GameBridgeMode::Disabled);
    assert_eq!(config.max_message_length, 180);
    assert_eq!(config.encoding, GameBridgeEncoding::Windows1252);
}

#[test]
fn game_bridge_accepts_sql_queue_mode() {
    let config = GameBridgeConfig::from_lookup(&lookup(&[("RATHENAFR_GMMSG_MODE", "sql_queue")]))
        .expect("bridge config");

    assert_eq!(config.mode, GameBridgeMode::SqlQueue);
}

#[test]
fn game_bridge_accepts_utf8_encoding() {
    let config = GameBridgeConfig::from_lookup(&lookup(&[("RATHENAFR_GMMSG_ENCODING", "utf8")]))
        .expect("bridge config");

    assert_eq!(config.encoding, GameBridgeEncoding::Utf8);
}
