/// Struct that represents the state of an axis on a joystick.
#[derive(Clone, Copy, Debug)]
pub struct AxisState {
    pub previous_direction: f32,
    pub current_direction: f32
}

impl Default for AxisState {
    /// Returns a default `AxisState` with `previous_direction` and `current_direction` set to 0.0.
    fn default() -> Self {
        Self {
            previous_direction: 0.0,
            current_direction: 0.0
        }
    }
}
