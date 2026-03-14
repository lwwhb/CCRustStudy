# 模块 4.5：3D 渲染管线

## 🎯 学习目标

- 理解完整的渲染管线流程
- 掌握深度测试和背面剔除
- 学习混合模式和透明度
- 理解多通道渲染
- 实现场景图管理
- 构建完整的 3D 场景渲染器

## 📚 核心概念

### 1. 渲染管线流程

```
顶点数据 → 顶点着色器 → 图元装配 → 光栅化 →
片段着色器 → 深度测试 → 混合 → 帧缓冲
```

### 2. 深度测试

```rust
depth_stencil: Some(wgpu::DepthStencilState {
    format: wgpu::TextureFormat::Depth32Float,
    depth_write_enabled: true,
    depth_compare: wgpu::CompareFunction::Less,
    stencil: wgpu::StencilState::default(),
    bias: wgpu::DepthBiasState::default(),
})
```

### 3. 背面剔除

```rust
primitive: wgpu::PrimitiveState {
    cull_mode: Some(wgpu::Face::Back),
    front_face: wgpu::FrontFace::Ccw,
    ..Default::default()
}
```

### 4. 混合模式

```rust
blend: Some(wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent::OVER,
})
```

### 5. 场景图

```rust
struct SceneNode {
    transform: Transform,
    mesh: Option<Mesh>,
    children: Vec<SceneNode>,
}
```

## 💻 实战项目：3D 场景渲染器

实现一个完整的 3D 场景渲染系统：
1. 场景图管理
2. 相机系统
3. 多对象渲染
4. 深度测试
5. 透明物体渲染

## 🧪 测试

```bash
cargo test
```

## 📝 练习

1. 实现场景图的层次变换
2. 添加多个相机视角
3. 实现透明物体的正确排序
4. 添加阴影映射

## 🔗 深入阅读

- [Real-Time Rendering](https://www.realtimerendering.com/)
- [Learn OpenGL - Depth Testing](https://learnopengl.com/Advanced-OpenGL/Depth-testing)
- [Scene Graphs](https://en.wikipedia.org/wiki/Scene_graph)

## ✅ 检查清单

- [ ] 理解渲染管线流程
- [ ] 实现深度测试
- [ ] 配置背面剔除
- [ ] 实现混合模式
- [ ] 构建场景图
- [ ] 渲染多个 3D 对象
- [ ] 完成练习题
