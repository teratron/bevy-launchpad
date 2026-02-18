use super::{
    bundle::FluentBundleType,
    cache::LocalizedStrings,
    component::LocalizedText,
    language::Language,
    resource::Localization,
    utils::{build_bundle, scan_ftl_files},
};
use bevy::ecs::system::NonSendMut;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        // Setup Localization
        let assets_dir = PathBuf::from("assets");
        let initial_lang = Language::default(); // EnUs

        // Load Main Bundle
        let main_bundle = load_bundle(&assets_dir, initial_lang).unwrap_or_else(|| {
            warn!("Failed to load main bundle for {}", initial_lang);
            FluentBundleType::new(vec![initial_lang.as_locale_str().parse().unwrap()])
        });

        // Load Fallback Bundle (EnUs)
        let fallback_bundle = if initial_lang != Language::EnUs {
            load_bundle(&assets_dir, Language::EnUs).unwrap_or_else(|| {
                FluentBundleType::new(vec![Language::EnUs.as_locale_str().parse().unwrap()])
            })
        } else {
            // If main language is EnUs, fallback can be empty/minimal
            FluentBundleType::new(vec![Language::EnUs.as_locale_str().parse().unwrap()])
        };

        // Insert NonSend Resource
        app.insert_non_send_resource(Localization::new(
            initial_lang,
            main_bundle,
            fallback_bundle,
        ));
        info!("Localization initialized with language: {}", initial_lang);

        app.insert_non_send_resource(LocalizedStrings::default());

        app.add_systems(Update, (update_localized_texts, init_localized_texts));
    }
}

pub fn load_bundle(assets_dir: &Path, lang: Language) -> Option<FluentBundleType> {
    let locale_str = lang.as_locale_str();
    let locale_dir = assets_dir.join("locales").join(locale_str).join("text");

    if !locale_dir.exists() {
        warn!("Locale directory not found: {:?}", locale_dir);
        return None;
    }

    let ftl_files = scan_ftl_files(&locale_dir);
    if ftl_files.is_empty() {
        warn!("No .ftl files found in {:?}", locale_dir);
        return None;
    }

    match build_bundle(locale_str, &ftl_files) {
        Ok(bundle) => Some(bundle),
        Err(e) => {
            error!("Failed to build bundle for {}: {}", locale_str, e);
            None
        }
    }
}

pub fn update_localized_texts(
    loc: Option<NonSend<Localization>>,
    mut query: Query<(&mut Text, &LocalizedText)>,
    cache: Option<NonSendMut<LocalizedStrings>>,
    mut last_lang: Local<Option<Language>>,
) {
    if let (Some(loc), Some(mut cache)) = (loc, cache)
        && *last_lang != Some(loc.language)
    {
        // Language changed or first run
        *last_lang = Some(loc.language);

        // Invalidate cache
        cache.invalidate();

        // Update all texts
        for (mut text, localized) in &mut query {
            text.0 = cache.get(&localized.key, &loc);
        }
    }
}

pub fn init_localized_texts(
    loc: Option<NonSend<Localization>>,
    mut query: Query<(&mut Text, &LocalizedText), Added<LocalizedText>>,
    cache: Option<NonSendMut<LocalizedStrings>>,
) {
    if let (Some(loc), Some(mut cache)) = (loc, cache) {
        for (mut text, localized) in &mut query {
            text.0 = cache.get(&localized.key, &loc);
        }
    }
}
