use std::collections::HashSet;
use lotus_proc_macros::Resource;
use winit::{event::MouseButton, keyboard::PhysicalKey, keyboard::KeyCode};

/// Global resource to store all the keyboard and mouse inputs done on runtime.
#[derive(Clone, Debug, Resource)]
pub struct Input {
    pub pressed_keys: HashSet<PhysicalKey>,
    pub pressed_mouse_buttons: HashSet<MouseButton>,
    pub previous_pressed_keys: HashSet<PhysicalKey>,
    pub previous_mouse_buttons: HashSet<MouseButton>,
    pub mouse_position: MousePosition
}

impl Input {
    /// Returns a default Input struct.
    pub fn default() -> Self {
        return Self {
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),
            previous_pressed_keys: HashSet::new(),
            previous_mouse_buttons: HashSet::new(),
            mouse_position: MousePosition::default()
        };
    }

    /// Updates the input state.
    pub fn update_hashes(&mut self) {
        self.previous_pressed_keys = self.pressed_keys.clone();
        self.previous_mouse_buttons = self.pressed_mouse_buttons.clone();
    }

    /// Returns if any keyboard key is pressed at the moment.
    pub fn is_some_key_pressed(&self) -> bool {
        return self.pressed_keys.len() > 0;
    }

    /// Returns if some of the following keyboard keys is pressed.
    pub fn is_some_of_keys_pressed(&self, keys: Vec<KeyCode>) -> bool {
        let mut is_some_of_keys_pressed: bool = false;
        
        for key in keys {
            is_some_of_keys_pressed = self.pressed_keys.contains(&PhysicalKey::Code(key));

            if is_some_of_keys_pressed {
                return is_some_of_keys_pressed;
            }
        }
        return is_some_of_keys_pressed;
    }

    /// Returns if a specific keyboard key is pressed at the moment.
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        return self.pressed_keys.contains(&PhysicalKey::Code(key));
    }

    /// Returns if any mouse button is pressed at the moment.
    pub fn is_some_mouse_button_pressed(&self) -> bool {
        return self.pressed_mouse_buttons.len() > 0;
    }

    /// Returns if a specific mouse button is pressed at the moment.
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        return self.pressed_mouse_buttons.contains(&mouse_button);
    }

    /// Returns if any keyboard key is released.
    pub fn is_some_key_released(&self) -> bool {
        return self.previous_pressed_keys.len() > 0;
    }

    /// Returns if some of the following keyboard keys is released.
    pub fn is_some_of_keys_released(&self, keys: Vec<KeyCode>) -> bool {
        let mut is_some_of_keys_released: bool = false;
        
        for key in keys {
            is_some_of_keys_released = self.previous_pressed_keys.contains(&PhysicalKey::Code(key)) && !self.pressed_keys.contains(&PhysicalKey::Code(key));

            if is_some_of_keys_released {
                return is_some_of_keys_released;
            }
        }
        return is_some_of_keys_released;
    }

    /// Returns if a specific keyboard key is released.
    pub fn is_key_released(&self, key: KeyCode) -> bool {
        return self.previous_pressed_keys.contains(&PhysicalKey::Code(key)) && !self.pressed_keys.contains(&PhysicalKey::Code(key));
    }

    /// Returns if any mouse button is released.
    pub fn is_some_mouse_button_released(&self) -> bool {
        return self.previous_mouse_buttons.len() > 0;
    }

    /// Returns if a specific mouse button is released.
    pub fn is_mouse_button_released(&self, mouse_button: MouseButton) -> bool {
        return self.previous_mouse_buttons.contains(&mouse_button) && !self.pressed_mouse_buttons.contains(&mouse_button);
    }
}

/// Struct that represents the current mouse position.
#[derive(Clone, Debug)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32
}

impl Default for MousePosition {
    fn default() -> Self {
        return Self {
            x: 0.0,
            y: 0.0
        };
    }
}
