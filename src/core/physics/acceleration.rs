use cgmath::Vector2;
use lotus_proc_macros::Component;

/// Struct to represent the acceleration/force of gravity.
#[derive(Clone, Component)]
pub struct Acceleration {
    pub value: Vector2<f32>
}

impl Acceleration {
    /// Create a new acceleration with parameters.
    pub fn new(value: Vector2<f32>) -> Self {
        return Self {
            value
        };
    }
}
