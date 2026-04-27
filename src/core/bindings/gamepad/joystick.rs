use super::axis_state::AxisState;

/// Struct that represents a joystick with its X and Y states.
#[derive(Clone, Copy, Debug)]
pub struct Joystick {
    pub x: AxisState,
    pub y: AxisState
}

impl Default for Joystick {
    /// Returns a default joystick with zeroed X and Y states.
    fn default() -> Self {
        Self {
            x: AxisState::default(),
            y: AxisState::default(),
        }
    }
}

impl Joystick {

}
