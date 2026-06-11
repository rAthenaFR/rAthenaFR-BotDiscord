use super::*;
use crate::i18n::{BotLocale, TranslationArg};

pub fn guilds_embed(guilds: &[GuildSummary], requested_limit: u32) -> CreateEmbed {
    guilds_embed_l10n(BotLocale::DEFAULT, guilds, requested_limit)
}

pub fn guilds_embed_l10n(
    locale: BotLocale,
    guilds: &[GuildSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if guilds.is_empty() {
        return warning_embed(
            &ts(locale, "embed-guild-ranking-title"),
            ts(locale, "embed-guild-ranking-empty"),
        );
    }

    let list = limited_list(guilds, requested_limit, |index, guild| {
        format!(
            "`{:>2}.` **{}** — {} `{}` — {} `{}/{}` — {} `{}` — {} `{}`",
            index + 1,
            guild.name,
            ts(locale, "field-level"),
            guild.level,
            ts(locale, "field-members"),
            guild.members,
            guild.max_members,
            ts(locale, "field-online-members"),
            guild.online_members,
            ts(locale, "field-leader"),
            guild.master,
        )
    });

    info_embed(
        &ts(locale, "embed-guild-ranking-title"),
        ts(locale, "embed-guild-ranking-description"),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-guilds")),
        false,
    )
    .field(ts(locale, "field-guilds"), list.value, false)
}

pub fn guild_detail_embed(guild: &GuildDetails) -> CreateEmbed {
    guild_detail_embed_l10n(BotLocale::DEFAULT, guild)
}

pub fn guild_detail_embed_l10n(locale: BotLocale, guild: &GuildDetails) -> CreateEmbed {
    success_embed(
        &ts(locale, "embed-guild-profile-title"),
        tsa(
            locale,
            "embed-guild-profile-description",
            &[TranslationArg::new("guild", &guild.name)],
        ),
    )
    .field(ts(locale, "field-leader"), guild.master.clone(), true)
    .field(
        ts(locale, "field-level"),
        format!("`{}`", guild.level),
        true,
    )
    .field(
        ts(locale, "field-members"),
        format!("`{}/{}`", guild.members, guild.max_members),
        true,
    )
    .field(
        ts(locale, "field-online-members"),
        format!("`{}`", guild.online_members),
        true,
    )
    .field(
        ts(locale, "field-average-level"),
        format!("`{}`", guild.average_level),
        true,
    )
    .field(
        "EXP",
        format!(
            "`{}` / `{}`",
            format_number(guild.exp),
            format_number(guild.next_exp)
        ),
        true,
    )
}

pub fn guild_not_found_embed(name: &str) -> CreateEmbed {
    guild_not_found_embed_l10n(BotLocale::DEFAULT, name)
}

pub fn guild_not_found_embed_l10n(locale: BotLocale, name: &str) -> CreateEmbed {
    warning_embed(
        &ts(locale, "embed-guild-search-title"),
        tsa(
            locale,
            "embed-guild-not-found",
            &[TranslationArg::new("guild", name)],
        ),
    )
}

pub fn guild_members_embed(
    guild_name: &str,
    members: &[GuildMemberSummary],
    requested_limit: u32,
) -> CreateEmbed {
    guild_members_embed_l10n(BotLocale::DEFAULT, guild_name, members, requested_limit)
}

pub fn guild_members_embed_l10n(
    locale: BotLocale,
    guild_name: &str,
    members: &[GuildMemberSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if members.is_empty() {
        return warning_embed(
            &ts(locale, "embed-guild-members-title"),
            tsa(
                locale,
                "embed-guild-members-empty",
                &[TranslationArg::new("guild", guild_name)],
            ),
        );
    }

    let list = limited_list(members, requested_limit, |index, member| {
        let status = if member.online { "🟢" } else { "⚫" };
        format!(
            "`{:>2}.` {} **{}** — {} `{}` — Base `{}` / Job `{}` — {} — {} `{}`",
            index + 1,
            status,
            member.name,
            ts(locale, "field-position"),
            member.position,
            member.base_level,
            member.job_level,
            job_name(member.class_id),
            ts(locale, "field-map"),
            member.map,
        )
    });

    info_embed(
        &ts(locale, "embed-guild-members-title"),
        tsa(
            locale,
            "embed-guild-members-description",
            &[TranslationArg::new("guild", guild_name)],
        ),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-guild-members")),
        false,
    )
    .field(ts(locale, "field-members"), list.value, false)
}

pub fn map_stats_embed(
    entries: &[MapStatsEntry],
    online_only: bool,
    requested_limit: u32,
) -> CreateEmbed {
    map_stats_embed_l10n(BotLocale::DEFAULT, entries, online_only, requested_limit)
}

pub fn map_stats_embed_l10n(
    locale: BotLocale,
    entries: &[MapStatsEntry],
    online_only: bool,
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            &ts(locale, "embed-map-stats-title"),
            ts(locale, "embed-map-stats-empty"),
        );
    }

    let mode = if online_only {
        ts(locale, "text-online-characters-only")
    } else {
        ts(locale, "text-all-visible-characters")
    };
    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` `{}` — {} `{}` — {} `{}`",
            entry.rank,
            entry.map,
            ts(locale, "field-characters"),
            entry.characters,
            ts(locale, "field-online"),
            entry.online_characters,
        )
    });

    info_embed(
        &ts(locale, "embed-map-stats-title"),
        tsa(
            locale,
            "embed-map-stats-description",
            &[TranslationArg::new("mode", &mode)],
        ),
    )
    .field(
        ts(locale, "field-summary"),
        list_summary_l10n(locale, &list, &ts(locale, "noun-map-lines")),
        false,
    )
    .field(ts(locale, "field-maps"), list.value, false)
}
