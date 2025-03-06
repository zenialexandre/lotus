// This is the shader that will be used to render objects with texture.

// Vertex shader
// -> Create the vertices to create the object.

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};

@group(1) @binding(0) var<uniform> transform: mat4x4<f32>;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = transform * vec4<f32>(model.position, 1.0);
    out.texture_coordinates = model.texture_coordinates;
    return out;
}

// Fragment shader
// -> Apply texture to the surface.

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.texture_coordinates);
}
