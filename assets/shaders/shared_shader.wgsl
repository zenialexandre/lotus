// Global shader to render shapes and sprites.
// Using the instancing pattern to increase performance.
// Using an array of textures to minimize GPU usage.

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(2) transform_col0: vec4<f32>,
    @location(3) transform_col1: vec4<f32>,
    @location(4) transform_col2: vec4<f32>,
    @location(5) transform_col3: vec4<f32>,
    @location(6) color: vec4<f32>,
    @location(10) texture_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) texture_index: u32,
};

@group(0) @binding(0) var<uniform> projection: mat4x4<f32>;
@group(0) @binding(1) var textures: texture_2d_array<f32>;
@group(0) @binding(2) var texture_sampler: sampler;

@vertex
fn vs_main(
    vertex_input: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var output: VertexOutput;
    let transform = mat4x4<f32>(
        instance.transform_col0,
        instance.transform_col1,
        instance.transform_col2,
        instance.transform_col3,
    );
    output.clip_position = projection * transform * vec4<f32>(vertex_input.position, 1.0);
    output.uv = vertex_input.uv;
    output.color = instance.color;
    output.texture_index = instance.texture_index;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    if input.texture_index == 0xFFFFFFFFu {
        return input.color;
    }
    return input.color * textureSample(
        textures,
        texture_sampler,
        input.uv,
        input.texture_index
    );
}
