use cgmath::{Deg, Matrix4, Vector2, Vector3};
use lotus_proc_macros::Component;
use super::super::managers::rendering_manager::RenderState;

/// Enumerator to represent the strategy that will be used for the positioning and scaling. 
#[derive(Clone, Default, Debug, PartialEq)]
pub enum Strategy {
    #[default]
    Normalized,
    Pixelated
}

/// Struct to represent the position of the transformation matrix.
#[derive(Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub strategy: Strategy
}

impl Position {
    /// Creates a new position struct.
    pub fn new(value: Vector2<f32>, strategy: Strategy) -> Self {
        return Self {
            x: value.x,
            y: value.y,
            strategy
        };
    }

    /// Update the position values.
    pub fn update_values(&mut self, value: Vector2<f32>) {
        self.x = value.x;
        self.y = value.y;
    }

    /// Returns the position as a vector.
    pub fn to_vec(&self) -> Vector2<f32> {
        return Vector2::new(self.x, self.y);
    }
}

/// Struct to represent the scale of the transformation matrix.
#[derive(Clone, Debug)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
    pub strategy: Strategy
}

impl Scale {
    /// Creates a new scale struct.
    pub fn new(value: Vector2<f32>, strategy: Strategy) -> Self {
        return Self {
            x: value.x,
            y: value.y,
            strategy
        };
    }

    /// Update the scale values.
    pub fn update_values(&mut self, value: Vector2<f32>) {
        self.x = value.x;
        self.y = value.y;
    }

    /// Returns the scale as a vector.
    pub fn to_vec(&self) -> Vector2<f32> {
        return Vector2::new(self.x, self.y);
    }
}

/// Struct to represent the transform matrix of every object rendered.
#[derive(Clone, Debug, Component)]
pub struct Transform {
    pub position: Position,
    pub rotation: f32,
    pub scale: Scale
}

impl Default for Transform {
    /// Returns a default transform struct.
    fn default() -> Self {
        return Self {
            position: Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
            rotation: 0.0,
            scale: Scale::new(Vector2::new(0.25, 0.25), Strategy::Normalized)
        };
    }
}

impl Transform {
    /// Create a new transform with parameters.
    pub fn new(position: Position, rotation: f32, scale: Scale) -> Self {
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

    /// Write the transform matrix updates to its related buffer on rendering surface.
    pub fn write_update_to_buffer(&self, render_state: &RenderState) {
        let transform_matrix: Matrix4<f32> = self.to_matrix();
        let transform_matrix_as_ref: &[[f32; 4]; 4] = transform_matrix.as_ref();

        if let Some(transform_buffer) = render_state.transform_buffer.as_ref() {
            render_state.queue.as_ref().unwrap().write_buffer(
                transform_buffer,
                0,
                bytemuck::cast_slice(&[*transform_matrix_as_ref])
            );
        }
    }

    /// Set the current position and sends it to the buffer.
    pub fn set_position(&mut self, render_state: &RenderState, position: Vector2<f32>) {
        self.position.x = position.x;
        self.position.y = position.y;
        self.write_update_to_buffer(render_state);
    }

    /// Set the curretn position x and sends it to the buffer.
    pub fn set_position_x(&mut self, render_state: &RenderState, x: f32) {
        self.position.x = x;
        self.write_update_to_buffer(render_state);
    }

    /// Set the curretn position y and sends it to the buffer.
    pub fn set_position_y(&mut self, render_state: &RenderState, y: f32) {
        self.position.y = y;
        self.write_update_to_buffer(render_state);
    }

    /// Get the current position.
    pub fn get_position(&self) -> Vector2<f32> {
        return Vector2::new(self.position.x, self.position.y);
    }

    /// Set the current rotation and sends it to the buffer.
    pub fn set_rotation(&mut self, render_state: &RenderState, rotation: f32) {
        self.rotation = rotation;
        self.write_update_to_buffer(render_state);
    }

    /// Get the current rotation.
    pub fn get_rotation(&self) -> f32 {
        return self.rotation;
    }

    /// Set the current scale and sends it to the buffer.
    pub fn set_scale(&mut self, render_state: &RenderState, scale: Vector2<f32>) {
        self.scale.x = scale.x;
        self.scale.y = scale.y;
        self.write_update_to_buffer(render_state);
    }

    /// Get the current scale.
    pub fn get_scale(&self) -> Vector2<f32> {
        return Vector2::new(self.scale.x, self.scale.y);
    }
}
