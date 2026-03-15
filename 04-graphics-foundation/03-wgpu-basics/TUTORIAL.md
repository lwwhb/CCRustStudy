# 模块 4.3：wgpu 基础 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解现代图形 API 的概念
2. 初始化 wgpu 渲染上下文
3. 创建渲染管线
4. 使用缓冲区和纹理
5. 绘制第一个三角形

## 🎯 为什么选择 wgpu？

### 传统 API vs 现代 API

**传统 OpenGL（状态机）**：
```c
// 全局状态
glBindBuffer(GL_ARRAY_BUFFER, vbo);
glBufferData(GL_ARRAY_BUFFER, size, data, GL_STATIC_DRAW);
glBindBuffer(GL_ARRAY_BUFFER, 0);  // 容易忘记

// 隐式依赖
glUseProgram(program);  // 必须在这之前
glDrawArrays(GL_TRIANGLES, 0, 3);

问题：
- 全局状态难以管理
- 隐式依赖容易出错
- 调试困难
- 性能不透明
```

**现代 wgpu（显式）**：
```rust
// 显式资源
let buffer = device.create_buffer(&descriptor);

// 显式绑定
let bind_group = device.create_bind_group(&descriptor);

// 显式命令
let mut encoder = device.create_command_encoder(&descriptor);
encoder.begin_render_pass(&descriptor);

优势：
- 显式资源管理
- 清晰的依赖关系
- 易于调试
- 性能可预测
```

### wgpu 的特点

```
1. 跨平台
   - Windows: DirectX 12
   - macOS/iOS: Metal
   - Linux/Android: Vulkan
   - Web: WebGPU

2. 安全
   - Rust 的内存安全
   - 编译时检查
   - 无未定义行为

3. 现代
   - 基于 WebGPU 标准
   - 显式同步
   - 多线程友好

4. 高性能
   - 低开销
   - 批处理优化
   - GPU 并行
```

## 📖 核心概念详解

### 1. wgpu 架构

```
应用程序
    ↓
Instance（实例）
    ↓
Adapter（适配器）- 选择 GPU
    ↓
Device（设备）- 逻辑 GPU
    ↓
Queue（队列）- 提交命令
    ↓
GPU 执行
```

#### 初始化流程

```rust
use wgpu;

async fn init_wgpu() -> (wgpu::Device, wgpu::Queue) {
    // 1. 创建实例
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    // 2. 获取适配器（GPU）
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    // 3. 创建设备和队列
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    (device, queue)
}
```

**关键概念**：
```
Instance:
- wgpu 的入口点
- 管理所有 GPU

Adapter:
- 代表物理 GPU
- 查询能力和限制

Device:
- 逻辑 GPU
- 创建资源（缓冲区、纹理等）

Queue:
- 提交命令到 GPU
- 异步执行
```

### 2. Surface（表面）

Surface 是渲染目标（窗口）。

```rust
use winit::window::Window;

fn create_surface(
    instance: &wgpu::Instance,
    window: &Window,
) -> wgpu::Surface {
    // 创建表面
    unsafe { instance.create_surface(window) }.unwrap()
}

fn configure_surface(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) {
    // 配置表面
    surface.configure(device, config);
}

// 表面配置
let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface.get_capabilities(&adapter).formats[0],
    width: size.width,
    height: size.height,
    present_mode: wgpu::PresentMode::Fifo,  // VSync
    alpha_mode: wgpu::CompositeAlphaMode::Auto,
    view_formats: vec![],
};
```

**Present Mode**：
```
Fifo (VSync):
- 等待垂直同步
- 无撕裂
- 可能有延迟

Immediate:
- 立即显示
- 可能撕裂
- 低延迟

Mailbox:
- 三重缓冲
- 无撕裂
- 低延迟
```

### 3. 渲染管线

渲染管线定义如何处理顶点和像素。

```
顶点数据
    ↓
顶点着色器（Vertex Shader）
    ↓
光栅化（Rasterization）
    ↓
片段着色器（Fragment Shader）
    ↓
输出颜色
```

#### 创建渲染管线

```rust
// 1. 加载着色器
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Shader"),
    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
});

// 2. 创建管线布局
let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("Pipeline Layout"),
    bind_group_layouts: &[],
    push_constant_ranges: &[],
});

// 3. 创建渲染管线
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&pipeline_layout),
    
    // 顶点阶段
    vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[],
    },
    
    // 片段阶段
    fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })],
    }),
    
    // 图元类型
    primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        ..Default::default()
    },
    
    // 深度/模板
    depth_stencil: None,
    
    // 多重采样
    multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    },
    
    // 多视图
    multiview: None,
});
```

### 4. 着色器（WGSL）

WGSL 是 WebGPU 的着色器语言。

```wgsl
// shader.wgsl

// 顶点着色器
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    // 三角形的三个顶点
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),   // 顶部
        vec2<f32>(-0.5, -0.5), // 左下
        vec2<f32>(0.5, -0.5)   // 右下
    );

    return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
}

// 片段着色器
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);  // 红色
}
```

**WGSL 基础**：
```wgsl
// 变量
var x: f32 = 1.0;
let y: i32 = 2;

// 向量
var v2: vec2<f32> = vec2<f32>(1.0, 2.0);
var v3: vec3<f32> = vec3<f32>(1.0, 2.0, 3.0);
var v4: vec4<f32> = vec4<f32>(1.0, 2.0, 3.0, 4.0);

// 矩阵
var m: mat4x4<f32>;

// 函数
fn add(a: f32, b: f32) -> f32 {
    return a + b;
}

// 内置函数
let len = length(v3);
let norm = normalize(v3);
let d = dot(v3, v3);
```

### 5. 缓冲区（Buffer）

缓冲区存储 GPU 数据。

```rust
// 顶点数据
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

let vertices = [
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

// 创建顶点缓冲区
let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Vertex Buffer"),
    contents: bytemuck::cast_slice(&vertices),
    usage: wgpu::BufferUsages::VERTEX,
});
```

**缓冲区用途**：
```
VERTEX: 顶点数据
INDEX: 索引数据
UNIFORM: 统一变量（如变换矩阵）
STORAGE: 存储缓冲区（大量数据）
COPY_SRC: 可以作为复制源
COPY_DST: 可以作为复制目标
```

### 6. 渲染循环

```rust
fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface: &wgpu::Surface,
    pipeline: &wgpu::RenderPipeline,
) -> Result<(), wgpu::SurfaceError> {
    // 1. 获取当前帧
    let output = surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    // 2. 创建命令编码器
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    // 3. 开始渲染通道
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ...[truncated 8869 chars]