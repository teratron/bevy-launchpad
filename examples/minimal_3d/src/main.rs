use bevy::prelude::*;
use bevy_launchpad::prelude::*;

use bevy_launchpad::core::boot::metadata::AppMetadata;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_metadata(AppMetadata {
                    name: "minimal_3d".into(),       // data dir: ~/.local/share/minimal_3d/
                    title: "Minimal 3D Demo".into(), // window title
                    version: "0.1.0".into(),
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    info!("Minimal 3D started");
}
