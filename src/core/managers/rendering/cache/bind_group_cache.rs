use wgpu::BindGroup;
use std::collections::HashMap;

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
