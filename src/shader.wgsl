// Vertex shader

[[block]]
struct Camera {
    view_pos: vec4<f32>;
    view_proj: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> camera: Camera;


struct VertexInput {
    [[location(0)]] position: vec3<f32>;
};
struct InstanceInput {
    [[location(10)]] model_matrix_0: vec4<f32>;
    [[location(11)]] model_matrix_1: vec4<f32>;
    [[location(12)]] model_matrix_2: vec4<f32>;
    [[location(13)]] model_matrix_3: vec4<f32>;
		[[location(14)]] color: vec4<f32>; //visual appearance

};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
		[[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
		out.color = instance.color; 
    return out;
}

// Fragment shader


[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}