use crate::rathenafr::*;
use serenity::all::{Colour, CreateEmbed, Timestamp};

const COLOR_SUCCESS: Colour = Colour::new(0x57F287);
const COLOR_WARNING: Colour = Colour::new(0xFEE75C);
const COLOR_ERROR: Colour = Colour::new(0xED4245);
const COLOR_INFO: Colour = Colour::new(0x5865F2);
const COLOR_PURPLE: Colour = Colour::new(0x9B59B6);
const EMBED_FIELD_VALUE_LIMIT: usize = 1000;

struct LimitedList {
    value: String,
    displayed_count: usize,
    available_count: usize,
    row_limit: usize,
}

pub fn command_error_embed(details: &str) -> CreateEmbed {
    let lower = details.to_ascii_lowercase();

    if lower.contains("doesn't exist")
        || lower.contains("does not exist")
        || lower.contains("unknown table")
    {
        return warning_embed(
            "Table de base de données rAthenaFR manquante",
            "Cette commande nécessite une table native rAthenaFR absente de la base ciblée. Vérifie que le schéma SQL rAthenaFR est entièrement importé et que le bot est connecté à la bonne base de données.",
        );
    }

    if lower.contains("unknown column") {
        return warning_embed(
            "Schéma rAthenaFR non supporté",
            "Cette commande attend une colonne absente de la base rAthenaFR ciblée. Ton schéma est peut-être ancien, personnalisé ou importé partiellement.",
        );
    }

    if lower.contains("access denied") || lower.contains("permission") {
        return warning_embed(
            "Problème de permissions base de données",
            "Le bot joint MariaDB, mais l’utilisateur SQL configuré n’a pas assez de droits de lecture pour cette commande.",
        );
    }

    if lower.contains("timed out") || lower.contains("pool timed out") || lower.contains("connect")
    {
        return warning_embed(
            "Problème de connexion à la base de données",
            "Le bot n’a pas pu joindre la base rAthenaFR ciblée à temps. Vérifie MariaDB, le réseau Docker, l’hôte, le port et les identifiants.",
        );
    }

    error_embed("La commande a échoué. Consulte les logs du bot pour les détails techniques.")
}

pub fn status_embed(status: &DatabaseStatus, services: &[RAthenaFrServiceStatus]) -> CreateEmbed {
    let all_services_online = services.iter().all(|service| service.online);
    let color = if all_services_online {
        COLOR_SUCCESS
    } else {
        COLOR_WARNING
    };

    base_embed(
        "Statut rAthenaFR",
        "État actuel des services et compteurs lus depuis la base rAthenaFR ciblée.",
        color,
    )
    .field("Services rAthenaFR", service_status_lines(services), false)
    .field("Base de données", format!("`{}`", status.database_name), true)
    .field("MariaDB", format!("`{}`", status.database_engine), true)
    .field(
        "Personnages connectés",
        format!("`{}`", status.online_characters),
        true,
    )
    .field("Personnages", format!("`{}`", status.characters), true)
    .field("Comptes", format!("`{}`", status.accounts), true)
    .field("Guildes", format!("`{}`", status.guilds), true)
    .field("Source", "`login`, `char`, `guild` + service checks", false)
}

pub fn online_embed(characters: &[CharacterSummary], requested_limit: u32) -> CreateEmbed {
    if characters.is_empty() {
        return warning_embed(
            "Personnages rAthenaFR connectés",
            "Aucun personnage n’est connecté pour le moment.",
        );
    }

    let list = limited_list(characters, requested_limit, |index, character| {
        format!(
            "`{:>2}.` **{}** — Niv. `{}` / Job `{}` — {} — `{}`",
            index + 1,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
            character.map,
        )
    });

    success_embed(
        "Personnages rAthenaFR connectés",
        "Personnages visibles connectés depuis la base de données.",
    )
    .field("Résumé", list_summary(&list, "personnages connectés"), false)
    .field("Personnages", list.value, false)
}

pub fn search_embed(
    query: &str,
    characters: &[CharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if characters.is_empty() {
        return warning_embed(
            "Recherche de personnage rAthenaFR",
            format!("Aucun personnage visible ne correspond à `{}`.", query),
        );
    }

    let list = limited_list(characters, requested_limit, |index, character| {
        format!(
            "`{:>2}.` **{}** — Niv. `{}` / Job `{}` — {} — `{}`",
            index + 1,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
            character.map,
        )
    });

    info_embed(
        "Recherche de personnage rAthenaFR",
        format!("Résultats de recherche pour `{}`.", query),
    )
    .field("Résumé", list_summary(&list, "personnages"), false)
    .field("Personnages", list.value, false)
}

pub fn ranking_embed(entries: &[RankingEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed("Classement des personnages rAthenaFR", "Aucun personnage trouvé.");
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — Niv. `{}` / Job `{}` — {} — `{}`",
            entry.rank,
            entry.name,
            entry.base_level,
            entry.job_level,
            job_name(entry.class_id),
            entry.map,
        )
    });

    info_embed("Classement des personnages rAthenaFR", "Meilleurs personnages par niveau.")
        .field("Résumé", list_summary(&list, "entrées de classement"), false)
        .field("Classement", list.value, false)
}

pub fn top_zeny_embed(entries: &[ZenyRankingEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed("Classement zeny rAthenaFR", "Aucun personnage trouvé.");
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny — Niv. `{}` / Job `{}` — {}",
            entry.rank,
            entry.name,
            format_number(entry.zeny),
            entry.base_level,
            entry.job_level,
            job_name(entry.class_id),
        )
    });

    CreateEmbed::new()
        .title(brand_text("Classement zeny rAthenaFR"))
        .description("Personnages visibles les plus riches. Les personnages GM peuvent être masqués de ce classement.")
        .color(COLOR_PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(footer_text()))
        .timestamp(Timestamp::now())
        .field("Résumé", list_summary(&list, "entrées de classement"), false)
        .field("Classement", list.value, false)
}

pub fn player_embed(profile: &PlayerProfile) -> CreateEmbed {
    let status = if profile.online {
        "🟢 Connecté"
    } else {
        "⚫ Hors ligne"
    };

    success_embed(
        "Profil de personnage rAthenaFR",
        format!("Informations détaillées pour **{}**.", profile.name),
    )
    .field("Statut", status, true)
    .field("Classe", job_name(profile.class_id), true)
    .field(
        "Niveaux",
        format!(
            "Base `{}` / Job `{}`",
            profile.base_level, profile.job_level
        ),
        true,
    )
    .field("Carte", format!("`{}`", profile.map), true)
    .field(
        "Guilde",
        profile
            .guild_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .unwrap_or("Aucun"),
        true,
    )
    .field("Zeny", format!("`{}`", format_number(profile.zeny)), true)
}

pub fn player_not_found_embed(name: &str) -> CreateEmbed {
    warning_embed(
        "Recherche de personnage rAthenaFR",
        format!("Aucun personnage ne correspond à `{}`.", name),
    )
}

pub fn guilds_embed(guilds: &[GuildSummary], requested_limit: u32) -> CreateEmbed {
    if guilds.is_empty() {
        return warning_embed("Classement des guildes rAthenaFR", "Aucune guilde trouvée.");
    }

    let list = limited_list(guilds, requested_limit, |index, guild| {
        format!(
            "`{:>2}.` **{}** — Niv. `{}` — Membres `{}/{}` — Connectés `{}` — Chef `{}`",
            index + 1,
            guild.name,
            guild.level,
            guild.members,
            guild.max_members,
            guild.online_members,
            guild.master,
        )
    });

    info_embed(
        "Classement des guildes rAthenaFR",
        "Meilleures guildes par niveau et nombre de membres.",
    )
    .field("Résumé", list_summary(&list, "guildes"), false)
    .field("Guildes", list.value, false)
}

pub fn guild_detail_embed(guild: &GuildDetails) -> CreateEmbed {
    success_embed(
        "Profil de guilde rAthenaFR",
        format!("Informations détaillées pour **{}**.", guild.name),
    )
    .field("Chef", guild.master.clone(), true)
    .field("Niveau", format!("`{}`", guild.level), true)
    .field(
        "Membres",
        format!("`{}/{}`", guild.members, guild.max_members),
        true,
    )
    .field(
        "Membres connectés",
        format!("`{}`", guild.online_members),
        true,
    )
    .field("Niveau moyen", format!("`{}`", guild.average_level), true)
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
    warning_embed(
        "Recherche de guilde rAthenaFR",
        format!("Aucune guilde ne correspond à `{}`.", name),
    )
}

pub fn guild_members_embed(
    guild_name: &str,
    members: &[GuildMemberSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if members.is_empty() {
        return warning_embed(
            "Membres de guilde rAthenaFR",
            format!("Aucun membre visible trouvé pour la guilde `{}`.", guild_name),
        );
    }

    let list = limited_list(members, requested_limit, |index, member| {
        let status = if member.online { "🟢" } else { "⚫" };
        format!(
            "`{:>2}.` {} **{}** — Pos. `{}` — Niv. `{}` / Job `{}` — {} — `{}`",
            index + 1,
            status,
            member.name,
            member.position,
            member.base_level,
            member.job_level,
            job_name(member.class_id),
            member.map,
        )
    });

    info_embed(
        "Membres de guilde rAthenaFR",
        format!("Membres visibles de la guilde `{}`.", guild_name),
    )
    .field("Résumé", list_summary(&list, "membres de guilde"), false)
    .field("Membres", list.value, false)
}

pub fn classes_embed(entries: &[ClassDistributionEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            "Répartition des classes rAthenaFR",
            "Aucun personnage visible trouvé.",
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — Personnages `{}` — Connectés `{}`",
            entry.rank,
            job_name(entry.class_id),
            entry.characters,
            entry.online_characters,
        )
    });

    info_embed(
        "Répartition des classes rAthenaFR",
        "Personnages visibles regroupés par `char.class`.",
    )
    .field("Résumé", list_summary(&list, "lignes de classes"), false)
    .field("Classes", list.value, false)
    .field("Source", "`char`, `login`", false)
}

pub fn map_stats_embed(
    entries: &[MapStatsEntry],
    online_only: bool,
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed("Statistiques de cartes rAthenaFR", "Aucune donnée de carte visible trouvée.");
    }

    let mode = if online_only {
        "personnages connectés uniquement"
    } else {
        "tous les personnages visibles"
    };
    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` `{}` — Personnages `{}` — Connectés `{}`",
            entry.rank, entry.map, entry.characters, entry.online_characters,
        )
    });

    info_embed(
        "Statistiques de cartes rAthenaFR",
        format!("Répartition des cartes depuis `char.last_map` pour {}.", mode),
    )
    .field("Résumé", list_summary(&list, "lignes de cartes"), false)
    .field("Cartes", list.value, false)
    .field("Source", "`char`, `login`", false)
}

pub fn map_online_embed(
    map: &str,
    characters: &[CharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if characters.is_empty() {
        return warning_embed(
            "Personnages connectés par carte rAthenaFR",
            format!("Aucun personnage visible connecté trouvé sur `{}`.", map),
        );
    }

    let list = limited_list(characters, requested_limit, |index, character| {
        format!(
            "`{:>2}.` **{}** — Niv. `{}` / Job `{}` — {}",
            index + 1,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
        )
    });

    success_embed(
        "Personnages connectés par carte rAthenaFR",
        format!("Personnages visibles connectés sur `{}`.", map),
    )
    .field("Résumé", list_summary(&list, "personnages connectés"), false)
    .field("Personnages", list.value, false)
    .field("Source", "`char`, `login`", false)
}

pub fn party_embed(party: &PartyDetails) -> CreateEmbed {
    success_embed(
        "Profil de groupe rAthenaFR",
        format!("Informations détaillées du groupe **{}**.", party.name),
    )
    .field(
        "Chef",
        party
            .leader_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .unwrap_or("Inconnu"),
        true,
    )
    .field("Membres", format!("`{}`", party.members), true)
    .field(
        "Membres connectés",
        format!("`{}`", party.online_members),
        true,
    )
    .field("Mode EXP", party_exp_mode(party.exp_mode), true)
    .field("Mode objets", party_item_mode(party.item_mode), true)
    .field("Source", "`party`, `char`, `login`", false)
}

pub fn party_not_found_embed(name: &str) -> CreateEmbed {
    warning_embed(
        "Recherche de groupe rAthenaFR",
        format!("Aucun groupe ne correspond à `{}`.", name),
    )
}

pub fn party_members_embed(
    party_name: &str,
    members: &[PartyMemberSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if members.is_empty() {
        return warning_embed(
            "Membres de groupe rAthenaFR",
            format!("Aucun membre visible trouvé pour le groupe `{}`.", party_name),
        );
    }

    let list = limited_list(members, requested_limit, |index, member| {
        let status = if member.online { "🟢" } else { "⚫" };
        let leader = if member.is_leader { " 👑" } else { "" };
        format!(
            "`{:>2}.` {} **{}**{} — Niv. `{}` / Job `{}` — {} — `{}`",
            index + 1,
            status,
            member.name,
            leader,
            member.base_level,
            member.job_level,
            job_name(member.class_id),
            member.map,
        )
    });

    info_embed(
        "Membres de groupe rAthenaFR",
        format!("Membres visibles du groupe `{}`.", party_name),
    )
    .field("Résumé", list_summary(&list, "membres du groupe"), false)
    .field("Membres", list.value, false)
    .field("Source", "`party`, `char`, `login`", false)
}

pub fn homunculus_embed(homunculus: &HomunculusProfile) -> CreateEmbed {
    let alive = if homunculus.alive {
        "🟢 Vivant"
    } else {
        "⚫ Non vivant"
    };
    let vaporized = if homunculus.vaporized { "Oui" } else { "Non" };
    let autofeed = if homunculus.autofeed {
        "Activé"
    } else {
        "Désactivé"
    };

    success_embed(
        "Profil d’homoncule rAthenaFR",
        format!("Homoncule possédé par **{}**.", homunculus.owner_name),
    )
    .field("Nom", homunculus.name.clone(), true)
    .field("ID classe", format!("`{}`", homunculus.class_id), true)
    .field("Niveau", format!("`{}`", homunculus.level), true)
    .field("Statut", alive, true)
    .field("Vaporisé", vaporized, true)
    .field("Auto-nourrissage", autofeed, true)
    .field("Intimité", format!("`{}`", homunculus.intimacy), true)
    .field("Faim", format!("`{}`", homunculus.hunger), true)
    .field(
        "HP / SP",
        format!(
            "HP `{}/{}` — SP `{}/{}`",
            homunculus.hp, homunculus.max_hp, homunculus.sp, homunculus.max_sp
        ),
        false,
    )
    .field("Source", "`homunculus`, `char`, `login`", false)
}

pub fn homunculus_not_found_embed(character: &str) -> CreateEmbed {
    warning_embed(
        "Recherche d’homoncule rAthenaFR",
        format!("Aucun homoncule visible trouvé pour `{}`.", character),
    )
}

pub fn pet_embed(pet: &PetProfile) -> CreateEmbed {
    let incubated = if pet.incubated { "Oui" } else { "Non" };
    let autofeed = if pet.autofeed { "Activé" } else { "Désactivé" };

    success_embed(
        "Profil de familier rAthenaFR",
        format!("Familier possédé par **{}**.", pet.owner_name),
    )
    .field("Nom", pet.name.clone(), true)
    .field("ID classe", format!("`{}`", pet.class_id), true)
    .field("Niveau", format!("`{}`", pet.level), true)
    .field("Intimité", format!("`{}`", pet.intimacy), true)
    .field("Faim", format!("`{}`", pet.hunger), true)
    .field("Incubé", incubated, true)
    .field("Auto-nourrissage", autofeed, true)
    .field("Source", "`pet`, `char`, `login`", false)
}

pub fn pet_not_found_embed(character: &str) -> CreateEmbed {
    warning_embed(
        "Recherche de familier rAthenaFR",
        format!("Aucun familier visible trouvé pour `{}`.", character),
    )
}

pub fn zeny_embed(summary: &ZenySummary) -> CreateEmbed {
    let richest = match &summary.richest_name {
        Some(name) if !name.is_empty() => format!(
            "**{}** — `{}` zeny",
            name,
            format_number(summary.richest_zeny)
        ),
        _ => "Aucun".to_string(),
    };

    CreateEmbed::new()
        .title(brand_text("Statistiques zeny rAthenaFR"))
        .description("Statistiques de zeny visibles depuis la base rAthenaFR ciblée. Les personnages GM peuvent être exclus via les filtres de classement.")
        .color(COLOR_PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(footer_text()))
        .timestamp(Timestamp::now())
        .field("Personnages comptés", format!("`{}`", summary.character_count), true)
        .field("Zeny total", format!("`{}`", format_number(summary.total_zeny)), true)
        .field("Zeny moyen", format!("`{}`", format_number(summary.average_zeny)), true)
        .field("Personnage visible le plus riche", richest, false)
        .field("Source", "`char`, `login`", false)
}

pub fn castles_embed(castles: &[CastleSummary], requested_limit: u32) -> CreateEmbed {
    if castles.is_empty() {
        return warning_embed(
            "Châteaux rAthenaFR",
            "Aucune donnée de château trouvée dans `guild_castle`.",
        );
    }

    let list = limited_list(castles, requested_limit, |_index, castle| {
        let owner = castle
            .owner_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .unwrap_or("Aucun propriétaire");

        format!(
            "Château `{}` — Propriétaire **{}** — Économie `{}` — Défense `{}` — Visible `{}`",
            castle.castle_id,
            castle.owner_name.as_deref().unwrap_or(owner),
            castle.economy,
            castle.defense,
            castle.visible_c,
        )
    });

    info_embed(
        "Châteaux rAthenaFR",
        "Propriétaires de châteaux et données économiques depuis la base de données.",
    )
    .field("Résumé", list_summary(&list, "castles"), false)
    .field("Châteaux", list.value, false)
    .field("Source", "`guild_castle`, `guild`", false)
}

pub fn castle_detail_embed(castle: &CastleDetails) -> CreateEmbed {
    let owner = castle
        .owner_name
        .as_deref()
        .filter(|name| !name.is_empty())
        .unwrap_or("Aucun propriétaire");

    success_embed(
        "Profil de château rAthenaFR",
        format!("Informations détaillées du château `{}`.", castle.castle_id),
    )
    .field("Propriétaire", owner, true)
    .field(
        "ID de guilde propriétaire",
        format!("`{}`", castle.owner_guild_id),
        true,
    )
    .field("Économie", format!("`{}`", castle.economy), true)
    .field("Défense", format!("`{}`", castle.defense), true)
    .field("C visible", format!("`{}`", castle.visible_c), true)
    .field(
        "Déclencheurs",
        format!(
            "Économie `{}` — Défense `{}`",
            castle.trigger_e, castle.trigger_d
        ),
        true,
    )
    .field(
        "Minuteurs",
        format!(
            "Prochain `{}` — Paiement `{}` — Créé `{}`",
            castle.next_time, castle.pay_time, castle.create_time
        ),
        false,
    )
    .field("Source", "`guild_castle`, `guild`", false)
}

pub fn castle_not_found_embed(castle_id: i64) -> CreateEmbed {
    warning_embed(
        "Recherche de château rAthenaFR",
        format!("Aucun château ne correspond à l’ID `{}`.", castle_id),
    )
}

pub fn guild_alliances_embed(
    guild_name: &str,
    alliances: &[GuildAllianceEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if alliances.is_empty() {
        return warning_embed(
            "Alliances de guilde rAthenaFR",
            format!(
                "Aucune alliance ou opposition trouvée pour la guilde `{}`.",
                guild_name
            ),
        );
    }

    let list = limited_list(alliances, requested_limit, |_index, entry| {
        let icon = if entry.relation == "Opposition" {
            "⚔️"
        } else {
            "🤝"
        };
        format!(
            "{} **{}** — `{}` — ID guilde `{}`",
            icon, entry.target_name, entry.relation, entry.target_guild_id,
        )
    });

    info_embed(
        "Alliances de guilde rAthenaFR",
        format!("Alliances et oppositions de la guilde `{}`.", guild_name),
    )
    .field("Résumé", list_summary(&list, "guild relations"), false)
    .field("Relations", list.value, false)
    .field("Source", "`guild_alliance`, `guild`", false)
}

pub fn guild_skills_embed(
    guild_name: &str,
    skills: &[GuildSkillEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if skills.is_empty() {
        return warning_embed(
            "Compétences de guilde rAthenaFR",
            format!(
                "Aucune compétence de guilde apprise trouvée pour `{}`.",
                guild_name
            ),
        );
    }

    let list = limited_list(skills, requested_limit, |_index, skill| {
        format!("ID compétence `{}` — Niveau `{}`", skill.skill_id, skill.level)
    });

    info_embed(
        "Compétences de guilde rAthenaFR",
        format!("Compétences apprises par la guilde `{}`.", guild_name),
    )
    .field("Résumé", list_summary(&list, "guild skills"), false)
    .field("Compétences", list.value, false)
    .field("Source", "`guild_skill`, `guild`", false)
}

pub fn homunculus_top_embed(
    entries: &[HomunculusRankingEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            "Classement des homoncules rAthenaFR",
            "Aucun homoncule visible trouvé.",
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — Propriétaire `{}` — ID classe `{}` — Niv. `{}` — Intimité `{}` — Faim `{}`",
            entry.rank,
            entry.name,
            entry.owner_name,
            entry.class_id,
            entry.level,
            entry.intimacy,
            entry.hunger,
        )
    });

    info_embed(
        "Classement des homoncules rAthenaFR",
        "Meilleurs homoncules par niveau et intimité.",
    )
    .field("Résumé", list_summary(&list, "entrées d’homoncules"), false)
    .field("Classement", list.value, false)
    .field("Source", "`homunculus`, `char`, `login`", false)
}

pub fn pet_top_embed(entries: &[PetRankingEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed("Classement des familiers rAthenaFR", "Aucun familier visible trouvé.");
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — Propriétaire `{}` — ID classe `{}` — Niv. `{}` — Intimité `{}` — Faim `{}`",
            entry.rank,
            entry.name,
            entry.owner_name,
            entry.class_id,
            entry.level,
            entry.intimacy,
            entry.hunger,
        )
    });

    info_embed(
        "Classement des familiers rAthenaFR",
        "Meilleurs familiers par intimité et niveau.",
    )
    .field("Résumé", list_summary(&list, "entrées de familiers"), false)
    .field("Classement", list.value, false)
    .field("Source", "`pet`, `char`, `login`", false)
}

pub fn quest_stats_embed(stats: &QuestStats) -> CreateEmbed {
    if stats.total_characters == 0 {
        return warning_embed(
            "Statistiques de quête rAthenaFR",
            format!(
                "Aucun personnage visible n’a la quête ID `{}` dans la base de données.",
                stats.quest_id
            ),
        );
    }

    info_embed(
        "Statistiques de quête rAthenaFR",
        format!("Statistiques globales pour la quête ID `{}`.", stats.quest_id),
    )
    .field("Personnages", format!("`{}`", stats.total_characters), true)
    .field("État 0", format!("`{}`", stats.state_0), true)
    .field("État 1", format!("`{}`", stats.state_1), true)
    .field("État 2", format!("`{}`", stats.state_2), true)
    .field("Source", "`quest`, `char`, `login`", false)
}

pub fn account_characters_embed(
    account_id: i64,
    characters: &[AccountCharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    if characters.is_empty() {
        return warning_embed(
            "Personnages du compte rAthenaFR",
            format!("Aucun personnage trouvé pour le compte `{}`.", account_id),
        );
    }

    let list = limited_list(characters, requested_limit, |_index, character| {
        let status = if character.online { "🟢" } else { "⚫" };
        let guild = character
            .guild_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .unwrap_or("Aucune guilde");

        format!(
            "Slot `{}` — {} **{}** — Niv. `{}` / Job `{}` — {} — `{}` — `{}` zeny — {}",
            character.slot,
            status,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
            character.map,
            format_number(character.zeny),
            guild,
        )
    });

    info_embed(
        "Personnages du compte rAthenaFR",
        format!("Liste staff uniquement des personnages du compte `{}`.", account_id),
    )
    .field("Résumé", list_summary(&list, "personnages du compte"), false)
    .field("Personnages", list.value, false)
    .field("Source", "`login`, `char`, `guild`", false)
}

pub fn account_status_embed(status: &AccountStatus) -> CreateEmbed {
    success_embed(
        "Statut de compte rAthenaFR",
        format!(
            "Statut sûr du compte `{}` réservé au staff.",
            status.account_id
        ),
    )
    .field("ID compte", format!("`{}`", status.account_id), true)
    .field("Login", format!("`{}`", status.userid), true)
    .field("Sexe", format!("`{}`", status.sex), true)
    .field("ID groupe", format!("`{}`", status.group_id), true)
    .field("État", account_state(status.state), true)
    .field("Nombre de connexions", format!("`{}`", status.logincount), true)
    .field(
        "Personnages",
        format!(
            "`{}` / slots `{}`",
            status.characters, status.character_slots
        ),
        true,
    )
    .field(
        "Personnages connectés",
        format!("`{}`", status.online_characters),
        true,
    )
    .field(
        "Zeny total",
        format!("`{}`", format_number(status.total_zeny)),
        true,
    )
    .field(
        "Dernière connexion",
        status
            .lastlogin
            .as_deref()
            .filter(|value| !value.is_empty())
            .unwrap_or("Jamais"),
        true,
    )
    .field("Fin de bannissement", unix_time_field(status.unban_time), true)
    .field(
        "Expiration",
        unix_time_field(status.expiration_time),
        true,
    )
    .field("Source", "`login`, `char`", false)
}

pub fn account_not_found_embed(account_id: i64) -> CreateEmbed {
    warning_embed(
        "Recherche de compte rAthenaFR",
        format!("Aucun compte ne correspond à l’ID `{}`.", account_id),
    )
}

pub fn character_quests_embed(
    character: &str,
    quests: &[CharacterQuestEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if quests.is_empty() {
        return warning_embed(
            "Quêtes du personnage rAthenaFR",
            format!("Aucune entrée de quête trouvée pour le personnage `{}`.", character),
        );
    }

    let list = limited_list(quests, requested_limit, |_index, quest| {
        format!(
            "Quête `{}` — État `{}` — Temps `{}` — Compteurs `{}/{}/{}`",
            quest.quest_id,
            quest_state_name(&quest.state),
            quest.time,
            quest.count1,
            quest.count2,
            quest.count3,
        )
    });

    info_embed(
        "Quêtes du personnage rAthenaFR",
        format!("Entrées de quêtes staff uniquement pour **{}**.", character),
    )
    .field("Résumé", list_summary(&list, "entrées de quêtes"), false)
    .field("Quêtes", list.value, false)
    .field("Source", "`quest`, `char`", false)
}

pub fn character_equipment_embed(
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if items.is_empty() {
        return warning_embed(
            "Équipement du personnage rAthenaFR",
            format!("Aucun objet équipé trouvé pour le personnage `{}`.", character),
        );
    }

    let list = limited_list(items, requested_limit, |_index, item| item_line(item));

    info_embed(
        "Équipement du personnage rAthenaFR",
        format!("Objets équipés staff uniquement pour **{}**.", character),
    )
    .field("Résumé", list_summary(&list, "objets équipés"), false)
    .field("Équipement", list.value, false)
    .field("Source", "`inventory`, `char`", false)
}

pub fn character_inventory_embed(
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if items.is_empty() {
        return warning_embed(
            "Inventaire du personnage rAthenaFR",
            format!(
                "Aucun objet non équipé trouvé dans l’inventaire du personnage `{}`.",
                character
            ),
        );
    }

    let list = limited_list(items, requested_limit, |_index, item| item_line(item));

    info_embed(
        "Inventaire du personnage rAthenaFR",
        format!("Objets d’inventaire staff uniquement pour **{}**.", character),
    )
    .field("Résumé", list_summary(&list, "objets d’inventaire"), false)
    .field("Objets", list.value, false)
    .field("Source", "`inventory`, `char`", false)
}

pub fn item_count_embed(summary: &ItemCountSummary) -> CreateEmbed {
    info_embed(
        "Comptage d’objet rAthenaFR",
        format!("Comptage global staff uniquement pour l’objet ID `{}`.", summary.item_id),
    )
    .field(
        "Inventaire",
        format!("`{}`", format_number(summary.inventory_amount)),
        true,
    )
    .field(
        "Chariot",
        format!("`{}`", format_number(summary.cart_amount)),
        true,
    )
    .field(
        "Stockage",
        format!("`{}`", format_number(summary.storage_amount)),
        true,
    )
    .field(
        "Stockage de guilde",
        format!("`{}`", format_number(summary.guild_storage_amount)),
        true,
    )
    .field(
        "Total",
        format!("`{}`", format_number(summary.total_amount)),
        true,
    )
    .field(
        "Source",
        "`inventory`, `cart_inventory`, `storage`, `guild_storage`",
        false,
    )
}

pub fn item_owners_embed(
    item_id: i64,
    owners: &[ItemOwnerEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if owners.is_empty() {
        return warning_embed(
            "Propriétaires d’objet rAthenaFR",
            format!("Aucun propriétaire trouvé pour l’objet ID `{}`.", item_id),
        );
    }

    let list = limited_list(owners, requested_limit, |_index, owner| {
        let account = owner
            .account_id
            .map(|value| format!(" — Compte `{}`", value))
            .unwrap_or_default();

        format!(
            "**{}** — `{}` — Quantité `{}`{}",
            owner.owner_name,
            owner.source,
            format_number(owner.amount),
            account,
        )
    });

    info_embed(
        "Propriétaires d’objet rAthenaFR",
        format!("Propriétaires staff uniquement pour l’objet ID `{}`.", item_id),
    )
    .field("Résumé", list_summary(&list, "propriétaires d’objet"), false)
    .field("Propriétaires", list.value, false)
    .field(
        "Source",
        "`inventory`, `cart_inventory`, `storage`, `guild_storage`, `char`, `login`, `guild`",
        false,
    )
}

pub fn account_overview_embed(
    status: &AccountStatus,
    characters: &[AccountCharacterSummary],
    requested_limit: u32,
) -> CreateEmbed {
    let character_list = limited_list(characters, requested_limit, |_index, character| {
        let status_icon = if character.online { "🟢" } else { "⚫" };
        format!(
            "Slot `{}` — {} **{}** — Niv. `{}` / Job `{}` — {} — `{}` zeny",
            character.slot,
            status_icon,
            character.name,
            character.base_level,
            character.job_level,
            job_name(character.class_id),
            format_number(character.zeny),
        )
    });
    let character_lines = if characters.is_empty() {
        "Aucun personnage trouvé.".to_string()
    } else {
        character_list.value.clone()
    };

    success_embed(
        "Résumé de compte rAthenaFR",
        format!(
            "Résumé compact staff uniquement pour le compte `{}`.",
            status.account_id
        ),
    )
    .field("Login", format!("`{}`", status.userid), true)
    .field("ID groupe", format!("`{}`", status.group_id), true)
    .field("État", account_state(status.state), true)
    .field(
        "Personnages",
        format!(
            "`{}` / slots `{}`",
            status.characters, status.character_slots
        ),
        true,
    )
    .field("Connecté", format!("`{}`", status.online_characters), true)
    .field(
        "Zeny total",
        format!("`{}`", format_number(status.total_zeny)),
        true,
    )
    .field(
        "Résumé de la liste des personnages",
        list_summary(&character_list, "personnages du compte"),
        false,
    )
    .field("Liste des personnages", trim_embed_value(character_lines), false)
    .field("Source", "`login`, `char`, `guild`", false)
}

pub fn ban_list_embed(entries: &[BanEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return success_embed(
            "Liste des bannissements rAthenaFR",
            "Aucun compte bloqué ou banni trouvé dans la table login.",
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "Compte `{}` — `{}` — État {} — Groupe `{}` — Déban `{}` — Expiration `{}` — Dernière connexion `{}` — Personnages `{}`",
            entry.account_id,
            entry.userid,
            account_state(entry.state),
            entry.group_id,
            unix_time_field(entry.unban_time),
            unix_time_field(entry.expiration_time),
            entry.lastlogin.as_deref().unwrap_or("Jamais"),
            entry.characters,
        )
    });

    info_embed(
        "Liste des bannissements rAthenaFR",
        "Comptes bloqués ou bannis staff uniquement depuis la table login.",
    )
    .field("Résumé", list_summary(&list, "comptes bloqués"), false)
    .field("Comptes", list.value, false)
    .field("Source", "`login`, `char`", false)
}

pub fn who_sell_embed(
    item_id: i64,
    sellers: &[MarketSellEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if sellers.is_empty() {
        return warning_embed(
            "Vendeurs du marché",
            format!("Aucune boutique de vente active ne vend l’objet `{}`.", item_id),
        );
    }

    let list = limited_list(sellers, requested_limit, |index, seller| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny x`{}` — `{}` at `{}` ({}, {})",
            index + 1,
            seller.merchant_name,
            format_number(seller.price),
            format_number(seller.amount),
            seller.shop_title,
            seller.map,
            seller.x,
            seller.y,
        )
    });

    info_embed(
        "Vendeurs du marché",
        format!("Offres de vente actives pour l’objet `{}`.", item_id),
    )
    .field("Résumé", list_summary(&list, "lignes de vendeurs"), false)
    .field("Vendeurs", list.value, false)
    .field(
        "Source",
        "`vendings`, `vending_items`, `cart_inventory`, `char`, `login`",
        false,
    )
}

pub fn who_buy_embed(item_id: i64, buyers: &[MarketBuyEntry], requested_limit: u32) -> CreateEmbed {
    if buyers.is_empty() {
        return warning_embed(
            "Acheteurs du marché",
            format!("Aucune boutique d’achat active n’achète l’objet `{}`.", item_id),
        );
    }

    let list = limited_list(buyers, requested_limit, |index, buyer| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny x`{}` — `{}` at `{}` ({}, {})",
            index + 1,
            buyer.buyer_name,
            format_number(buyer.price),
            format_number(buyer.amount),
            buyer.shop_title,
            buyer.map,
            buyer.x,
            buyer.y,
        )
    });

    info_embed(
        "Acheteurs du marché",
        format!("Offres d’achat actives pour l’objet `{}`.", item_id),
    )
    .field("Résumé", list_summary(&list, "lignes d’acheteurs"), false)
    .field("Acheteurs", list.value, false)
    .field(
        "Source",
        "`buyingstores`, `buyingstore_items`, `char`, `login`",
        false,
    )
}

pub fn market_embed(overview: &MarketOverview) -> CreateEmbed {
    let lowest_sell = overview
        .lowest_sell_price
        .map(format_number)
        .unwrap_or_else(|| "Aucun".to_string());
    let highest_buy = overview
        .highest_buy_price
        .map(format_number)
        .unwrap_or_else(|| "Aucun".to_string());

    info_embed(
        "Vue d’ensemble du marché",
        format!("Résumé vente/achat pour l’objet `{}`.", overview.item_id),
    )
    .field("Vendeurs", format!("`{}`", overview.sellers), true)
    .field(
        "Quantité en vente",
        format!("`{}`", format_number(overview.sell_amount)),
        true,
    )
    .field("Prix de vente le plus bas", format!("`{}`", lowest_sell), true)
    .field("Acheteurs", format!("`{}`", overview.buyers), true)
    .field(
        "Quantité en achat",
        format!("`{}`", format_number(overview.buy_amount)),
        true,
    )
    .field("Prix d’achat le plus élevé", format!("`{}`", highest_buy), true)
    .field("Source", "Tables natives des boutiques de vente et d’achat", false)
}

pub fn venders_embed(stores: &[VendingStoreEntry], requested_limit: u32) -> CreateEmbed {
    if stores.is_empty() {
        return warning_embed(
            "Boutiques de vente actives",
            "Aucune boutique de vente active trouvée.",
        );
    }

    let list = limited_list(stores, requested_limit, |_index, store| {
        let min_price = store
            .min_price
            .map(format_number)
            .unwrap_or_else(|| "Aucun".to_string());
        format!(
            "`{:>2}.` **{}** — `{}` — Objets `{}` / Quantité `{}` — Min `{}`z — `{}` ({}, {})",
            store.rank,
            store.merchant_name,
            store.shop_title,
            store.item_count,
            format_number(store.total_amount),
            min_price,
            store.map,
            store.x,
            store.y,
        )
    });

    info_embed(
        "Boutiques de vente actives",
        "Boutiques de vente actuelles depuis la base de données.",
    )
    .field("Résumé", list_summary(&list, "boutiques de vente"), false)
    .field("Boutiques", list.value, false)
    .field(
        "Source",
        "`vendings`, `vending_items`, `char`, `login`",
        false,
    )
}

pub fn buyers_embed(stores: &[BuyingStoreEntry], requested_limit: u32) -> CreateEmbed {
    if stores.is_empty() {
        return warning_embed("Boutiques d’achat actives", "Aucune boutique d’achat active trouvée.");
    }

    let list = limited_list(stores, requested_limit, |_index, store| {
        let max_price = store
            .max_price
            .map(format_number)
            .unwrap_or_else(|| "Aucun".to_string());
        format!(
            "`{:>2}.` **{}** — `{}` — Objets `{}` / Quantité `{}` — Max `{}`z — Limite `{}`z — `{}` ({}, {})",
            store.rank,
            store.buyer_name,
            store.shop_title,
            store.item_count,
            format_number(store.total_amount),
            max_price,
            format_number(store.zeny_limit),
            store.map,
            store.x,
            store.y,
        )
    });

    info_embed(
        "Boutiques d’achat actives",
        "Boutiques d’achat actuelles depuis la base de données.",
    )
    .field("Résumé", list_summary(&list, "boutiques d’achat"), false)
    .field("Boutiques", list.value, false)
    .field(
        "Source",
        "`buyingstores`, `buyingstore_items`, `char`, `login`",
        false,
    )
}

pub fn staff_only_embed() -> CreateEmbed {
    error_embed(
        "Cette commande est réservée au staff. Configure `RATHENAFR_STAFF_ROLE_IDS` avec les IDs des rôles Discord autorisés à utiliser les commandes de compte.",
    )
}

pub fn missing_database_table_embed(table_name: &str) -> CreateEmbed {
    warning_embed(
        "Commande indisponible",
        format!(
            "Cette commande est indisponible car la table requise `{}` est introuvable.",
            table_name
        ),
    )
}

pub fn error_embed(message: &str) -> CreateEmbed {
    base_embed("Erreur du bot rAthenaFR", message, COLOR_ERROR)
}

fn service_status_lines(services: &[RAthenaFrServiceStatus]) -> String {
    if services.is_empty() {
        return "Aucun endpoint de service rAthenaFR configuré.".to_string();
    }

    services
        .iter()
        .map(|service| {
            let state = if service.online {
                "🟢 Connecté"
            } else {
                "🔴 Hors ligne"
            };

            format!("**{}**: {}", service.name, state)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn success_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    base_embed(title, description, COLOR_SUCCESS)
}

fn warning_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    base_embed(title, description, COLOR_WARNING)
}

fn info_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    base_embed(title, description, COLOR_INFO)
}

fn base_embed(title: &str, description: impl Into<String>, color: Colour) -> CreateEmbed {
    CreateEmbed::new()
        .title(brand_text(title))
        .description(brand_text(description.into()))
        .color(color)
        .footer(serenity::all::CreateEmbedFooter::new(footer_text()))
        .timestamp(Timestamp::now())
}

fn display_name() -> String {
    std::env::var("RATHENAFR_DISPLAY_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "rAthenaFR".to_string())
}

fn footer_text() -> String {
    format!("Bot Discord {}", display_name())
}

fn brand_text(value: impl Into<String>) -> String {
    value.into().replace("rAthenaFR", &display_name())
}

fn party_exp_mode(value: i32) -> String {
    match value {
        0 => "Individuel".to_string(),
        1 => "Partage équitable".to_string(),
        other => format!("Inconnu (`{}`)", other),
    }
}

fn party_item_mode(value: i32) -> String {
    match value {
        0 => "Individuel".to_string(),
        1 => "Ramassage partagé".to_string(),
        2 => "Ramassage partagé + partage équitable".to_string(),
        other => format!("Inconnu (`{}`)", other),
    }
}

fn account_state(value: i64) -> String {
    match value {
        0 => "`0` Actif".to_string(),
        5 => "`5` Banni".to_string(),
        other => format!("`{}`", other),
    }
}

fn unix_time_field(value: i64) -> String {
    if value <= 0 {
        "Aucun".to_string()
    } else {
        format!("`{}`", value)
    }
}

fn quest_state_name(value: &str) -> String {
    match value {
        "0" => "0 Ouverte".to_string(),
        "1" => "1 Terminée".to_string(),
        "2" => "2 Expirée".to_string(),
        other => other.to_string(),
    }
}

fn item_line(item: &CharacterItemEntry) -> String {
    let identified = if item.identify {
        "identifié"
    } else {
        "inconnu"
    };
    let refine = if item.refine > 0 {
        format!("+{} ", item.refine)
    } else {
        String::new()
    };
    let cards = [item.card0, item.card1, item.card2, item.card3]
        .into_iter()
        .filter(|card| *card != 0)
        .map(|card| card.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let card_text = if cards.is_empty() {
        "Aucune carte".to_string()
    } else {
        format!("Cartes `{}`", cards)
    };

    format!(
        "{}Objet `{}` x`{}` — Équipé `{}` — {} — Lié `{}` — Grade `{}` — UID `{}` — {}",
        refine,
        item.item_id,
        format_number(item.amount),
        item.equip,
        identified,
        item.bound,
        item.enchant_grade,
        item.unique_id,
        card_text,
    )
}

fn format_number(value: i64) -> String {
    let raw = value.abs().to_string();
    let mut output = String::new();

    for (index, character) in raw.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            output.push(',');
        }
        output.push(character);
    }

    let mut formatted = output.chars().rev().collect::<String>();
    if value < 0 {
        formatted.insert(0, '-');
    }
    formatted
}

fn limited_list<T, F>(items: &[T], requested_limit: u32, formatter: F) -> LimitedList
where
    F: Fn(usize, &T) -> String,
{
    let row_limit = display_limit(requested_limit);
    let mut lines = Vec::new();
    let mut value_len = 0;

    for (index, item) in items.iter().take(row_limit).enumerate() {
        if !push_limited_line(&mut lines, &mut value_len, formatter(index, item)) {
            break;
        }
    }

    let displayed_count = lines.len();

    LimitedList {
        value: lines.join("\n"),
        displayed_count,
        available_count: items.len(),
        row_limit,
    }
}

fn list_summary(list: &LimitedList, noun: &str) -> String {
    let total_text = if list.available_count > list.row_limit {
        format!("au moins `{}`", list.row_limit + 1)
    } else {
        format!("`{}`", list.available_count)
    };

    let mut summary = format!(
        "Affichage de `{}` sur {} {} correspondants.",
        list.displayed_count, total_text, noun
    );

    let hidden_by_row_limit = list.available_count > list.row_limit;
    let hidden_by_embed_limit = list.displayed_count < list.available_count.min(list.row_limit);
    let hidden_reason = match (hidden_by_row_limit, hidden_by_embed_limit) {
        (true, true) => Some("la limite d’affichage configurée et les limites de champ des embeds Discord"),
        (true, false) => Some("la limite d’affichage configurée"),
        (false, true) => Some("les limites de champ des embeds Discord"),
        (false, false) => None,
    };

    if let Some(reason) = hidden_reason {
        summary.push_str(" D’autres résultats ont été masqués par ");
        summary.push_str(reason);
        summary.push('.');
    }

    summary
}

fn display_limit(requested_limit: u32) -> usize {
    (requested_limit as usize).max(1)
}

fn push_limited_line(lines: &mut Vec<String>, value_len: &mut usize, line: String) -> bool {
    let separator_len = if lines.is_empty() { 0 } else { 1 };
    let available_len = EMBED_FIELD_VALUE_LIMIT.saturating_sub(*value_len + separator_len);

    if available_len == 0 {
        return false;
    }

    let line_len = line.chars().count();
    if line_len > available_len {
        if lines.is_empty() {
            let trimmed = trim_line(line, available_len);
            *value_len += separator_len + trimmed.chars().count();
            lines.push(trimmed);
            return true;
        }

        return false;
    }

    *value_len += separator_len + line_len;
    lines.push(line);
    true
}

fn trim_line(value: String, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value;
    }

    if limit == 0 {
        return String::new();
    }

    if limit <= 3 {
        return ".".repeat(limit);
    }

    let mut trimmed = value.chars().take(limit - 3).collect::<String>();
    trimmed.push_str("...");
    trimmed
}

fn trim_embed_value(value: String) -> String {
    if value.chars().count() <= EMBED_FIELD_VALUE_LIMIT {
        return value;
    }

    let body_limit = EMBED_FIELD_VALUE_LIMIT.saturating_sub(4);
    let mut trimmed = value.chars().take(body_limit).collect::<String>();

    if let Some(last_line_break) = trimmed.rfind('\n') {
        trimmed.truncate(last_line_break);
    }

    if trimmed.is_empty() {
        trimmed = value.chars().take(body_limit).collect::<String>();
    }

    trimmed.push_str("\n...");
    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limited_list_respects_requested_limit() {
        let items = vec![1, 2, 3];

        let list = limited_list(&items, 2, |_index, item| format!("Ligne {item}"));

        assert_eq!(list.value, "Ligne 1\nLigne 2");
        assert_eq!(list.displayed_count, 2);
        assert_eq!(list.available_count, 3);
        assert_eq!(list_summary(&list, "lignes"), "Affichage de `2` sur au moins `3` lignes correspondants. D’autres résultats ont été masqués par la limite d’affichage configurée.");
    }

    #[test]
    fn limited_list_summary_uses_exact_total_when_all_rows_fit() {
        let items = vec![1, 2];

        let list = limited_list(&items, 5, |_index, item| format!("Ligne {item}"));

        assert_eq!(list.value, "Ligne 1\nLigne 2");
        assert_eq!(
            list_summary(&list, "lignes"),
            "Affichage de `2` sur `2` lignes correspondants."
        );
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
}
