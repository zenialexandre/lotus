/// Enumerator used to separate different rendering processes by type.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum RenderingType {
    #[default]
    Shape,
    Texture,
    Text
}

impl RenderingType {
    /// Returns the specific rendering type index by its enumerator.
    pub fn to_shader_index(&self) -> u32 {
        return match self {
            RenderingType::Shape => 0,
            RenderingType::Texture => 1,
            RenderingType::Text => 2
        };
    }
}
