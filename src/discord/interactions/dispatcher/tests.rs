use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn staff_role_logic_requires_configured_matching_role() {
    assert!(!test_staff_role(&[10], &[], &[], &[]));
    assert!(!test_staff_role(&[10], &[20], &[], &[]));
    assert!(test_staff_role(&[10], &[10], &[], &[]));
    assert!(test_staff_role(&[30], &[], &[30], &[]));
    assert!(test_staff_role(&[40], &[], &[], &[40]));
}

#[test]
fn account_manage_permission_requires_configured_role() {
    let gm_roles = [30];
    let admin_roles = [40];
    let owner_roles = [50];
    let has_role = |member_role_ids: &[u64], minimum_role| {
        has_configured_role(
            member_role_ids,
            minimum_role,
            ConfiguredRoles {
                helper: &[],
                moderator: &[],
                gm: &gm_roles,
                legacy_staff: &[],
                admin: &admin_roles,
                owner: &owner_roles,
            },
        )
    };

    assert!(!has_role(&gm_roles, StaffRole::Admin));
    assert!(has_role(&admin_roles, StaffRole::Admin));
    assert!(has_role(&owner_roles, StaffRole::Admin));

    let config = test_account_commands_config(StaffRole::Gm, StaffRole::Owner, false);
    assert_eq!(account_manage::required_role(&config, "ban"), StaffRole::Gm);
    assert_eq!(
        account_manage::required_role(&config, "unban"),
        StaffRole::Gm
    );
    assert_eq!(
        account_manage::required_role(&config, "delete"),
        StaffRole::Owner
    );
}

#[test]
fn command_path_includes_nested_subcommands() {
    assert_eq!(
        command_path_from_parts("staff", &["account-manage", "delete"]),
        "staff account-manage delete"
    );
}

#[test]
fn command_path_keeps_first_level_subcommands() {
    assert_eq!(
        command_path_from_parts("gmmsg", &["server"]),
        "gmmsg server"
    );
}

#[test]
fn command_path_keeps_simple_commands() {
    assert_eq!(command_path_from_parts("server", &[]), "server");
}

#[test]
fn mvp_list_component_id_round_trips_page_state() {
    let custom_id = mvp_list_component_id("next", 3, 10);

    assert_eq!(
        parse_mvp_list_component_id(&custom_id),
        Some(MvpListPageRequest {
            page: 3,
            page_size: 10,
        })
    );
}

#[test]
fn mvp_list_component_id_rejects_invalid_state() {
    assert_eq!(parse_mvp_list_component_id("other:3:10"), None);
    assert_eq!(parse_mvp_list_component_id("mvp_list:3:10"), None);
    assert_eq!(parse_mvp_list_component_id("mvp_list:3:0:next"), None);
    assert_eq!(parse_mvp_list_component_id("mvp_list:3:10:unknown"), None);
    assert_eq!(
        parse_mvp_list_component_id("mvp_list:3:10:next:extra"),
        None
    );
}

#[test]
fn mvp_list_page_helpers_keep_page_in_range() {
    assert_eq!(mvp_list_page_count(61, 10), 7);
    assert_eq!(clamp_mvp_list_page(99, 61, 10), 6);
    assert_eq!(clamp_mvp_list_page(0, 0, 10), 0);
    assert_eq!(mvp_list_max_page_size(25), 10);
    assert_eq!(mvp_list_max_page_size(5), 5);
}

fn test_account_commands_config(
    manage_min_role: StaffRole,
    delete_min_role: StaffRole,
    delete_enabled: bool,
) -> crate::config::AccountCommandsConfig {
    crate::config::AccountCommandsConfig {
        creation_enabled: false,
        password_mode: crate::config::AccountPasswordMode::Plain,
        manage_enabled: true,
        delete_enabled,
        manage_min_role,
        delete_min_role,
    }
}

fn test_staff_role(
    member_role_ids: &[u64],
    staff_role_ids: &[u64],
    admin_role_ids: &[u64],
    owner_role_ids: &[u64],
) -> bool {
    has_configured_role(
        member_role_ids,
        StaffRole::Helper,
        ConfiguredRoles {
            helper: staff_role_ids,
            moderator: &[],
            gm: &[],
            legacy_staff: staff_role_ids,
            admin: admin_role_ids,
            owner: owner_role_ids,
        },
    )
}

#[test]
fn account_username_validation_is_strict() {
    assert_eq!(validate_account_username("User_123").unwrap(), "User_123");
    assert!(validate_account_username("abc").is_err());
    assert!(validate_account_username("invalid-name").is_err());
}

#[test]
fn account_birthdate_validation_is_strict() {
    assert_eq!(
        validate_account_birthdate(" 2000-02-29 ").unwrap(),
        "2000-02-29"
    );
    assert!(validate_account_birthdate("1899-12-31").is_err());
    assert!(validate_account_birthdate("2001-02-29").is_err());
    assert!(validate_account_birthdate("2001/02/28").is_err());
}

#[test]
fn account_email_defaults_and_validates() {
    assert_eq!(validate_account_email(None).unwrap(), "a@a.com");
    assert_eq!(
        validate_account_email(Some(" user@example.test ")).unwrap(),
        "user@example.test"
    );
    assert!(validate_account_email(Some("invalid")).is_err());
}

#[test]
fn gmmsg_success_log_result_uses_test_wording() {
    assert_eq!(
        gmmsg_success_log_result("server", "mode test: Broadcast: bonjour"),
        "Mode test actif : aucun message n’a été envoyé en jeu."
    );
    assert_eq!(
        gmmsg_success_log_result("server", "Message ajouté à la file d’envoi en jeu."),
        "Message ajouté à la file d’envoi en jeu."
    );
}

#[test]
fn gmmsg_error_log_result_uses_staff_wording() {
    assert_eq!(
        gmmsg_error_log_result(
            GameBridgeMode::SqlQueue,
            "La table `discord_gmmsg_queue` est absente. Exécutez le script SQL d’installation du bridge GMMSG."
        ),
        "La table `discord_gmmsg_queue` est absente."
    );
    assert_eq!(
        gmmsg_error_log_result(
            GameBridgeMode::Disabled,
            "Le bridge en jeu n’est pas configuré."
        ),
        "GMMSG est désactivé dans la configuration."
    );
    assert_eq!(
        gmmsg_error_log_result(
            GameBridgeMode::Bridge,
            "Le bridge en jeu n’est pas configuré : aucune implémentation map-server n’est active."
        ),
        "Le bridge en jeu n’est pas configuré."
    );
}

#[test]
fn sensitive_staff_commands_are_not_cacheable() {
    let sensitive_commands = [
        "createaccount",
        "staff",
        "mod",
        "debug",
        "audit",
        "db",
        "gmmsg",
    ];

    for command_name in sensitive_commands {
        assert!(
            !CACHED_COMMAND_NAMES.contains(&command_name),
            "{command_name} must not be cacheable"
        );
    }
}

#[tokio::test]
async fn cached_data_reuses_value_before_expiration() {
    let cache = TimedCache::<String, usize>::default();
    let calls = AtomicUsize::new(0);
    let key = "status".to_string();

    let first = cached_data(
        "status",
        key.clone(),
        Some(Duration::from_secs(1)),
        &cache,
        async { Ok(calls.fetch_add(1, Ordering::SeqCst) + 1) },
    )
    .await
    .expect("first value");

    let second = cached_data("status", key, Some(Duration::from_secs(1)), &cache, async {
        Ok(calls.fetch_add(1, Ordering::SeqCst) + 1)
    })
    .await
    .expect("cached value");

    assert_eq!(first, 1);
    assert_eq!(second, 1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn cached_data_fetches_again_when_disabled() {
    let cache = TimedCache::<String, usize>::default();
    let calls = AtomicUsize::new(0);

    let first = cached_data("status", "key".to_string(), None, &cache, async {
        Ok(calls.fetch_add(1, Ordering::SeqCst) + 1)
    })
    .await
    .expect("first value");

    let second = cached_data("status", "key".to_string(), None, &cache, async {
        Ok(calls.fetch_add(1, Ordering::SeqCst) + 1)
    })
    .await
    .expect("second value");

    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(cache.get(&"key".to_string()), None);
}
