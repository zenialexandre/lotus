use std::collections::HashMap;
use wgpu::{Buffer, BindGroup, BindingResource, BindGroupDescriptor, BindGroupEntry};
use super::{
    utils,
    super::super::{
        rendering::manager::RenderState,
        super::{texture::{texture::Texture, sprite_sheet::SpriteSheet}, ecs::entity::Entity}
    }
};
use crate::utils::constants::cache::{RENDERING_TYPE_BIND_GROUP, TEXTURE_BIND_GROUP, TRANSFORM_BIND_GROUP};

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

pub(crate) fn get_rendering_type_bind_group(render_state: &mut RenderState, entity: Option<&Entity>, rendering_type_buffer: Buffer) -> BindGroup {
    let uuid: String = utils::extract_id_from_entity(entity);
    let key: (String, String) = (uuid, RENDERING_TYPE_BIND_GROUP.to_string());

    if let Some(rendering_type_bind_group) = render_state.bind_group_cache.find(key.clone()) {
        return rendering_type_bind_group.clone();
    } else {
        let rendering_type_bind_group: BindGroup = render_state.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
            label: Some("Rendering Type Bind Group"),
            layout: &render_state.rendering_type_bind_group_layout.as_ref().unwrap(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: rendering_type_buffer.as_entire_binding()
                }
            ]
        });
        render_state.bind_group_cache.cache.insert(key, rendering_type_bind_group.clone());
        return rendering_type_bind_group;
    }
}

pub(crate) fn get_texture_bind_group(
    render_state: &mut RenderState,
    entity: Option<&Entity>,
    texture: &Texture,
    sprite_sheet: Option<&SpriteSheet>
) -> BindGroup {
    let uuid: String = utils::extract_id_from_entity(entity);
    let key: (String, String) = (
        uuid,
        if sprite_sheet.is_none() { TEXTURE_BIND_GROUP.to_string() } else { sprite_sheet.unwrap().path.to_string() }
    );

    if let Some(texture_bind_group) = render_state.bind_group_cache.find(key.clone()) {
        return texture_bind_group.clone();
    } else {
        let texture_bind_group: BindGroup = render_state.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &render_state.texture_bind_group_layout.as_ref().unwrap(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.texture_view)
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler)
                }
            ]
        });
        render_state.bind_group_cache.cache.insert(key, texture_bind_group.clone());
        return texture_bind_group.clone();
    }
}

pub(crate) fn get_transform_bind_group(
    render_state: &mut RenderState,
    entity: Option<&Entity>,
    transform_buffer: Buffer,
    projection_buffer: Buffer,
    view_buffer: Buffer
) -> BindGroup {
    let uuid: String = utils::extract_id_from_entity(entity);
    let key: (String, String) = (uuid, TRANSFORM_BIND_GROUP.to_string());

    if let Some(transform_bind_group) = render_state.bind_group_cache.find(key.clone()) {
        return transform_bind_group.clone();
    } else {
        let transform_bind_group: BindGroup = render_state.device.as_ref().unwrap().create_bind_group(&BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &render_state.transform_bind_group_layout.as_ref().unwrap(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 1,
                    resource: projection_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 2,
                    resource: view_buffer.as_entire_binding()
                }
            ]
        });
        render_state.bind_group_cache.cache.insert(key, transform_bind_group.clone());
        return transform_bind_group;
    }
}
