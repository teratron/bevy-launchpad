use bevy::prelude::*;
use bevy_launchpad::prelude::*;  // only core types exported without "ui" feature

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)  // no window, no renderer
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_metadata(AppMetadata {
                    name:  "game_server".into(),
                    title: "Game Server".into(),
                    ..default()
                })
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .allow_multiple_instances(true)
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), server_start)
        .add_systems(Update, server_tick.run_if(in_state(AppState::Playing)))
        .run();
}

fn server_start() {
    info!("Server started — listening for connections");
}

fn server_tick(time: Res<Time>) {
    // Runs every frame — no renderer overhead
    if (time.elapsed_secs() as u32) % 5 == 0 {
        // info!("Server tick: {:.0}s", time.elapsed_secs());
    }
}
