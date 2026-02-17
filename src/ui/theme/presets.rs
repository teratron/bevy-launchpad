use super::colors::ThemeColors;
use super::fonts::ThemeFonts;
use super::spacing::ThemeSpacing;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
pub struct ThemeConfig {
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub spacing: ThemeSpacing,
}

impl ThemeConfig {
    pub fn dark() -> Self {
        Self::default()
    }

    pub fn light() -> Self {
        Self {
            colors: ThemeColors {
                primary: Color::srgb(0.1, 0.3, 0.7),
                secondary: Color::srgb(0.3, 0.5, 0.8),
                background: Color::WHITE,
                text: Color::BLACK,
            },
            ..default()
        }
    }

    /// Dark theme with Noto Sans (use when Cyrillic / full Unicode needed).
    pub fn dark_with_noto() -> Self {
        Self {
            fonts: ThemeFonts::noto_sans(),
            ..Self::dark()
        }
    }
}
