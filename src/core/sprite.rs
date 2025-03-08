use super::{rendering_manager::Vertex, shape::{GeometryType, Orientation}, transform::Transform};

pub struct Sprite {
    pub path: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub transform: Transform
}

impl Sprite {
    pub fn new(path: String, transform: Transform) -> Self {
        let vertices: Vec<Vertex> = GeometryType::Square.to_vertex_array(Orientation::Horizontal);
        let indices: Vec<u16> = GeometryType::Square.to_index_array();

        return Self {
            path,
            vertices,
            indices,
            transform
        };
    }
}
