/// # Shaders Source Code as str.
/// Used to add the WGSL on the WGPU process.
pub mod shader {
    pub const BATCH_SHADER: &str = include_str!("../../assets/shaders/batch_shader.wgsl");
    pub const COLOR_SHADER: &str = include_str!("../../assets/shaders/color_shader.wgsl");
    pub const TEXTURE_SHADER: &str = include_str!("../../assets/shaders/texture_shader.wgsl");
    pub const BACKGROUND_SHADER: &str = include_str!("../../assets/shaders/background_shader.wgsl");
}
