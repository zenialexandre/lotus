use super::gamepad_axis_state::GamepadAxisState;

/// Struct that represents a joystick with its X and Y states.
#[derive(Clone, Copy, Debug)]
pub struct GamepadJoystick {
    pub x: GamepadAxisState,
    pub y: GamepadAxisState
}

impl Default for GamepadJoystick {
    /// Returns a default joystick with zeroed X and Y states.
    fn default() -> Self {
        Self {
            x: GamepadAxisState::default(),
            y: GamepadAxisState::default(),
        }
    }
}
