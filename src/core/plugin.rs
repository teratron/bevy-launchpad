use crate::core::boot::sequence::update_boot_progress;
use crate::core::states::transitions::{TransitionStateEvent, handle_state_transitions};
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use std::marker::PhantomData;

/// Core framework plugin.
pub struct LaunchpadCorePlugin<S: States + FreelyMutableState> {
    _state: PhantomData<S>,
}

impl<S: States + FreelyMutableState> Default for LaunchpadCorePlugin<S> {
    fn default() -> Self {
        Self {
            _state: PhantomData,
        }
    }
}

impl<S: States + FreelyMutableState> Plugin for LaunchpadCorePlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::core::boot::BootSequence>();
        app.init_resource::<crate::core::states::TransitionConfig>();
        app.init_resource::<crate::core::splash::SplashConfig>();

        bevy::ecs::message::MessageRegistry::register_message::<TransitionStateEvent<S>>(
            app.world_mut(),
        );

        app.add_systems(Update, update_boot_progress);
        app.add_systems(Update, handle_state_transitions::<S>);
    }
}
