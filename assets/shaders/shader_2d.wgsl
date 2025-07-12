// Shader responsible for rendering 2D entities (shapes, textures and text).

//const SHAPE: u32 = 0u;
const BACKGROUND: u32 = 1u;
const TEXTURE: u32 = 2u;
const TEXT: u32 = 3u;

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

@group(1) @binding(0) var<uniform> color: vec4<f32>;

@group(2) @binding(0) var<uniform> transform: mat4x4<f32>;
@group(2) @binding(1) var<uniform> projection: mat4x4<f32>;
@group(2) @binding(2) var<uniform> view: mat4x4<f32>;

@group(3) @binding(0) var<uniform> rendering_type: u32;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    if (rendering_type == BACKGROUND) {
        out.clip_position = vec4<f32>(model.position, 1.0);
    } else {
        out.clip_position = projection * view * transform * vec4<f32>(model.position, 1.0);
    }

    if (rendering_type == BACKGROUND || rendering_type == TEXTURE) {
        out.texture_coordinates = model.texture_coordinates;   
    }
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (rendering_type == BACKGROUND || rendering_type == TEXTURE) {
        return textureSample(texture, texture_sampler, in.texture_coordinates);
    }
    return vec4(color.rgb, 1.0); // Applying Blending::REPLACE.
}
