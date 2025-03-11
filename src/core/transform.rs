use cgmath::{Deg, Matrix4, Vector2, Vector3};

use lotus_proc_macros::Component;

#[derive(Clone, Debug, Component)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>
}

impl Default for Transform {
    fn default() -> Self {
        return Self {
            position: Vector2::new(0., 0.),
            rotation: 0.,
            scale: Vector2::new(1., 1.)
        };
    }
}

impl Transform {
    pub fn new(position: Vector2<f32>, rotation: f32, scale: Vector2<f32>) -> Self {
        return Self {
            position,
            rotation,
            scale
        };
    }

    pub fn to_matrix(&self) -> Matrix4<f32> {
        return Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.)) *
            Matrix4::from_angle_z(Deg(self.rotation)) *
            Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 1.);
    }
}
