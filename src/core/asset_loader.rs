use std::{fs, io::Result, path::PathBuf, sync::OnceLock};
use crate::utils::constants::asset::CARGO_MANIFEST_DIR;

pub(crate) static ASSETS_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Struct to represent our asset loader.
/// The main purpose of this object is to relate the relative paths to the CARGO_MANIFEST_DIR.
#[derive(Clone)]
pub struct AssetLoader;

impl AssetLoader {
    pub(crate) fn new() -> Result<()> {
        let manifest_dir: PathBuf = PathBuf::from(CARGO_MANIFEST_DIR);
        let assets_dir: PathBuf = manifest_dir.join("assets");

        if assets_dir.exists() {
            ASSETS_DIR.set(assets_dir).unwrap();
        }
        return Ok(());
    }

    pub(crate) fn get_path(relative_path: &str) -> PathBuf {
        let base_dir: &PathBuf = ASSETS_DIR.get().expect("Asset Loader not initialized.");
        return base_dir.join(relative_path);
    }

    pub(crate) fn load_bytes(relative_path: &str) -> Result<Vec<u8>> {
        let path: PathBuf = Self::get_path(relative_path);
        return fs::read(path);
    }
}
