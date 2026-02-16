//! Settings menu module.

pub mod audio;
pub mod controls;
pub mod general;
pub mod graphics;
pub mod panel;

use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
pub struct SettingsConfig {
    pub audio_volume: f32,
    pub fullscreen: bool,
}
