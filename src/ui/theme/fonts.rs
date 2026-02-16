use bevy::prelude::*;

/// Resource for defining theme fonts.
#[derive(Resource, Debug, Clone)]
pub struct ThemeFonts {
    pub main: String,
    pub bold: String,
    pub mono: String,
}

impl Default for ThemeFonts {
    fn default() -> Self {
        Self {
            main: "fonts/Inter-Regular.ttf".to_string(),
            bold: "fonts/Inter-Bold.ttf".to_string(),
            mono: "fonts/JetBrainsMono-Regular.ttf".to_string(),
        }
    }
}
