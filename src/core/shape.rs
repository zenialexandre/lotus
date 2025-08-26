use lotus_proc_macros::Component;
use super::{color::Color, managers::rendering::manager::Vertex};

/// Struct that represents every solid geometric form on the engine.
#[derive(Clone, Debug, Component)]
pub struct Shape {
    pub orientation: Orientation,
    pub geometry_type: GeometryType,
    pub color: Color
}

impl Shape {
    /// Create a new shape with parameters.
    pub fn new(orientation: Orientation, geometry_type: GeometryType, color: Color) -> Self {
        return Self {
            orientation,
            geometry_type,
            color
        };
    }

    /// Alter the orientation of a certain shape.
    pub fn orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    /// Alter the geometry type of a certain shape.
    pub fn geometry_type(&mut self, geometry_type: GeometryType) {
        self.geometry_type = geometry_type;
    }

    /// Alter the color of a certain shape.
    pub fn color(&mut self, color: Color) {
        self.color = color;
    }
}

/// Struct to represent the specific characteristics of a circle.
#[derive(Clone, Debug, PartialEq)]
pub struct Circle {
    pub number_of_segments: u16,
    pub radius: f32
}

impl Circle {
    /// Create a new circle with parameters.
    pub fn new(number_of_segments: u16, radius: f32) -> Self {
        return Self {
            number_of_segments,
            radius
        };
    }
}

impl Default for Circle {
    fn default() -> Self {
        return Self {
            number_of_segments: 64,
            radius: 0.5
        };
    }
}

/// Enumerator that represent the orientation of the geometric form.
#[derive(Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical
}

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
    pub fn to_vertex_array(&self, orientation: Orientation) -> Vec<Vertex> {
        match self {
            GeometryType::Triangle => {
                vec![
                    Vertex::build_default([0.0, 0.5, 0.0], [0.0, 0.0]),    // Top
                    Vertex::build_default([-0.5, -0.5, 0.0], [0.0, 0.0]),  // Left Down
                    Vertex::build_default([0.5, -0.5, 0.0], [0.0, 0.0])    // Right Down
                ]
            },
            GeometryType::Square => {
                vec![ 
                    Vertex::build_default([-1.0, -1.0, 0.0], [0.0, 1.0]), // Bottom Left
                    Vertex::build_default([1.0, -1.0, 0.0], [1.0, 1.0]),  // Bottom Right
                    Vertex::build_default([1.0, 1.0, 0.0], [1.0, 0.0]),   // Top Right
                    Vertex::build_default([-1.0, 1.0, 0.0], [0.0, 0.0])   // Top Left
                ]
            },
            GeometryType::Rectangle => {
                match orientation {
                    Orientation::Horizontal => {
                        vec![
                            Vertex::build_default([-0.75, -0.25, 0.0], [0.0, 1.0]), // Left Down
                            Vertex::build_default([0.75, -0.25, 0.0], [1.0, 1.0]),  // Right Down
                            Vertex::build_default([0.75, 0.25, 0.0], [1.0, 0.0]),   // Right Up
                            Vertex::build_default([-0.75, 0.25, 0.0], [0.0, 0.0])   // Left Up
                        ]
                    },
                    Orientation::Vertical => {
                        vec![
                            Vertex::build_default([-0.25, -0.75, 0.0], [0.0, 1.0]), // Left Down
                            Vertex::build_default([0.25, -0.75, 0.0], [1.0, 1.0]),  // Right Down
                            Vertex::build_default([0.25, 0.75, 0.0], [1.0, 0.0]),   // Right Up
                            Vertex::build_default([-0.25, 0.75, 0.0], [0.0, 0.0])   // Left Up
                        ]
                    }
                }
            },
            GeometryType::Circle(circle) => {
                let mut vertices: Vec<Vertex> = Vec::new();
                vertices.push(Vertex::build_default([0.0, 0.0, 0.0], [0.5, 0.5])); // Central Point

                for i in 0..circle.number_of_segments {
                    let theta: f32 = 2.0 * std::f32::consts::PI * (i as f32) / (circle.number_of_segments as f32);
                    let x: f32 = circle.radius * theta.cos();
                    let y: f32 = circle.radius * theta.sin();
                    vertices.push(Vertex::build_default([x, y, 0.0], [0.5 + x, 0.5 + y]));
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
