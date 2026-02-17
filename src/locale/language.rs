use bevy::prelude::*;

/// Supported application languages.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    En,
    Ru,
    De,
    Fr,
    Es,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ru => "ru",
            Self::De => "de",
            Self::Fr => "fr",
            Self::Es => "es",
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" => Ok(Self::En),
            "ru" | "ru-ru" => Ok(Self::Ru),
            "de" => Ok(Self::De),
            "fr" => Ok(Self::Fr),
            "es" => Ok(Self::Es),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}
