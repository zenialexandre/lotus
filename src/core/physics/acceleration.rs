use glam::Vec2;
use lotus_proc_macros::Component;

/// Struct to represent the acceleration.
#[derive(Clone, Component)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32
}

impl Acceleration {
    /// Create a new acceleration with parameters.
    pub fn new(value: Vec2) -> Self {
        return Self {
            x: value.x,
            y: value.y
        };
    }

    /// Update the acceleration values.
    pub fn update_values(&mut self, value: Vec2) {
        self.x = value.x;
        self.y = value.y;
    }

    /// Returns the acceleration as a vector.
    pub fn to_vec(&self) -> Vec2 {
        return Vec2::new(self.x, self.y);
    }
}
