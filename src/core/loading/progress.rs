use bevy::prelude::*;

/// States for the loading process.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingState {
    #[default]
    Loading,
    Ready,
}
