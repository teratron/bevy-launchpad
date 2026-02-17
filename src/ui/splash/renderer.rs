use crate::core::splash::{SplashConfig, SplashState, SplashTimer};
use bevy::prelude::*;

/// Component representing a splash screen element.
#[derive(Component)]
pub struct SplashScreen;

/// System that spawns the splash screen.
pub fn setup_splash_renderer(mut commands: Commands, config: Res<SplashConfig>) {
    let screens = config.effective_screens();
    if screens.is_empty() {
        return;
    }

    commands.init_resource::<SplashTimer>();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        SplashScreen,
    ));
}

/// System to update splash screen rendering and transitions.
pub fn update_splash_renderer(
    mut commands: Commands,
    time: Res<Time>,
    timer: Option<ResMut<SplashTimer>>,
    config: Res<SplashConfig>,
    mut splash_query: Query<(Entity, &mut BackgroundColor, Option<&Children>), With<SplashScreen>>,
) {
    let Some(mut timer) = timer else { return };
    let screens = config.effective_screens();
    if timer.screen_index >= screens.len() {
        // All screens finished — cleanup is handled by state exit in core
        return;
    }

    let screen_cfg = &screens[timer.screen_index];
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        match timer.state {
            SplashState::FadeIn => {
                timer.state = SplashState::Visible;
                let duration = screen_cfg.max_duration.unwrap_or(2.0)
                    - screen_cfg.fade_in
                    - screen_cfg.fade_out;
                timer
                    .timer
                    .set_duration(std::time::Duration::from_secs_f32(duration.max(0.1)));
                timer.timer.reset();
            }
            SplashState::Visible => {
                timer.state = SplashState::FadeOut;
                timer
                    .timer
                    .set_duration(std::time::Duration::from_secs_f32(screen_cfg.fade_out));
                timer.timer.reset();
            }
            SplashState::FadeOut => {
                timer.screen_index += 1;
                if timer.screen_index < screens.len() {
                    timer.state = SplashState::FadeIn;
                    let next_screen = &screens[timer.screen_index];
                    timer
                        .timer
                        .set_duration(std::time::Duration::from_secs_f32(next_screen.fade_in));
                    timer.timer.reset();
                } else {
                    commands.insert_resource(crate::core::splash::sequence::SplashDone);
                }
            }
        }
    }

    // Update visuals (fade effects)
    if let Some((_entity, mut bg_color, _children)) = splash_query.iter_mut().next() {
        let alpha = match timer.state {
            SplashState::FadeIn => timer.timer.fraction(),
            SplashState::Visible => 1.0,
            SplashState::FadeOut => 1.0 - timer.timer.fraction(),
        };

        *bg_color = BackgroundColor(Color::from(
            Hsla::from(screen_cfg.background).with_alpha(alpha),
        ));

        // Image rendering logic would go here, ensuring the image is spawned/updated
        // when the screen index changes.
    }
}
