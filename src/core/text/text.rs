use std::cell::RefCell;
use cgmath::Vector2;
use lotus_proc_macros::Component;
use glyph_brush::{ab_glyph::FontArc, GlyphBrush, GlyphBrushBuilder, Rectangle};
use uuid::Uuid;
use wgpu::{Extent3d, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect};
use winit::dpi::PhysicalSize;
use super::{
    font::Font,
    super::{
        physics::transform::{Position, Strategy},
        color::Color,
        texture::texture::Texture,
        managers::rendering::manager::{RenderState, Vertex}
    }
};

/// Struct to represent a text to be rendered.
#[derive(Clone, Component)]
pub struct Text {
    pub(crate) uuid: Uuid,
    pub font: Font,
    pub position: Position,
    pub color: Color,
    pub content: String,
    pub(crate) original_resolution: Vector2<f32>
}

impl Text {
    /// Create a new text struct.
    pub fn new(render_state: &mut RenderState, font: Font, position: Position, color: Color, content: String) -> Self {
        return Self {
            uuid: Uuid::new_v4(),
            font,
            position,
            color,
            content,
            original_resolution: Vector2::new(
                render_state.physical_size.as_ref().unwrap().width as f32,
                render_state.physical_size.as_ref().unwrap().height as f32
            )
        };
    }

    /// Returns the text position by its positioning strategy.
    ///
    /// We need to send the data as pixelated values to our rasterizer (glyph_brush).
    pub fn get_position_by_strategy(&self, physical_size: &PhysicalSize<u32>) -> (f32, f32) {
        let width: f32 = physical_size.width as f32;
        let height: f32 = physical_size.height as f32;
        let aspect_ratio: f32 = width / height;

        if self.position.strategy == Strategy::Normalized {
            let x_ratio: f32 = self.position.x / aspect_ratio;
            let x: f32 = ((x_ratio + 1.0) / 2.0) * width;
            let y: f32 = ((1.0 - self.position.y) / 2.0) * height;
            return (x, y);
        } else {
            let scaled_pixelated_x: f32 = width / self.original_resolution.x;
            let scaled_pixelated_y: f32 = height / self.original_resolution.y;
            return (self.position.x * scaled_pixelated_x, self.position.y * scaled_pixelated_y)
        }
    }
}

/// Struct to store the texts to be rendered.
pub struct TextRenderer {
    pub glyph_brush: RefCell<GlyphBrush<[Vertex; 4]>>,
    pub vertices: RefCell<Vec<Vertex>>,
    pub indices: RefCell<Vec<u16>>,
    pub text: Text
}

impl TextRenderer {
    /// Create a new text renderer struct.
    pub fn new(render_state: &RenderState, text: &Text) -> Self {
        let font: FontArc = FontArc::try_from_vec(text.font.bytes.clone()).expect("Failed to load font.");
        let glyph_brush: GlyphBrush<[Vertex; 4]> = GlyphBrushBuilder::using_font(font).build();

        return Self {
            glyph_brush: RefCell::new(glyph_brush),
            vertices: RefCell::new(Vec::new()),
            indices: RefCell::new(Vec::new()),
            text: text.clone()
        };
    }

    pub(crate) fn update_texture(&self, render_state: &RenderState, texture: &Texture, size: Rectangle<u32>, data: &[u8]) {
        render_state.queue.as_ref().unwrap().write_texture(
            TexelCopyTextureInfo {
                texture: &texture.wgpu_texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: size.min[0],
                    y: size.min[1],
                    z: 0,
                },
                aspect: TextureAspect::All,
            },
            data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.width()),
                rows_per_image: Some(size.height()),
            },
            Extent3d {
                width: size.width(),
                height: size.height(),
                depth_or_array_layers: 1,
            },
        );
    }

    /*
    /// Updates the text rendering context with the new font.
    pub fn update_font(&mut self, font: Font, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.font = font;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with the new position.
    pub fn update_position(&mut self, position: Position, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.position = position;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with the new content.
    pub fn update_content(&mut self, content: String, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.content = content;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with the new color.
    pub fn update_color(&mut self, color: Color, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.color = color;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with a new text struct.
    pub fn update_text_data(&mut self, text: &Text, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text = text.clone();

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }*/
}
