use super::colors::ThemeColors;
use bevy::prelude::*;

/// Predefined theme presets.
#[derive(Resource, Debug, Clone, Default)]
pub struct ThemeConfig {
    pub colors: ThemeColors,
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
        }
    }
}
