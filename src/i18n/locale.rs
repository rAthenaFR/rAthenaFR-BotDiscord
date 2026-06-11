#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BotLocale {
    FrFr,
    EnUs,
    EsEs,
    DeDe,
}

impl BotLocale {
    pub const DEFAULT: Self = Self::FrFr;
    pub const ALL: [Self; 4] = [Self::FrFr, Self::EnUs, Self::EsEs, Self::DeDe];

    pub const fn discord_tag(self) -> &'static str {
        match self {
            Self::FrFr => "fr",
            Self::EnUs => "en-US",
            Self::EsEs => "es-ES",
            Self::DeDe => "de",
        }
    }

    pub const fn file_name(self) -> &'static str {
        match self {
            Self::FrFr => "fr-FR.ftl",
            Self::EnUs => "en-US.ftl",
            Self::EsEs => "es-ES.ftl",
            Self::DeDe => "de-DE.ftl",
        }
    }

    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }

    pub fn from_discord(value: &str) -> Self {
        match value {
            "fr" | "fr-FR" | "fr-BE" | "fr-CA" | "fr-CH" => Self::FrFr,
            "en" | "en-US" | "en-GB" => Self::EnUs,
            "es" | "es-ES" | "es-419" => Self::EsEs,
            "de" | "de-DE" => Self::DeDe,
            _ => Self::DEFAULT,
        }
    }
}
