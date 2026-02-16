use bevy::prelude::*;

/// Configuration for the boot sequence.
#[derive(Resource, Debug, Clone, Default)]
pub struct BootConfig {
    /// List of paths to initialize
    pub paths: Vec<String>,
    /// Whether to check for a single instance
    pub single_instance: bool,
}
