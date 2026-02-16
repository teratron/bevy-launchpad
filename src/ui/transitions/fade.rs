use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FadeTransition {
    pub duration: f32,
}
