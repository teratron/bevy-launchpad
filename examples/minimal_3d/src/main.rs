use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting,
    Loading,
    Splash,
    Menu,
    Playing,
    Paused,
}

impl LaunchpadStates for GameState {
    fn booting() -> Self {
        GameState::Booting
    }
    fn loading() -> Self {
        GameState::Loading
    }
    fn splash() -> Self {
        GameState::Splash
    }
    fn menu() -> Self {
        GameState::Menu
    }
    fn playing() -> Self {
        GameState::Playing
    }
    fn paused() -> Self {
        GameState::Paused
    }
}

use bevy_launchpad::core::boot::metadata::AppMetadata;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(register_embedded_assets)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name: "minimal_3d_example".to_string(),
                    title: "Minimal 3D Example".to_string(),
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn 3D camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn a cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    info!("Minimal 3D Game Started!");
}
