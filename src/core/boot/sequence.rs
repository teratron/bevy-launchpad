use bevy::prelude::*;

/// Represents the current progress of the boot sequence.
#[derive(Resource, Debug, Default)]
pub struct BootSequence {
    /// Percentage completed (0.0 to 1.0)
    pub progress: f32,
    /// Whether booting is finished
    pub is_finished: bool,
}

/// System to simulate boot progress.
pub fn update_boot_progress(mut boot: ResMut<BootSequence>, time: Res<Time>) {
    if !boot.is_finished {
        boot.progress += time.delta_secs() * 0.5; // Simulate loading
        if boot.progress >= 1.0 {
            boot.progress = 1.0;
            boot.is_finished = true;
            info!("Boot sequence finished!");
        }
    }
}
