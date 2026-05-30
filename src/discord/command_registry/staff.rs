use super::options::{account_id_option, character_lookup_option, item_id_option, limit_option};
use serenity::all::CreateCommand;

pub(super) fn command_definitions() -> Vec<CreateCommand> {
    vec![
        charquests_command(),
        charequipment_command(),
        charinventory_command(),
        itemcount_command(),
        itemowners_command(),
        accountoverview_command(),
        banlist_command(),
        accountchars_command(),
        accountstatus_command(),
    ]
}

fn charquests_command() -> CreateCommand {
    CreateCommand::new("charquests")
        .description("Staff uniquement : liste les quêtes liées à un personnage depuis la base de données.")
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
        .description("Staff uniquement : liste l’inventaire d’un personnage depuis la base de données.")
        .add_option(character_lookup_option())
        .add_option(limit_option())
}

fn itemcount_command() -> CreateCommand {
    CreateCommand::new("itemcount")
        .description("Staff uniquement : compte un objet par ID dans les tables natives d’inventaire.")
        .add_option(item_id_option())
}

fn itemowners_command() -> CreateCommand {
    CreateCommand::new("itemowners")
        .description("Staff uniquement : liste les propriétaires visibles d’un objet dans l’inventaire.")
        .add_option(item_id_option())
        .add_option(limit_option())
}

fn accountoverview_command() -> CreateCommand {
    CreateCommand::new("accountoverview")
        .description("Staff uniquement : affiche un résumé sûr et compact d’un compte.")
        .add_option(account_id_option())
        .add_option(limit_option())
}

fn banlist_command() -> CreateCommand {
    CreateCommand::new("banlist")
        .description("Staff uniquement : liste les comptes bloqués ou bannis depuis la table login.")
        .add_option(limit_option())
}

fn accountchars_command() -> CreateCommand {
    CreateCommand::new("accountchars")
        .description("Staff uniquement : liste les personnages liés à un compte depuis la base de données.")
        .add_option(account_id_option())
        .add_option(limit_option())
}

fn accountstatus_command() -> CreateCommand {
    CreateCommand::new("accountstatus")
        .description("Staff uniquement : affiche les champs sûrs d’état du compte depuis la table login.")
        .add_option(account_id_option())
}
