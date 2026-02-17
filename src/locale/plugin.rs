use crate::locale::language::Language;
use bevy::prelude::*;

/// Plugin for handling localization.
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<Language>() {
            app.init_resource::<Language>();
        }
    }
}
