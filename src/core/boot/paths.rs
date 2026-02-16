use bevy::prelude::*;

/// Resource for managing platform-specific paths.
#[derive(Resource, Debug, Clone, Default)]
pub struct BootPaths {
    pub data_dir: String,
    pub config_dir: String,
}
