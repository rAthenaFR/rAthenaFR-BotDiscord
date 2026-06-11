use super::{keys::I18nKey, loader, locale::BotLocale};

#[derive(Debug, Clone, Copy)]
pub struct TranslationArg<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> TranslationArg<'a> {
    pub const fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}

pub fn translate(locale: BotLocale, key: I18nKey) -> String {
    loader::lookup(locale, key.as_str())
        .unwrap_or(key.as_str())
        .to_string()
}

pub fn translate_with_args(locale: BotLocale, key: I18nKey, args: &[TranslationArg<'_>]) -> String {
    let mut value = translate(locale, key);

    for arg in args {
        let token = format!("{{ ${} }}", arg.name);
        value = value.replace(&token, arg.value);
    }

    value
}
