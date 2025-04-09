use cgmath::Vector2;
use lotus_proc_macros::Component;
use wgpu_text::{glyph_brush::ab_glyph::FontArc, BrushBuilder, TextBrush};
use winit::dpi::PhysicalSize;
use super::{color::Color, managers::rendering_manager::RenderState};

/// Struct to represent a text to be rendered.
#[derive(Clone, Component)]
pub struct Text {
    pub font: Font,
    pub position: Vector2<f32>,
    pub color: Color,
    pub content: String
}

impl Text {
    /// Create a new text struct.
    pub fn new(font: Font, position: Vector2<f32>, color: Color, content: String) -> Self {
        return Self {
            font,
            position,
            color,
            content
        };
    }

    /// Returns the text position as pixels.
    pub fn get_position_as_pixels(&self, physical_size: &PhysicalSize<u32>) -> (f32, f32) {
        let x: f32 = self.position.x * physical_size.width as f32;
        let y: f32 = self.position.y * physical_size.height as f32;
        return (x, y);
    }
}

/// Struct to represent a font.
#[derive(Clone)]
pub struct Font {
    pub bytes: Vec<u8>,
    pub size: f32
}

impl Font {
    /// Create a new font struct.
    pub fn new(bytes: Vec<u8>, size: f32) -> Self {
        return Self {
            bytes,
            size
        };
    }
}

/// Enumerator that represents the available default fonts on the engine.
/// The end-user can use it's own fonts at any moment.
#[derive(Clone)]
pub enum Fonts {
    UnderdogRegular,
    CodystarLight,
    CodystarRegular,
    RobotoMono,
    RobotoMonoItalic
}

impl Fonts {
    /// Returns the bytes of the following font.
    pub fn get_bytes(&self) -> Vec<u8> {
        return match self {
            Self::UnderdogRegular => include_bytes!("../assets/fonts/Underdog-Regular.ttf").to_vec(),
            Self::CodystarLight => include_bytes!("../assets/fonts/Codystar-Light.ttf").to_vec(),
            Self::CodystarRegular => include_bytes!("../assets/fonts/Codystar-Regular.ttf").to_vec(),
            Self::RobotoMono => include_bytes!("../assets/fonts/RobotoMono-VariableFont_wght.ttf").to_vec(),
            Self::RobotoMonoItalic => include_bytes!("../assets/fonts/RobotoMono-Italic-VariableFont_wght.ttf").to_vec()
        }
    }
}

/// Struct to store the texts to be rendered.
pub struct TextRenderer {
    pub text_brush: TextBrush<FontArc>,
    pub text: Text
}

impl TextRenderer {
    /// Create a new text renderer struct.
    pub fn new(render_state: &RenderState, text: &Text) -> Self {
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
}
