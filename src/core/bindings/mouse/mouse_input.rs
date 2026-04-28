use super::{super::input::Input, mouse_position::MousePosition};
use lotus_proc_macros::Resource;
use std::collections::HashSet;
use winit::event::MouseButton;

/// Global resource to store all the mouse inputs done on runtime.
#[derive(Clone, Debug, Resource)]
pub struct MouseInput {
    pub pressed: HashSet<MouseButton>,
    pub previously_pressed: HashSet<MouseButton>,
    pub mouse_position: MousePosition,
}

impl Input for MouseInput {
    /// Updates the mouse input state.
    fn update_hashes(&mut self) {
        self.previously_pressed = self.pressed.clone();
    }

    /// Returns if any mouse button is pressed at the moment.
    fn is_some_pressed(&self) -> bool {
        return self.pressed.len() > 0;
    }

    /// Returns if any mouse button is released.
    fn is_some_released(&self) -> bool {
        return self.previously_pressed.len() > 0;
    }
}

impl Default for MouseInput {
    /// Returns a default MouseInput struct.
    fn default() -> Self {
        return Self {
            pressed: HashSet::new(),
            previously_pressed: HashSet::new(),
            mouse_position: MousePosition::default(),
        };
    }
}

impl MouseInput {
    /// Returns if a specific mouse button is pressed at the moment.
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        return self.pressed.contains(&mouse_button);
    }

    /// Returns if a specific mouse button is released.
    pub fn is_mouse_button_released(&self, mouse_button: MouseButton) -> bool {
        return self.previously_pressed.contains(&mouse_button)
            && !self.pressed.contains(&mouse_button);
    }
}
