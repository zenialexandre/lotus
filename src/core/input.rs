use std::collections::HashSet;
use lotus_proc_macros::Resource;
use winit::{event::MouseButton, keyboard::PhysicalKey};

#[derive(Clone, Debug, Resource)]
pub struct Input {
    pub pressed_keys: HashSet<PhysicalKey>,
    pub pressed_mouse_buttons: HashSet<MouseButton>,
    pub mouse_position: (f32, f32)
}

impl Input {
    pub fn default() -> Self {
        return Self {
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),
            mouse_position: (0.0, 0.0)
        };
    }

    pub fn is_some_key_pressed(&self) -> bool {
        return self.pressed_keys.len() > 0;
    }

    pub fn is_key_pressed(&self, key: PhysicalKey) -> bool {
        return self.pressed_keys.contains(&key);
    }

    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        return self.pressed_mouse_buttons.contains(&mouse_button);
    }
}
