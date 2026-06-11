#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BotLocale {
    FrFr,
    EnUs,
    EsEs,
    DeDe,
    JaJp,
    KoKr,
    ZhCn,
}

impl BotLocale {
    pub const DEFAULT: Self = Self::FrFr;
    pub const ALL: [Self; 7] = [
        Self::FrFr,
        Self::EnUs,
        Self::EsEs,
        Self::DeDe,
        Self::JaJp,
        Self::KoKr,
        Self::ZhCn,
    ];

    pub const fn discord_tag(self) -> &'static str {
        match self {
            Self::FrFr => "fr",
            Self::EnUs => "en-US",
            Self::EsEs => "es-ES",
            Self::DeDe => "de",
            Self::JaJp => "ja",
            Self::KoKr => "ko",
            Self::ZhCn => "zh-CN",
        }
    }

    pub const fn file_name(self) -> &'static str {
        match self {
            Self::FrFr => "fr-FR.ftl",
            Self::EnUs => "en-US.ftl",
            Self::EsEs => "es-ES.ftl",
            Self::DeDe => "de-DE.ftl",
            Self::JaJp => "ja-JP.ftl",
            Self::KoKr => "ko-KR.ftl",
            Self::ZhCn => "zh-CN.ftl",
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
            "ja" | "ja-JP" => Self::JaJp,
            "ko" | "ko-KR" => Self::KoKr,
            "zh" | "zh-CN" | "zh-SG" | "zh-TW" | "zh-HK" => Self::ZhCn,
            _ => Self::DEFAULT,
        }
    }
}
