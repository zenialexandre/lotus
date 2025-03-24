use lotus_proc_macros::Component;

use super::{
    managers::rendering_manager::Vertex,
    shape::{
        GeometryType,
        Orientation
    }
};

/// # Struct to represent a sprite with its data.
/// A sprite is represented as two triangles, or a square.
#[derive(Clone, Debug, Component)]
pub struct Sprite {
    pub path: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>
}

impl Sprite {
    /// Create a new sprite with its file path as the parameter.
    pub fn new(path: String) -> Self {
        let vertices: Vec<Vertex> = GeometryType::Square.to_vertex_array(Orientation::Horizontal);
        let indices: Vec<u16> = GeometryType::Square.to_index_array();
        let sprite: Sprite = Self {
            path,
            vertices,
            indices
        };
        return sprite;
    }
}
