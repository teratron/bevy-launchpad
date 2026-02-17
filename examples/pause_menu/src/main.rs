use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_splash(SplashConfig {
                    skip_all: true,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), spawn_ball)
        .add_systems(
            Update,
            (move_ball.run_if(in_state(AppState::Playing)), toggle_pause),
        )
        .add_systems(OnEnter(AppState::Paused), on_pause)
        .add_systems(OnExit(AppState::Paused), on_resume)
        .run();
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.0),
            custom_size: Some(Vec2::splat(30.0)),
            ..default()
        },
        Ball {
            velocity: Vec2::new(200.0, 150.0),
        },
    ));
    info!("Ball spawned. Press Escape to pause.");
}

fn move_ball(mut q: Query<(&mut Transform, &mut Ball)>, windows: Query<&Window>, time: Res<Time>) {
    for window in &windows {
        let (hw, hh) = (window.width() * 0.5, window.height() * 0.5);

        for (mut t, mut ball) in &mut q {
            t.translation += ball.velocity.extend(0.0) * time.delta_secs();
            if t.translation.x.abs() > hw {
                ball.velocity.x *= -1.0;
            }
            if t.translation.y.abs() > hh {
                ball.velocity.y *= -1.0;
            }
        }
    }
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut nxt: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            AppState::Playing => nxt.set(AppState::Paused),
            AppState::Paused => nxt.set(AppState::Playing),
            _ => {}
        }
    }
}

fn on_pause() {
    info!("Game paused  — library shows pause menu");
}
fn on_resume() {
    info!("Game resumed — pause menu dismissed");
}
