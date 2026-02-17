use crate::ui::common::UiChildSpawner;
use crate::ui::theme::ThemeConfig;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
}

pub fn spawn_slider(
    parent: &mut UiChildSpawner,
    min: f32,
    max: f32,
    value: f32,
    theme: &ThemeConfig,
) -> Entity {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(24.0),
                align_items: AlignItems::Center,
                ..default()
            },
            Slider { min, max, value },
        ))
        .with_children(|p| {
            // Track
            p.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(4.0),
                    ..default()
                },
                BackgroundColor(theme.colors.primary.mix(&Color::BLACK, 0.5)),
            ))
            .with_children(|track| {
                // Fill
                track.spawn((
                    Node {
                        width: Val::Percent(value * 100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(theme.colors.secondary),
                ));
            });

            // Knob
            p.spawn((
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(value * 100.0 - 4.0), // Approximate knob center
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(theme.colors.secondary),
                BorderColor::all(theme.colors.text),
            ));
        })
        .id()
}
