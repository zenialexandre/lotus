use std::{env, fs, io::Result, path::PathBuf};
use once_cell::sync::Lazy;

pub(crate) static ASSETS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let manifest_dir: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env::current_dir().unwrap().to_string_lossy().into_owned()));
    let assets_dir: PathBuf = manifest_dir.join("assets");

    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir).expect("Failed to create the assets directory.");
    }
    return assets_dir;
});

/// Struct to represent our asset loader.
/// The main purpose of this object is to relate the relative paths to the CARGO_MANIFEST_DIR.
#[derive(Clone)]
pub struct AssetLoader;

impl AssetLoader {
    pub(crate) fn get_path(relative_path: &str) -> PathBuf {
        return ASSETS_DIR.join(relative_path);
    }

    pub(crate) fn load_bytes(relative_path: &str) -> Result<Vec<u8>> {
        let path: PathBuf = Self::get_path(relative_path);
        return fs::read(path);
    }
}
