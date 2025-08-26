// Shader responsible for rendering 2D entities (shapes, textures and texts).

const SHAPE: u32 = 0u;
const BACKGROUND: u32 = 1u;
const TEXTURE: u32 = 2u;
const TEXT: u32 = 3u;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv_coordinates: vec2<f32>,
    @location(2) text_pixelated_position: vec2<f32>,
    @location(3) text_uv_coordinates: vec2<f32>,
    @location(4) color: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_coordinates: vec2<f32>,
    @location(1) color: vec4<f32>
};

@group(0) @binding(0) var<uniform> rendering_type: u32;

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@group(2) @binding(0) var<uniform> screen_size: vec2<f32>;
@group(2) @binding(1) var<uniform> transform: mat4x4<f32>;
@group(2) @binding(2) var<uniform> projection: mat4x4<f32>;
@group(2) @binding(3) var<uniform> view: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    if (rendering_type == BACKGROUND) {
        out.clip_position = vec4<f32>(in.position, 1.0);
    } else if (rendering_type == TEXTURE || rendering_type == SHAPE) {
        out.clip_position = projection * view * transform * vec4<f32>(in.position, 1.0);
        out.color = in.color;
    } else if (rendering_type == TEXT) {
        let normalized_position = from_pixel_to_normalized(in.text_pixelated_position, screen_size);
        out.clip_position = projection * view * transform * vec4<f32>(normalized_position, 0.0, 1.0);
        out.uv_coordinates = in.text_uv_coordinates;
        out.color = in.color;
    }

    if (rendering_type == BACKGROUND || rendering_type == TEXTURE) {
        out.uv_coordinates = in.uv_coordinates;
    }
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (rendering_type == BACKGROUND || rendering_type == TEXTURE) {
        return textureSample(texture, texture_sampler, in.uv_coordinates);
    } else if (rendering_type == SHAPE) {
        return vec4(in.color.rgb, 1.0);
    } else {
        var alpha: f32 = textureSample(texture, texture_sampler, in.uv_coordinates).r;        
        return vec4<f32>(in.color.rgb, in.color.a * alpha);
    }
}

fn from_pixel_to_normalized(original_position: vec2<f32>, screen_size: vec2<f32>) -> vec2<f32> {
    let aspect_ratio: f32 = screen_size.x / screen_size.y;

    return vec2<f32>(
        original_position.x / screen_size.x * 2.0 * aspect_ratio - aspect_ratio,
        -(original_position.y / screen_size.y * 2.0 - 1.0)
    );
}
