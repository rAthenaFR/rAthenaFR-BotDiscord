use serenity::all::{CommandOptionType, CreateCommandOption};

pub(super) fn character_name_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::String,
        "name",
        "Nom du personnage à rechercher.",
    )
    .required(true)
}

pub(super) fn character_lookup_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::String,
        "character",
        "Nom du personnage à inspecter.",
    )
    .required(true)
}

pub(super) fn guild_name_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::String,
        "name",
        "Nom de la guilde à rechercher.",
    )
    .required(true)
}

pub(super) fn party_name_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::String,
        "name",
        "Nom du groupe à rechercher.",
    )
    .required(true)
}

pub(super) fn account_id_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::Integer,
        "account_id",
        "ID du compte dans la base cible.",
    )
    .min_int_value(1)
    .required(true)
}

pub(super) fn castle_id_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::Integer,
        "castle_id",
        "ID du château dans la base cible.",
    )
    .min_int_value(0)
    .required(true)
}

pub(super) fn quest_id_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::Integer,
        "quest_id",
        "ID de la quête dans la base cible.",
    )
    .min_int_value(1)
    .required(true)
}

pub(super) fn item_id_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::Integer,
        "item_id",
        "ID de l’objet à inspecter dans la base cible.",
    )
    .min_int_value(1)
    .required(true)
}

pub(super) fn limit_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::Integer,
        "limit",
        "Nombre maximum de lignes à afficher.",
    )
    .min_int_value(1)
    .required(false)
}
