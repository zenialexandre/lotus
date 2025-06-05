use cgmath::Vector2;
use lotus_proc_macros::Component;
use wgpu::Queue;
use wgpu_text::{glyph_brush::ab_glyph::FontArc, BrushBuilder, TextBrush};
use winit::dpi::PhysicalSize;
use super::{physics::transform::{Position, Strategy}, color::Color, asset_loader::AssetLoader, managers::rendering::manager::RenderState};

const UNDERDOG_REGULAR_PATH: &str = "../../assets/fonts/Underdog-Regular.ttf";
const CODYSTAR_LIGHT_PATH: &str = "../../assets/fonts/Codystar-Light.ttf";
const CODYSTAR_REGULAR_PATH: &str = "../../assets/fonts/Codystar-Regular.ttf";
const ROBOTO_MONO_PATH: &str = "../../assets/fonts/RobotoMono-VariableFont_wght.ttf";
const ROBOTO_MONO_ITALIC: &str = "../../assets/fonts/RobotoMono-Italic-VariableFont_wght.ttf";

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

    /// Returns the text position by the strategy.
    pub fn get_position_by_strategy(&self, physical_size: &PhysicalSize<u32>) -> (f32, f32) {
        // TODO: Positioning became out of hand. Necessity to stop using wgpu_text and use my own wrapper.

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

/// Struct to represent a font.
#[derive(Clone)]
pub struct Font {
    pub bytes: Vec<u8>,
    pub size: f32
}

impl Font {
    /// Create a new font struct.
    pub fn new(path: String, size: f32) -> Self {
        let bytes: Vec<u8> = if path.contains("../../") {
            let font: Fonts = Fonts::from_path(path).unwrap();
            font.get_bytes()
        } else {
            AssetLoader::load_bytes(&path).ok().unwrap()
        };

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
    /// Returns the path of the following font.
    pub fn get_path(&self) -> String {
        return match self {
            Self::UnderdogRegular => UNDERDOG_REGULAR_PATH.to_string(),
            Self::CodystarLight => CODYSTAR_LIGHT_PATH.to_string(),
            Self::CodystarRegular => CODYSTAR_REGULAR_PATH.to_string(),
            Self::RobotoMono => ROBOTO_MONO_PATH.to_string(),
            Self::RobotoMonoItalic => ROBOTO_MONO_ITALIC.to_string()
        }
    }

    /// Returns the bytes of the following font.
    pub fn get_bytes(&self) -> Vec<u8> {
        return match self {
            Self::UnderdogRegular => include_bytes!("../../assets/fonts/Underdog-Regular.ttf").to_vec(),
            Self::CodystarLight => include_bytes!("../../assets/fonts/Codystar-Light.ttf").to_vec(),
            Self::CodystarRegular => include_bytes!("../../assets/fonts/Codystar-Regular.ttf").to_vec(),
            Self::RobotoMono => include_bytes!("../../assets/fonts/RobotoMono-VariableFont_wght.ttf").to_vec(),
            Self::RobotoMonoItalic => include_bytes!("../../assets/fonts/RobotoMono-Italic-VariableFont_wght.ttf").to_vec()
        }
    }

    /// Returns the enumerator value from the path.
    pub fn from_path(path: String) -> Option<Self> {
        let path_as_str: &str = &path;

        return match path_as_str {
            UNDERDOG_REGULAR_PATH => Some(Self::UnderdogRegular),
            CODYSTAR_LIGHT_PATH => Some(Self::CodystarLight),
            CODYSTAR_REGULAR_PATH => Some(Self::CodystarRegular),
            ROBOTO_MONO_PATH => Some(Self::RobotoMono),
            ROBOTO_MONO_ITALIC => Some(Self::RobotoMonoItalic),
            _ => None
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
    }

    #[deprecated(
        since = "0.1.31",
        note = "Use the `update_content` function instead."
    )]
    /// Updates the text rendering context with the new content.
    pub fn update_brush(&mut self, content: String, queue: Option<Queue>, physical_size: Option<PhysicalSize<u32>>) {
        self.text.content = content;

        self.text_brush.update_matrix(
            wgpu_text::ortho(physical_size.as_ref().unwrap().width as f32, physical_size.as_ref().unwrap().height as f32),
            queue.as_ref().unwrap()
        );
    }
}
