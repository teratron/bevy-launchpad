use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct SplashTimer {
    pub timer: Timer,
    pub screen_index: usize,
    pub state: SplashState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplashState {
    FadeIn,
    Visible,
    FadeOut,
}

impl Default for SplashTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            screen_index: 0,
            state: SplashState::FadeIn,
        }
    }
}
