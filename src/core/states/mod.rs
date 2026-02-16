//! State management module.

pub mod machine;
pub mod transitions;

pub use machine::StateMachine;
pub use transitions::{TransitionConfig, TransitionStateEvent, handle_state_transitions};
