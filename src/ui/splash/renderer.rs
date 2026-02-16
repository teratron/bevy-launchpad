use crate::core::splash::{SplashConfig, SplashTimer};
use bevy::prelude::*;

/// Component representing a splash screen element.
#[derive(Component)]
pub struct SplashScreen;

/// System to setup the splash screen rendering.
pub fn setup_splash_renderer(
    mut commands: Commands,
    config: Res<SplashConfig>,
    asset_server: Res<AssetServer>,
) {
    if let Some(first_screen) = config.screens.first() {
        commands.insert_resource(SplashTimer(Timer::from_seconds(
            first_screen.duration,
            TimerMode::Once,
        )));

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                SplashScreen,
            ))
            .with_children(|parent| {
                if !first_screen.logo.is_empty() {
                    parent.spawn(ImageNode {
                        image: asset_server.load(&first_screen.logo),
                        ..default()
                    });
                }
            });
    }
}

/// System to update splash screen rendering.
pub fn update_splash_renderer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    splash_query: Query<Entity, With<SplashScreen>>,
) {
    if timer.tick(time.delta()).just_finished() {
        for entity in &splash_query {
            commands.entity(entity).despawn();
        }
    }
}
