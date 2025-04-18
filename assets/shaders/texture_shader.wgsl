//! This is the shader that will be used to render objects with texture.

// VertexInput will store the instance data.
struct VertexInput {
    // Vertex
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
    
    // Instance
    @location(2) transform_row_0: vec4<f32>,
    @location(3) transform_row_1: vec4<f32>,
    @location(4) transform_row_2: vec4<f32>,
    @location(5) transform_row_3: vec4<f32>,
    @location(6) color: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> is_background: u32;

@group(1) @binding(0) var<uniform> projection: mat4x4<f32>;
@group(1) @binding(1) var<uniform> view: mat4x4<f32>;

// Vertex shader
// -> Create the vertices to create the object.

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    if (is_background == 1) {
        out.clip_position = vec4<f32>(model.position, 1.0);
    } else {
        let transform_from_instance = mat4x4<f32>(
            model.transform_row_0,
            model.transform_row_1,
            model.transform_row_2,
            model.transform_row_3
        );
        out.clip_position = projection * view * transform_from_instance * vec4<f32>(model.position, 1.0);
    }
    out.texture_coordinates = model.texture_coordinates;
    return out;
}

// Fragment shader
// -> Apply texture to the surface.

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.texture_coordinates);
}
