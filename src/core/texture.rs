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

pub(crate) struct Texture {
    #[allow(unused)]
    pub(crate) texture: wgpu::Texture,
    pub(crate) texture_view: TextureView,
    pub(crate) sampler: Sampler
}

impl Texture {
    pub(crate) fn _from_bytes(
        device: &Device,
        queue: &Queue,
        bytes: &[u8],
        label: &str
    ) -> Result<Self> {
        let image: DynamicImage = image::load_from_memory(bytes)?;
        return Self::from_image(device, queue, &image, Some(label));
    }

    pub(crate) fn from_image(
        device: &Device,
        queue: &Queue,
        image: &DynamicImage,
        label: Option<&str>
    ) -> Result<Self> {
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
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

        return Ok(Self { texture, texture_view, sampler });
    }
}
