use crate::ui::common::UiChildSpawner;
use crate::ui::theme::ThemeConfig;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Checkbox {
    pub checked: bool,
}

pub fn spawn_checkbox(
    parent: &mut UiChildSpawner,
    label: impl Into<String>,
    checked: bool,
    theme: &ThemeConfig,
) -> Entity {
    let label_text = label.into();

    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            Checkbox { checked },
        ))
        .with_children(|p| {
            // Checkbox box
            p.spawn((
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(theme.colors.primary),
                BackgroundColor(if checked {
                    theme.colors.secondary
                } else {
                    Color::NONE
                }),
            ));

            // Label
            p.spawn((
                Text::new(label_text),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(theme.colors.text),
            ));
        })
        .id()
}
