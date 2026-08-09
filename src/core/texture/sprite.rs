use lotus_proc_macros::Component;

/// Struct to represent a sprite with its data.
///
/// A sprite is represented as two triangles, or a square.
#[derive(Clone, Debug, Component)]
pub struct Sprite {
    pub path: String
}

impl Sprite {
    /// Create a new sprite with its file path as the parameter.
    pub fn new(path: String) -> Self {
        return Self {
            path
        };
    }
}
