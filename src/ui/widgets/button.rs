use crate::ui::common::{BUTTON_H, BUTTON_W, UiChildSpawner};
use crate::ui::theme::ThemeConfig;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::Button;
use bevy::prelude::*;

/// Spawns a standard button with text.
/// If `locale` feature is enabled, the text is treated as a localization key.
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

    let text_key = text_content.clone();
    cmd.with_children(|p| {
        let mut text_cmd = p.spawn((Text::new(text_content), TextColor(theme.colors.text)));

        #[cfg(feature = "locale")]
        text_cmd.insert(crate::locale::LocalizedText::new(text_key));
    });

    cmd
}
