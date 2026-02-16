use bevy::prelude::*;

/// Generic state machine resource.
#[derive(Resource, Debug, Clone, Default)]
pub struct StateMachine<S: States> {
    pub current: S,
    pub history: Vec<S>,
}
