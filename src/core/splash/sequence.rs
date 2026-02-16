use bevy::prelude::*;

/// Configuration for a splash screen sequence.
#[derive(Resource, Default, Debug, Clone)]
pub struct SplashConfig {
    pub screens: Vec<SplashScreenConfig>,
}

#[derive(Debug, Clone)]
pub struct SplashScreenConfig {
    pub logo: String,
    pub duration: f32,
    pub skippable: bool,
}
