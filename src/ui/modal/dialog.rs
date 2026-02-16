use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::hierarchy::ChildOf;
use bevy::prelude::*;

/// Type alias for child spawner commands used in UI hierarchy.
pub type ChildSpawnerCommands<'w> = RelatedSpawnerCommands<'w, ChildOf>;

#[derive(Component)]
pub struct ModalRoot;

pub fn spawn_dialog(parent: &mut ChildSpawnerCommands, _title: &str, _message: &str) {
    parent.spawn((
        Node {
            width: Val::Px(400.0),
            height: Val::Px(300.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ModalRoot,
    ));
}
