use super::*;

pub(super) fn sanitize_gm_message(
    value: &str,
    max_length: usize,
) -> std::result::Result<String, String> {
    sanitize_gm_message_l10n(BotLocale::DEFAULT, value, max_length)
}

pub(super) fn sanitize_gm_message_l10n(
    locale: BotLocale,
    value: &str,
    max_length: usize,
) -> std::result::Result<String, String> {
    let sanitized = value
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>()
        .replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if sanitized.trim().is_empty() {
        return Err(translate(locale, I18nKey::ErrorGmmsgEmpty));
    }

    if sanitized.chars().count() > max_length {
        return Err(translate_with_args(
            locale,
            I18nKey::ErrorGmmsgTooLong,
            &[TranslationArg::new("max", &max_length.to_string())],
        ));
    }

    Ok(sanitized)
}

pub(super) fn validate_hex_color(value: &str) -> std::result::Result<String, String> {
    validate_hex_color_l10n(BotLocale::DEFAULT, value)
}

pub(super) fn validate_hex_color_l10n(
    locale: BotLocale,
    value: &str,
) -> std::result::Result<String, String> {
    let value = value.trim().trim_start_matches('#');
    if value.len() == 6 && value.chars().all(|character| character.is_ascii_hexdigit()) {
        Ok(value.to_ascii_uppercase())
    } else {
        Err(translate(locale, I18nKey::ErrorGmmsgHexColor))
    }
}

pub(super) fn gmmsg_success_log_result(action: &str, details: &str) -> String {
    gmmsg_success_log_result_l10n(BotLocale::DEFAULT, action, details)
}

pub(super) fn gmmsg_success_log_result_l10n(
    locale: BotLocale,
    action: &str,
    details: &str,
) -> String {
    if action == "test" || details.to_ascii_lowercase().contains("mode test") {
        translate(locale, I18nKey::GmmsgTestNoSend)
    } else {
        details.to_string()
    }
}

pub(super) fn gmmsg_error_log_result(mode: GameBridgeMode, error: &str) -> String {
    gmmsg_error_log_result_l10n(BotLocale::DEFAULT, mode, error)
}

pub(super) fn gmmsg_error_log_result_l10n(
    locale: BotLocale,
    mode: GameBridgeMode,
    error: &str,
) -> String {
    if error.contains("discord_gmmsg_queue") && error.contains("absente") {
        return translate(locale, I18nKey::GmmsgMissingQueue);
    }

    if error.contains("non compatibles avec l’encodage Windows-1252") {
        return translate(locale, I18nKey::GmmsgEncodingUnsupported);
    }

    if mode == GameBridgeMode::Disabled && error.contains("Le bridge en jeu n’est pas configuré")
    {
        return translate(locale, I18nKey::GmmsgDisabled);
    }

    if error.contains("Le bridge en jeu n’est pas configuré")
        || error.contains("aucune implémentation map-server")
        || error.contains("bridge actuel")
    {
        return translate(locale, I18nKey::GmmsgBridgeNotConfigured);
    }

    error.to_string()
}

pub(super) fn trim_discord_message(value: &str) -> String {
    const MAX_EMBED_DESCRIPTION: usize = 3900;
    if value.chars().count() <= MAX_EMBED_DESCRIPTION {
        return value.to_string();
    }

    value
        .chars()
        .take(MAX_EMBED_DESCRIPTION.saturating_sub(20))
        .collect::<String>()
        + "\n... sortie tronquée"
}

pub(super) fn validate_account_username(value: &str) -> std::result::Result<String, String> {
    validate_account_username_l10n(BotLocale::DEFAULT, value)
}

pub(super) fn validate_account_username_l10n(
    locale: BotLocale,
    value: &str,
) -> std::result::Result<String, String> {
    let trimmed = value.trim();

    if !(4..=23).contains(&trimmed.len()) {
        return Err(translate(locale, I18nKey::ErrorAccountUsernameLength));
    }

    if !trimmed
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(translate(locale, I18nKey::ErrorAccountUsernameChars));
    }

    Ok(trimmed.to_string())
}

pub(super) fn validate_account_password(value: &str) -> std::result::Result<String, String> {
    validate_account_password_l10n(BotLocale::DEFAULT, value)
}

pub(super) fn validate_account_password_l10n(
    locale: BotLocale,
    value: &str,
) -> std::result::Result<String, String> {
    if !(8..=32).contains(&value.len()) {
        return Err(translate(locale, I18nKey::ErrorAccountPasswordLength));
    }

    if value
        .chars()
        .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(translate(locale, I18nKey::ErrorAccountPasswordChars));
    }

    Ok(value.to_string())
}

pub(super) fn validate_account_sex(value: &str) -> std::result::Result<String, String> {
    validate_account_sex_l10n(BotLocale::DEFAULT, value)
}

pub(super) fn validate_account_sex_l10n(
    locale: BotLocale,
    value: &str,
) -> std::result::Result<String, String> {
    match value.trim().to_ascii_uppercase().as_str() {
        "M" => Ok("M".to_string()),
        "F" => Ok("F".to_string()),
        _ => Err(translate(locale, I18nKey::ErrorAccountSex)),
    }
}

pub(super) fn validate_account_birthdate(value: &str) -> std::result::Result<String, String> {
    validate_account_birthdate_l10n(BotLocale::DEFAULT, value)
}

pub(super) fn validate_account_birthdate_l10n(
    locale: BotLocale,
    value: &str,
) -> std::result::Result<String, String> {
    let birthdate = value.trim();
    let birthdate_is_valid = if birthdate.len() == 10 {
        let parts = birthdate.split('-').collect::<Vec<_>>();

        if parts.len() == 3 && parts[0].len() == 4 && parts[1].len() == 2 && parts[2].len() == 2 {
            match (
                parts[0].parse::<u16>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                (Ok(year), Ok(month), Ok(day)) => {
                    let leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                    let max_day = match month {
                        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                        4 | 6 | 9 | 11 => 30,
                        2 if leap_year => 29,
                        2 => 28,
                        _ => 0,
                    };

                    year >= 1900 && max_day > 0 && day >= 1 && day <= max_day
                }
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    if !birthdate_is_valid {
        return Err(translate(locale, I18nKey::ErrorAccountBirthdate));
    }

    Ok(birthdate.to_string())
}

pub(super) fn validate_account_email(value: Option<&str>) -> std::result::Result<String, String> {
    validate_account_email_l10n(BotLocale::DEFAULT, value)
}

pub(super) fn validate_account_email_l10n(
    locale: BotLocale,
    value: Option<&str>,
) -> std::result::Result<String, String> {
    let email = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("a@a.com");

    if email.len() > 39 {
        return Err(translate(locale, I18nKey::ErrorAccountEmailLength));
    }

    if !email.contains('@') || email.chars().any(|character| character.is_control()) {
        return Err(translate(locale, I18nKey::ErrorAccountEmailInvalid));
    }

    Ok(email.to_string())
}
