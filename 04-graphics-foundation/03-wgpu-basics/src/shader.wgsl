// 顶点着色器
@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(position, 1.0);
    out.color = color;
    return out;
}

// 顶点着色器输出
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

// 片段着色器
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
