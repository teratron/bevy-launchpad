use bevy::prelude::*;

/// Resource for tracking asset loading progress.
#[derive(Resource, Debug, Default)]
pub struct AssetTracker {
    pub total_assets: usize,
    pub loaded_assets: usize,
}

impl AssetTracker {
    pub fn progress(&self) -> f32 {
        if self.total_assets == 0 {
            1.0
        } else {
            self.loaded_assets as f32 / self.total_assets as f32
        }
    }
}
