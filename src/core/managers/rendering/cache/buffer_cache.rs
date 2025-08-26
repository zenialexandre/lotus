use wgpu::Buffer;
use std::collections::HashMap;

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
