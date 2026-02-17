use crate::core::states::mapping::LaunchpadStates;
use bevy::prelude::*;
// use bevy::state::state::FreelyMutableState;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Booting,
    Loading,
    Splash,
    Menu,
    Playing,
    Paused,
}

impl LaunchpadStates for AppState {
    fn booting() -> Self {
        Self::Booting
    }
    fn loading() -> Self {
        Self::Loading
    }
    fn splash() -> Self {
        Self::Splash
    }
    fn menu() -> Self {
        Self::Menu
    }
    fn playing() -> Self {
        Self::Playing
    }
    fn paused() -> Self {
        Self::Paused
    }
}
