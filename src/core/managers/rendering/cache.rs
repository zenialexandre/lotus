use wgpu::*;
use std::collections::HashMap;
use super::super::super::ecs::entity::Entity;

pub const FIXED_UUID: &str = "fixed_uuid";
pub const VERTEX: &str = "vertex";
pub const INDEX: &str = "index";
pub const IS_BACKGROUND: &str = "is_background";
pub const IS_TEXTURE: &str = "is_texture";
pub const PROJECTION: &str = "projection";
pub const VIEW: &str = "view";
pub const TRANSFORM_BUFFER: &str = "transform_buffer";
pub const TRANSFORM_BIND_GROUP: &str = "transform_bind_group";
pub const COLOR_BUFFER: &str = "color_buffer";
pub const COLOR_BIND_GROUP: &str = "color_bind_group";
pub const TEXTURE_BIND_GROUP: &str = "texture_bind_group";

/// Struct for caching buffers.
pub struct BufferCache {
    pub cache: HashMap<(String, String), Buffer>
}

impl BufferCache {
    /// Create a new caching for buffers.
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new()
        };
    }

    /// Find the cached buffer by the key.
    pub fn find(&self, key: (String, String)) -> Option<Buffer> {
        return self.cache.get(&key).cloned();
    }
}

/// Struct for caching bind groups.
pub struct BindGroupCache {
    pub cache: HashMap<(String, String), BindGroup>
}

impl BindGroupCache {
    /// Create a new caching for bind groups.
    pub fn new() -> Self {
        return Self {
            cache: HashMap::new()
        };
    }

    /// Find the cached bind group by the key.
    pub fn find(&self, key: (String, String)) -> Option<BindGroup> {
        return self.cache.get(&key).cloned();
    }
}

pub(crate) fn extract_id_from_entity(entity: Option<&Entity>) -> String {
    return entity.map(|e| e.0.to_string()).unwrap_or_else(|| FIXED_UUID.to_string());
}
