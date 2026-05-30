pub mod embeds;

mod command_registry;
mod interactions;

pub use command_registry::deploy_commands;
pub use interactions::create_client;
