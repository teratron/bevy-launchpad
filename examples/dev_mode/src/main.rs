use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    // Run any of these in terminal to test different modes:
    //
    //   cargo run --example dev_mode
    //     → normal flow: boot → splash → menu
    //
    //   cargo run --example dev_mode -- --skip-splash
    //     → skip splash, go straight to menu
    //
    //   cargo run --example dev_mode -- --skip-splash --state Playing
    //     → skip splash AND menu, start in Playing immediately
    //
    //   BEVY_LAUNCHPAD_SKIP_SPLASH=1 cargo run --example dev_mode
    //     → env-var override (no code change needed)
    //
    //   BEVY_LAUNCHPAD_THEME=light cargo run --example dev_mode
    //     → force light theme

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_metadata(AppMetadata {
                    name:  "dev_mode_demo".into(),
                    title: "Dev Mode Demo".into(),
                    ..default()
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::studio("branding/studio.png"),
                    ],
                    show_default_branding: true,
                    ..default()
                    // Note: CliArgs.skip_splash = true → SplashConfig.skip_all
                    //       is set automatically by LaunchpadPlugin
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Dev mode: reached Playing state");
}
