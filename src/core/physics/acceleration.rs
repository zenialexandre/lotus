use cgmath::Vector2;
use lotus_proc_macros::Component;

/// Struct to represent the acceleration/force of gravity.
#[derive(Clone, Component)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32
}

impl Acceleration {
    /// Create a new acceleration with parameters.
    pub fn new(value: Vector2<f32>) -> Self {
        return Self {
            x: value.x,
            y: value.y
        };
    }

    /// Update the acceleration values.
    pub fn update_values(&mut self, value: Vector2<f32>) {
        self.x = value.x;
        self.y = value.y;
    }

    /// Returns the acceleration as a vector.
    pub fn to_vec(&self) -> Vector2<f32> {
        return Vector2::new(self.x, self.y);
    }
}
