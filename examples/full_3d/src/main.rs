use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(Component)]
struct RotatingCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_metadata(AppMetadata {
                    name: "galaxy_quest".into(),
                    title: "Galaxy Quest 3D".into(),
                    version: "0.1.0".into(),
                    ..default()
                })
                // AAA four-screen splash
                .with_splash(SplashConfig {
                    screens: vec![
                        // 1. Legal / age-rating — 5 s, cannot skip
                        SplashScreenConfig::legal("branding/legal.png").on_background(Color::WHITE),
                        // 2. Publisher
                        SplashScreenConfig::studio("branding/publisher.png")
                            .with_duration(2.5, 4.0),
                        // 3. Developer
                        SplashScreenConfig::studio("branding/developer.png")
                            .with_duration(2.0, 3.5)
                            .with_fade(0.5, 0.5),
                        // 4. Engine — skippable, dark background
                        SplashScreenConfig::engine("branding/engine.png")
                            .with_duration(1.5, 3.0)
                            .with_fade(0.4, 0.4)
                            .on_background(Color::srgb(0.02, 0.02, 0.05)),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                // Custom game font (sci-fi feel)
                .with_fonts(ThemeFonts::from_project(
                    "fonts/Orbitron-Regular.ttf",
                    "fonts/Orbitron-Bold.ttf",
                ))
                // Custom sci-fi color scheme
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary: Color::srgb(0.0, 0.78, 1.0),
                        secondary: Color::srgb(0.0, 0.45, 0.78),
                        background: Color::srgb(0.02, 0.02, 0.06),
                        text: Color::srgb(0.9, 0.95, 1.0),
                    },
                    fonts: ThemeFonts::from_project(
                        "fonts/Orbitron-Regular.ttf",
                        "fonts/Orbitron-Bold.ttf",
                    ),
                    spacing: ThemeSpacing::default(),
                })
                .with_locale("en-US")
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup_3d)
        .add_systems(
            Update,
            (rotate_cube, check_pause).run_if(in_state(AppState::Playing)),
        )
        .run();
}

fn setup_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Rotating cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.78, 1.0),
            emissive: LinearRgba::new(0.0, 0.3, 0.5, 1.0),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RotatingCube,
    ));
    // Lights
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 800.0,
            color: Color::srgb(0.0, 0.5, 1.0),
            ..default()
        },
        Transform::from_xyz(-4.0, 2.0, -4.0),
    ));
    info!("Galaxy Quest level loaded");
}

fn rotate_cube(mut q: Query<&mut Transform, With<RotatingCube>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs() * 0.8);
        t.rotate_x(time.delta_secs() * 0.3);
    }
}

fn check_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut nxt: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if *state.get() == AppState::Playing {
            nxt.set(AppState::Paused);
        }
    }
}
