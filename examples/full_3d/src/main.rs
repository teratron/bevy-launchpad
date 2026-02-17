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
use bevy_launchpad::core::splash::sequence::{SplashConfig, SplashScreenConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name: "full_3d_game".into(),
                    title: "Full 3D Game".into(),
                    ..default()
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::default_branding(),
                        SplashScreenConfig::studio("branding/studio_logo.png"),
                    ],
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(Update, rotate_cube.run_if(in_state(GameState::Playing)))
        .run();
}

#[derive(Component)]
struct RotatingCube;

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RotatingCube,
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    info!("Full 3D Game Level Started");
}

fn rotate_cube(mut query: Query<&mut Transform, With<RotatingCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}
