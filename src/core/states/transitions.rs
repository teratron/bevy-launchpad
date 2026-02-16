use bevy::ecs::message::MessageReader;
use bevy::log::info;
use bevy::prelude::{App, Plugin, ResMut, Resource, Update};
use bevy::state::prelude::{NextState, States};
use bevy::state::state::FreelyMutableState;

/// Configuration for state transitions.
#[derive(Resource, Debug, Clone, Default)]
pub struct TransitionConfig {
    /// Fade duration in seconds
    pub fade_duration: f32,
}

/// Event sent to trigger a state transition.
#[derive(bevy::ecs::message::Message, Debug, Clone, PartialEq)]
pub struct TransitionStateEvent<S: States> {
    pub next: S,
}

pub struct StateTransitionPlugin<S: States + FreelyMutableState> {
    _phantom: std::marker::PhantomData<S>,
}

impl<S: States + FreelyMutableState> Default for StateTransitionPlugin<S> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S: States + FreelyMutableState> Plugin for StateTransitionPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_state_transitions::<S>);
    }
}

/// System that listens for `TransitionStateEvent` and updates the state.
pub fn handle_state_transitions<S: States + FreelyMutableState>(
    mut message_reader: MessageReader<TransitionStateEvent<S>>,
    mut next_state: ResMut<NextState<S>>,
) {
    for event in message_reader.read() {
        info!("Transitioning to state: {:?}", event.next);
        next_state.set(event.next.clone());
    }
}
