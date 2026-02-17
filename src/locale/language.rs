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
