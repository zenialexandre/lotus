use cgmath::{Deg, Matrix4, Vector2};

pub struct Camera2d {
    pub position: Vector2<f32>,
    pub zoom: f32,
    pub rotation: Deg<f32>,
    pub viewport_size: Vector2<f32>,
    pub dirty: bool,
    pub view_matrix: Matrix4<f32>,
}