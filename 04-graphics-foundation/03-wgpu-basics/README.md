# 模块 4.3：wgpu 基础

## 🎯 学习目标

- 理解 GPU 编程基础概念
- 学习 wgpu 初始化流程
- 掌握渲染管线的创建
- 理解缓冲区和纹理
- 学习着色器基础
- 绘制第一个三角形

## 📚 核心概念

### 1. wgpu 初始化

```rust
use wgpu;

async fn init_wgpu() {
    // 创建实例
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    // 获取适配器
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    // 创建设备和队列
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();
}
```

### 2. 渲染管线

```rust
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&pipeline_layout),
    vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[],
    },
    fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
            format: surface_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })],
    }),
    primitive: wgpu::PrimitiveState::default(),
    depth_stencil: None,
    multisample: wgpu::MultisampleState::default(),
    multiview: None,
});
```

### 3. 顶点缓冲区

```rust
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

let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Vertex Buffer"),
    contents: bytemuck::cast_slice(&vertices),
    usage: wgpu::BufferUsages::VERTEX,
});
```

### 4. 着色器 (WGSL)

```wgsl
// 顶点着色器
@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}

// 片段着色器
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); // 红色
}
```

## 💻 实战项目：渲染三角形

使用 wgpu 渲染一个彩色三角形。

### 功能需求

1. 初始化 wgpu
2. 创建渲染管线
3. 创建顶点缓冲区
4. 渲染循环
5. 清屏和绘制

### 项目结构

```
wgpu-basics/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── renderer.rs     # 渲染器
│   ├── pipeline.rs     # 管线管理
│   └── vertex.rs       # 顶点定义
└── README.md
```

## 🧪 练习题

### 练习 1：改变三角形颜色

```rust
// 修改片段着色器，使三角形变成蓝色
```

### 练习 2：绘制正方形

```rust
// 使用两个三角形绘制一个正方形
```

### 练习 3：顶点颜色插值

```rust
// 为每个顶点设置不同颜色，观察插值效果
```

## 📖 深入阅读

- [wgpu Documentation](https://docs.rs/wgpu/)
- [Learn wgpu](https://sotrh.github.io/learn-wgpu/)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)

## ✅ 检查清单

- [ ] 理解 GPU 编程基础
- [ ] 初始化 wgpu 实例和设备
- [ ] 创建渲染管线
- [ ] 定义顶点数据
- [ ] 创建顶点缓冲区
- [ ] 编写简单着色器
- [ ] 实现渲染循环

## 🚀 下一步

完成本模块后，继续学习 [模块 4.4：着色器编程](../04-shaders/)。

## 注意事项

由于 wgpu 需要图形环境和 GPU 支持，本模块主要演示概念和代码结构。实际运行需要：
- 支持 Vulkan/Metal/DX12 的 GPU
- 图形显示环境
- 适当的驱动程序

在学习环境中，我们将专注于理解概念和 API 使用，而不是实际渲染输出。
