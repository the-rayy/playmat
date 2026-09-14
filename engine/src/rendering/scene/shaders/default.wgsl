struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    out.color = model.color;

    var pos = model_matrix * vec4<f32>(model.position, 1.0);

    // Debug: push the model toward the top-left corner in clip space.
    // Clip space ranges from -1 (left/bottom) to +1 (right/top) after perspective divide,
    // so we shift x left and y up by shrinking the model and offsetting it.
    let corner_scale = 1.0;   // shrink so it fits in the corner
    let corner_offset = vec2<f32>(-0.8, 0.8); // top-left corner in NDC

    pos.x = pos.x * corner_scale + corner_offset.x * pos.w;
    pos.y = pos.y * corner_scale + corner_offset.y * pos.w;
    pos.z = pos.z + 0.5;

    out.clip_position = pos;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

