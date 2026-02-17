use crate::ui::common::{BUTTON_H, BUTTON_W, UiChildSpawner};
use crate::ui::theme::ThemeConfig;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::Button;
use bevy::prelude::*;

/// Spawns a standard button with text.
pub fn spawn_button<'a, 'w>(
    parent: &'a mut UiChildSpawner<'w>,
    text: impl Into<String>,
    theme: &ThemeConfig,
) -> EntityCommands<'a> {
    let text_content: String = text.into();

    let mut cmd = parent.spawn((
        Button,
        Node {
            width: Val::Px(BUTTON_W),
            height: Val::Px(BUTTON_H),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.colors.primary),
    ));

    cmd.with_children(|p| {
        p.spawn((Text::new(text_content), TextColor(theme.colors.text)));
    });

    cmd
}
