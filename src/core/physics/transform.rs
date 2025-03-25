use cgmath::{Deg, Matrix4, Vector2, Vector3};
use lotus_proc_macros::Component;

/// Struct to represent the transform matrix of every object rendered.
#[derive(Clone, Debug, Component)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>
}

impl Default for Transform {
    /// Returns a default transform struct.
    fn default() -> Self {
        return Self {
            position: Vector2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vector2::new(0.25, 0.25)
        };
    }
}

impl Transform {
    /// Create a new transform with parameters.
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Self {
        return Self {
            position,
            rotation,
            scale
        };
    }

    /// Returns the current transform struct as a matrix of f32s.
    pub fn to_matrix(&self) -> Matrix4<f32> {
        return Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.)) *
            Matrix4::from_angle_z(Deg(self.rotation)) *
            Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 1.);
    }

    fn _interpolate(&mut self, alpha: f32) {
        let interpolated_position: Vector2<f32> = self.position * (1.0 - alpha) + self.position * alpha;
        self.set_position(interpolated_position);
    }

    /// Set the current position.
    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }

    /// Get the current position.
    pub fn get_position(&self) -> Vector2<f32> {
        return self.position;
    }

    /// Set the current rotation.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Get the current rotation.
    pub fn get_rotation(&self) -> f32 {
        return self.rotation;
    }

    /// Set the current scale.
    pub fn set_scale(&mut self, scale: Vector2<f32>) {
        self.scale = scale;
    }

    /// Get the current scale.
    pub fn get_scale(&self) -> Vector2<f32> {
        return self.scale;
    }
}
