use cgmath::{Deg, Matrix4, Vector2, Vector3};
use lotus_proc_macros::Component;
use super::super::managers::rendering::manager::RenderState;

/// Enumerator to represent the strategy that will be used for positioning.
#[derive(Clone, Default, Debug, PartialEq)]
pub enum Strategy {
    /// The default value.
    ///
    /// The normalized strategy should receive values between -1.0 and 1.0.
    ///
    /// The center of the screen will always be 0.0 for the width and 0.0 for the height.
    #[default]
    Normalized,

    /// The pixelated strategy should receive values that makes sense with the current window resolution.
    ///
    /// If the current window resolution is 800 pixels of width and 800 pixels of height.
    ///
    /// Then the center of the screen is 400.0 for the width and 400.0 for the height.
    ///
    /// The top left border will always be 0.0 for the width and 0.0 for the height.
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

/// Struct to represent the transform matrix of every object rendered.
#[derive(Clone, Debug, Component)]
pub struct Transform {
    pub position: Position,
    pub rotation: f32,
    pub scale: Vector2<f32>,
    pub(crate) dirty_position: bool,
    pub(crate) dirty_scale: bool
}

impl Default for Transform {
    /// Returns a default transform struct.
    fn default() -> Self {
        return Self {
            position: Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
            rotation: 0.0,
            scale: Vector2::new(1.0, 1.0),
            dirty_position: true,
            dirty_scale: true
        };
    }
}

impl Transform {
    /// Create a new transform with parameters.
    pub fn new(position: Position, rotation: f32, scale: Vector2<f32>) -> Self {
        return Self {
            position,
            rotation,
            scale,
            dirty_position: true,
            dirty_scale: true
        };
    }

    /// Create a new transform with only the position as an argument.
    pub fn new_simple(position: Position) -> Self {
        return Self {
            position,
            ..Default::default()
        }
    }

    /// Returns the current transform struct as a matrix of f32s.
    pub fn to_matrix(&self) -> Matrix4<f32> {
        return Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.0)) *
            Matrix4::from_angle_z(Deg(self.rotation)) *
            Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, 1.0);
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

    /// Set the position on initialization.
    pub fn position(self, position: Position) -> Self {
        return Self {
            position,
            ..self
        };
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

    /// Set the current position and sends it to the buffer.
    ///
    /// Useful to set a brand new position using pixelated values.
    ///
    /// Your pixelated coordinate will be normalized.
    pub fn set_position_pixelated(&mut self, render_state: &RenderState, position: Vector2<f32>) {
        self.dirty_position = true;
        self.set_position(render_state, position);
    }

    /// Set the curretn position x and sends it to the buffer.
    ///
    /// Useful to set a brand new position x using pixelated values.
    ///
    /// Your pixelated coordinate will be normalized.
    pub fn set_position_x_pixelated(&mut self, render_state: &RenderState, x: f32) {
        self.dirty_position = true;
        self.set_position_x(render_state, x);
    }

    /// Set the curretn position y and sends it to the buffer.
    ///
    /// Useful to set a brand new position y using pixelated values.
    ///
    /// Your pixelated coordinate will be normalized.
    pub fn set_position_y_pixelated(&mut self, render_state: &RenderState, y: f32) {
        self.dirty_position = true;
        self.set_position_y(render_state, y);
    }

    /// Get the current position.
    pub fn get_position(&self) -> Vector2<f32> {
        return Vector2::new(self.position.x, self.position.y);
    }

    /// Set the rotation on initialization.
    pub fn rotation(self, rotation: f32) -> Self {
        return Self {
            rotation,
            ..self
        };
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

    /// Set the scale on initialization.
    pub fn scale(self, scale: Vector2<f32>) -> Self {
        return Self {
            scale,
            ..self
        };
    }

    /// Set the current scale and sends it to the buffer.
    pub fn set_scale(&mut self, render_state: &RenderState, scale: Vector2<f32>) {
        self.scale = scale;
        self.dirty_scale = true;
        self.write_update_to_buffer(render_state);
    }

    /// Get the current scale.
    pub fn get_scale(&self) -> Vector2<f32> {
        return self.scale;
    }
}
