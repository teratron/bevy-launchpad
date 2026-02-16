use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Loads a struct from a RON file.
pub fn load_ron<T>(path: impl AsRef<Path>) -> Result<T, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path)?;
    let data = ron::from_str(&content)?;
    Ok(data)
}

/// Saves a struct to a RON file.
pub fn save_ron<T>(path: impl AsRef<Path>, data: &T) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let content = ron::to_string(data)?;
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

/// Helper trait for types that can be persisted as RON.
pub trait Persistable: Serialize + for<'de> Deserialize<'de> + Sized {
    fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        load_ron(path)
    }

    fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        save_ron(path, self)
    }
}
