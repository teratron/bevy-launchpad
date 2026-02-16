use bevy::state::prelude::States;
use bevy::state::state::FreelyMutableState;

/// Trait to map user-defined states to framework-expected functional states.
pub trait LaunchpadStates: States + FreelyMutableState {
    /// Initial booting state.
    fn booting() -> Self;
    /// Loading assets/data state.
    fn loading() -> Self;
    /// Splash screen sequence state.
    fn splash() -> Self;
    /// Main menu state.
    fn menu() -> Self;
    /// Active gameplay state.
    fn playing() -> Self;
    /// Paused gameplay state.
    fn paused() -> Self;
}
