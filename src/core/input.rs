use std::collections::HashSet;
use lotus_proc_macros::Resource;
use winit::{event::MouseButton, keyboard::PhysicalKey};

/// Global resource to store all the keyboard and mouse inputs done on runtime.
#[derive(Clone, Debug, Resource)]
pub struct Input {
    pub pressed_keys: HashSet<PhysicalKey>,
    pub pressed_mouse_buttons: HashSet<MouseButton>,
    pub previous_pressed_keys: HashSet<PhysicalKey>,
    pub previous_mouse_buttons: HashSet<MouseButton>,
    pub mouse_position: (f32, f32)
}

impl Input {
    /// Returns a default Input struct.
    pub fn default() -> Self {
        return Self {
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),
            previous_pressed_keys: HashSet::new(),
            previous_mouse_buttons: HashSet::new(),
            mouse_position: (0.0, 0.0)
        };
    }

    /// Updates the input state.
    pub fn update_hashes(&mut self) {
        self.previous_pressed_keys = self.pressed_keys.clone();
        self.previous_mouse_buttons = self.pressed_mouse_buttons.clone();
    }

    /// Returns if any key is pressed at the moment.
    pub fn is_some_key_pressed(&self) -> bool {
        return self.pressed_keys.len() > 0;
    }

    /// Returns if a specific keyboard key is pressed at the moment.
    pub fn is_key_pressed(&self, key: PhysicalKey) -> bool {
        return self.pressed_keys.contains(&key);
    }

    /// Returns if a specific mouse button is pressed at the moment.
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        return self.pressed_mouse_buttons.contains(&mouse_button);
    }

    /// Returns if a specific keyboard key is released.
    pub fn is_key_released(&self, key: PhysicalKey) -> bool {
        return self.previous_pressed_keys.contains(&key) && !self.pressed_keys.contains(&key);
    }

    /// Returns if a specific mouse button is released.
    pub fn is_mouse_button_released(&self, mouse_button: MouseButton) -> bool {
        return self.previous_mouse_buttons.contains(&mouse_button) && !self.pressed_mouse_buttons.contains(&mouse_button);
    }
}
