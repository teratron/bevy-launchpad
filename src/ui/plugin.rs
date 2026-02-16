use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LaunchpadUiPlugin;

impl Plugin for LaunchpadUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::ui::theme::ThemeConfig>();
        app.init_resource::<crate::ui::menu::main_menu::MainMenuConfig>();
        app.init_resource::<crate::ui::menu::settings::SettingsConfig>();

        app.add_systems(Update, crate::ui::menu::main_menu::handle_menu_interactions);
        app.add_systems(Update, crate::ui::splash::renderer::update_splash_renderer);
    }
}
