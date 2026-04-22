use std::collections::HashSet;
use lotus_proc_macros::Resource;
use winit::keyboard::{KeyCode, PhysicalKey};
use super::input::Input;

/// Global resource to store all the keyboard inputs done on runtime.
#[derive(Clone, Debug, Resource)]
pub struct KeyboardInput {
    pub pressed: HashSet<PhysicalKey>,
    pub previously_pressed: HashSet<PhysicalKey>
}

impl Input for KeyboardInput {
    /// Updates the keyboard input state.
    fn update_hashes(&mut self) {
        self.previously_pressed = self.pressed.clone();
    }

    /// Returns if any keyboard key is pressed at the moment.
    fn is_some_pressed(&self) -> bool {
        return self.pressed.len() > 0;
    }

    /// Returns if any keyboard key is released.
    fn is_some_released(&self) -> bool {
        return self.previously_pressed.len() > 0;
    }
}

impl KeyboardInput {
    /// Returns a default KeyboardInput struct.
    pub fn default() -> Self {
        return Self {
            pressed: HashSet::new(),
            previously_pressed: HashSet::new(),
        };
    }

    /// Returns if some of the following keyboard keys is pressed.
    pub fn is_some_of_keys_pressed(&self, keys: Vec<KeyCode>) -> bool {
        let mut is_some_of_keys_pressed: bool = false;
        
        for key in keys {
            is_some_of_keys_pressed = self.pressed.contains(&PhysicalKey::Code(key));

            if is_some_of_keys_pressed {
                return is_some_of_keys_pressed;
            }
        }
        return is_some_of_keys_pressed;
    }

    /// Returns if a specific keyboard key is pressed at the moment.
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        return self.pressed.contains(&PhysicalKey::Code(key));
    }

    /// Returns if some of the following keyboard keys is released.
    pub fn is_some_of_keys_released(&self, keys: Vec<KeyCode>) -> bool {
        let mut is_some_of_keys_released: bool = false;
        
        for key in keys {
            is_some_of_keys_released = self.previously_pressed.contains(&PhysicalKey::Code(key)) && !self.pressed.contains(&PhysicalKey::Code(key));

            if is_some_of_keys_released {
                return is_some_of_keys_released;
            }
        }
        return is_some_of_keys_released;
    }

    /// Returns if a specific keyboard key is released.
    pub fn is_key_released(&self, key: KeyCode) -> bool {
        return self.previously_pressed.contains(&PhysicalKey::Code(key)) && !self.pressed.contains(&PhysicalKey::Code(key));
    }
}
