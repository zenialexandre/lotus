use cgmath::Vector2;
use lotus_proc_macros::Component;

/// Struct to represent the velocity that can be applied over objects for smooth movemnet.
#[derive(Clone, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Velocity {
    /// Create a new velocity with parameters.
    pub fn new(value: Vector2<f32>) -> Self {
        return Self {
            x: value.x,
            y: value.y
        };
    }

    /// Update the velocity values.
    pub fn update_values(&mut self, value: Vector2<f32>) {
        self.x = value.x;
        self.y = value.y;
    }

    /// Returns the velocity as a vector.
    pub fn to_vec(&self) -> Vector2<f32> {
        return Vector2::new(self.x, self.y);
    }
}
