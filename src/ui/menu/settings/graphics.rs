use crate::ui::common::UiChildSpawner;
use crate::ui::theme::ThemeConfig;
use crate::ui::widgets::checkbox::spawn_checkbox;
use crate::ui::widgets::dropdown::spawn_dropdown;
use bevy::prelude::*;

pub fn setup_graphics_settings(parent: &mut UiChildSpawner, theme: &ThemeConfig) {
    parent.spawn((
        Text::new("Graphics Settings"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(theme.colors.text),
    ));

    // Resolution
    parent.spawn((
        Text::new("Resolution"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(theme.colors.text),
    ));
    spawn_dropdown(
        parent,
        vec![
            "1920x1080".to_string(),
            "2560x1440".to_string(),
            "3840x2160".to_string(),
        ],
        0,
        theme,
    );

    // Fullscreen
    spawn_checkbox(parent, "Fullscreen", true, theme);

    // VSync
    spawn_checkbox(parent, "VSync", true, theme);
}
