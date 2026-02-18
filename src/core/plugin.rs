use crate::core::assets::AssetsRootStrategy;
use crate::core::boot::metadata::AppMetadata;
use crate::core::boot::sequence::update_boot_progress;
use crate::core::loading::tracker::AssetTracker;
use crate::core::splash::sequence::SplashConfig;
use crate::core::states::mapping::LaunchpadStates;
use crate::core::states::transitions::{
    TransitionConfig, TransitionStateEvent, auto_transition_booting, handle_state_transitions,
};
use bevy::prelude::*;
use bevy::state::prelude::in_state;
use std::marker::PhantomData;

/// Core logic plugin for Launchpad.
/// Handles boot sequence, state management, and asset loading logic.
pub struct LaunchpadCorePlugin<S: LaunchpadStates> {
    _marker: PhantomData<S>,
}

impl<S: LaunchpadStates> Default for LaunchpadCorePlugin<S> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<S: LaunchpadStates> Plugin for LaunchpadCorePlugin<S> {
    fn build(&self, app: &mut App) {
        // Init state (BUG-05)
        app.init_state::<S>();

        // Init resources if not already present (builder usually inserts them)
        if !app.world().contains_resource::<AppMetadata>() {
            app.init_resource::<AppMetadata>();
        }
        if !app.world().contains_resource::<SplashConfig>() {
            app.init_resource::<SplashConfig>();
        }
        if !app.world().contains_resource::<TransitionConfig>() {
            app.init_resource::<TransitionConfig>();
        }
        if !app.world().contains_resource::<AssetsRootStrategy>() {
            app.init_resource::<AssetsRootStrategy>();
        }

        app.init_resource::<crate::core::boot::BootSequence>();
        app.init_resource::<crate::core::boot::BootConfig>();
        app.init_resource::<crate::core::assets::resolver::AssetsRootStrategy>();

        // Messages
        app.add_message::<TransitionStateEvent<S>>();

        // Systems
        app.add_systems(
            Update,
            (
                update_boot_progress.run_if(in_state(S::booting())),
                handle_state_transitions::<S>,
                auto_transition_booting::<S>.run_if(in_state(S::booting())),
                crate::core::states::transitions::auto_transition_loading::<S>.run_if(in_state(S::loading())),
                splash_to_menu::<S>.run_if(in_state(S::splash())),
            )
                .chain(),
        );
    }
}

/// Splash → Menu immediately if no screens are configured to show.
/// (Normal transition works via TransitionStateEvent from renderer)
pub fn splash_to_menu<S: LaunchpadStates>(
    config: Res<SplashConfig>,
    mut next: ResMut<NextState<S>>,
) {
    if config.effective_screens().is_empty() {
        next.set(S::menu());
    }
}


