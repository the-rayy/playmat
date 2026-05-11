// src/renderer/scene.wgsl

struct Uniforms {
    mvp: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> u: Uniforms;

struct VertIn {
    @location(0) position: vec3<f32>,
    @location(1) color:    vec3<f32>,
}

struct VertOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0)       color:    vec3<f32>,
}

@vertex
fn vs_main(in: VertIn) -> VertOut {
    var out: VertOut;
    out.clip_pos = u.mvp * vec4<f32>(in.position, 1.0);
    out.color    = in.color;
    return out;
}

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
