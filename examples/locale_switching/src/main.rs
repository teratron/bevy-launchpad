use bevy::ecs::system::NonSendMut;
use bevy::prelude::*;
use bevy_launchpad::locale::bundle::FluentBundleType;
use bevy_launchpad::locale::plugin::load_bundle;
use bevy_launchpad::locale::resource::Localization;
use bevy_launchpad::prelude::*;
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                // Noto Sans required for Cyrillic characters
                .with_fonts(ThemeFonts::noto_sans())
                // Use implicit default or explicit "en-US"
                .with_locale("en-US")
                .with_splash(SplashConfig {
                    skip_all: true,
                    ..default()
                })
                .build(),
        )
        // Add a state to run our setup in
        .add_systems(OnEnter(AppState::Menu), setup_ui)
        .add_systems(Update, switch_locale)
        .run();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Instructions
    commands.spawn((
        Text::new("Press 1 for English, 2 for Russian"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    // Localized Hello World
    commands.spawn((
        Text::new("Loading..."),
        TextFont {
            font: asset_server.load("fonts/NotoSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        },
        LocalizedText::new("hello-world"),
    ));
}

fn switch_locale(keys: Res<ButtonInput<KeyCode>>, loc: Option<NonSendMut<Localization>>) {
    let Some(mut loc) = loc else { return };

    let mut new_lang = None;
    if keys.just_pressed(KeyCode::Digit1) {
        new_lang = Some(Language::EnUs);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        new_lang = Some(Language::RuRu);
    }

    if let Some(lang) = new_lang
        && loc.language != lang
    {
        info!("Switching to {}", lang);
        let assets_dir = PathBuf::from("assets");
        // Load new bundle
        // We use unwrap_or_else to ensure we have a valid bundle type even if loading fails
        let bundle = load_bundle(&assets_dir, lang).unwrap_or_else(|| {
            warn!("Failed to load bundle for {}", lang);
            FluentBundleType::new(vec![lang.as_locale_str().parse().unwrap()])
        });
        loc.update_language(lang, bundle);
    }
}
