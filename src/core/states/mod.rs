//! State management module.

pub mod machine;
pub mod mapping;
pub mod transitions;

pub use machine::StateMachine;
pub use mapping::LaunchpadStates;
pub use transitions::{TransitionConfig, TransitionStateEvent, handle_state_transitions};
