use cgmath::{Deg, Matrix4, Vector2, Vector3};
use lotus_proc_macros::Component;

use crate::EngineContext;

#[derive(Clone, Debug, Component)]
pub struct Transform {
    position: Vector2<f32>,
    rotation: f32,
    scale: Vector2<f32>
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

    pub fn write_update_to_buffer(&self, engine_context: &EngineContext) {
        let transform_matrix: Matrix4<f32> = self.to_matrix();
        let transform_matrix_as_ref: &[[f32; 4]; 4] = transform_matrix.as_ref();

        if let Some(transform_buffer) = engine_context.render_state.transform_buffer.as_ref() {
            engine_context.render_state.queue.write_buffer(
                transform_buffer,
                0,
                bytemuck::cast_slice(&[*transform_matrix_as_ref])
            );
        }
    }

    pub fn set_position(&mut self, engine_context: &EngineContext, position: Vector2<f32>) {
        self.position = position;

        self.write_update_to_buffer(engine_context);
    }

    pub fn get_position(&self) -> Vector2<f32> {
        return self.position;
    }

    pub fn set_rotation(&mut self, engine_context: &EngineContext, rotation: f32) {
        self.rotation = rotation;

        self.write_update_to_buffer(engine_context);
    }

    pub fn get_rotation(&self) -> f32 {
        return self.rotation;
    }

    pub fn set_scale(&mut self, engine_context: &EngineContext, scale: Vector2<f32>) {
        self.scale = scale;

        self.write_update_to_buffer(engine_context);
    }

    pub fn get_scale(&self) -> Vector2<f32> {
        return self.scale;
    }
}
