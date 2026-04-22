use std::collections::HashSet;
use gilrs::Button;
use lotus_proc_macros::Resource;
use super::input::Input;

#[derive(Clone, Debug, Resource)]
pub struct GamepadInput {
    pub pressed: HashSet<Button>,
    pub previously_pressed: HashSet<Button>
}

impl Input for GamepadInput {
    fn update_hashes(&mut self) {
        self.previously_pressed = self.pressed.clone();
    }

    fn is_some_pressed(&self) -> bool {
        return self.pressed.len() > 0;
    }

    fn is_some_released(&self) -> bool {
        return self.previously_pressed.len() > 0;
    }
}

impl GamepadInput {
    /// Returns a default GamepadInput struct.
    pub fn default() -> Self {
        return Self {
            pressed: HashSet::new(),
            previously_pressed: HashSet::new()
        };
    }
}
