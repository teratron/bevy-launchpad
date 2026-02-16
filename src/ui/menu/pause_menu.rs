use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenuRoot;

pub fn setup_pause_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        PauseMenuRoot,
    ));
}
