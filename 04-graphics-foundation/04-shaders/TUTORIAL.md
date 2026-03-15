# 模块 4.4：着色器编程（WGSL）- 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 深入理解 WGSL 语法
2. 掌握顶点和片段着色器
3. 学习纹理映射
4. 实现光照模型
5. 使用 Uniform 缓冲区

## 🎯 为什么需要着色器？

### CPU vs GPU

**CPU 渲染（慢）**：
```rust
// CPU 逐像素计算颜色
for y in 0..height {
    for x in 0..width {
        let color = calculate_pixel_color(x, y);
        set_pixel(x, y, color);
    }
}

问题：
- 串行执行
- 1920x1080 = 2,073,600 像素
- 60 FPS = 124,416,000 次计算/秒
- CPU 无法承受
```

**GPU 渲染（快）**：
```wgsl
// GPU 并行计算所有像素
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return calculate_pixel_color();
}

优势：
- 并行执行
- 数千个核心同时工作
- 专为图形设计
- 轻松达到 60 FPS
```

### 着色器的作用

```
渲染管线：

顶点数据
    ↓
顶点着色器（Vertex Shader）
- 变换顶点位置
- 计算顶点属性
    ↓
光栅化
- 生成片段（像素候选）
    ↓
片段着色器（Fragment Shader）
- 计算像素颜色
- 应用纹理和光照
    ↓
输出颜色
```

## 📖 核心概念详解

### 1. WGSL 基础语法

#### 变量和类型

```wgsl
// 变量声明
var x: f32 = 1.0;        // 可变变量
let y: i32 = 2;          // 不可变变量
const PI: f32 = 3.14159; // 编译时常量

// 基础类型
var a: bool = true;
var b: i32 = -42;        // 有符号整数
var c: u32 = 42;         // 无符号整数
var d: f32 = 3.14;       // 32位浮点
var e: f64 = 3.14159;    // 64位浮点（不常用）

// 向量类型
var v2: vec2<f32> = vec2<f32>(1.0, 2.0);
var v3: vec3<f32> = vec3<f32>(1.0, 2.0, 3.0);
var v4: vec4<f32> = vec4<f32>(1.0, 2.0, 3.0, 4.0);

// 向量构造简写
var v2 = vec2(1.0, 2.0);
var v3 = vec3(1.0, 2.0, 3.0);
var v4 = vec4(1.0, 2.0, 3.0, 4.0);

// 向量分量访问
var r = v4.x;  // 或 v4.r
var g = v4.y;  // 或 v4.g
var b = v4.z;  // 或 v4.b
var a = v4.w;  // 或 v4.a

// Swizzling
var xy = v4.xy;
var bgr = v4.bgr;
var rgba = v4.rgba;

// 矩阵类型
var m2x2: mat2x2<f32>;  // 2x2 矩阵
var m3x3: mat3x3<f32>;  // 3x3 矩阵
var m4x4: mat4x4<f32>;  // 4x4 矩阵
```

#### 函数

```wgsl
// 函数定义
fn add(a: f32, b: f32) -> f32 {
    return a + b;
}

// 无返回值
fn print_value(x: f32) {
    // 无 return
}

// 多个返回值（使用结构体）
struct Result {
    sum: f32,
    product: f32,
}

fn calculate(a: f32, b: f32) -> Result {
    return Result(a + b, a * b);
}
```

#### 控制流

```wgsl
// if-else
if x > 0.0 {
    // ...
} else if x < 0.0 {
    // ...
} else {
    // ...
}

// for 循环
for (var i = 0; i < 10; i++) {
    // ...
}

// while 循环
while x < 10.0 {
    x += 1.0;
}

// loop（无限循环）
loop {
    if condition {
        break;
    }
    continuing {
        // 每次迭代结束执行
    }
}
```

#### 内置函数

```wgsl
// 数学函数
let s = sin(angle);
let c = cos(angle);
let t = tan(angle);
let sq = sqrt(x);
let p = pow(x, y);
let a = abs(x);
let mn = min(a, b);
let mx = max(a, b);
let cl = clamp(x, min_val, max_val);

// 向量函数
let len = length(v);           // 长度
let norm = normalize(v);       // 归一化
let d = dot(v1, v2);          // 点积
let cr = cross(v1, v2);       // 叉积（仅3D）
let dist = distance(p1, p2);  // 距离
let refl = reflect(v, n);     // 反射

// 插值函数
let mixed = mix(a, b, t);     // 线性插值
let smooth = smoothstep(edge0, edge1, x);

// 纹理采样
let color = textureSample(tex, samp, uv);
```

### 2. 顶点着色器

顶点着色器处理每个顶点。

#### 基础顶点着色器

```wgsl
// 顶点输入
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

// 顶点输出
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

// 顶点着色器
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // 变换位置到裁剪空间
    out.clip_position = vec4<f32>(in.position, 1.0);
    
    // 传递颜色到片段着色器
    out.color = in.color;
    
    return out;
}
```

#### 使用 Uniform 变换

```wgsl
// Uniform 缓冲区
struct Uniforms {
    view_proj: mat4x4<f32>,  // 视图投影矩阵
    model: mat4x4<f32>,      // 模型矩阵
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // 应用变换
    let world_position = uniforms.model * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.view_proj * world_position;
    out.color = in.color;
    
    return out;
}
```

**变换流程**：
```
局部空间（模型）
    ↓ 模型矩阵
世界空间
    ↓ 视图矩阵
相机空间
    ↓ 投影矩阵
裁剪空间
```

### 3. 片段着色器

片段着色器计算每个像素的颜色。

#### 基础片段着色器

```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 直接返回顶点颜色
    return vec4<f32>(in.color, 1.0);
}
```

#### 纹理映射

```wgsl
// 纹理和采样器
@group(0) @binding(1)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(2)
var s_diffuse: sampler;

// 顶点输出（包含纹理坐标）
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 采样纹理
    let color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    return color;
}
```

**纹理坐标**：
```
(0,0) -------- (1,0)
  |              |
  |   纹理图像   |
  |              |
(0,1) -------- (1,1)

左下角 = (0, 0)
右上角 = (1, 1)
```

### 4. 光照模型

#### Phong 光照模型

```wgsl
struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    ambient: f32,
    diffuse: f32,
    specular: f32,
}

@group(1) @binding(0)
var<uniform> light: Light;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 法向量（归一化）
    let normal = normalize(in.normal);
    
    // 光照方向
    let light_dir = normalize(light.position - in.world_position);
    
    // 视线方向
    let view_dir = normalize(camera_position - in.world_position);
    
    // 1. 环境光（Ambient）
    let ambient = light.ambient * light.color;
    
    // 2. 漫反射（Diffuse）
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = light.diffuse * diff * light.color;
    
    // 3. 镜面反射（Specular）
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular = light.specular * spec * light.color;
    
    // 组合
    let result = (ambient + diffuse + specular) * object_color;
    
    return vec4<f32>(result, 1.0);
}
```

**Phong 光照组成**：
```
环境光（Ambient）:
- 模拟间接光照
- 常量，不依赖方向

漫反射（Diffuse）:
- 模拟粗糙表面
- 依赖光照方向和法向量
- Lambert 余弦定律

镜面反射（Specular）:
- 模拟光滑表面
- 依赖视线方向
- 高光效果
```

#### 法线贴图

```wgsl
@group(0) @binding(3)
var t_normal: texture_2d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 从法线贴图采样
    let normal_map = textureSample(t_normal, s_diffuse, in.tex_coords).xyz;
    
    // 转换到 [-1, 1] 范围
    let normal = normal_map * 2.0 - 1.0;
    
    // 转换到世界空间（需要 TBN 矩阵）
    let N = normalize(in.TBN * normal);
    
    // 使用转换后的法向量进行光照计算
    // ...
}
```

### 5. Uniform 缓冲区

Uniform 是着色器的输入参数。

#### Rust 端定义

```rust
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],  // 4x4 矩阵
    model: [[f32; 4]; 4],
}

// 创建 Uniform 缓冲区
let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Uniform Buffer"),
    contents: bytemuck::cast_slice(&[uniforms]),
    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
});

// 创建绑定组
let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    label: Some("Uniform Bind Group"),
    layout: &bind_group_layout,
    entries: &[
        wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        },
    ],
});

// 更新 Uniform
queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[new_uniforms]));
```

#### WGSL 端使用

```wgsl
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // 使用 uniform
    let pos = uniforms.view_proj * uniforms.model * vec4<f32>(in.position, 1.0);
    // ...
}
```

**绑定组和绑定点**：
```
@group(0) @binding(0) - Uniform 缓冲区
@group(0) @binding(1) - 纹理
@group(0) @binding(2) - 采样器
@group(1) @binding(0) - 光照数据
```

## 💻 实战项目：纹理映射立方体

### 项目需求

创建一个带纹理和光照的旋转立方体。

### 步骤 1：顶点数据

```rust
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

// 立方体顶点（6个面，每面2个三角形）
const VERTICES: &[Vertex] = &[
    // 前面
    Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },
    
    // 后面
    Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    
    // ... 其他4个面
];

const INDICES: &[u16] = &[
    0, 1, 2,  2, 3, 0,  // 前面
    4, 5, 6,  6, 7, 4,  // 后面
    // ... 其他面
];
```

### 步骤 2：着色器

```wgsl
// shader.wgsl

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(2)
var s_diffuse: sampler;

@group(1) @binding(0)
var<uniform> light: Light;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // 世界空间位置
    let world_pos = uniforms.model * vec4<f32>(in.position, 1.0);
    out.world_position = world_pos.xyz;
    
    // 裁剪空间位置
    out.clip_position = uniforms.view_proj * world_pos;
    
    // 纹理坐标
    out.tex_coords = in.tex_coords;
    
    // 世界空间法向量
    out.world_normal = uniforms.normal_matrix * in.normal;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 采样纹理
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    // 归一化法向量
    let normal = normalize(in.world_normal);
    
    // 光照方向
    let light_dir = normalize(light.position - in.world_position);
    
    // 环境光
    let ambient_strength = 0.1;
    let ambient = ambient_strength * light.color;
    
    // 漫反射
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = diff * light.color;
    
    // 组合
    let result = (ambient + diffuse) * tex_color.rgb;
    
    return vec4<f32>(result, tex_color.a);
}
```

### 步骤 3：更新循环

```rust
fn update(&mut self, dt: f32) {
    // 旋转立方体
    self.rotation += dt;
    
    // 更新模型矩阵
    let model = na::Matrix4::from_axis_angle(
        &na::Vector3::y_axis(),
        self.rotation,
    );
    
    // 更新 uniform
    self.uniforms.model = model.into();
    self.uniforms.normal_matrix = model
        .fixed_view::<3, 3>(0, 0)
        .transpose()
        .try_inverse()
        .unwrap()
        .into();
    
    // 写入缓冲区
    self.queue.write_buffer(
        &self.uniform_buffer,
        0,
        bytemuck::cast_slice(&[self.uniforms]),
    );
}
```

## 🔍 深入理解

### 着色器编译

```
WGSL 源代码
    ↓
wgpu 验证
    ↓
中间表示（IR）
    ↓
后端编译器
    ↓
平台特定代码
- SPIR-V (Vulkan)
- MSL (Metal)
- HLSL (DirectX)
```

### 性能优化

```wgsl
// ❌ 低效：重复计算
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let l = normalize(light_pos - in.pos);
    let v = normalize(camera_pos - in.pos);
    // 每个片段都计算 normalize
}

// ✅ 高效：在顶点着色器预计算
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    out.normal = normalize(model_matrix * in.normal);
    // 只在顶点计算，片段着色器插值
}
```

### 常见陷阱

```wgsl
// ❌ 错误：忘记归一化
let normal = in.normal;  // 插值后可能不是单位向量
let diff = dot(normal, light_dir);

// ✅ 正确
let normal = normalize(in.normal);
let diff = dot(normal, light_dir);

// ❌ 错误：颜色范围
let color = normal;  // 法向量 [-1, 1]

// ✅ 正确
let color = normal * 0.5 + 0.5;  // 转换到 [0, 1]
```

## 📝 练习题

### 练习 1：实现 Blinn-Phong 光照
```wgsl
// 提示：使用半程向量
let halfway = normalize(light_dir + view_dir);
let spec = pow(max(dot(normal, halfway), 0.0), shininess);
```

### 练习 2：实现多光源
```wgsl
struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(1) @binding(0)
var<uniform> lights: array<Light, 4>;

// 累加所有光源的贡献
```

### 练习 3：实现雾效果
```wgsl
let fog_color = vec3<f32>(0.5, 0.5, 0.5);
let fog_density = 0.05;
let distance = length(camera_pos - world_pos);
let fog_factor = exp(-fog_density * distance);
let final_color = mix(fog_color, color, fog_factor);
```

## 🎯 学习检查清单

- [ ] 理解 WGSL 基础语法
- [ ] 编写顶点着色器
- [ ] 编写片段着色器
- [ ] 使用纹理映射
- [ ] 实现 Phong 光照
- [ ] 使用 Uniform 缓冲区
- [ ] 理解变换矩阵
- [ ] 处理法向量
- [ ] 优化着色器性能

## 🔗 延伸阅读

- [WGSL 规范](https://www.w3.org/TR/WGSL/)
- [Learn WGSL](https://google.github.io/tour-of-wgsl/)
- [Phong 光照模型](https://en.wikipedia.org/wiki/Phong_reflection_model)
- [法线贴图教程](https://learnopengl.com/Advanced-Lighting/Normal-Mapping)

---

**掌握着色器编程，释放 GPU 的强大能力！** 🚀
