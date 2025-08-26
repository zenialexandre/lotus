use super::super::super::super::{ecs::entity::Entity};
use crate::utils::constants::cache::FIXED_UUID;

pub(crate) fn extract_id_from_entity(entity: Option<&Entity>) -> String {
    return entity.map(|e| e.0.to_string()).unwrap_or_else(|| FIXED_UUID.to_string());
}
