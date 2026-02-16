use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::relationship::RelatedSpawnerCommands;

/// Type alias for child spawner commands used in UI hierarchy.
pub type UiChildSpawner<'w> = RelatedSpawnerCommands<'w, ChildOf>;

/// Standard UI spacing and sizes
pub mod constants {
    pub const BUTTON_WIDTH: f32 = 200.0;
    pub const BUTTON_HEIGHT: f32 = 50.0;
    pub const MENU_SPACING: f32 = 20.0;
}
