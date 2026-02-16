use crate::ui::common::UiChildSpawner;
use crate::ui::theme::ThemeConfig;
use crate::ui::widgets::slider::spawn_slider;
use bevy::prelude::*;

pub fn setup_audio_settings(parent: &mut UiChildSpawner, theme: &ThemeConfig) {
    parent.spawn((
        Text::new("Audio Settings"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(theme.colors.text),
    ));

    // Master Volume
    parent.spawn((
        Text::new("Master Volume"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(theme.colors.text),
    ));
    spawn_slider(parent, 0.0, 1.0, 0.8, theme);

    // Music Volume
    parent.spawn((
        Text::new("Music Volume"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(theme.colors.text),
    ));
    spawn_slider(parent, 0.0, 1.0, 0.5, theme);

    // SFX Volume
    parent.spawn((
        Text::new("SFX Volume"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(theme.colors.text),
    ));
    spawn_slider(parent, 0.0, 1.0, 0.7, theme);
}
