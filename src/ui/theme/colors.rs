use bevy::prelude::*;

/// Resource for defining theme colors.
#[derive(Resource, Debug, Clone)]
pub struct ThemeColors {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub text: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: Color::srgb(0.2, 0.4, 0.8),
            secondary: Color::srgb(0.4, 0.6, 0.9),
            background: Color::srgb(0.05, 0.05, 0.05),
            text: Color::WHITE,
        }
    }
}
