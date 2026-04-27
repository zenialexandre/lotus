use super::{shape::Circle, orientation::Orientation, super::{managers::render::manager::Vertex}};

/// Enumerator that represent the actual shape of the geometric form.
#[derive(Clone, Debug, PartialEq)]
pub enum GeometryType {
    Triangle,
    Square,
    Rectangle,
    Circle(Circle)
}

impl GeometryType {
    /// Returns the current array of vertices of a shape by its orientation and geometric type.
    pub fn to_vertex_array(&self, orientation: Orientation, color: [f32; 4]) -> Vec<Vertex> {
        match self {
            GeometryType::Triangle => {
                vec![
                    Vertex { position: [0.0, 0.5, 0.0], uv_coordinates: [0.0, 0.0], color },    // Top
                    Vertex { position: [-0.5, -0.5, 0.0], uv_coordinates: [0.0, 0.0], color },  // Left Down
                    Vertex { position: [0.5, -0.5, 0.0], uv_coordinates: [0.0, 0.0], color }    // Right Down
                ]
            },
            GeometryType::Square => {
                vec![
                    Vertex { position: [-1.0, -1.0, 0.0], uv_coordinates: [0.0, 1.0], color }, // Bottom Left
                    Vertex { position: [1.0, -1.0, 0.0], uv_coordinates: [1.0, 1.0], color },  // Bottom Right
                    Vertex { position: [1.0, 1.0, 0.0], uv_coordinates: [1.0, 0.0], color },   // Top Right
                    Vertex { position: [-1.0, 1.0, 0.0], uv_coordinates: [0.0, 0.0], color }   // Top Left
                ]
            },
            GeometryType::Rectangle => {
                match orientation {
                    Orientation::Horizontal => {
                        vec![
                            Vertex { position: [-0.75, -0.25, 0.0], uv_coordinates: [0.0, 1.0], color }, // Left Down
                            Vertex { position: [0.75, -0.25, 0.0], uv_coordinates: [1.0, 1.0], color },  // Right Down
                            Vertex { position: [0.75, 0.25, 0.0], uv_coordinates: [1.0, 0.0], color },   // Right Up
                            Vertex { position: [-0.75, 0.25, 0.0], uv_coordinates: [0.0, 0.0], color }   // Left Up
                        ]
                    },
                    Orientation::Vertical => {
                        vec![
                            Vertex { position: [-0.25, -0.75, 0.0], uv_coordinates: [0.0, 1.0], color }, // Left Down
                            Vertex { position: [0.25, -0.75, 0.0], uv_coordinates: [1.0, 1.0], color },  // Right Down
                            Vertex { position: [0.25, 0.75, 0.0], uv_coordinates: [1.0, 0.0], color },   // Right Up
                            Vertex { position: [-0.25, 0.75, 0.0], uv_coordinates: [0.0, 0.0], color }   // Left Up
                        ]
                    }
                }
            },
            GeometryType::Circle(circle) => {
                let mut vertices: Vec<Vertex> = Vec::new();
                vertices.push(Vertex { position: [0.0, 0.0, 0.0], uv_coordinates: [0.5, 0.5], color }); // Central Point

                for i in 0..circle.number_of_segments {
                    let theta: f32 = 2.0 * std::f32::consts::PI * (i as f32) / (circle.number_of_segments as f32);
                    let x: f32 = circle.radius * theta.cos();
                    let y: f32 = circle.radius * theta.sin();
                    vertices.push(Vertex { position: [x, y, 0.0], uv_coordinates: [0.5 + x, 0.5 + y], color });
                }
                vertices
            }
        }
    }

    /// Returns the current array of indices of a shape by its geometric type.
    pub fn to_index_array(&self) -> Vec<u16> {
        match self {
            GeometryType::Triangle => vec![0, 1, 2],
            GeometryType::Square => {
                vec![
                    0, 1, 2, // First Triangle
                    2, 3, 0  // Second Triangle
                ]
            },
            GeometryType::Rectangle => {
                vec![
                    0, 1, 2, // First Triangle
                    2, 3, 0  // Second Triangle
                ]
            },
            GeometryType::Circle(circle) => {
                let mut indices: Vec<u16> = Vec::new();

                for i in 0..circle.number_of_segments {
                    indices.push(0);
                    indices.push((i + 1) as u16);
                    indices.push(((i + 1) % circle.number_of_segments + 1) as u16);
                }
                indices
            }
        }
    }
}
