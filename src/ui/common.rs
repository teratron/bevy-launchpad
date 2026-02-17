use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::relationship::RelatedSpawnerCommands;

/// Standard child-spawner type used across all UI sub-modules.
pub type UiChildSpawner<'w> = RelatedSpawnerCommands<'w, ChildOf>;

pub const BUTTON_W: f32 = 200.0;
pub const BUTTON_H: f32 = 50.0;
pub const MENU_GAP: f32 = 20.0;
pub const PANEL_PADDING: f32 = 24.0;
