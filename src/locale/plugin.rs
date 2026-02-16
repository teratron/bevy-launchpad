use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::locale::language::Language>();
    }
}
