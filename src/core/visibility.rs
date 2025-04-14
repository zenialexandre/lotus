use lotus_proc_macros::Component;

/// Struct to represent an entity visibility on the world.
#[derive(Clone, Component)]
pub struct Visibility {
    pub value: bool
}

impl Default for Visibility {
    fn default() -> Self {
        return Self {
            value: true
        };
    }
}
