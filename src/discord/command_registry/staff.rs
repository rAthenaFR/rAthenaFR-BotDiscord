use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        staff_command(),
        mod_command(),
        debug_command(),
        audit_command(),
        db_command(),
        gmmsg_command(),
    ]
}

fn staff_command() -> CreateCommand {
    CreateCommand::new("staff")
        .description("Commandes staff essentielles en lecture seule.")
        .add_option(
            subcommand("player", "Fiche complète d'un personnage.")
                .add_sub_option(character_option()),
        )
        .add_option(
            subcommand(
                "account",
                "Compte lié au personnage, sans mot de passe ni hash.",
            )
            .add_sub_option(character_option()),
        )
        .add_option(
            subcommand(
                "chars",
                "Personnages d'un compte ou du compte lié à un personnage.",
            )
            .add_sub_option(lookup_option()),
        )
        .add_option(
            subcommand("inventory", "Inventaire complet du personnage.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("equipment", "Equipement porte par le personnage.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("cart", "Contenu du cart.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("storage", "Storage du compte.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("guildstorage", "Coffre de guilde.")
                .add_sub_option(guild_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("whohas", "Recherche qui possède un item.")
                .add_sub_option(item_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("item-search", "Recherche d'un item dans les conteneurs.")
                .add_sub_option(item_option())
                .add_sub_option(limit_option()),
        )
        .add_option(subcommand("zeny", "Zeny d'un personnage.").add_sub_option(character_option()))
        .add_option(
            subcommand("zenylog", "Historique zeny si les logs existent.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("picklog", "Historique items si les logs existent.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("trade-log", "Échanges joueur/joueur si les logs existent.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("mvp-log", "MVP tués par un joueur.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("loginlog", "Historique de connexion.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("ip-accounts", "Comptes partageant les mêmes IP.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("multiaccount", "Détection multi-compte.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("banned", "Liste des comptes bannis ou bloqués.")
                .add_sub_option(limit_option()),
        )
}

fn mod_command() -> CreateCommand {
    CreateCommand::new("mod")
        .description("Commandes de modération en lecture seule.")
        .add_option(
            subcommand("chatlog", "Messages récents d'un joueur.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("chat-search", "Recherche dans les logs de chat.")
                .add_sub_option(text_option("text", "Texte à rechercher."))
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("report-context", "Contexte rapide d'un signalement.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("branchlog", "Utilisation Dead Branch/Bloody Branch.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
}

fn debug_command() -> CreateCommand {
    CreateCommand::new("debug")
        .description("Commandes debug rAthena en lecture seule.")
        .add_option(
            subcommand("quest", "Quêtes actives/terminées d'un personnage.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("char-vars", "Variables personnage.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("acc-vars", "Variables compte.")
                .add_sub_option(character_option())
                .add_sub_option(limit_option()),
        )
}

fn audit_command() -> CreateCommand {
    CreateCommand::new("audit")
        .description("Audit staff et logs GM en lecture seule.")
        .add_option(
            subcommand("atcommands", "Commandes GM utilisées par un GM.")
                .add_sub_option(text_option("gm", "Nom du GM."))
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand(
                "item-created",
                "Items créés par commandes/admin/scripts si détectables.",
            )
            .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("zeny-created", "Zeny créé ou retiré si détectable.")
                .add_sub_option(limit_option()),
        )
        .add_option(
            subcommand("gm-activity", "Résumé d'activité d'un GM.")
                .add_sub_option(text_option("gm", "Nom du GM."))
                .add_sub_option(limit_option()),
        )
}

fn db_command() -> CreateCommand {
    CreateCommand::new("db")
        .description("Diagnostic base rAthena en lecture seule.")
        .add_option(subcommand(
            "health",
            "Tables présentes, manquantes et logs actifs.",
        ))
        .add_option(
            subcommand("tables", "Liste les tables rAthena détectées.")
                .add_sub_option(limit_option()),
        )
        .add_option(subcommand("count", "Nombre de lignes par table utile."))
        .add_option(subcommand("logs-size", "Volume des logs SQL."))
        .add_option(
            subcommand("last-update", "État de sql_updates si la table existe.")
                .add_sub_option(limit_option()),
        )
}

fn gmmsg_command() -> CreateCommand {
    CreateCommand::new("gmmsg")
        .description("Messages en jeu via GameBridge.")
        .add_option(
            subcommand("server", "Message global serveur.").add_sub_option(message_option()),
        )
        .add_option(
            subcommand("map", "Message sur une map, si le bridge le supporte.")
                .add_sub_option(text_option("map", "Nom de la map."))
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("blue", "Annonce bleue, si le bridge la supporte.")
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("color", "Annonce couleur RRGGBB, si le bridge la supporte.")
                .add_sub_option(text_option("hex", "Couleur RRGGBB."))
                .add_sub_option(message_option()),
        )
        .add_option(
            subcommand("test", "Mode test/log uniquement.").add_sub_option(message_option()),
        )
}

fn subcommand(name: &str, description: &str) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, name, description)
}

fn text_option(name: &str, description: &str) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::String, name, description).required(true)
}

fn character_option() -> CreateCommandOption {
    text_option("character", "Nom du personnage.")
}

fn lookup_option() -> CreateCommandOption {
    text_option("lookup", "Nom de personnage ou account_id.")
}

fn guild_option() -> CreateCommandOption {
    text_option("guild", "Nom de guilde.")
}

fn item_option() -> CreateCommandOption {
    text_option("item", "Nom partiel ou ID de l'item.")
}

fn message_option() -> CreateCommandOption {
    text_option("message", "Message à envoyer.")
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
