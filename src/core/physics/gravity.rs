use lotus_proc_macros::Component;

/// Struct to represent the gravity in our world.
///
/// It starts with value equal to 9.8 (standard gravity of Earth).
///
/// Gravity will only be applied to entities with the 'RigidBody' and 'Velocity' components.
#[derive(Clone, Component)]
pub struct Gravity {
    pub value: f32
}

impl Gravity {
    /// Create a custom Gravity struct.
    pub fn new(value: f32) -> Self {
        return Self {
            value
        };
    }
}

impl Default for Gravity {
    fn default() -> Self {
        return Self {
            value: 9.8
        };
    }
}
