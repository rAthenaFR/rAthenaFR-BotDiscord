use super::options::{
    castle_id_option, character_lookup_option, character_name_option, guild_name_option,
    item_id_option, limit_option, party_name_option, quest_id_option,
};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        status_command(),
        online_command(),
        top_command(),
        player_command(),
        guilds_command(),
        search_command(),
        createaccount_command(),
        topzeny_command(),
        guild_command(),
        guildmembers_command(),
        classes_command(),
        mapstats_command(),
        maponline_command(),
        party_command(),
        partymembers_command(),
        homunculus_command(),
        pet_command(),
        zeny_command(),
        castles_command(),
        castle_command(),
        guildalliances_command(),
        guildskills_command(),
        homunculustop_command(),
        pettop_command(),
        queststats_command(),
        whosell_command(),
        whobuy_command(),
        market_command(),
        venders_command(),
        buyers_command(),
    ]
}

fn status_command() -> CreateCommand {
    CreateCommand::new("status")
        .description("Affiche l’état des services et les compteurs de la base de données.")
}

fn online_command() -> CreateCommand {
    CreateCommand::new("online")
        .description("Liste les personnages actuellement connectés depuis la base de données.")
        .add_option(limit_option())
}

fn top_command() -> CreateCommand {
    CreateCommand::new("top")
        .description("Affiche le classement des personnages par niveau depuis la base de données.")
        .add_option(limit_option())
}

fn player_command() -> CreateCommand {
    CreateCommand::new("player")
        .description("Recherche le profil d’un personnage dans la base de données.")
        .add_option(character_name_option())
}

fn guilds_command() -> CreateCommand {
    CreateCommand::new("guilds")
        .description("Affiche les guildes les plus fortes depuis la base de données.")
        .add_option(limit_option())
}

fn search_command() -> CreateCommand {
    CreateCommand::new("search")
        .description("Recherche personnages, objets et monstres par nom partiel ou ID.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "query",
                "Nom partiel ou ID à rechercher.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "category",
                "Catégorie à rechercher.",
            )
            .required(false)
            .add_string_choice("Tout", "all")
            .add_string_choice("Joueurs", "players")
            .add_string_choice("Items", "items")
            .add_string_choice("Monstres", "monsters"),
        )
        .add_option(limit_option())
}

fn createaccount_command() -> CreateCommand {
    CreateCommand::new("createaccount")
        .description("Créer un compte rAthena")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "username", "Nom du compte")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "password",
                "Mot de passe du compte",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "sex", "Sexe du compte")
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
            CreateCommandOption::new(CommandOptionType::String, "email", "Email du compte")
                .required(false),
        )
}

fn topzeny_command() -> CreateCommand {
    CreateCommand::new("topzeny")
        .description("Affiche les personnages visibles les plus riches depuis la base de données.")
        .add_option(limit_option())
}

fn guild_command() -> CreateCommand {
    CreateCommand::new("guild")
        .description("Affiche les informations détaillées d’une guilde depuis la base de données.")
        .add_option(guild_name_option())
}

fn guildmembers_command() -> CreateCommand {
    CreateCommand::new("guildmembers")
        .description("Liste les membres d’une guilde depuis la base de données.")
        .add_option(guild_name_option())
        .add_option(limit_option())
}

fn classes_command() -> CreateCommand {
    CreateCommand::new("classes")
        .description(
            "Affiche la répartition des personnages visibles par classe depuis la base de données.",
        )
        .add_option(limit_option())
}

fn mapstats_command() -> CreateCommand {
    CreateCommand::new("mapstats")
        .description(
            "Affiche la répartition des personnages visibles par carte depuis la base de données.",
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "online_only",
                "Compter uniquement les personnages actuellement connectés.",
            )
            .required(false),
        )
        .add_option(limit_option())
}

fn maponline_command() -> CreateCommand {
    CreateCommand::new("maponline")
        .description(
            "Liste les personnages connectés sur une carte précise depuis la base de données.",
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "map",
                "Nom de la carte, par exemple prontera.",
            )
            .required(true),
        )
        .add_option(limit_option())
}

fn party_command() -> CreateCommand {
    CreateCommand::new("party")
        .description(
            "Affiche les informations détaillées d’un groupe rAthena depuis la base de données.",
        )
        .add_option(party_name_option())
}

fn partymembers_command() -> CreateCommand {
    CreateCommand::new("partymembers")
        .description("Liste les membres visibles d’un groupe rAthena depuis la base de données.")
        .add_option(party_name_option())
        .add_option(limit_option())
}

fn homunculus_command() -> CreateCommand {
    CreateCommand::new("homunculus")
        .description("Affiche l’homoncule possédé par un personnage depuis la base de données.")
        .add_option(character_lookup_option())
}

fn pet_command() -> CreateCommand {
    CreateCommand::new("pet")
        .description("Affiche le familier possédé par un personnage depuis la base de données.")
        .add_option(character_lookup_option())
}

fn zeny_command() -> CreateCommand {
    CreateCommand::new("zeny")
        .description("Affiche les statistiques visibles de zeny depuis la base de données.")
}

fn castles_command() -> CreateCommand {
    CreateCommand::new("castles")
        .description("Affiche les propriétaires de châteaux et leurs données économiques depuis la base de données.")
        .add_option(limit_option())
}

fn castle_command() -> CreateCommand {
    CreateCommand::new("castle")
        .description("Affiche les informations détaillées d’un château depuis la base de données.")
        .add_option(castle_id_option())
}

fn guildalliances_command() -> CreateCommand {
    CreateCommand::new("guildalliances")
        .description("Affiche les alliances et oppositions d’une guilde depuis la base de données.")
        .add_option(guild_name_option())
        .add_option(limit_option())
}

fn guildskills_command() -> CreateCommand {
    CreateCommand::new("guildskills")
        .description("Affiche les compétences apprises par une guilde depuis la base de données.")
        .add_option(guild_name_option())
        .add_option(limit_option())
}

fn homunculustop_command() -> CreateCommand {
    CreateCommand::new("homunculustop")
        .description("Affiche le classement des homoncules depuis la base de données.")
        .add_option(limit_option())
}

fn pettop_command() -> CreateCommand {
    CreateCommand::new("pettop")
        .description("Affiche le classement des familiers depuis la base de données.")
        .add_option(limit_option())
}

fn queststats_command() -> CreateCommand {
    CreateCommand::new("queststats")
        .description("Affiche les statistiques globales d’une quête à partir de son ID.")
        .add_option(quest_id_option())
}

fn whosell_command() -> CreateCommand {
    CreateCommand::new("whosell")
        .description("Recherche les boutiques qui vendent un objet par ID dans la base cible.")
        .add_option(item_id_option())
        .add_option(limit_option())
}

fn whobuy_command() -> CreateCommand {
    CreateCommand::new("whobuy")
        .description("Recherche les boutiques qui achètent un objet par ID dans la base cible.")
        .add_option(item_id_option())
        .add_option(limit_option())
}

fn market_command() -> CreateCommand {
    CreateCommand::new("market")
        .description("Affiche un résumé achat/vente du marché pour un objet par ID.")
        .add_option(item_id_option())
}

fn venders_command() -> CreateCommand {
    CreateCommand::new("venders")
        .description("Liste les boutiques de vente actives depuis la base cible.")
        .add_option(limit_option())
}

fn buyers_command() -> CreateCommand {
    CreateCommand::new("buyers")
        .description("Liste les boutiques d’achat actives depuis la base cible.")
        .add_option(limit_option())
}
