// Shader responsible for rendering 2D entities (Sprites and Shapes).

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
@group(0) @binding(2) var<uniform> is_background: u32;
@group(0) @binding(3) var<uniform> is_texture: u32;

@group(1) @binding(0) var<uniform> color: vec4<f32>;

@group(2) @binding(0) var<uniform> transform: mat4x4<f32>;
@group(2) @binding(1) var<uniform> projection: mat4x4<f32>;
@group(2) @binding(2) var<uniform> view: mat4x4<f32>;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    if (is_background == 1) {
        out.clip_position = vec4<f32>(model.position, 1.0);
    } else {
        out.clip_position = projection * view * transform * vec4<f32>(model.position, 1.0);
    }

    if (is_texture == 1) {
        out.texture_coordinates = model.texture_coordinates;   
    }
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (is_texture == 1) {
        return textureSample(texture, texture_sampler, in.texture_coordinates);
    }
    return vec4(color[0], color[1], color[2], 1.0); // Applying Blending::REPLACE.
}
