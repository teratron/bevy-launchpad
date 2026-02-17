//! Splash screen logic module.

pub mod sequence;
pub mod timer;

pub use sequence::{SplashConfig, SplashScreenConfig, SplashSource, SkipTrigger};
pub use timer::{SplashState, SplashTimer};
