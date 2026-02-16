use bevy::prelude::*;

/// Manifest of assets to be loaded.
#[derive(Resource, Debug, Clone, Default)]
pub struct AssetManifest {
    pub assets: Vec<String>,
}
