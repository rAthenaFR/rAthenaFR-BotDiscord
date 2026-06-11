#![allow(unused_imports)]

pub mod keys;
pub mod loader;
pub mod locale;
pub mod translator;

pub use keys::I18nKey;
pub use locale::BotLocale;
pub use translator::{translate, translate_with_args, TranslationArg};

#[cfg(test)]
mod tests;
