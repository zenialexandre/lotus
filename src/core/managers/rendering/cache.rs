use wgpu::*;
use std::collections::HashMap;
use super::super::super::ecs::entity::Entity;

pub(crate) const FIXED_UUID: &str = "fixed_uuid";
pub(crate) const VERTEX: &str = "vertex";
pub(crate) const INDEX: &str = "index";
pub(crate) const PROJECTION: &str = "projection";
pub(crate) const VIEW: &str = "view";
pub(crate) const COLOR_BUFFER: &str = "color_buffer";
pub(crate) const TRANSFORM_BUFFER: &str = "transform_buffer";
pub(crate) const RENDERING_TYPE_BUFFER: &str = "rendering_type_buffer";
pub(crate) const TEXTURE_BIND_GROUP: &str = "texture_bind_group";
pub(crate) const COLOR_BIND_GROUP: &str = "color_bind_group";
pub(crate) const TRANSFORM_BIND_GROUP: &str = "transform_bind_group";
pub(crate) const RENDERING_TYPE_BIND_GROUP: &str = "rendering_type_bind_group";

/// Struct for caching Buffers.
pub struct BufferCache {
    pub cache: HashMap<(String, String), Buffer>
}

impl BufferCache {
    /// Create a new caching for Buffers.
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new()
        };
    }

    /// Find the cached Buffer by the key.
    pub fn find(&self, key: (String, String)) -> Option<Buffer> {
        return self.cache.get(&key).cloned();
    }

    /// Clean the cached Buffer data related to a certain entity if its found.
    pub fn clean(&mut self, entity_id: String) {
        self.cache.retain(|(entity_id_from_map, _), _| entity_id_from_map != &entity_id);
    }
}

/// Struct for caching Bind Groups.
pub struct BindGroupCache {
    pub cache: HashMap<(String, String), BindGroup>
}

impl BindGroupCache {
    /// Create a new caching for Bind Groups.
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new()
        };
    }

    /// Find the cached Bind Group by the key.
    pub fn find(&self, key: (String, String)) -> Option<BindGroup> {
        return self.cache.get(&key).cloned();
    }

    /// Clean the cached Bind Group data related to a certain entity if its found.
    pub fn clean(&mut self, entity_id: String) {
        self.cache.retain(|(entity_id_from_map, _), _| entity_id_from_map != &entity_id);
    }
}

pub(crate) fn extract_id_from_entity(entity: Option<&Entity>) -> String {
    return entity.map(|e| e.0.to_string()).unwrap_or_else(|| FIXED_UUID.to_string());
}
