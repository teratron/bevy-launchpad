use crate::ui::theme::ThemeConfig;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::Button;
use bevy::prelude::*;

/// Type alias for child spawner commands used in UI hierarchy.
pub type ChildSpawnerCommands<'w> = RelatedSpawnerCommands<'w, ChildOf>;

/// Spawns a standard button with text.
pub fn spawn_button<'a, 'w>(
    parent: &'a mut ChildSpawnerCommands<'w>,
    text: impl Into<String>,
    theme: &ThemeConfig,
) -> EntityCommands<'a> {
    let text_content: String = text.into();

    let mut cmd = parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.colors.primary),
    ));

    cmd.with_children(|p: &mut ChildSpawnerCommands| {
        p.spawn((Text::new(text_content), TextColor(theme.colors.text)));
    });

    cmd
}
