use crate::ui::widgets::button::{Button, spawn_button};
use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct MainMenuConfig {
    pub title: String,
    pub buttons: Vec<MenuButton>,
}

#[derive(Debug, Clone)]
pub enum MenuButton {
    Play,
    Settings,
    Exit,
    Custom {
        label: String,
        target_state_name: String,
    },
}

impl Default for MainMenuConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            buttons: vec![MenuButton::Play, MenuButton::Settings, MenuButton::Exit],
        }
    }
}

use crate::ui::theme::ThemeConfig;

#[derive(Component)]
pub struct MainMenuRoot;

/// Marker for the Play button.
#[derive(Component)]
pub struct PlayButton;

/// Marker for the Exit button.
#[derive(Component)]
pub struct ExitButton;

pub fn setup_main_menu(
    mut commands: Commands,
    config: Res<MainMenuConfig>,
    theme: Res<ThemeConfig>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(theme.colors.background),
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Title
            if !config.title.is_empty() {
                parent.spawn((
                    Text::new(&config.title),
                    TextFont {
                        font_size: 60.0,
                        ..default()
                    },
                    TextColor(theme.colors.text),
                ));
            }

            // Buttons
            for button in &config.buttons {
                match button {
                    MenuButton::Play => {
                        spawn_button(parent, "Play", &theme).insert(PlayButton);
                    }
                    MenuButton::Settings => {
                        spawn_button(parent, "Settings", &theme);
                    }
                    MenuButton::Exit => {
                        spawn_button(parent, "Exit", &theme).insert(ExitButton);
                    }
                    MenuButton::Custom { label, .. } => {
                        spawn_button(parent, label, &theme);
                    }
                }
            }
        });
}

pub type InteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
>;

pub fn handle_menu_interactions(mut interaction_query: InteractionQuery, theme: Res<ThemeConfig>) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(theme.colors.secondary);
            }
            Interaction::Hovered => {
                // Slightly lighter than primary
                *color = BackgroundColor(theme.colors.primary.mix(&Color::WHITE, 0.1));
            }
            Interaction::None => {
                *color = BackgroundColor(theme.colors.primary);
            }
        }
    }
}

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
