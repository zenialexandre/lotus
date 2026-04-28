/// Struct that represents the current mouse position.
#[derive(Clone, Debug)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
}

impl Default for MousePosition {
    fn default() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }
}
