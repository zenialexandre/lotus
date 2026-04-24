use uuid::Uuid;

/// # Struct to represent entities.
///
/// Each entity has a unique identification number generated randomly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub Uuid);

impl Entity {
    /// Returns a dummy Entity for specific cases.
    pub(crate) fn dummy() -> Self {
        return Self(Uuid::new_v4());
    }
}
