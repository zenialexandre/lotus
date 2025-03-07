#[derive(Clone, Copy)]
pub enum Color {
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE
}

impl Color {
    pub fn to_rgba(&self) -> [f64; 4] {
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
    let color_rgba: [f64; 4] = Color::to_rgba(&color);
    return wgpu::Color { r: color_rgba[0], g: color_rgba[1], b: color_rgba[2], a: color_rgba[3] };
}

pub(crate) fn to_array(color: wgpu::Color) -> [f64; 4] {
    return [
        color.r,
        color.g,
        color.b,
        color.a
    ];
}
