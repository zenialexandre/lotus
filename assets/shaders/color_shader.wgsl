// This is the shader that will be used to render objects with color.

struct VertexInput {
    @location(0) position: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
};

@group(0) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(0) var<uniform> transform: mat4x4<f32>;
@group(1) @binding(1) var<uniform> projection: mat4x4<f32>;
@group(1) @binding(2) var<uniform> view: mat4x4<f32>;

// Vertex shader
// -> Create the vertices to create the object.

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = projection * view * transform * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
// -> Apply color to the surface.

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return color;
}
