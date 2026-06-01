use super::options::{account_id_option, character_lookup_option, item_id_option, limit_option};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        charquests_command(),
        charequipment_command(),
        charinventory_command(),
        itemcount_command(),
        itemowners_command(),
        accountlist_command(),
        accountoverview_command(),
        accountmanage_command(),
        banlist_command(),
        accountchars_command(),
        accountstatus_command(),
    ]
}

fn charquests_command() -> CreateCommand {
    CreateCommand::new("charquests")
        .description(
            "Staff uniquement : liste les quêtes liées à un personnage depuis la base de données.",
        )
        .add_option(character_lookup_option())
        .add_option(limit_option())
}

fn charequipment_command() -> CreateCommand {
    CreateCommand::new("charequipment")
        .description("Staff uniquement : liste les objets équipés d’un personnage depuis la base de données.")
        .add_option(character_lookup_option())
        .add_option(limit_option())
}

fn charinventory_command() -> CreateCommand {
    CreateCommand::new("charinventory")
        .description(
            "Staff uniquement : liste l’inventaire d’un personnage depuis la base de données.",
        )
        .add_option(character_lookup_option())
        .add_option(limit_option())
}

fn itemcount_command() -> CreateCommand {
    CreateCommand::new("itemcount")
        .description(
            "Staff uniquement : compte un objet par ID dans les tables natives d’inventaire.",
        )
        .add_option(item_id_option())
}

fn itemowners_command() -> CreateCommand {
    CreateCommand::new("itemowners")
        .description(
            "Staff uniquement : liste les propriétaires visibles d’un objet dans l’inventaire.",
        )
        .add_option(item_id_option())
        .add_option(limit_option())
}

fn accountlist_command() -> CreateCommand {
    CreateCommand::new("accountlist")
        .description("GM uniquement : liste les comptes créés dans la table login.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "page",
                "Page à afficher, triée du plus récent au plus ancien.",
            )
            .min_int_value(1)
            .required(false),
        )
        .add_option(limit_option())
}

fn accountoverview_command() -> CreateCommand {
    CreateCommand::new("accountoverview")
        .description("Staff uniquement : affiche un résumé sûr et compact d’un compte.")
        .add_option(account_id_option())
        .add_option(limit_option())
}

fn accountmanage_command() -> CreateCommand {
    CreateCommand::new("accountmanage")
        .description("GM uniquement : gère complètement un compte utilisateur rAthena.")
        .add_option(account_id_option())
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "action", "Action à effectuer.")
                .required(true)
                .add_string_choice("Éditer le compte", "edit")
                .add_string_choice("Supprimer tout le compte", "delete"),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "confirm",
                "Confirmation pour delete : DELETE-ALL-ID.",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "username", "Nouveau login.")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "password",
                "Nouveau mot de passe.",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "sex", "Sexe du compte.")
                .required(false)
                .add_string_choice("Homme", "M")
                .add_string_choice("Femme", "F"),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "birthdate",
                "Nouvelle date de naissance au format YYYY-MM-DD.",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "email", "Nouvel email.")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Integer, "group_id", "Nouveau group_id.")
                .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "state",
                "Nouvel état du compte.",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "unban_time",
                "Nouveau timestamp de fin de bannissement, 0 pour aucun.",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "expiration_time",
                "Nouveau timestamp d’expiration, 0 pour aucune.",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "character_slots",
                "Nouveau nombre de slots de personnages.",
            )
            .required(false),
        )
}

fn banlist_command() -> CreateCommand {
    CreateCommand::new("banlist")
        .description(
            "Staff uniquement : liste les comptes bloqués ou bannis depuis la table login.",
        )
        .add_option(limit_option())
}

fn accountchars_command() -> CreateCommand {
    CreateCommand::new("accountchars")
        .description(
            "Staff uniquement : liste les personnages liés à un compte depuis la base de données.",
        )
        .add_option(account_id_option())
        .add_option(limit_option())
}

fn accountstatus_command() -> CreateCommand {
    CreateCommand::new("accountstatus")
        .description(
            "Staff uniquement : affiche les champs sûrs d’état du compte depuis la table login.",
        )
        .add_option(account_id_option())
}
