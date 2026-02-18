use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported UI languages.
#[derive(
    Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[reflect(Resource)]
pub enum Language {
    #[default]
    EnUs,
    RuRu,
    DeDe,
    FrFr,
    EsEs,
    JaJp,
}

impl Language {
    /// BCP-47 locale string used for Fluent bundle IDs and directory lookup.
    pub fn as_locale_str(&self) -> &'static str {
        match self {
            Self::EnUs => "en-US",
            Self::RuRu => "ru-RU",
            Self::DeDe => "de-DE",
            Self::FrFr => "fr-FR",
            Self::EsEs => "es-ES",
            Self::JaJp => "ja-JP",
        }
    }

    /// Human-readable display name in the language itself.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::EnUs => "English",
            Self::RuRu => "Русский",
            Self::DeDe => "Deutsch",
            Self::FrFr => "Français",
            Self::EsEs => "Español",
            Self::JaJp => "日本語",
        }
    }
}

impl FromStr for Language {
    type Err = ();

    /// Parses "en", "en-US", "en_US", "english" case-insensitively.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "en" | "en-us" | "en_us" | "english" => Ok(Self::EnUs),
            "ru" | "ru-ru" | "ru_ru" | "russian" | "русский" => Ok(Self::RuRu),
            "de" | "de-de" | "de_de" | "german" | "deutsch" => Ok(Self::DeDe),
            "fr" | "fr-fr" | "fr_fr" | "french" | "français" => Ok(Self::FrFr),
            "es" | "es-es" | "es_es" | "spanish" | "español" => Ok(Self::EsEs),
            "ja" | "ja-jp" | "ja_jp" | "japanese" | "日本語" => Ok(Self::JaJp),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_locale_str())
    }
}
