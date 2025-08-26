
use std::{collections::HashMap, sync::Arc};
use image::DynamicImage;
use wgpu::{Device, Queue};
use super::{texture::Texture, super::asset_loader::AssetLoader};
use crate::utils::constants::cache::DUMMY_TEXTURE;

/// Struct to represent the textures current on the application cache.
pub struct TextureCache {
    textures: HashMap<String, Arc<Texture>>
}

impl TextureCache {
    /// Create a new texture cache cleaned.
    pub fn new() -> Self {
        return Self {
            textures: HashMap::new()
        };
    }

    /// Returns a texture from the cache based on the file path.
    pub fn get_texture(&self, key: String) -> Option<Arc<Texture>> {
        return self.textures.get(&key).cloned();
    }

    /// Add a texture to the cache and returns it afterwards.
    pub fn load_texture(&mut self, key: String, device: &Device, queue: &Queue) -> Option<Arc<Texture>> {
        if !self.textures.contains_key(&key) {
            let texture: Texture;

            if key != DUMMY_TEXTURE.to_string() {
                let image: DynamicImage = image::load_from_memory(&AssetLoader::load_bytes(&key).ok().unwrap()).unwrap();
                texture = Texture::from_image(device, queue, &image, Some(&key)).unwrap();
            } else {
                texture = Texture::dummy(device, queue, Some(&key)).unwrap();
            }
            let texture_arc: Arc<Texture> = Arc::new(texture);
            self.textures.insert(key.clone(), Arc::clone(&texture_arc));
            return Some(texture_arc);
        }
        return Some(self.get_texture(key).expect("Texture should be on cache."));
    }
}
