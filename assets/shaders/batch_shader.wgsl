// This is the shader that will be used in batch rendering.
// Should work as a global shader.

// Struct to define behaviour.
struct Flags {
    use_texture: u32,
    is_background: u32
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> color: vec4<f32>;
@group(0) @binding(3) var<uniform> flags: Flags;

@group(1) @binding(0) var<uniform> transform: mat4x4<f32>;
@group(1) @binding(1) var<uniform> projection: mat4x4<f32>;

// Vertex shader
// -> Create the vertices to create the object.
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = model.texture_coordinates;

    if flags.is_background == 1 {
        out.clip_position = vec4<f32>(model.position, 1.0);
    } else {
        out.clip_position = projection * transform * vec4<f32>(model.position, 1.0);
    }
    return out;
}

// Fragment shader
// -> Apply texture to the surface.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32>;

    if flags.use_texture == 1 {
        return textureSample(texture, texture_sampler, in.texture_coordinates);
    } else {
        output_color = vec4<f32>(color.rgb, 1.0);
        return output_color;
    }
}
