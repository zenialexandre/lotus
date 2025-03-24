use cgmath::Vector2;
use lotus_proc_macros::Component;

/// Struct to represent the velocity that can be applied over objects for smooth movemnet.
#[derive(Clone, Component)]
pub struct Velocity {
    pub value: Vector2<f32>
}

impl Velocity {
    /// Create a new velocity with parameters.
    pub fn new(value: Vector2<f32>) -> Self {
        return Self {
            value
        };
    }
}
