#[derive(Clone, Debug, Copy)]
pub enum Color {
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE
}

impl Color {
    pub fn to_rgba(&self) -> [f32; 4] {
        match self {
            Color::BLACK => [0.0, 0.0, 0.0, 1.0],
            Color::WHITE => [1.0, 1.0, 1.0, 1.0],
            Color::RED => [1.0, 0.0, 0.0, 1.0],
            Color::GREEN => [0.0, 1.0, 0.0, 1.0],
            Color::BLUE => [0.0, 0.0, 1.0, 1.0]
        }
    }
}

pub(crate) fn to_wgpu(color: Color) -> wgpu::Color {
    let color_rgba: [f32; 4] = Color::to_rgba(&color);
    return wgpu::Color { r: color_rgba[0] as f64, g: color_rgba[1] as f64, b: color_rgba[2] as f64, a: color_rgba[3] as f64 };
}

pub(crate) fn to_array(color: wgpu::Color) -> [f32; 4] {
    return [
        color.r as f32,
        color.g as f32,
        color.b as f32,
        color.a as f32
    ];
}
