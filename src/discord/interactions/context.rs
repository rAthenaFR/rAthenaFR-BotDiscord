use super::dispatcher::BotState;
use crate::i18n::BotLocale;
use serenity::all::{CommandInteraction, Context};

/// Contexte commun prévu pour les handlers de commandes.
///
/// Il évite de repasser séparément `Context`, `CommandInteraction`, `BotState`
/// et la locale dans chaque handler pendant la migration.
pub struct CommandContext<'a> {
    pub serenity: &'a Context,
    pub command: &'a CommandInteraction,
    pub state: &'a BotState,
    pub locale: BotLocale,
}

impl<'a> CommandContext<'a> {
    pub const fn new(
        serenity: &'a Context,
        command: &'a CommandInteraction,
        state: &'a BotState,
        locale: BotLocale,
    ) -> Self {
        Self {
            serenity,
            command,
            state,
            locale,
        }
    }
}
