use crate::ColorOption;

/// The struct that holds a Color object in the engine.
#[derive(Clone, Debug, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    /// Create a new Color struct based on parameters.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Self { r, g, b, a};
    }

    /// Create a new Color struct by using a option.
    pub fn by_option(color_option: ColorOption) -> Self {
        let rgba: [f32; 4] = color_option.to_rgba();
        return Self { r: rgba[0], g: rgba[1], b: rgba[2], a: rgba[3] };
    }

    /// Returns the current Color as an WGPU Color struct.
    pub fn to_wgpu(self) -> wgpu::Color {
        return wgpu::Color { r: self.r as f64, g: self.g as f64, b: self.b as f64, a: self.a as f64 };
    }

    /// Returns the current Color as a array.
    pub fn to_array(self) -> [f32; 4] {
        return [self.r, self.g, self.b, self.a];
    }

    /// Returns a WGPU Color struct as an array of f32.
    pub fn to_array_by_wgpu(color: wgpu::Color) -> [f32; 4] {
        return [color.r as f32, color.g as f32, color.b as f32, color.a as f32];
    }
}
