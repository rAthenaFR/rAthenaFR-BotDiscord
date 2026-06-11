#![allow(dead_code, unused_imports)]

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

mod account;
mod castle;
mod character;
mod common;
mod guild;
mod i18n;
mod market;
mod mob;
mod mvp;
mod player;
mod ranking;
mod server;
mod shared;
mod staff_log;

#[cfg(test)]
mod tests;

pub use account::*;
pub use castle::*;
pub use character::*;
pub use common::*;
pub use guild::*;
pub(super) use i18n::*;
pub use market::*;
pub use mob::*;
pub use mvp::*;
pub use player::*;
pub use ranking::*;
pub use server::*;
pub(super) use shared::*;
pub use staff_log::*;
