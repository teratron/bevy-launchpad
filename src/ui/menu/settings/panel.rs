use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// Type alias for child spawner commands used in UI hierarchy.
pub type ChildSpawnerCommands<'w> = RelatedSpawnerCommands<'w, ChildOf>;

#[derive(Component)]
pub struct SettingsPanel;

pub fn setup_settings_panel(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(80.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        SettingsPanel,
    ));
}
