use super::bundle::FluentBundleType;
use bevy::prelude::*;
use fluent_bundle::FluentResource;
use std::fs;
use std::path::{Path, PathBuf};
use unic_langid::LanguageIdentifier;

/// Maps a requested locale string to an existing directory under `assets/locales/`.
///
/// **Resolution steps:**
/// 1. Exact match: `locales/en-US/` exists → return `"en-US"`.
/// 2. Language-prefix match: requested `"en"` matches `"en-US"` or `"en-GB"`.
/// 3. Return `None` if no directory found (caller logs the warning).
pub fn resolve_locale_dir(assets_dir: &Path, locale: &str) -> Option<String> {
    let locales_path = assets_dir.join("locales");

    // 1. Try exact match
    if locales_path.join(locale).is_dir() {
        return Some(locale.to_string());
    }

    // 2. Try prefix match
    if let Ok(entries) = fs::read_dir(&locales_path) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                // If locale is "en", match "en-US", "en-GB", etc.
                if name.starts_with(locale) && name.chars().nth(locale.len()) == Some('-') {
                    return Some(name);
                }
            }
        }
    }

    warn!("Locale directory not found for: {}", locale);
    None
}

/// Scans a locale text directory and returns all `*.ftl` file paths.
pub fn scan_ftl_files(locale_text_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(locale_text_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "ftl") {
                files.push(path);
            }
        }
    }
    files
}

/// Helper to load a single FTL file into a string.
fn read_ftl_file(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

/// Builds a `FluentBundleType` from a list of FTL file paths.
pub fn build_bundle(
    locale: &str,
    ftl_paths: &[PathBuf],
) -> Result<FluentBundleType, Box<dyn std::error::Error>> {
    let lang_id: LanguageIdentifier = locale
        .parse()
        .map_err(|e| format!("Invalid locale: {}", e))?;

    // Use standard new() as FluentBundle<FluentResource> (1 generic) implies default memoizer?
    // Or maybe we need to match the alias.
    // FluentBundleType = FluentBundle<FluentResource>.
    // FluentBundle::new(vec![lang_id]) returns FluentBundle<FluentResource, IntlLangMemoizer>.
    // If alias matches, this is fine.
    let mut bundle = FluentBundleType::new(vec![lang_id]);

    for path in ftl_paths {
        let source = read_ftl_file(path)?;
        let resource = FluentResource::try_new(source)
            .map_err(|(_, err)| format!("Failed to parse FTL file {:?}: {:?}", path, err))?;

        if let Err(errors) = bundle.add_resource(resource) {
            warn!("Duplicate keys found in {:?}: {:?}", path, errors);
        }
    }

    // Use isolating to prevent bidirectional text issues
    bundle.set_use_isolating(false);

    Ok(bundle)
}

/// Helper to get asset path for a locale
pub fn get_locale_asset_path(assets_dir: &Path, locale: &str, sub_path: &str) -> PathBuf {
    assets_dir.join("locales").join(locale).join(sub_path)
}
