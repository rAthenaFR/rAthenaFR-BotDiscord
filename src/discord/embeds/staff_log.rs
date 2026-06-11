use super::*;
use crate::i18n::BotLocale;

pub fn gmmsg_staff_log_embed(
    status: GmmsgLogStatus,
    discord_user_id: u64,
    action: &str,
    message: &str,
    result: &str,
) -> CreateEmbed {
    gmmsg_staff_log_embed_l10n(
        BotLocale::DEFAULT,
        status,
        discord_user_id,
        action,
        message,
        result,
    )
}

pub fn gmmsg_staff_log_embed_l10n(
    locale: BotLocale,
    status: GmmsgLogStatus,
    discord_user_id: u64,
    action: &str,
    message: &str,
    result: &str,
) -> CreateEmbed {
    let (title, description, color, result_field) = match status {
        GmmsgLogStatus::Sent => (
            ts(locale, "embed-gmmsg-log-sent-title"),
            ts(locale, "embed-gmmsg-log-sent-description"),
            COLOR_SUCCESS,
            ts(locale, "field-result"),
        ),
        GmmsgLogStatus::Failed => (
            ts(locale, "embed-gmmsg-log-failed-title"),
            ts(locale, "embed-gmmsg-log-failed-description"),
            COLOR_ERROR,
            ts(locale, "field-error"),
        ),
    };

    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .field(
            ts(locale, "field-user"),
            format!("ID : <@{}>", discord_user_id),
            false,
        )
        .field(
            ts(locale, "field-action"),
            format!("`{}`", sanitize_embed_mentions(action)),
            true,
        )
        .field(
            ts(locale, "field-message"),
            truncate_embed_field(&sanitize_embed_mentions(message), GMMSG_LOG_MESSAGE_LIMIT),
            false,
        )
        .field(
            result_field,
            truncate_embed_field(&sanitize_embed_mentions(result), EMBED_FIELD_VALUE_LIMIT),
            false,
        )
        .footer(serenity::all::CreateEmbedFooter::new(
            "rAthenaFR-BotDiscord • GMMSG",
        ))
        .timestamp(Timestamp::now())
}

pub fn account_manage_staff_log_embed(
    status: AccountManageLogStatus,
    discord_user_id: u64,
    action: &str,
    account: &str,
    result: &str,
    reason: Option<&str>,
) -> CreateEmbed {
    account_manage_staff_log_embed_l10n(
        BotLocale::DEFAULT,
        status,
        discord_user_id,
        action,
        account,
        result,
        reason,
    )
}

pub fn account_manage_staff_log_embed_l10n(
    locale: BotLocale,
    status: AccountManageLogStatus,
    discord_user_id: u64,
    action: &str,
    account: &str,
    result: &str,
    reason: Option<&str>,
) -> CreateEmbed {
    let (title, color, result_field) = match status {
        AccountManageLogStatus::Success => (
            ts(locale, "embed-account-log-success-title"),
            COLOR_SUCCESS,
            ts(locale, "field-result"),
        ),
        AccountManageLogStatus::Refused | AccountManageLogStatus::Failed => (
            ts(locale, "embed-account-log-refused-title"),
            COLOR_ERROR,
            ts(locale, "field-error"),
        ),
    };

    let mut embed = CreateEmbed::new()
        .title(title)
        .color(color)
        .field(
            ts(locale, "field-staff-user"),
            format!("ID : <@{}>", discord_user_id),
            false,
        )
        .field(
            ts(locale, "field-action"),
            format!("`{}`", sanitize_embed_mentions(action)),
            true,
        )
        .field(
            ts(locale, "field-account"),
            truncate_embed_field(&sanitize_embed_mentions(account), EMBED_FIELD_VALUE_LIMIT),
            true,
        )
        .field(
            result_field,
            truncate_embed_field(&sanitize_embed_mentions(result), EMBED_FIELD_VALUE_LIMIT),
            false,
        );

    if status == AccountManageLogStatus::Success {
        if let Some(reason) = reason.map(str::trim).filter(|value| !value.is_empty()) {
            embed = embed.field(
                ts(locale, "field-reason"),
                truncate_embed_field(&sanitize_embed_mentions(reason), EMBED_FIELD_VALUE_LIMIT),
                false,
            );
        }
    }

    embed
        .footer(serenity::all::CreateEmbedFooter::new(
            "rAthenaFR-BotDiscord • Account Manage",
        ))
        .timestamp(Timestamp::now())
}
