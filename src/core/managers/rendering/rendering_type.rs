#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) enum RenderingType {
    #[default]
    SHAPE,
    BACKGROUND,
    TEXTURE,
    TEXT
}

impl RenderingType {
    pub fn to_shader_index(&self) -> u32 {
        return match self {
            RenderingType::SHAPE => 0,
            RenderingType::BACKGROUND => 1,
            RenderingType::TEXTURE => 2,
            RenderingType::TEXT => 3
        };
    }
}
