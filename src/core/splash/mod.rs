//! Splash screen logic module.

pub mod sequence;
pub mod timer;

pub use sequence::{SkipTrigger, SplashConfig, SplashScreenConfig, SplashSource};
pub use timer::{SplashState, SplashTimer};
