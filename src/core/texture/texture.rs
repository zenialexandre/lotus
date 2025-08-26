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

/// Struct to represent a texture to be used on the rendering process.
pub struct Texture {
    pub wgpu_texture: wgpu::Texture,
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
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        return Ok(Self {
            wgpu_texture: texture,
            texture_view,
            sampler
        });
    }

    pub(crate) fn dummy(
        device: &Device,
        queue: &Queue,
        label: Option<&str>
    ) -> Result<Self> {
        let color: [u8; 4] = [1.0 as u8, 1.0 as u8, 1.0 as u8, 1.0 as u8];
        let size: Extent3d = Extent3d {
            width: 1,
            height: 1,
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
            view_formats: &[],
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                aspect: TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
            },
            &color,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            size,
        );

        let texture_view: TextureView = texture.create_view(&TextureViewDescriptor::default());
        let sampler: Sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        return Ok(Self {
            wgpu_texture: texture,
            texture_view,
            sampler,
        });
    }
}
