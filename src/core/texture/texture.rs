use std::{collections::HashMap, sync::Arc};
use wgpu::{
    TextureView,
    TextureDescriptor,
    TextureUsages,
    TextureFormat,
    TextureDimension,
    TexelCopyTextureInfo,
    TexelCopyBufferLayout,
    TextureAspect,
    Origin3d,
    Sampler,
    SamplerDescriptor,
    TextureViewDescriptor,
    AddressMode,
    FilterMode,
    Device,
    Queue,
    Extent3d
};
use image::{
    GenericImageView,
    DynamicImage
};
use anyhow::*;
use super::super::asset_loader::AssetLoader;

/// Struct to represent a texture to be used on the rendering process.
pub struct Texture {
    #[allow(unused)]
    pub label: String,
    pub texture: wgpu::Texture,
    pub texture_view: TextureView,
    pub sampler: Sampler
}

impl Texture {
    /// Returns a texture struct from the bytes of a real image.
    pub fn from_bytes(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: &str
    ) -> Result<Self> {
        let image: DynamicImage = image::load_from_memory(bytes)?;
        return Self::from_image(device, queue, &image, Some(label));
    }

    /// Returns a texture struct from a real image.
    pub fn from_image(
        device: &Device,
        queue: &Queue,
        image: &DynamicImage,
        label: Option<&str>
    ) -> Result<Self> {
        let rgba = image.to_rgba8();
        let dimensions: (u32, u32) = image.dimensions();
        let size: Extent3d = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };
        let texture: wgpu::Texture = device.create_texture(&TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[]
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                aspect: TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO
            },
            &rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1)
            },
            size
        );

        let texture_view: TextureView = texture.create_view(&TextureViewDescriptor::default());
        let sampler: Sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        return Ok(Self { label: label.unwrap().to_string(), texture, texture_view, sampler });
    }
}

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
            let image: DynamicImage = image::load_from_memory(&AssetLoader::load_bytes(&key).ok().unwrap()).unwrap();
            let texture: Texture = Texture::from_image(device, queue, &image, Some(&key)).unwrap();
            let texture_arc: Arc<Texture> = Arc::new(texture);
            self.textures.insert(key.clone(), Arc::clone(&texture_arc));
            return Some(texture_arc);
        }
        return Some(self.get_texture(key).expect("Texture should be on cache."));
    }
}
