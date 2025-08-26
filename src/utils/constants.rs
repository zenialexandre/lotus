/// Constants related to adding the WGSL source code on the WGPU process.
pub mod shader {
    pub const SHADER_2D: &str = include_str!("../../assets/shaders/shader_2d.wgsl");
}

/// Constants related to the cache rendering process.
pub mod cache {
    pub const FIXED_UUID: &str = "fixed_uuid";
    pub const VERTEX: &str = "vertex";
    pub const INDEX: &str = "index";
    pub const PROJECTION: &str = "projection";
    pub const VIEW: &str = "view";
    pub const TRANSFORM_BUFFER: &str = "transform_buffer";
    pub const RENDERING_TYPE_BUFFER: &str = "rendering_type_buffer";
    pub const TEXTURE_BIND_GROUP: &str = "texture_bind_group";
    pub const TRANSFORM_BIND_GROUP: &str = "transform_bind_group";
    pub const RENDERING_TYPE_BIND_GROUP: &str = "rendering_type_bind_group";
    pub const DUMMY_TEXTURE: &str = "dummy_texture";
}

/// Constants related to native engine fonts.
pub mod font {
    pub const UNDERDOG_REGULAR_PATH: &str = "../assets/fonts/Underdog-Regular.ttf";
    pub const CODYSTAR_LIGHT_PATH: &str = "../assets/fonts/Codystar-Light.ttf";
    pub const CODYSTAR_REGULAR_PATH: &str = "../assets/fonts/Codystar-Regular.ttf";
    pub const ROBOTO_MONO_PATH: &str = "../assets/fonts/RobotoMono-VariableFont_wght.ttf";
    pub const ROBOTO_MONO_ITALIC_PATH: &str = "../assets/fonts/RobotoMono-Italic-VariableFont_wght.ttf";
}
