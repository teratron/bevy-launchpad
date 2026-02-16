use bevy::prelude::*;

/// Resource to ensure only a single instance of a component or system is active.
#[derive(Resource, Default)]
pub struct SingleInstance;

pub fn mod_placeholder() {}
