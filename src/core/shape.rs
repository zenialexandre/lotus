use lotus_proc_macros::Component;

use super::{color::Color, managers::rendering_manager::Vertex};

#[derive(Clone, Debug, Component)]
pub struct Shape {
    pub orientation: Orientation,
    pub geometry_type: GeometryType,
    pub color: Color
}

impl Shape {
    pub fn new(orientation: Orientation, geometry_type: GeometryType, color: Color) -> Self {
        let shape: Shape = Self {
            orientation,
            geometry_type,
            color
        };
        return shape;
    }
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub number_of_segments: u16,
    pub radius: f32
}

impl Circle {
    pub fn new(number_of_segments: u16, radius: f32) -> Self {
        return Self {
            number_of_segments,
            radius
        };
    }
}

#[derive(Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Clone, Debug)]
pub enum GeometryType {
    Triangle,
    Square,
    Rectangle,
    Circle(Circle)
}

impl GeometryType {
    pub fn to_vertex_array(&self, orientation: Orientation) -> Vec<Vertex> {
        match self {
            GeometryType::Triangle => {
                vec![
                    Vertex { position: [0.0, 0.5, 0.0], texture_coordinates: [0.0, 0.0] },    // Top
                    Vertex { position: [-0.5, -0.5, 0.0], texture_coordinates: [0.0, 0.0] },  // Left Down
                    Vertex { position: [0.5, -0.5, 0.0], texture_coordinates: [0.0, 0.0] }    // Right Down
                ]
            },
            GeometryType::Square => {
                vec![ 
                    Vertex { position: [-1.0, -1.0, 0.0], texture_coordinates: [0.0, 1.0] }, // Bottom Left
                    Vertex { position: [1.0, -1.0, 0.0], texture_coordinates: [1.0, 1.0] },  // Bottom Right
                    Vertex { position: [1.0, 1.0, 0.0], texture_coordinates: [1.0, 0.0] },   // Top Right
                    Vertex { position: [-1.0, 1.0, 0.0], texture_coordinates: [0.0, 0.0] }   // Top Left
                ]
            },
            GeometryType::Rectangle => {
                match orientation {
                    Orientation::Horizontal => {
                        vec![
                            Vertex { position: [-0.75, -0.25, 0.0], texture_coordinates: [0.0, 1.0] }, // Left Down
                            Vertex { position: [0.75, -0.25, 0.0], texture_coordinates: [1.0, 1.0] },  // Right Down
                            Vertex { position: [0.75, 0.25, 0.0], texture_coordinates: [1.0, 0.0] },   // Right Up
                            Vertex { position: [-0.75, 0.25, 0.0], texture_coordinates: [0.0, 0.0] }   // Left Up
                        ]
                    },
                    Orientation::Vertical => {
                        vec![
                            Vertex { position: [-0.25, -0.75, 0.0], texture_coordinates: [0.0, 1.0] }, // Left Down
                            Vertex { position: [0.25, -0.75, 0.0], texture_coordinates: [1.0, 1.0] },  // Right Down
                            Vertex { position: [0.25, 0.75, 0.0], texture_coordinates: [1.0, 0.0] },   // Right Up
                            Vertex { position: [-0.25, 0.75, 0.0], texture_coordinates: [0.0, 0.0] }   // Left Up
                        ]
                    }
                }
            },
            GeometryType::Circle(circle) => {
                let mut vertices: Vec<Vertex> = Vec::new();
                vertices.push(Vertex { position: [0.0, 0.0, 0.0], texture_coordinates: [0.5, 0.5] }); // Central Point

                for i in 0..circle.number_of_segments {
                    let theta: f32 = 2.0 * std::f32::consts::PI * (i as f32) / (circle.number_of_segments as f32);
                    let x: f32 = circle.radius * theta.cos();
                    let y: f32 = circle.radius * theta.sin();
                    vertices.push(Vertex { position: [x, y, 0.0], texture_coordinates: [0.5 + x, 0.5 + y] });
                }
                vertices
            }
        }
    }

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
