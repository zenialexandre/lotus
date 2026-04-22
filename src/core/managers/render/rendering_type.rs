/// Enumerator used to separate different rendering processes by type.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum RenderingType {
    #[default]
    Shape,
    Background,
    Texture,
    Text
}

impl RenderingType {
    /// Returns the specific rendering type index by its enumerator.
    pub fn to_shader_index(&self) -> u32 {
        return match self {
            RenderingType::Shape => 0,
            RenderingType::Background => 1,
            RenderingType::Texture => 2,
            RenderingType::Text => 3
        };
    }
}
