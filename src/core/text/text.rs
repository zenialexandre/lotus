use std::collections::HashMap;
use cgmath::Vector2;
use lotus_proc_macros::{Component, Resource};
use uuid::Uuid;
use wgpu::Queue;
use wgpu_text::{glyph_brush::ab_glyph::FontArc, BrushBuilder, TextBrush};
use winit::dpi::PhysicalSize;
use super::{
    font::Font,
    super::{
        event_dispatcher::{EventDispatcher, Event, EventType, SubEventType},
        ecs::{entity::Entity, world::World, resource::ResourceRefMut},
        physics::transform::{Position, Strategy},
        color::Color,
        managers::rendering::manager::RenderState
    }
};

// Struct to represent the resource that holds the text rendering context.
#[derive(Resource)]
pub(crate) struct TextHolder {
    pub(crate) text_renderers: HashMap<Uuid, TextRenderer>
}

impl Default for TextHolder {
    fn default() -> Self {
        return Self {
            text_renderers: HashMap::new()
        };
    }
}

/// Struct to represent a text to be rendered.
#[derive(Clone, Component)]
pub struct Text {
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
    pub(crate) fn get_position_by_strategy(&self, physical_size: &PhysicalSize<u32>) -> (f32, f32) {
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

    /// Creates a new event to update the text font.
    pub fn font(&self, world: &World, entity: Entity, font: Font) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
        event_dispatcher.send(Event::new(entity, EventType::Text(SubEventType::UpdateTextFont), font));
    }

    /// Creates a new event to update the text position struct.
    pub fn position(&self, world: &World, entity: Entity, position: Position) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
        event_dispatcher.send(Event::new(entity, EventType::Text(SubEventType::UpdateTextPosition), position))
    }

    /// Creates a new event to update the text content.
    pub fn content(&self, world: &World, entity: Entity, content: String) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
        event_dispatcher.send(Event::new(entity, EventType::Text(SubEventType::UpdateTextContent), content))
    }

    /// Creates a new event to update the text color.
    pub fn color(&self, world: &World, entity: Entity, color: Color) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();
        event_dispatcher.send(Event::new(entity, EventType::Text(SubEventType::UpdateTextColor), color))
    }
}

/// Struct to store the texts to be rendered.
pub(crate) struct TextRenderer {
    pub(crate) text_brush: TextBrush<FontArc>,
    pub(crate) text: Text
}

impl TextRenderer {
    /// Create a new text renderer struct.
    pub(crate) fn new(render_state: &RenderState, text: &Text) -> Self {
        let font: FontArc = FontArc::try_from_vec(text.font.bytes.clone()).expect("Failed to load font.");
        let text_brush: TextBrush<FontArc> = BrushBuilder::using_font(font).build(
            &render_state.device.as_ref().unwrap(),
            render_state.physical_size.unwrap().width,
            render_state.physical_size.unwrap().height,
            render_state.surface_configuration.as_ref().unwrap().format
        );

        return Self {
            text_brush,
            text: text.clone()
        };
    }

    /// Updates the text rendering context with the new font.
    pub(crate) fn font(&mut self, font: Font, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.font = font;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with the new position.
    pub(crate) fn position(&mut self, position: Position, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.position = position;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with the new content.
    pub(crate) fn content(&mut self, content: String, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.content = content;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }

    /// Updates the text rendering context with the new color.
    pub(crate) fn color(&mut self, color: Color, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.color = color;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }
}
