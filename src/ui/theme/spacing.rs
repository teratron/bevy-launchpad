use bevy::prelude::*;

/// Resource for defining theme spacing/dimensions.
#[derive(Resource, Debug, Clone)]
pub struct ThemeSpacing {
    pub padding: f32,
    pub margin: f32,
    pub corner_radius: f32,
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self {
            padding: 16.0,
            margin: 8.0,
            corner_radius: 4.0,
        }
    }
}
