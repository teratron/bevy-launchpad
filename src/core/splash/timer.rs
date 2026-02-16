use bevy::prelude::*;

/// Timer for splash screen duration.
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct SplashTimer(pub Timer);
