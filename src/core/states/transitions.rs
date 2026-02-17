use crate::core::boot::sequence::BootSequence;
use crate::core::states::mapping::LaunchpadStates;
use bevy::ecs::message::{Message, MessageReader};
use bevy::prelude::*;
use bevy::state::prelude::{NextState, States};

/// Configuration for state transitions.
#[derive(Resource, Debug, Clone, Default)]
pub struct TransitionConfig {
    /// Fade duration in seconds
    pub fade_duration: f32,
}

/// Event sent to trigger a state transition.
#[derive(Message, Debug, Clone, PartialEq)]
pub struct TransitionStateEvent<S: States> {
    pub next: S,
}

pub struct StateTransitionPlugin<S: LaunchpadStates> {
    _phantom: std::marker::PhantomData<S>,
}

impl<S: LaunchpadStates> Default for StateTransitionPlugin<S> {
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<S: LaunchpadStates> Plugin for StateTransitionPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_message::<TransitionStateEvent<S>>();
        app.add_systems(Update, handle_state_transitions::<S>);
    }
}

/// System that listens for `TransitionStateEvent` and updates the state.
pub fn handle_state_transitions<S: LaunchpadStates>(
    mut message_reader: MessageReader<TransitionStateEvent<S>>,
    mut next_state: ResMut<NextState<S>>,
) {
    for event in message_reader.read() {
        info!("Transitioning to state: {:?}", event.next);
        next_state.set(event.next.clone());
    }
}

/// Automatically transitions from Booting to Loading when boot sequence completes.
pub fn auto_transition_booting<S: LaunchpadStates>(
    boot: Res<BootSequence>,
    mut next_state: ResMut<NextState<S>>,
) {
    if boot.is_finished {
        next_state.set(S::loading());
    }
}

/// Automatically transitions from Loading to Splash.
/// NOTE: In a real app, this would wait for assets to load.
pub fn auto_transition_loading<S: LaunchpadStates>(mut next_state: ResMut<NextState<S>>) {
    // Placeholder logic: immediately move to splash
    next_state.set(S::splash());
}
