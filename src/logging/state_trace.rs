use bevy::prelude::*;
use std::fmt::Debug;

use super::subsystem::Subsystem;
use crate::ltag; // Use our macro

/// Generic system that logs AppState transitions.
/// Registered for every state type S that implements `States` and `Debug`.
pub fn log_state_transition<S: States + Debug + Clone + PartialEq + Send + Sync + 'static>(
    state: Res<State<S>>,
    mut prev: Local<Option<S>>,
) {
    let current = state.get().clone();

    // Initialize prev on first run
    if prev.is_none() {
        *prev = Some(current.clone());
        return;
    }

    if prev.as_ref() != Some(&current) {
        let prev_name = prev
            .as_ref()
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "None".to_string());

        ltag!(info, Subsystem::State, "{} -> {:?}", prev_name, current);

        *prev = Some(current);
    }
}
