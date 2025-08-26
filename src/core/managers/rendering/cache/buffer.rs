use std::collections::HashMap;

use cgmath::Matrix4;
use wgpu::{Buffer, BufferUsages, util::{BufferInitDescriptor, DeviceExt}};
use super::{
    utils,
    super::super::{
        rendering::manager::{RenderState, Vertex},
        super::{camera::camera2d::Camera2d, ecs::entity::Entity}
    }
};
use crate::utils::constants::cache::{VERTEX, INDEX, PROJECTION, VIEW, TRANSFORM_BUFFER};

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

pub(crate) fn get_conditional_buffer(render_state: &mut RenderState, title: &str, entity: Option<&Entity>, value: u32) -> Buffer {
    let uuid: String = utils::extract_id_from_entity(entity);
    let key: (String, String) = (uuid, title.to_string().clone());

    if let Some(buffer) = render_state.buffer_cache.find(key.clone()) {
        render_state.queue.as_ref().unwrap().write_buffer(
            &buffer,
            0,
            bytemuck::cast_slice(&[value])
        );
        return buffer.clone();
    } else {
        let buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some(title),
            contents: bytemuck::cast_slice(&[value]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.buffer_cache.cache.insert(key, buffer.clone());
        return buffer.clone();
    }
}

pub(crate) fn get_vertex_and_index_buffers(render_state: &mut RenderState, entity: Option<&Entity>, vertex_array: &[Vertex], index_array: &[u16]) -> (Buffer, Buffer) {
    let uuid: String = utils::extract_id_from_entity(entity);
    let vertex_key: (String, String) = (uuid.clone(), VERTEX.to_string());
    let index_key: (String, String) = (uuid.clone(), INDEX.to_string());

    if let (Some(vertex_buffer), Some(index_buffer)) = (
        render_state.buffer_cache.find(vertex_key.clone()),
        render_state.buffer_cache.find(index_key.clone())
    ) {
        render_state.queue.as_ref().unwrap().write_buffer(
            &vertex_buffer,
            0,
            bytemuck::cast_slice(vertex_array)
        );
        return (vertex_buffer, index_buffer);
    } else {
        let vertex_buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex_array),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST
        });
        let index_buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(index_array),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST
        });
        render_state.buffer_cache.cache.insert(vertex_key, vertex_buffer.clone());
        render_state.buffer_cache.cache.insert(index_key, index_buffer.clone());
        return (vertex_buffer.clone(), index_buffer.clone());
    }
}

pub(crate) fn get_transform_buffer(render_state: &mut RenderState, entity: Option<&Entity>, transform_matrix_unwrapped: [[f32; 4]; 4]) -> Buffer {
    let uuid: String = utils::extract_id_from_entity(entity);
    let key: (String, String) = (uuid, TRANSFORM_BUFFER.to_string());

    if let Some(transform_buffer) = render_state.buffer_cache.find(key.clone()) {
        render_state.queue.as_ref().unwrap().write_buffer(
            &transform_buffer,
            0,
            bytemuck::cast_slice(&[transform_matrix_unwrapped])
        );
        return transform_buffer.clone();
    } else {
        let transform_buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.buffer_cache.cache.insert(key, transform_buffer.clone());
        return transform_buffer.clone();
    }
}

pub(crate) fn get_projection_or_view_buffer(render_state: &mut RenderState, is_projection: bool, entity: Option<&Entity>, camera2d: &Camera2d) -> Buffer {
    let title: &str = if is_projection { PROJECTION } else { VIEW };
    let uuid: String = utils::extract_id_from_entity(entity);
    let key: (String, String) = (uuid, title.to_string().clone());
    let matrix: Matrix4<f32> = if is_projection { render_state.get_projection_matrix(camera2d) } else { camera2d.view_matrix };
    let matrix_unwrapped: [[f32; 4]; 4] = *matrix.as_ref();

    if let Some(buffer) = render_state.buffer_cache.find(key.clone()) {
        render_state.queue.as_ref().unwrap().write_buffer(
            &buffer,
            0,
            bytemuck::cast_slice(&[matrix_unwrapped])
        );
        return buffer.clone();
    } else {
        let buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some(title),
            contents: bytemuck::cast_slice(&[matrix_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.buffer_cache.cache.insert(key, buffer.clone());
        return buffer;
    }
}
