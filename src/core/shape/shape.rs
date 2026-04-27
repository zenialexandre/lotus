use lotus_proc_macros::Component;
use super::{super::color::color::Color, geometry_type::GeometryType, orientation::Orientation};

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
