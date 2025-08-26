use std::{collections::HashMap, sync::Arc};
use glyph_brush::Rectangle;
use image::DynamicImage;
use wgpu::{Device, Queue};
use super::{texture::Texture, super::asset_loader::AssetLoader, super::super::utils::constants::cache::DUMMY_TEXTURE};

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

    /// Add a texture to the cache and returns it afterwards or just search for it and returns.
    ///
    /// It's a 'generic' function for both images and texts.
    ///
    /// Texts have a singular necessity for texture recreation, so use the [`recreate`] argument for it.
    pub fn load_texture(
        &mut self,
        key: String,
        device: &Device,
        queue: &Queue,
        recreate: bool,
        text_size: Option<Rectangle<u32>>,
        text_data: Option<&[u8]>
    ) -> Option<Arc<Texture>> {
        if recreate && text_size.is_some() && text_data.is_some() {
            let recreated_texture: Texture = Texture::from_text(
                device,
                queue,
                Some(&key),
                text_size.unwrap(),
                text_data.unwrap()
            ).unwrap();
            let recreated_texture_arc: Arc<Texture> = Arc::new(recreated_texture);

            self.textures.insert(key.clone(), recreated_texture_arc.clone());
            return Some(recreated_texture_arc);
        }

        if !self.textures.contains_key(&key.clone()) {
            let texture: Texture;

            if key != DUMMY_TEXTURE.to_string() {
                if text_size.is_some() && text_data.is_some() {
                    texture = Texture::from_text(device, queue, Some(&key.clone()), text_size.unwrap(), text_data.unwrap()).unwrap();
                } else {
                    let image: DynamicImage = image::load_from_memory(&AssetLoader::load_bytes(&key.clone()).ok().unwrap()).unwrap();
                    texture = Texture::from_image(device, queue, &image, Some(&key.clone())).unwrap();
                }
            } else {
                texture = Texture::dummy(device, queue, Some(&key.clone())).unwrap();
            }
            let texture_arc: Arc<Texture> = Arc::new(texture);
            self.textures.insert(key.clone(), Arc::clone(&texture_arc));
            return Some(texture_arc);
        }
        return Some(self.get_texture(key.clone()).expect("Texture should be on cache."));
    }
}
