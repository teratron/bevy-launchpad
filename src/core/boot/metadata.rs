use bevy::prelude::*;

/// Centralized application metadata resource.
#[derive(Resource, Debug, Clone)]
pub struct AppMetadata {
    /// The internal name of the application (used for file system paths).
    pub name: String,
    /// The public-facing title of the game (used for window title).
    pub title: String,
    /// The current version of the application.
    pub version: String,
    /// A short description of the application.
    pub description: String,
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self {
            name: "bevy_game".to_string(),
            title: "Bevy Game".to_string(),
            version: "0.1.0".to_string(),
            description: "A game built with Bevy Launchpad".to_string(),
        }
    }
}
