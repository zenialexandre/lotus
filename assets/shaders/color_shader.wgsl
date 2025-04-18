//! This is the shader that will be used to render objects with color.

// VertexInput will store the instance data.
struct VertexInput {
    // Vertex
    @location(0) position: vec3<f32>,
    
    // Instance
    @location(2) transform_row_0: vec4<f32>,
    @location(3) transform_row_1: vec4<f32>,
    @location(4) transform_row_2: vec4<f32>,
    @location(5) transform_row_3: vec4<f32>,
    @location(6) color: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
};

@group(0) @binding(0) var<uniform> projection: mat4x4<f32>;
@group(0) @binding(1) var<uniform> view: mat4x4<f32>;

// Vertex shader
// -> Create the vertices to create the object.

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let transform_from_instance = mat4x4<f32>(
        model.transform_row_0,
        model.transform_row_1,
        model.transform_row_2,
        model.transform_row_3
    );
    out.clip_position = projection * view * transform_from_instance * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}

// Fragment shader
// -> Apply color to the surface.

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
