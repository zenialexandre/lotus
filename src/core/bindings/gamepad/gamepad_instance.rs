use std::collections::HashSet;
use gilrs::Axis;
use super::{super::input::Input, joystick::Joystick, gamepad_button::GamepadButton};

/// Struct to hold a gamepad instance.
#[derive(Clone)]
pub struct GamepadInstance {
    pub pressed: HashSet<GamepadButton>,
    pub previously_pressed: HashSet<GamepadButton>,
    pub left_joystick: Joystick,
    pub right_joystick: Joystick,
    pub is_connected: bool
}

impl Input for GamepadInstance {
    /// Updates the gamepad input state.
    fn update_hashes(&mut self) {
        self.previously_pressed = self.pressed.clone();
    }

    /// Returns if any button is pressed.
    fn is_some_pressed(&self) -> bool {
        return self.pressed.len() > 0;
    }

    /// Returns if any button is released.
    fn is_some_released(&self) -> bool {
        return self.previously_pressed.len() > 0;
    }
}

impl Default for GamepadInstance {
    /// Create a default GamepadInstance struct.
    fn default() -> Self {
        return Self {
            pressed: HashSet::new(),
            previously_pressed: HashSet::new(),
            left_joystick: Joystick::default(),
            right_joystick: Joystick::default(),
            is_connected: true
        };
    }
}

impl GamepadInstance {
    /// Sets the current state of connection to `true`.
    pub(crate) fn connect(&mut self) {
        self.is_connected = true;
    }

    /// Sets the current state of connection to `false`.
    pub(crate) fn disconnect(&mut self) {
        self.is_connected = false;
    }

    /// Manages joystick persistence after events.
    pub(crate) fn joystick(&mut self, axis: &Axis, direction: &f32) {
        match axis {
            Axis::LeftStickX => {
                self.left_joystick.x.previous_direction = self.left_joystick.x.current_direction;
                self.left_joystick.x.current_direction = *direction;
            },
            Axis::LeftStickY => {
                self.left_joystick.y.previous_direction = self.left_joystick.y.current_direction;
                self.left_joystick.y.current_direction = *direction;
            },
            Axis::RightStickX => {
                self.right_joystick.x.previous_direction = self.right_joystick.x.current_direction;
                self.right_joystick.x.current_direction = *direction;
            },
            Axis::RightStickY => {
                self.right_joystick.y.previous_direction = self.right_joystick.y.current_direction;
                self.right_joystick.y.current_direction = *direction;
            },
            _ => {}
        }
    }

    /// Returns if the right joystick is moving.
    pub fn is_right_joystick_moving(&self, deadzone: f32) -> bool {
        return (self.right_joystick.x.current_direction != 0.0 && self.right_joystick.x.current_direction.abs() > deadzone) ||
            (self.right_joystick.y.current_direction != 0.0 && self.right_joystick.y.current_direction.abs() > deadzone);
    }

    /// Returns if the right joystick is stopping.
    pub fn has_right_joystick_stopped(&self) -> bool {
        return self.right_joystick.x.current_direction == 0.0 && self.right_joystick.y.current_direction == 0.0;
    }

    /// Returns if the left joystick is moving.
    pub fn is_left_joystick_moving(&self, deadzone: f32) -> bool {
        return (self.left_joystick.x.current_direction != 0.0 && self.left_joystick.x.current_direction.abs() > deadzone) ||
            (self.left_joystick.y.current_direction != 0.0 && self.left_joystick.y.current_direction.abs() > deadzone);
    }

    /// Returns if the left joystick is stopping.
    pub fn has_left_joystick_stopped(&self) -> bool {
        return self.left_joystick.x.current_direction == 0.0 && self.left_joystick.y.current_direction == 0.0;
    }

    /// Returns if any joystick is moving.
    pub fn is_any_joystick_moving(&self, deadzone: f32) -> bool {
        return self.is_left_joystick_moving(deadzone) || self.is_right_joystick_moving(deadzone);
    }

    /// Returns if any joystick is stopping.
    pub fn is_any_joystick_stopped(&self) -> bool {
        return self.has_left_joystick_stopped() || self.has_right_joystick_stopped();
    }

    /// Returns if one of the buttons inside a list is pressed.
    pub fn is_some_of_buttons_pressed(&self, buttons: Vec<GamepadButton>) -> bool {
        let mut is_some_of_buttons_pressed: bool = false;

        for button in buttons {
            is_some_of_buttons_pressed = self.pressed.contains(&button);

            if is_some_of_buttons_pressed {
                return is_some_of_buttons_pressed;
            }
        }
        return is_some_of_buttons_pressed;
    }

    /// Returns if a specific list of buttons are all pressed.
    pub fn is_buttons_pressed(&self, buttons: Vec<GamepadButton>) -> bool {
        return buttons.iter().all(|element| self.pressed.contains(element));
    }

    /// Returns if a specific button is pressed.
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        return self.pressed.contains(&button);
    }

    /// Returns if one of the buttons inside a list is released.
    pub fn is_some_of_buttons_released(&self, buttons: Vec<GamepadButton>) -> bool {
        let mut is_some_of_buttons_released: bool = false;

        for button in buttons {
            is_some_of_buttons_released = self.previously_pressed.contains(&button) && !self.pressed.contains(&button);

            if is_some_of_buttons_released {
                return is_some_of_buttons_released;
            }
        }
        return is_some_of_buttons_released;
    }

    /// Returns if a specific list of buttons are all released.
    pub fn is_buttons_released(&self, buttons: Vec<GamepadButton>) -> bool {
        return buttons.iter().all(|element| self.previously_pressed.contains(element) && !self.pressed.contains(element));
    }

    /// Returns if a specific button is released.
    pub fn is_button_released(&self, button: GamepadButton) -> bool {
        return self.previously_pressed.contains(&button) && !self.pressed.contains(&button);
    }
}
