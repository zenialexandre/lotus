/// A handy enumerator to store colors that can be used on solid objects.
#[derive(Clone, Debug, Copy)]
pub enum Color {
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    CYAN,
    MAGENTA,
    ORANGE,
    PURPLE,
    PINK,
    BROWN,
    LIGHTGRAY,
    GRAY,
    DARKGRAY,
    GOLD,
    SILVER,
    TURQUOISE,
    VIOLET,
    LIMEGREEN,
    LAVENDER,
    SALMON,
    PEACH,
    MOSSGREEN,
    NAVYBLUE,
    BURGUNDY
}

impl Color {
    /// Based on the current enumerator value, returns the color as an RGBA array of f32.
    pub fn to_rgba(&self) -> [f32; 4] {
        match self {
            Color::BLACK => [0.0, 0.0, 0.0, 1.0],
            Color::WHITE => [1.0, 1.0, 1.0, 1.0],
            Color::RED => [1.0, 0.0, 0.0, 1.0],
            Color::GREEN => [0.0, 1.0, 0.0, 1.0],
            Color::BLUE => [0.0, 0.0, 1.0, 1.0],
            Color::YELLOW => [1.0, 1.0, 0.0, 1.0],
            Color::CYAN => [0.0, 1.0, 1.0, 1.0],
            Color::MAGENTA => [1.0, 0.0, 1.0, 1.0],
            Color::ORANGE => [1.0, 0.65, 0.0, 1.0],
            Color::PURPLE => [0.5, 0.0, 0.5, 1.0],
            Color::PINK => [1.0, 0.75, 0.8, 1.0],
            Color::BROWN => [0.65, 0.16, 0.16, 1.0],
            Color::LIGHTGRAY => [0.75, 0.75, 0.75, 1.0],
            Color::GRAY => [0.5, 0.5, 0.5, 1.0],
            Color::DARKGRAY => [0.25, 0.25, 0.25, 1.0],
            Color::GOLD => [1.0, 0.84, 0.0, 1.0],
            Color::SILVER => [0.75, 0.75, 0.75, 1.0],
            Color::TURQUOISE => [0.25, 0.88, 0.82, 1.0],
            Color::VIOLET => [0.56, 0.0, 1.0, 1.0],
            Color::LIMEGREEN => [0.75, 1.0, 0.0, 1.0],
            Color::LAVENDER => [0.9, 0.9, 0.98, 1.0],
            Color::SALMON => [0.98, 0.5, 0.45, 1.0],
            Color::PEACH => [1.0, 0.8, 0.6, 1.0],
            Color::MOSSGREEN => [0.6, 0.8, 0.2, 1.0],
            Color::NAVYBLUE => [0.0, 0.0, 0.5, 1.0],
            Color::BURGUNDY => [0.5, 0.0, 0.13, 1.0]
        }
    }

    /// Returns the current Color as an WGPU Color struct.
    pub fn to_wgpu(color: Color) -> wgpu::Color {
        let color_rgba: [f32; 4] = Color::to_rgba(&color);
        return wgpu::Color { r: color_rgba[0] as f64, g: color_rgba[1] as f64, b: color_rgba[2] as f64, a: color_rgba[3] as f64 };
    }

    /// Returns a WGPU Color struct as an array of f32.
    pub fn to_array(color: wgpu::Color) -> [f32; 4] {
        return [
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32
        ];
    }
}
