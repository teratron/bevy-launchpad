use crate::core::boot::sequence::update_boot_progress;
use crate::core::states::mapping::LaunchpadStates;
use crate::core::states::transitions::{
    TransitionStateEvent, auto_transition_booting, auto_transition_loading,
    handle_state_transitions,
};
use bevy::prelude::*;
use std::marker::PhantomData;

/// Core framework plugin.
pub struct LaunchpadCorePlugin<S: LaunchpadStates> {
    _state: PhantomData<S>,
}

impl<S: LaunchpadStates> Default for LaunchpadCorePlugin<S> {
    fn default() -> Self {
        Self {
            _state: PhantomData,
        }
    }
}

impl<S: LaunchpadStates> Plugin for LaunchpadCorePlugin<S> {
    fn build(&self, app: &mut App) {
        // Resources like AppMetadata and AppPaths are already inserted by LaunchpadPlugin::build

        app.init_resource::<crate::core::boot::BootSequence>();
        app.init_resource::<crate::core::states::TransitionConfig>();
        app.init_resource::<crate::core::splash::SplashConfig>();

        bevy::ecs::message::MessageRegistry::register_message::<TransitionStateEvent<S>>(
            app.world_mut(),
        );

        app.add_systems(Update, update_boot_progress);
        app.add_systems(Update, handle_state_transitions::<S>);

        // Automatic flow
        app.add_systems(
            Update,
            (
                auto_transition_booting::<S>.run_if(in_state(S::booting())),
                auto_transition_loading::<S>.run_if(in_state(S::loading())),
            ),
        );
    }
}
