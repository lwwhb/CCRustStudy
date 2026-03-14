use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3, Vec4};

// ============================================================================
// 着色器数据结构
// ============================================================================

/// 顶点数据
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn new(
        position: [f32; 3],
        normal: [f32; 3],
        tex_coords: [f32; 2],
        color: [f32; 4],
    ) -> Self {
        Self {
            position,
            normal,
            tex_coords,
            color,
        }
    }
}

/// Uniform 数据（用于传递给着色器的常量）
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    /// 视图投影矩阵
    pub view_proj: [[f32; 4]; 4],
    /// 模型矩阵
    pub model: [[f32; 4]; 4],
    /// 时间（用于动画）
    pub time: f32,
    /// 填充字段（对齐到 16 字节）
    pub _padding: [f32; 3],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            model: Mat4::IDENTITY.to_cols_array_2d(),
            time: 0.0,
            _padding: [0.0; 3],
        }
    }

    pub fn update_view_proj(&mut self, view_proj: Mat4) {
        self.view_proj = view_proj.to_cols_array_2d();
    }

    pub fn update_model(&mut self, model: Mat4) {
        self.model = model.to_cols_array_2d();
    }

    pub fn update_time(&mut self, time: f32) {
        self.time = time;
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self::new()
    }
}

/// 光照数据
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LightUniforms {
    /// 光源位置
    pub position: [f32; 3],
    pub _padding1: f32,
    /// 光源颜色
    pub color: [f32; 3],
    pub _padding2: f32,
    /// 环境光强度
    pub ambient_strength: f32,
    /// 漫反射强度
    pub diffuse_strength: f32,
    /// 镜面反射强度
    pub specular_strength: f32,
    /// 镜面反射指数
    pub shininess: f32,
}

impl LightUniforms {
    pub fn new() -> Self {
        Self {
            position: [2.0, 2.0, 2.0],
            _padding1: 0.0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0.0,
            ambient_strength: 0.1,
            diffuse_strength: 0.8,
            specular_strength: 0.5,
            shininess: 32.0,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position.to_array();
    }

    pub fn set_color(&mut self, color: Vec3) {
        self.color = color.to_array();
    }
}

impl Default for LightUniforms {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 着色器代码（WGSL）
// ============================================================================

/// 基础顶点着色器
pub const BASIC_VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let world_position = uniforms.model * vec4<f32>(input.position, 1.0);
    output.clip_position = uniforms.view_proj * world_position;
    output.world_position = world_position.xyz;
    output.color = input.color;
    output.tex_coords = input.tex_coords;
    output.normal = (uniforms.model * vec4<f32>(input.normal, 0.0)).xyz;

    return output;
}
"#;

/// 基础片段着色器（颜色）
pub const BASIC_FRAGMENT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;

/// Phong 光照片段着色器
pub const PHONG_FRAGMENT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
}

struct LightUniforms {
    position: vec3<f32>,
    color: vec3<f32>,
    ambient_strength: f32,
    diffuse_strength: f32,
    specular_strength: f32,
    shininess: f32,
}

@group(1) @binding(0)
var<uniform> light: LightUniforms;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let light_dir = normalize(light.position - input.world_position);
    let view_dir = normalize(-input.world_position);

    // 环境光
    let ambient = light.ambient_strength * light.color;

    // 漫反射
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = light.diffuse_strength * diff * light.color;

    // 镜面反射
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), light.shininess);
    let specular = light.specular_strength * spec * light.color;

    let result = (ambient + diffuse + specular) * input.color.rgb;
    return vec4<f32>(result, input.color.a);
}
"#;

/// 纹理采样片段着色器
pub const TEXTURE_FRAGMENT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, input.tex_coords);
    return tex_color * input.color;
}
"#;

/// 动画着色器（基于时间的颜色变化）
pub const ANIMATED_FRAGMENT_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_position: vec3<f32>,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let r = (sin(uniforms.time) + 1.0) / 2.0;
    let g = (sin(uniforms.time + 2.094) + 1.0) / 2.0;
    let b = (sin(uniforms.time + 4.189) + 1.0) / 2.0;

    return vec4<f32>(r, g, b, 1.0) * input.color;
}
"#;

// ============================================================================
// 着色器管理
// ============================================================================

/// 着色器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Basic,
    Phong,
    Texture,
    Animated,
}

/// 着色器管理器
pub struct ShaderManager {
    shaders: Vec<(ShaderType, String, String)>,
}

impl ShaderManager {
    pub fn new() -> Self {
        let mut manager = Self {
            shaders: Vec::new(),
        };

        // 注册内置着色器
        manager.register(
            ShaderType::Basic,
            BASIC_VERTEX_SHADER.to_string(),
            BASIC_FRAGMENT_SHADER.to_string(),
        );

        manager.register(
            ShaderType::Phong,
            BASIC_VERTEX_SHADER.to_string(),
            PHONG_FRAGMENT_SHADER.to_string(),
        );

        manager.register(
            ShaderType::Texture,
            BASIC_VERTEX_SHADER.to_string(),
            TEXTURE_FRAGMENT_SHADER.to_string(),
        );

        manager.register(
            ShaderType::Animated,
            BASIC_VERTEX_SHADER.to_string(),
            ANIMATED_FRAGMENT_SHADER.to_string(),
        );

        manager
    }

    pub fn register(&mut self, shader_type: ShaderType, vertex: String, fragment: String) {
        self.shaders.push((shader_type, vertex, fragment));
    }

    pub fn get(&self, shader_type: ShaderType) -> Option<(&str, &str)> {
        self.shaders
            .iter()
            .find(|(t, _, _)| *t == shader_type)
            .map(|(_, v, f)| (v.as_str(), f.as_str()))
    }

    pub fn list_shaders(&self) -> Vec<ShaderType> {
        self.shaders.iter().map(|(t, _, _)| *t).collect()
    }
}

impl Default for ShaderManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
    println!("=== 着色器编程演示 ===\n");

    // 创建着色器管理器
    let manager = ShaderManager::new();

    println!("可用的着色器类型:");
    for shader_type in manager.list_shaders() {
        println!("  - {:?}", shader_type);
    }

    println!("\n=== Uniform 数据示例 ===\n");

    // 创建 Uniform 数据
    let mut uniforms = Uniforms::new();
    uniforms.update_time(1.5);
    println!("Uniforms: {:?}", uniforms);

    // 创建光照数据
    let mut light = LightUniforms::new();
    light.set_position(Vec3::new(5.0, 5.0, 5.0));
    light.set_color(Vec3::new(1.0, 0.9, 0.8));
    println!("\nLight Uniforms: {:?}", light);

    println!("\n=== 顶点数据示例 ===\n");

    // 创建顶点
    let vertex = Vertex::new(
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.5, 0.5],
        [1.0, 0.0, 0.0, 1.0],
    );
    println!("Vertex: {:?}", vertex);

    println!("\n着色器演示完成！");
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::new(
            [1.0, 2.0, 3.0],
            [0.0, 1.0, 0.0],
            [0.5, 0.5],
            [1.0, 1.0, 1.0, 1.0],
        );

        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.normal, [0.0, 1.0, 0.0]);
        assert_eq!(vertex.tex_coords, [0.5, 0.5]);
    }

    #[test]
    fn test_uniforms_creation() {
        let uniforms = Uniforms::new();
        assert_eq!(uniforms.time, 0.0);
    }

    #[test]
    fn test_uniforms_update() {
        let mut uniforms = Uniforms::new();
        uniforms.update_time(5.0);
        assert_eq!(uniforms.time, 5.0);
    }

    #[test]
    fn test_light_uniforms() {
        let mut light = LightUniforms::new();
        light.set_position(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(light.position, [1.0, 2.0, 3.0]);

        light.set_color(Vec3::new(0.5, 0.5, 0.5));
        assert_eq!(light.color, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_shader_manager() {
        let manager = ShaderManager::new();
        let shaders = manager.list_shaders();
        assert_eq!(shaders.len(), 4);
        assert!(shaders.contains(&ShaderType::Basic));
        assert!(shaders.contains(&ShaderType::Phong));
    }

    #[test]
    fn test_get_shader() {
        let manager = ShaderManager::new();
        let (vertex, fragment) = manager.get(ShaderType::Basic).unwrap();
        assert!(vertex.contains("vs_main"));
        assert!(fragment.contains("fs_main"));
    }

    #[test]
    fn test_shader_code_validity() {
        // 验证着色器代码包含必要的关键字
        assert!(BASIC_VERTEX_SHADER.contains("@vertex"));
        assert!(BASIC_FRAGMENT_SHADER.contains("@fragment"));
        assert!(PHONG_FRAGMENT_SHADER.contains("ambient"));
        assert!(PHONG_FRAGMENT_SHADER.contains("diffuse"));
        assert!(PHONG_FRAGMENT_SHADER.contains("specular"));
    }

    #[test]
    fn test_vertex_size() {
        // 验证顶点结构体大小符合预期
        assert_eq!(std::mem::size_of::<Vertex>(), 48);
    }

    #[test]
    fn test_uniforms_alignment() {
        // 验证 Uniforms 结构体对齐
        assert_eq!(std::mem::size_of::<Uniforms>(), 144);
    }

    #[test]
    fn test_light_uniforms_alignment() {
        // 验证 LightUniforms 结构体对齐
        assert_eq!(std::mem::size_of::<LightUniforms>(), 48);
    }
}
