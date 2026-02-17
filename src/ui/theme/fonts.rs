use bevy::prelude::*;

/// Resource for defining theme fonts.
#[derive(Resource, Debug, Clone)]
pub struct ThemeFonts {
    /// None  → Bevy built-in FiraMono (zero binary cost, always available).
    /// Some  → path loaded via AssetServer or embedded:// URI.
    pub regular: Option<String>,
    pub bold: Option<String>,
    pub mono: Option<String>, // None → Bevy built-in FiraMono
}

impl ThemeFonts {
    /// Bevy's built-in font everywhere — zero bytes added to binary.
    pub fn bevy_default() -> Self {
        Self {
            regular: None,
            bold: None,
            mono: None,
        }
    }

    /// Embedded Noto Sans — full Unicode including Cyrillic and CJK.
    pub fn noto_sans() -> Self {
        Self {
            regular: Some("embedded://bevy_launchpad/fonts/NotoSans-Regular.ttf".into()),
            bold: Some("embedded://bevy_launchpad/fonts/NotoSans-Bold.ttf".into()),
            mono: None,
        }
    }

    /// Developer's own fonts from their assets/ folder.
    pub fn from_project(regular: impl Into<String>, bold: impl Into<String>) -> Self {
        Self {
            regular: Some(regular.into()),
            bold: Some(bold.into()),
            mono: None,
        }
    }
}

impl Default for ThemeFonts {
    fn default() -> Self {
        Self::bevy_default()
    }
}
