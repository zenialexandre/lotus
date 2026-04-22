/// A handy enumerator to store colors that can be used as options.
#[derive(Clone, Debug, Copy)]
pub enum ColorOption {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Orange,
    Purple,
    Pink,
    Brown,
    Lightgray,
    Gray,
    Darkgray,
    Gold,
    Silver,
    Turquoise,
    Violet,
    Limegreen,
    Lavender,
    Salmon,
    Peach,
    Mossgreen,
    Navyblue,
    Burgundy
}

impl ColorOption {
    /// Based on the current enumerator value, returns the color as an RGBA array of f32.
    pub fn to_rgba(&self) -> [f32; 4] {
        match self {
            ColorOption::Black => [0.0, 0.0, 0.0, 1.0],
            ColorOption::White => [1.0, 1.0, 1.0, 1.0],
            ColorOption::Red => [1.0, 0.0, 0.0, 1.0],
            ColorOption::Green => [0.0, 1.0, 0.0, 1.0],
            ColorOption::Blue => [0.0, 0.0, 1.0, 1.0],
            ColorOption::Yellow => [1.0, 1.0, 0.0, 1.0],
            ColorOption::Cyan => [0.0, 1.0, 1.0, 1.0],
            ColorOption::Magenta => [1.0, 0.0, 1.0, 1.0],
            ColorOption::Orange => [1.0, 0.65, 0.0, 1.0],
            ColorOption::Purple => [0.5, 0.0, 0.5, 1.0],
            ColorOption::Pink => [1.0, 0.75, 0.8, 1.0],
            ColorOption::Brown => [0.65, 0.16, 0.16, 1.0],
            ColorOption::Lightgray => [0.75, 0.75, 0.75, 1.0],
            ColorOption::Gray => [0.5, 0.5, 0.5, 1.0],
            ColorOption::Darkgray => [0.25, 0.25, 0.25, 1.0],
            ColorOption::Gold => [1.0, 0.84, 0.0, 1.0],
            ColorOption::Silver => [0.75, 0.75, 0.75, 1.0],
            ColorOption::Turquoise => [0.25, 0.88, 0.82, 1.0],
            ColorOption::Violet => [0.56, 0.0, 1.0, 1.0],
            ColorOption::Limegreen => [0.75, 1.0, 0.0, 1.0],
            ColorOption::Lavender => [0.9, 0.9, 0.98, 1.0],
            ColorOption::Salmon => [0.98, 0.5, 0.45, 1.0],
            ColorOption::Peach => [1.0, 0.8, 0.6, 1.0],
            ColorOption::Mossgreen => [0.6, 0.8, 0.2, 1.0],
            ColorOption::Navyblue => [0.0, 0.0, 0.5, 1.0],
            ColorOption::Burgundy => [0.5, 0.0, 0.13, 1.0]
        }
    }
}
