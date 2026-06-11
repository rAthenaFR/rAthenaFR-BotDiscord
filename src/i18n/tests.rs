use super::{loader, BotLocale};
use std::collections::{BTreeMap, BTreeSet};

fn catalog_entries(locale: BotLocale) -> BTreeMap<&'static str, &'static str> {
    loader::catalog(locale)
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }

            line.split_once('=')
                .map(|(key, value)| (key.trim(), value.trim()))
        })
        .collect()
}

fn placeholders(value: &str) -> BTreeSet<&str> {
    value
        .match_indices("{ $")
        .filter_map(|(start, _)| {
            let name = &value[start + 3..];
            let end = name.find([' ', '}'])?;
            Some(&name[..end])
        })
        .collect()
}

fn typed_keys() -> BTreeSet<String> {
    include_str!("keys.rs")
        .split('"')
        .filter(|value| {
            !value.is_empty()
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        })
        .map(ToOwned::to_owned)
        .collect()
}

#[test]
fn every_catalog_contains_the_same_keys_and_placeholders() {
    let fallback = catalog_entries(BotLocale::DEFAULT);

    for locale in BotLocale::all() {
        let entries = catalog_entries(*locale);
        assert_eq!(
            entries.keys().collect::<BTreeSet<_>>(),
            fallback.keys().collect::<BTreeSet<_>>(),
            "catalog key mismatch for {}",
            locale.file_name()
        );

        for (key, fallback_value) in &fallback {
            assert_eq!(
                placeholders(entries[key]),
                placeholders(fallback_value),
                "placeholder mismatch for {key} in {}",
                locale.file_name()
            );
        }
    }
}

#[test]
fn every_typed_key_exists_in_all_catalogs() {
    let expected = typed_keys();
    assert!(!expected.is_empty());

    for locale in BotLocale::all() {
        let entries = catalog_entries(*locale);
        let missing = expected
            .iter()
            .filter(|key| !entries.contains_key(key.as_str()))
            .collect::<Vec<_>>();

        assert!(
            missing.is_empty(),
            "missing typed keys in {}: {missing:?}",
            locale.file_name()
        );
    }
}

#[test]
fn discord_locales_are_normalized_with_french_fallback() {
    for value in ["fr", "fr-FR", "fr-BE", "fr-CA", "fr-CH"] {
        assert_eq!(BotLocale::from_discord(value), BotLocale::FrFr);
    }
    for value in ["en", "en-US", "en-GB"] {
        assert_eq!(BotLocale::from_discord(value), BotLocale::EnUs);
    }
    for value in ["es", "es-ES", "es-419"] {
        assert_eq!(BotLocale::from_discord(value), BotLocale::EsEs);
    }
    for value in ["de", "de-DE"] {
        assert_eq!(BotLocale::from_discord(value), BotLocale::DeDe);
    }

    assert_eq!(BotLocale::DEFAULT, BotLocale::FrFr);
    assert_eq!(BotLocale::from_discord("unsupported"), BotLocale::FrFr);
}
