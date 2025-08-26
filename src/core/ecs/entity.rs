use uuid::Uuid;

/// # Struct to represent the entities.
///
/// Each entity has a unique ID generated randomly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub Uuid);
