use cgmath::{Matrix4, Vector2};
use wgpu::{Buffer, BindGroup, BindingResource, BindGroupDescriptor, BindGroupEntry, BufferUsages, util::{BufferInitDescriptor, DeviceExt}};
use super::super::super::{
    rendering::manager::{RenderState, Vertex},
    super::{texture::{texture::Texture, sprite_sheet::SpriteSheet}, camera::camera2d::Camera2d, ecs::entity::Entity},
    super::super::utils::constants::cache::{
        FIXED_UUID,
        VERTEX,
        INDEX,
        PROJECTION,
        VIEW,
        SCREEN_SIZE_BUFFER,
        TRANSFORM_BUFFER,
        RENDERING_TYPE_BIND_GROUP,
        TEXTURE_BIND_GROUP,
        TRANSFORM_BIND_GROUP
    }
};

pub(crate) fn get_rendering_type_bind_group(render_state: &mut RenderState, entity: Option<&Entity>, rendering_type_buffer: Buffer) -> BindGroup {
    let uuid: String = extract_id_from_entity(entity);
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
    let uuid: String = extract_id_from_entity(entity);
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
    screen_size_buffer: Buffer,
    transform_buffer: Buffer,
    projection_buffer: Buffer,
    view_buffer: Buffer
) -> BindGroup {
    let uuid: String = extract_id_from_entity(entity);
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
                    resource: screen_size_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 1,
                    resource: transform_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 2,
                    resource: projection_buffer.as_entire_binding()
                },
                BindGroupEntry {
                    binding: 3,
                    resource: view_buffer.as_entire_binding()
                }
            ]
        });
        render_state.bind_group_cache.cache.insert(key, transform_bind_group.clone());
        return transform_bind_group;
    }
}

pub(crate) fn get_conditional_buffer(render_state: &mut RenderState, title: &str, entity: Option<&Entity>, value: u32) -> Buffer {
    let uuid: String = extract_id_from_entity(entity);
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
    let uuid: String = extract_id_from_entity(entity);
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

pub(crate) fn get_screen_size_buffer(render_state: &mut RenderState, entity: Option<&Entity>) -> Buffer {
    let uuid: String = extract_id_from_entity(entity);
    let key: (String, String) = (uuid, SCREEN_SIZE_BUFFER.to_string());
    let screen_size_unwrapped: [f32; 2] = *Vector2::new(
        render_state.physical_size.as_ref().unwrap().width as f32,
        render_state.physical_size.as_ref().unwrap().height as f32
    ).as_ref();

    if let Some(buffer) = render_state.buffer_cache.find(key.clone()) {
        render_state.queue.as_ref().unwrap().write_buffer(
            &buffer,
            0,
            bytemuck::cast_slice(&[screen_size_unwrapped])
        );
        return buffer.clone();
    } else {
        let buffer: Buffer = render_state.device.as_ref().unwrap().create_buffer_init(&BufferInitDescriptor {
            label: Some(SCREEN_SIZE_BUFFER),
            contents: bytemuck::cast_slice(&[screen_size_unwrapped]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });
        render_state.buffer_cache.cache.insert(key, buffer.clone());
        return buffer;
    }
}

pub(crate) fn get_transform_buffer(render_state: &mut RenderState, entity: Option<&Entity>, transform_matrix_unwrapped: [[f32; 4]; 4]) -> Buffer {
    let uuid: String = extract_id_from_entity(entity);
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
    let uuid: String = extract_id_from_entity(entity);
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

pub(crate) fn extract_id_from_entity(entity: Option<&Entity>) -> String {
    return entity.map(|e| e.0.to_string()).unwrap_or_else(|| FIXED_UUID.to_string());
}
