pub(in crate::discord::interactions::dispatcher) mod access;
pub(in crate::discord::interactions::dispatcher) mod account;
pub(in crate::discord::interactions::dispatcher) mod account_manage;
pub(in crate::discord::interactions::dispatcher) mod audit;
pub(in crate::discord::interactions::dispatcher) mod core;
pub(in crate::discord::interactions::dispatcher) mod debug;
pub(in crate::discord::interactions::dispatcher) mod gm_message;
pub(in crate::discord::interactions::dispatcher) mod inventory;
pub(in crate::discord::interactions::dispatcher) mod logs;
pub(in crate::discord::interactions::dispatcher) mod moderation;

// Les méthodes sont définies comme impl Handler dans chaque sous-module.

pub(in crate::discord::interactions::dispatcher) use access::{
    has_configured_role, ConfiguredRoles,
};
