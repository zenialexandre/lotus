use lotus_proc_macros::Component;

/// Struct to represent an entity visibility on the world.
#[derive(Clone, Component)]
pub struct Visibility(pub bool);

impl Visibility {
    /// Creates a new visibility struct.
    pub fn new(value: bool) -> Self {
        return Self(value);
    }
}

impl Default for Visibility {
    fn default() -> Self {
        return Self(true);
    }
}
