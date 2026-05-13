/// Enumerator used to separate different rendering processes by type.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum RenderType {
    #[default]
    Shape,
    Texture,
    Text
}

impl RenderType {
    /// Returns the specific rendering type index by its enumerator.
    pub fn to_shader_index(&self) -> u32 {
        return match self {
            RenderType::Shape => 0,
            RenderType::Texture => 1,
            RenderType::Text => 2
        };
    }
}
