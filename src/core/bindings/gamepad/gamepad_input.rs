use std::collections::HashMap;
use gilrs::GamepadId;
use lotus_proc_macros::Resource;
use super::gamepad_instance::GamepadInstance;

/// Global resource to store all the gamepad inputs done on runtime.
#[derive(Clone, Resource)]
pub struct GamepadInput {
    pub instances: HashMap<GamepadId, GamepadInstance>,
    pub left_joystick_deadzone: f32,
    pub right_joystick_deadzone: f32
}

impl Default for GamepadInput {
    /// Returns a default GamepadInput struct.
    fn default() -> Self {
        return Self {
            instances: HashMap::new(),
            left_joystick_deadzone: 0.0,
            right_joystick_deadzone: 0.0
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

    /// Sets the deadzone for the left joystick.
    pub fn set_left_joystick_deadzone(&mut self, deadzone: f32) {
        self.left_joystick_deadzone = deadzone;
    }

    /// Sets the deadzone for the right joystick.
    pub fn set_right_joystick_deadzone(&mut self, deadzone: f32) {
        self.right_joystick_deadzone = deadzone;
    }
}
