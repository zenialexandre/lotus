use std::collections::{HashMap, HashSet};
use gilrs::{Button, GamepadId};
use lotus_proc_macros::Resource;
use super::input::Input;

/// Global resource to store all the gamepad inputs done on runtime.
#[derive(Clone, Resource)]
pub struct GamepadInput {
    pub instances: HashMap<GamepadId, GamepadInstance>
}

impl GamepadInput {
    /// Returns a default GamepadInput struct.
    pub fn default() -> Self {
        return Self {
            instances: HashMap::new()
        };
    }
}

/// Struct to hold a gamepad instance.
#[derive(Clone)]
pub struct GamepadInstance {
    pub pressed: HashSet<Button>,
    pub previously_pressed: HashSet<Button>,
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

impl GamepadInstance {
    /// Create a new gamepad struct instance.
    pub fn new() -> Self {
        return Self {
            pressed: HashSet::new(),
            previously_pressed: HashSet::new(),
            is_connected: true
        };
    }

    /// Returns if one of the buttons inside a list is pressed.
    pub fn is_some_of_buttons_pressed(&self, buttons: Vec<Button>) -> bool {
        return false;
    }

    /// Returns if a specific list of buttons are all pressed.
    pub fn is_buttons_pressed(&self, buttons: Vec<Button>) -> bool {
        return buttons.iter().all(|element| self.pressed.contains(element));
    }

    /// Returns if a specific button is pressed.
    pub fn is_button_pressed(&self, button: Button) -> bool {
        return self.pressed.contains(&button);
    }

    /// Returns if one of the buttons inside a list is released.
    pub fn is_some_of_buttons_released(&self, buttons: Vec<Button>) -> bool {
        return false;
    }

    /// Returns if a specific list of buttons are all released.
    pub fn is_buttons_released(&self, buttons: Vec<Button>) -> bool {
        return buttons.iter().all(|element| self.previously_pressed.contains(element) && !self.pressed.contains(element));
    }

    /// Returns if a specific button is released.
    pub fn is_button_released(&self, button: Button) -> bool {
        return self.previously_pressed.contains(&button) && !self.pressed.contains(&button);
    }
}
