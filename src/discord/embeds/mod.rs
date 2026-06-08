use crate::rathenafr::*;
use serenity::all::{Colour, CreateEmbed, Timestamp};

const COLOR_SUCCESS: Colour = Colour::new(0x57F287);
const COLOR_WARNING: Colour = Colour::new(0xFEE75C);
const COLOR_ERROR: Colour = Colour::new(0xED4245);
const COLOR_INFO: Colour = Colour::new(0x5865F2);
const COLOR_PURPLE: Colour = Colour::new(0x9B59B6);
const EMBED_FIELD_VALUE_LIMIT: usize = 1000;
const EMBED_LIST_SEPARATOR_LEN: usize = 2;
const GMMSG_LOG_MESSAGE_LIMIT: usize = 900;
const COMMAND_DISPLAY_NAME: &str = "rAthena";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GmmsgLogStatus {
    Sent,
    Failed,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountManageLogStatus {
    Success,
    Refused,
    Failed,
}

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
            "Le bot joint MariaDB, mais l’utilisateur SQL configuré n’a pas assez de droits pour cette commande.",
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
        "Statut rAthena",
        "État actuel des services et compteurs lus depuis la base rAthena ciblée.",
        color,
    )
    .field("Services rAthena", service_status_lines(services), false)
    .field(
        "Base de données",
        format!("`{}`", status.database_name),
        true,
    )
    .field("MariaDB", format!("`{}`", status.database_engine), true)
    .field(
        "Personnages connectés",
        format!("`{}`", status.online_characters),
        true,
    )
    .field("Personnages", format!("`{}`", status.characters), true)
    .field("Comptes", format!("`{}`", status.accounts), true)
    .field("Guildes", format!("`{}`", status.guilds), true)
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
            "`{:>2}.` **{}** — Base `{}` / Job `{}` — {} — Carte `{}`",
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
    .field(
        "Résumé",
        list_summary(&list, "personnages connectés"),
        false,
    )
    .field("Personnages", list.value, false)
}

pub fn ranking_embed(entries: &[RankingEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            "Classement des personnages rAthenaFR",
            "Aucun personnage trouvé.",
        );
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — Base `{}` / Job `{}` — {} — Carte `{}`",
            entry.rank,
            entry.name,
            entry.base_level,
            entry.job_level,
            job_name(entry.class_id),
            entry.map,
        )
    });

    info_embed(
        "Classement des personnages rAthenaFR",
        "Meilleurs personnages par niveau.",
    )
    .field(
        "Résumé",
        list_summary(&list, "entrées de classement"),
        false,
    )
    .field("Classement", list.value, false)
}

pub fn top_zeny_embed(entries: &[ZenyRankingEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed("Classement zeny rAthenaFR", "Aucun personnage trouvé.");
    }

    let list = limited_list(entries, requested_limit, |_index, entry| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny — Base `{}` / Job `{}` — {}",
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
        "Profil de personnage rAthena",
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
        return warning_embed("Classement des guildes rAthena", "Aucune guilde trouvée.");
    }

    let list = limited_list(guilds, requested_limit, |index, guild| {
        format!(
            "`{:>2}.` **{}** — Niveau `{}` — Membres `{}/{}` — Connectés `{}` — Chef `{}`",
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
            format!(
                "Aucun membre visible trouvé pour la guilde `{}`.",
                guild_name
            ),
        );
    }

    let list = limited_list(members, requested_limit, |index, member| {
        let status = if member.online { "🟢" } else { "⚫" };
        format!(
            "`{:>2}.` {} **{}** — Position `{}` — Base `{}` / Job `{}` — {} — Carte `{}`",
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

pub fn map_stats_embed(
    entries: &[MapStatsEntry],
    online_only: bool,
    requested_limit: u32,
) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            "Statistiques de cartes rAthenaFR",
            "Aucune donnée de carte visible trouvée.",
        );
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
        format!(
            "Répartition des cartes depuis `char.last_map` pour {}.",
            mode
        ),
    )
    .field("Résumé", list_summary(&list, "lignes de cartes"), false)
    .field("Cartes", list.value, false)
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
    .field("Résumé", list_summary(&list, "châteaux"), false)
    .field("Châteaux", list.value, false)
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
}

pub fn castle_not_found_embed(castle_id: i64) -> CreateEmbed {
    warning_embed(
        "Recherche de château rAthenaFR",
        format!("Aucun château ne correspond à l’ID `{}`.", castle_id),
    )
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
        format!(
            "Liste staff uniquement des personnages du compte `{}`.",
            account_id
        ),
    )
    .field(
        "Résumé",
        list_summary(&list, "personnages du compte"),
        false,
    )
    .field("Personnages", list.value, false)
}

pub fn account_creation_disabled_embed() -> CreateEmbed {
    warning_embed(
        "Création de compte désactivée",
        "La commande `/createaccount` existe, mais la création publique est désactivée sur ce bot.",
    )
}

pub fn account_created_embed(account: &CreatedAccount) -> CreateEmbed {
    success_embed(
        "Compte rAthena créé",
        format!("Le compte `{}` a été créé.", account.userid),
    )
    .field("ID compte", format!("`{}`", account.account_id), true)
    .field("Sexe", format!("`{}`", account.sex), true)
    .field("Email", format!("`{}`", account.email), true)
    .field(
        "Important",
        "Le mot de passe n’est jamais réaffiché par le bot.",
        false,
    )
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
    .field(
        "Nombre de connexions",
        format!("`{}`", status.logincount),
        true,
    )
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
    .field(
        "Fin de bannissement",
        unix_time_field(status.unban_time),
        true,
    )
    .field("Expiration", unix_time_field(status.expiration_time), true)
}

pub fn character_quests_embed(
    character: &str,
    quests: &[CharacterQuestEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if quests.is_empty() {
        return warning_embed(
            "Quêtes du personnage rAthenaFR",
            format!(
                "Aucune entrée de quête trouvée pour le personnage `{}`.",
                character
            ),
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
}

pub fn character_equipment_embed(
    character: &str,
    items: &[CharacterItemEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if items.is_empty() {
        return warning_embed(
            "Équipement du personnage rAthenaFR",
            format!(
                "Aucun objet équipé trouvé pour le personnage `{}`.",
                character
            ),
        );
    }

    let list = limited_list(items, requested_limit, |_index, item| item_line(item));

    info_embed(
        "Équipement du personnage rAthenaFR",
        format!("Objets équipés staff uniquement pour **{}**.", character),
    )
    .field("Résumé", list_summary(&list, "objets équipés"), false)
    .field("Équipement", list.value, false)
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
        format!(
            "Objets d’inventaire staff uniquement pour **{}**.",
            character
        ),
    )
    .field("Résumé", list_summary(&list, "objets d’inventaire"), false)
    .field("Objets", list.value, false)
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
        format!(
            "Propriétaires staff uniquement pour l’objet ID `{}`.",
            item_id
        ),
    )
    .field(
        "Résumé",
        list_summary(&list, "propriétaires d’objet"),
        false,
    )
    .field("Propriétaires", list.value, false)
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
}

pub fn who_sell_embed(
    item_id: i64,
    sellers: &[MarketSellEntry],
    requested_limit: u32,
) -> CreateEmbed {
    if sellers.is_empty() {
        return warning_embed(
            "Vendeurs du marché",
            format!(
                "Aucune boutique de vente active ne vend l’objet `{}`.",
                item_id
            ),
        );
    }

    let list = limited_list(sellers, requested_limit, |index, seller| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny x`{}` — `{}` sur `{}` ({}, {})",
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
}

pub fn who_buy_embed(item_id: i64, buyers: &[MarketBuyEntry], requested_limit: u32) -> CreateEmbed {
    if buyers.is_empty() {
        return warning_embed(
            "Acheteurs du marché",
            format!(
                "Aucune boutique d’achat active n’achète l’objet `{}`.",
                item_id
            ),
        );
    }

    let list = limited_list(buyers, requested_limit, |index, buyer| {
        format!(
            "`{:>2}.` **{}** — `{}` zeny x`{}` — `{}` sur `{}` ({}, {})",
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
    .field(
        "Prix de vente le plus bas",
        format!("`{}`", lowest_sell),
        true,
    )
    .field("Acheteurs", format!("`{}`", overview.buyers), true)
    .field(
        "Quantité en achat",
        format!("`{}`", format_number(overview.buy_amount)),
        true,
    )
    .field(
        "Prix d’achat le plus élevé",
        format!("`{}`", highest_buy),
        true,
    )
}

pub fn staff_only_embed() -> CreateEmbed {
    error_embed(
        "Vous n’avez pas la permission d’exécuter cette commande. Vérifiez les rôles Discord configurés avec `RATHENAFR_*_ROLE_IDS`.",
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

pub fn text_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    info_embed(title, description)
}

pub fn mvp_list_panel_embed(lines: &[String], page: usize, page_size: usize) -> CreateEmbed {
    let page_size = page_size.max(1);
    let total_count = lines.len();
    let total_pages = total_count.div_ceil(page_size).max(1);
    let page = page.min(total_pages.saturating_sub(1));
    let start = page.saturating_mul(page_size);
    let end = start.saturating_add(page_size).min(total_count);

    let description = if total_count == 0 {
        "Aucun MVP avec spawn régulier n’a été trouvé.".to_string()
    } else {
        let mut description = lines[start..end].join("\n\n");
        if page == 0 && total_count > page_size {
            description.push_str(&format!(
                "\n\nAffichage limité aux {} premiers MVP.",
                end - start
            ));
        }
        description
    };

    CreateEmbed::new()
        .title(brand_text("MVP réguliers"))
        .description(brand_text(truncate_embed_field(&description, 3900)))
        .color(COLOR_PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(format!(
            "Timers basés sur les logs MVP SQL rAthenaFR - Page {}/{} - {} MVP régulier(s)",
            page + 1,
            total_pages,
            total_count
        )))
        .timestamp(Timestamp::now())
}

pub fn mvp_last_embed(entries: &[MvpKillEntry], requested_limit: u32) -> CreateEmbed {
    if entries.is_empty() {
        return warning_embed(
            "Aucun MVP enregistré",
            "Aucun kill MVP n’a encore été enregistré dans les logs SQL.",
        );
    }

    let display_limit = (requested_limit as usize).clamp(1, 10);
    let displayed_entries = entries.iter().take(display_limit);
    let has_more_entries = entries.len() > display_limit;
    let mut embed = CreateEmbed::new()
        .title("Derniers MVP tués")
        .color(COLOR_PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(
            "Timers et logs basés sur les logs SQL rAthenaFR",
        ))
        .timestamp(Timestamp::now());

    if has_more_entries {
        embed = embed.description(format!(
            "Affichage limité aux {} derniers MVP.",
            display_limit
        ));
    }

    for entry in displayed_entries {
        embed = embed.field(
            truncate_embed_field(&mvp_kill_field_name(entry), 256),
            truncate_embed_field(&mvp_kill_field_value(entry), EMBED_FIELD_VALUE_LIMIT),
            false,
        );
    }

    embed
}

pub fn success_message_embed(title: &str, description: impl Into<String>) -> CreateEmbed {
    success_embed(title, description)
}

pub fn command_disabled_embed(command_path: &str) -> CreateEmbed {
    warning_embed(
        "Commande désactivée",
        format!("La commande `{command_path}` est désactivée par configuration."),
    )
}

pub fn error_embed(message: &str) -> CreateEmbed {
    base_embed("Erreur du bot rAthenaFR", message, COLOR_ERROR)
}

pub fn gmmsg_staff_log_embed(
    status: GmmsgLogStatus,
    discord_user_id: u64,
    action: &str,
    message: &str,
    result: &str,
) -> CreateEmbed {
    let (title, description, color, result_field) = match status {
        GmmsgLogStatus::Sent => (
            "Message GM envoyé",
            "Un message a été ajouté à la file d’envoi en jeu.",
            COLOR_SUCCESS,
            "Résultat",
        ),
        GmmsgLogStatus::Failed => (
            "Message GM non envoyé",
            "La commande a été traitée, mais le message n’a pas pu être envoyé.",
            COLOR_ERROR,
            "Erreur",
        ),
    };

    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .field("Utilisateur", format!("ID : <@{}>", discord_user_id), false)
        .field(
            "Action",
            format!("`{}`", sanitize_embed_mentions(action)),
            true,
        )
        .field(
            "Message",
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
    let (title, color, result_field) = match status {
        AccountManageLogStatus::Success => ("Compte modifié", COLOR_SUCCESS, "Résultat"),
        AccountManageLogStatus::Refused | AccountManageLogStatus::Failed => {
            ("Modification du compte refusée", COLOR_ERROR, "Erreur")
        }
    };

    let mut embed = CreateEmbed::new()
        .title(title)
        .color(color)
        .field(
            "Utilisateur staff",
            format!("ID : <@{}>", discord_user_id),
            false,
        )
        .field(
            "Action",
            format!("`{}`", sanitize_embed_mentions(action)),
            true,
        )
        .field(
            "Compte",
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
                "Raison",
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

fn service_status_lines(services: &[RAthenaFrServiceStatus]) -> String {
    if services.is_empty() {
        return "Aucun service rAthena n’est configuré.".to_string();
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
    value.into().replace("rAthenaFR", COMMAND_DISPLAY_NAME)
}

fn sanitize_embed_mentions(value: &str) -> String {
    value
        .replace("@everyone", "@\u{200B}everyone")
        .replace("@here", "@\u{200B}here")
}

fn truncate_embed_field(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit {
        return value.to_string();
    }

    let mut output = value
        .chars()
        .take(limit.saturating_sub(1))
        .collect::<String>();
    output.push('…');
    output
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

fn mvp_kill_field_name(entry: &MvpKillEntry) -> String {
    let name = if entry.monster_name.trim().is_empty() {
        format!("MVP #{}", entry.monster_id)
    } else {
        entry.monster_name.clone()
    };

    sanitize_embed_mentions(&name)
}

fn mvp_kill_field_value(entry: &MvpKillEntry) -> String {
    let date = match entry.mvp_timestamp.filter(|timestamp| *timestamp > 0) {
        Some(timestamp) => format!("<t:{timestamp}:F> (<t:{timestamp}:R>)"),
        None => entry
            .mvp_date
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Non disponible")
            .to_string(),
    };
    let mvp_exp = entry
        .mvp_exp
        .filter(|value| *value > 0)
        .map(format_number_fr)
        .unwrap_or_else(|| "Non disponible".to_string());
    let killer_name = if entry.killer_name.trim().is_empty() {
        format!("Personnage #{}", entry.killer_id)
    } else {
        entry.killer_name.clone()
    };
    let prize_name = if entry.prize_name.trim().is_empty() {
        format!("Item #{}", entry.prize_id)
    } else {
        entry.prize_name.clone()
    };
    let mut lines = vec![
        format!("Joueur : {}", sanitize_embed_mentions(&killer_name)),
        format!("Carte : `{}`", sanitize_embed_mentions(&entry.map)),
        format!("Date : {date}"),
        format!("EXP MVP : {mvp_exp}"),
        format!("Récompense : {}", sanitize_embed_mentions(&prize_name)),
    ];

    if let Some(aegis_name) = entry
        .monster_aegis_name
        .as_deref()
        .filter(|value| !value.eq_ignore_ascii_case(&entry.monster_name))
    {
        lines.push(format!(
            "Nom technique : `{}`",
            sanitize_embed_mentions(aegis_name)
        ));
    }
    if let Some(aegis_name) = entry
        .prize_aegis_name
        .as_deref()
        .filter(|value| !value.eq_ignore_ascii_case(&prize_name))
    {
        lines.push(format!(
            "Récompense technique : `{}`",
            sanitize_embed_mentions(aegis_name)
        ));
    }

    lines.join("\n")
}

fn format_number_fr(value: i64) -> String {
    let raw = value.unsigned_abs().to_string();
    let mut output = String::new();

    for (index, character) in raw.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            output.push(' ');
        }
        output.push(character);
    }

    let mut formatted = output.chars().rev().collect::<String>();
    if value < 0 {
        formatted.insert(0, '-');
    }
    formatted
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
        if !push_limited_line(
            &mut lines,
            &mut value_len,
            format_list_line(formatter(index, item)),
        ) {
            break;
        }
    }

    let displayed_count = lines.len();

    LimitedList {
        value: lines.join("\n\n"),
        displayed_count,
        available_count: items.len(),
        row_limit,
    }
}

fn list_summary(list: &LimitedList, noun: &str) -> String {
    let total_text = if list.available_count > list.row_limit {
        format!("au moins {}", list.row_limit + 1)
    } else {
        list.available_count.to_string()
    };

    let mut summary = format!(
        "{} affiché(s) sur {} {}.",
        list.displayed_count, total_text, noun
    );

    let hidden_by_row_limit = list.available_count > list.row_limit;
    let hidden_by_embed_limit = list.displayed_count < list.available_count.min(list.row_limit);
    let hidden_reason = match (hidden_by_row_limit, hidden_by_embed_limit) {
        (true, true) => {
            Some("la limite d’affichage configurée et les limites de champ des embeds Discord")
        }
        (true, false) => Some("la limite d’affichage configurée"),
        (false, true) => Some("les limites de champ des embeds Discord"),
        (false, false) => None,
    };

    if let Some(reason) = hidden_reason {
        summary.push_str(" Masqué par ");
        summary.push_str(reason);
        summary.push('.');
    }

    summary
}

fn display_limit(requested_limit: u32) -> usize {
    (requested_limit as usize).max(1)
}

fn push_limited_line(lines: &mut Vec<String>, value_len: &mut usize, line: String) -> bool {
    let separator_len = if lines.is_empty() {
        0
    } else {
        EMBED_LIST_SEPARATOR_LEN
    };
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

fn format_list_line(value: String) -> String {
    let parts = value
        .split(" — ")
        .filter(|part| !part.trim().is_empty())
        .collect::<Vec<_>>();

    if parts.len() <= 1 {
        return value;
    }

    let mut formatted = parts[0].trim().to_string();

    for detail in parts.iter().skip(1) {
        formatted.push_str("\n• ");
        formatted.push_str(detail.trim());
    }

    formatted
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

#[cfg(test)]
mod tests {
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
        assert!(value.contains("EXP MVP : 2 142 720"));
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
        assert!(value.contains("EXP MVP : Non disponible"));
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
}
