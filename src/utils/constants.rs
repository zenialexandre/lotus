/// # Shaders Source Code as str.
/// Used to add the WGSL on the WGPU process.
pub mod shader {
    pub const SHADER_2D: &str = include_str!("../../assets/shaders/shader_2d.wgsl");
    pub const COLOR_SHADER: &str = include_str!("../../assets/shaders/color_shader.wgsl");
    pub const TEXTURE_SHADER: &str = include_str!("../../assets/shaders/texture_shader.wgsl");
    pub const BACKGROUND_SHADER: &str = include_str!("../../assets/shaders/background_shader.wgsl");
}
