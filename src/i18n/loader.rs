use super::locale::BotLocale;

const FR_FR: &str = include_str!("../../locales/fr-FR.ftl");
const EN_US: &str = include_str!("../../locales/en-US.ftl");
const ES_ES: &str = include_str!("../../locales/es-ES.ftl");
const DE_DE: &str = include_str!("../../locales/de-DE.ftl");

pub fn catalog(locale: BotLocale) -> &'static str {
    match locale {
        BotLocale::FrFr => FR_FR,
        BotLocale::EnUs => EN_US,
        BotLocale::EsEs => ES_ES,
        BotLocale::DeDe => DE_DE,
    }
}

pub fn lookup(locale: BotLocale, key: &str) -> Option<&'static str> {
    lookup_in_catalog(catalog(locale), key)
        .or_else(|| lookup_in_catalog(catalog(BotLocale::DEFAULT), key))
}

fn lookup_in_catalog(catalog: &'static str, key: &str) -> Option<&'static str> {
    catalog.lines().find_map(|line| {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let (entry_key, value) = line.split_once('=')?;
        if entry_key.trim() == key {
            Some(value.trim())
        } else {
            None
        }
    })
}
