use std::collections::{HashMap, HashSet};
use gilrs::{Axis, Button, GamepadId};
use lotus_proc_macros::Resource;
use super::input::Input;

/// Global resource to store all the gamepad inputs done on runtime.
#[derive(Clone, Resource)]
pub struct GamepadInput {
    pub instances: HashMap<GamepadId, GamepadInstance>
}

impl Default for GamepadInput {
    /// Returns a default GamepadInput struct.
    fn default() -> Self {
        return Self {
            instances: HashMap::new()
        };
    }
}

impl GamepadInput {
    /// Returns the first gamepad instance that is connected.
    /// 
    /// For reading purposes only.
    pub fn get_first_connected(&self) -> Option<(&GamepadId, &GamepadInstance)> {
        return self.instances.iter()
            .filter(|(_, instance)| instance.is_connected)
            .next();
    }

    /// Returns a gamepad instance that are connected by its index/id.
    /// 
    /// For reading purposes only.
    pub fn get_by_index(&self, index: usize) -> Option<(&GamepadId, &GamepadInstance)> {
        return self.instances.iter().nth(index);
    }

    /// Returns all gamepad instances that are connected.
    /// 
    /// For reading purposes only.
    pub fn get_all_connected(&self) -> Vec<(&GamepadId, &GamepadInstance)> {
        return self.instances.iter()
            .filter(|(_, instance)| instance.is_connected)
            .collect();
    }
}

/// Struct to hold a gamepad instance.
#[derive(Clone)]
pub struct GamepadInstance {
    pub pressed: HashSet<Button>,
    pub previously_pressed: HashSet<Button>,
    pub joystick_actions: HashMap<Axis, f32>,
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
            joystick_actions: HashMap::new(),
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

    /// Returns if one of the buttons inside a list is pressed.
    pub fn is_some_of_buttons_pressed(&self, buttons: Vec<Button>) -> bool {
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
    pub fn is_buttons_pressed(&self, buttons: Vec<Button>) -> bool {
        return buttons.iter().all(|element| self.pressed.contains(element));
    }

    /// Returns if a specific button is pressed.
    pub fn is_button_pressed(&self, button: Button) -> bool {
        return self.pressed.contains(&button);
    }

    /// Returns if one of the buttons inside a list is released.
    pub fn is_some_of_buttons_released(&self, buttons: Vec<Button>) -> bool {
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
    pub fn is_buttons_released(&self, buttons: Vec<Button>) -> bool {
        return buttons.iter().all(|element| self.previously_pressed.contains(element) && !self.pressed.contains(element));
    }

    /// Returns if a specific button is released.
    pub fn is_button_released(&self, button: Button) -> bool {
        return self.previously_pressed.contains(&button) && !self.pressed.contains(&button);
    }
}
