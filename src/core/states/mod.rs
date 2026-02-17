//! State management module.

pub mod app_state;
pub mod machine;
pub mod mapping;
pub mod transitions;

pub use app_state::AppState;
pub use mapping::LaunchpadStates;
pub use machine::StateMachine;
pub use transitions::{TransitionConfig, TransitionStateEvent, handle_state_transitions};
