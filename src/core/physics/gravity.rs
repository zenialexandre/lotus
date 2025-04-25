use lotus_proc_macros::Resource;

/// Struct to represent the gravity in our world.
/// It starts disabled and with value equal to 9.8 (standard gravity of Earth).
///
/// Gravity will only be applied to entities with the 'RigidBody' and 'Velocity' components.
#[derive(Clone, Resource)]
pub struct Gravity {
    pub enabled: bool,
    pub value: f32
}

impl Gravity {
    /// Enables the gravity in our world.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the gravity in our world.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Enables the gravity in our world with a custom gravity value.
    pub fn enable_with_custom_gravity(&mut self, value: f32) {
        self.enabled = true;
        self.value = value;
    }
}

impl Default for Gravity {
    fn default() -> Self {
        return Self {
            enabled: false,
            value: 9.8
        };
    }
}
