use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        server_command(),
        online_command(),
        player_command(),
        guild_command(),
        castle_command(),
        item_command(),
        who_drops_command(),
        mob_command(),
        mvp_command(),
        top_command(),
        rank_command(),
        market_command(),
        createaccount_command(),
    ]
}

pub(super) fn createaccount_definition() -> CreateCommand {
    createaccount_command()
}

fn server_command() -> CreateCommand {
    CreateCommand::new("server").description("Résumé public du serveur rAthena.")
}

fn online_command() -> CreateCommand {
    CreateCommand::new("online")
        .description("Joueurs connectés et répartition par map.")
        .add_option(subcommand("count", "Nombre de joueurs connectés."))
        .add_option(
            subcommand("list", "Liste des joueurs connectés.").add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("map", "Répartition des joueurs connectés par map.")
                .add_sub_option(limit_option()),
        )
}

fn player_command() -> CreateCommand {
    CreateCommand::new("player")
        .description("Profil public d'un personnage.")
        .add_option(character_name_option())
}

fn guild_command() -> CreateCommand {
    CreateCommand::new("guild")
        .description("Informations publiques des guildes.")
        .add_option(
            subcommand("info", "Informations publiques d'une guilde.")
                .add_sub_option(guild_name_option()),
        )
        .add_option(
            subcommand("members", "Liste publique des membres d'une guilde.")
                .add_sub_option(guild_name_option())
                .add_sub_option(limit_option()),
        )
}

fn castle_command() -> CreateCommand {
    CreateCommand::new("castle")
        .description("Châteaux et propriétaires WoE.")
        .add_option(
            subcommand("list", "Liste des châteaux et propriétaires.")
                .add_sub_option(limit_option()),
        )
        .add_option(subcommand("info", "Détail d'un château.").add_sub_option(castle_id_option()))
}

fn item_command() -> CreateCommand {
    CreateCommand::new("item")
        .description("Recherche et fiche item.")
        .add_option(
            subcommand("info", "Fiche complète d'un item.").add_sub_option(item_query_option()),
        )
        .add_option(
            subcommand("search", "Recherche d'items par nom partiel.")
                .add_sub_option(text_option("text", "Texte à rechercher."))
                .add_sub_option(limit_option()),
        )
}

fn who_drops_command() -> CreateCommand {
    CreateCommand::new("who-drops")
        .description("Liste les monstres qui drop un item.")
        .add_option(item_query_option())
        .add_option(limit_option())
}

fn mob_command() -> CreateCommand {
    CreateCommand::new("mob")
        .description("Recherche et drops de monstres.")
        .add_option(subcommand("info", "Fiche monstre.").add_sub_option(mob_query_option()))
        .add_option(
            subcommand("drops", "Drops d'un monstre.")
                .add_sub_option(mob_query_option())
                .add_sub_option(limit_option()),
        )
}

fn mvp_command() -> CreateCommand {
    CreateCommand::new("mvp")
        .description("MVP et journaux MVP si disponibles.")
        .add_option(subcommand("list", "Liste des MVP.").add_sub_option(limit_option()))
        .add_option(
            subcommand("last", "Derniers MVP tués depuis les logs.").add_sub_option(limit_option()),
        )
        .add_option(subcommand("top", "Top tueurs MVP.").add_sub_option(limit_option()))
}

fn top_command() -> CreateCommand {
    CreateCommand::new("top")
        .description("Classements publics.")
        .add_option(
            subcommand("level", "Classement par base level.").add_sub_option(limit_option()),
        )
        .add_option(subcommand("job", "Classement par job level.").add_sub_option(limit_option()))
        .add_option(subcommand("guild", "Classement des guildes.").add_sub_option(limit_option()))
        .add_option(
            subcommand("zeny", "Classement zeny selon configuration.")
                .add_sub_option(limit_option()),
        )
}

fn rank_command() -> CreateCommand {
    CreateCommand::new("rank")
        .description("Résumé des positions publiques d'un personnage.")
        .add_option(character_name_option())
}

fn market_command() -> CreateCommand {
    CreateCommand::new("market")
        .description("Marché vending/buying store si les tables existent.")
        .add_option(
            subcommand("info", "Résumé achat/vente d'un item.").add_sub_option(item_query_option()),
        )
        .add_option(
            subcommand("sell", "Prix de vente actuels.")
                .add_sub_option(item_query_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("buy", "Buying stores actifs.")
                .add_sub_option(item_query_option())
                .add_sub_option(limit_option()),
        )
}

fn createaccount_command() -> CreateCommand {
    CreateCommand::new("createaccount")
        .description("Créer un compte rAthena.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "username", "Nom du compte.")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "password",
                "Mot de passe du compte.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "sex", "Sexe du compte.")
                .required(true)
                .add_string_choice("Homme", "M")
                .add_string_choice("Femme", "F"),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "birthdate",
                "Date de naissance au format YYYY-MM-DD, exemple : 1998-07-14",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "email", "Email du compte.")
                .required(false),
        )
}

fn subcommand(name: &str, description: &str) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, name, description)
}

fn text_option(name: &str, description: &str) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::String, name, description).required(true)
}

fn character_name_option() -> CreateCommandOption {
    text_option("name", "Nom du personnage.")
}

fn guild_name_option() -> CreateCommandOption {
    text_option("name", "Nom de la guilde.")
}

fn castle_id_option() -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::Integer, "castle_id", "ID du château.")
        .min_int_value(0)
        .required(true)
}

fn item_query_option() -> CreateCommandOption {
    text_option("item", "Nom partiel ou ID de l'item.")
}

fn mob_query_option() -> CreateCommandOption {
    text_option("mob", "Nom partiel ou ID du monstre.")
}

fn limit_option() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::Integer,
        "limit",
        "Nombre maximum de lignes à afficher.",
    )
    .min_int_value(1)
    .required(false)
}
