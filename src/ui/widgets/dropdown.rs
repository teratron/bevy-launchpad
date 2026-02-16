use crate::ui::common::UiChildSpawner;
use crate::ui::theme::ThemeConfig;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Dropdown {
    pub options: Vec<String>,
    pub selected_index: usize,
}

pub fn spawn_dropdown(
    parent: &mut UiChildSpawner,
    options: Vec<String>,
    selected_index: usize,
    theme: &ThemeConfig,
) -> Entity {
    let selected_text = options[selected_index].clone();

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
            BorderColor::all(theme.colors.primary),
            BackgroundColor(theme.colors.primary),
            Dropdown {
                options,
                selected_index,
            },
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(selected_text),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.colors.text),
            ));

            // Caret icon placeholder
            p.spawn((
                Text::new("▼"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.colors.text),
            ));
        })
        .id()
}
