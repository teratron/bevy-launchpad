//! Splash screen logic module.

pub mod sequence;
pub mod timer;

pub use sequence::{SplashConfig, SplashScreenConfig};
pub use timer::{SplashState, SplashTimer};
