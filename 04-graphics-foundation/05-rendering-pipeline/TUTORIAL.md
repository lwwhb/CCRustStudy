# 模块 4.5：3D 渲染管线 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解完整的渲染管线
2. 实现深度测试
3. 掌握背面剔除
4. 学习混合模式
5. 构建 3D 场景渲染器

## 🎯 为什么需要渲染管线？

### 简单渲染 vs 完整管线

**简单渲染（问题）**：
```rust
// 直接绘制所有三角形
for triangle in triangles {
    draw(triangle);
}

问题：
- 远处物体遮挡近处物体
- 看到物体背面
- 透明物体不正确
- 性能低下
```

**完整管线（解决方案）**：
```rust
// 1. 深度测试 - 正确的遮挡关系
// 2. 背面剔除 - 不绘制看不见的面
// 3. 混合 - 正确的透明效果
// 4. 优化 - 提升性能

优势：
- 正确的视觉效果
- 高性能
- 支持复杂场景
```

### 渲染管线流程

```
应用阶段（CPU）
    ↓
顶点处理（GPU）
- 顶点着色器
- 变换和投影
    ↓
图元装配
- 组装三角形
    ↓
光栅化
- 生成片段
    ↓
片段处理（GPU）
- 片段着色器
- 深度测试
- 模板测试
- 混合
    ↓
帧缓冲
```

## 📖 核心概念详解

### 1. 深度测试

深度测试确保近处物体遮挡远处物体。

#### 深度缓冲区

```rust
use wgpu;

// 创建深度纹理
fn create_depth_texture(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::Texture {
    let size = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };

    device.create_texture(&desc)
}

// 创建深度纹理视图
let depth_texture = create_depth_texture(&device, &config);
let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
```

#### 配置深度测试

```rust
// 在渲染管线中启用深度测试
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    // ... 其他配置
    
    depth_stencil: Some(wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,  // 深度值更小的通过
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    }),
    
    // ...
});
```

**深度比较函数**：
```
Less:       新片段深度 < 缓冲区深度（最常用）
LessEqual:  新片段深度 <= 缓冲区深度
Greater:    新片段深度 > 缓冲区深度
Equal:      新片段深度 == 缓冲区深度
Always:     总是通过
Never:      总是失败
```

#### 使用深度测试

```rust
// 在渲染通道中附加深度纹理
let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("Render Pass"),
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            store: wgpu::StoreOp::Store,
        },
    })],
    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
        view: &depth_view,
        depth_ops: Some(wgpu::Operations {
            load: wgpu::LoadOp::Clear(1.0),  // 清除为最大深度
            store: wgpu::StoreOp::Store,
        }),
        stencil_ops: None,
    }),
    // ...
});
```

**深度值范围**：
```
0.0 = 最近（相机位置）
1.0 = 最远

清除深度缓冲区为 1.0，
这样所有物体都会比初始值近
```

### 2. 背面剔除

背面剔除跳过不可见的三角形背面。

#### 配置剔除模式

```rust
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    // ...
    
    primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        
        // 正面方向（逆时针）
        front_face: wgpu::FrontFace::Ccw,
        
        // 剔除背面
        cull_mode: Some(wgpu::Face::Back),
        
        polygon_mode: wgpu::PolygonMode::Fill,
        ..Default::default()
    },
    
    // ...
});
```

**剔除模式**：
```
None:  不剔除（绘制所有面）
Front: 剔除正面
Back:  剔除背面（最常用）
```

**顶点顺序**：
```
逆时针（CCW）= 正面
   1
   |\
   | \
   |  \
   2---3

顺时针（CW）= 背面
   1
   |\ 
   | \
   |  \
   3---2
```

#### 何时不使用剔除

```rust
// 双面材质（如树叶、布料）
cull_mode: None,

// 透明物体
cull_mode: None,

// 内部可见的物体（如房间内部）
cull_mode: Some(wgpu::Face::Front),  // 剔除正面，显示背面
```

### 3. 混合模式

混合控制透明和半透明效果。

#### Alpha 混合

```rust
let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    // ...
    
    fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
            format: config.format,
            
            // Alpha 混合
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            
            write_mask: wgpu::ColorWrites::ALL,
        })],
    }),
    
    // ...
});
```

**混合公式**：
```
最终颜色 = 源颜色 × 源因子 + 目标颜色 × 目标因子

Alpha 混合：
final = src.rgb × src.a + dst.rgb × (1 - src.a)

例如：
源颜色 = (1.0, 0.0, 0.0, 0.5)  // 半透明红色
目标颜色 = (0.0, 0.0, 1.0, 1.0)  // 不透明蓝色

final.r = 1.0 × 0.5 + 0.0 × 0.5 = 0.5
final.g = 0.0 × 0.5 + 0.0 × 0.5 = 0.0
final.b = 0.0 × 0.5 + 1.0 × 0.5 = 0.5
结果 = (0.5, 0.0, 0.5)  // 紫色
```

#### 常见混合模式

```rust
// 1. 不透明（无混合）
blend: Some(wgpu::BlendState::REPLACE),

// 2. Alpha 混合（透明）
blend: Some(wgpu::BlendState::ALPHA_BLENDING),

// 3. 加法混合（光效）
blend: Some(wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent::REPLACE,
}),

// 4. 乘法混合（阴影）
blend: Some(wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::DstColor,
        dst_factor: wgpu::BlendFactor::Zero,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent::REPLACE,
}),
```

### 4. 多通道渲染

复杂场景需要多个渲染通道。

#### 渲染顺序

```rust
// 1. 不透明物体（从前到后，启用深度写入）
render_opaque_objects();

// 2. 天空盒（禁用深度写入）
render_skybox();

// 3. 透明物体（从后到前，禁用深度写入）
render_transparent_objects();

// 4. UI（最后渲染，禁用深度测试）
render_ui();
```

**为什么这样排序？**
```
不透明物体：
- 从前到后：早期深度测试，跳过被遮挡的片段
- 启用深度写入：建立深度缓冲区

天空盒：
- 最后渲染不透明物体
- 禁用深度写入：不影响其他物体
- 深度测试 LessEqual：只在没有物体的地方绘制

透明物体：
- 从后到前：正确的混合顺序
- 禁用深度写入：允许后面的透明物体显示
- 启用深度测试：被不透明物体遮挡

UI：
- 最后渲染
- 禁用深度测试：总是在最前面
```

### 5. 场景图

组织和管理 3D 场景。

```rust
use nalgebra as na;

// 场景节点
struct SceneNode {
    name: String,
    transform: na::Matrix4<f32>,
    mesh: Option<MeshHandle>,
    children: Vec<SceneNode>,
}

impl SceneNode {
    fn new(name: String) -> Self {
        Self {
            name,
            transform: na::Matrix4::identity(),
            mesh: None,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    // 递归渲染
    fn render(
        &self,
        render_pass: &mut wgpu::RenderPass,
        parent_transform: &na::Matrix4<f32>,
    ) {
        // 计算世界变换
        let world_transform = parent_transform * self.transform;

        // 渲染自己的网格
        if let Some(mesh) = &self.mesh {
            // 更新 uniform
            // 绘制网格
        }

        // 递归渲染子节点
        for child in &self.children {
            child.render(render_pass, &world_transform);
        }
    }
}
```

**场景图示例**：
```
根节点
├── 汽车
│   ├── 车身
│   ├── 前轮
│   │   ├── 轮胎
│   │   └── 轮毂
│   └── 后轮
│       ├── 轮胎
│       └── 轮毂
└── 地面
```

## 💻 实战项目：3D 场景渲染器

### 项目需求

构建一个完整的 3D 场景渲染器，支持：
1. 多个 3D 模型
2. 相机控制
3. 光照
4. 深度测试
5. 透明物体

### 步骤 1：场景管理器

```rust
pub struct Scene {
    objects: Vec<Object3D>,
    camera: Camera,
    light: Light,
}

pub struct Object3D {
    mesh: Mesh,
    material: Material,
    transform: na::Matrix4<f32>,
    transparent: bool,
}

impl Scene {
    pub fn new(camera: Camera, light: Light) -> Self {
        Self {
            objects: Vec::new(),
            camera,
            light,
        }
    }

    pub fn add_object(&mut self, object: Object3D) {
        self.objects.push(object);
    }

    pub fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        // 分离不透明和透明物体
        let mut opaque: Vec<_> = self.objects
            .iter()
            .filter(|o| !o.transparent)
            .collect();

        let mut transparent: Vec<_> = self.objects
            .iter()
            .filter(|o| o.transparent)
            .collect();

        // 排序
        self.sort_objects(&mut opaque, &mut transparent);

        // 渲染
        self.render_opaque(&opaque);
        self.render_transparent(&transparent);
    }

    fn sort_objects(
        &self,
        opaque: &mut Vec<&Object3D>,
        transparent: &mut Vec<&Object3D>,
    ) {
        let camera_pos = self.camera.position;

        // 不透明物体：从前到后
        opaque.sort_by(|a, b| {
            let dist_a = (a.get_position() - camera_pos).magnitude();
            let dist_b = (b.get_position() - camera_pos).magnitude();
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        // 透明物体：从后到前
        transparent.sort_by(|a, b| {
            let dist_a = (a.get_position() - camera_pos).magnitude();
            let dist_b = (b.get_position() - camera_pos).magnitude();
            dist_b.partial_cmp(&dist_a).unwrap()
        });
    }
}
```

### 步骤 2：渲染器

```rust
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    
    opaque_pipeline: wgpu::RenderPipeline,
    transparent_pipeline: wgpu::RenderPipeline,
    
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
}

impl Renderer {
    pub fn render(&mut self, scene: &Scene) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        // 渲染通道
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            // 渲染不透明物体
            render_pass.set_pipeline(&self.opaque_pipeline);
            for object in &scene.opaque_objects {
                self.render_object(&mut render_pass, object, scene);
            }

            // 渲染透明物体
            render_pass.set_pipeline(&self.transparent_pipeline);
            for object in &scene.transparent_objects {
                self.render_object(&mut render_pass, object, scene);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
```

### 步骤 3：相机控制

```rust
pub struct Camera {
    pub position: na::Point3<f32>,
    pub target: na::Point3<f32>,
    pub up: na::Vector3<f32>,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn projection_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }

    pub fn view_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    // 相机移动
    pub fn move_forward(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        self.position += direction * distance;
        self.target += direction * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        self.position += right * distance;
        self.target += right * distance;
    }

    // 相机旋转
    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        let direction = self.target - self.position;
        let distance = direction.magnitude();

        // 计算新方向
        let rotation = na::Rotation3::from_axis_angle(&self.up, yaw)
            * na::Rotation3::from_axis_angle(&na::Vector3::x_axis(), pitch);

        let new_direction = rotation * direction.normalize();
        self.target = self.position + new_direction * distance;
    }
}
```

### 步骤 4：完整示例

```rust
fn main() {
    // 初始化
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut renderer = Renderer::new(&window);

    // 创建场景
    let camera = Camera::new(
        na::Point3::new(0.0, 2.0, 5.0),
        na::Point3::new(0.0, 0.0, 0.0),
        na::Vector3::y(),
        45.0_f32.to_radians(),
        800.0 / 600.0,
        0.1,
        100.0,
    );

    let light = Light {
        position: na::Point3::new(5.0, 5.0, 5.0),
        color: na::Vector3::new(1.0, 1.0, 1.0),
        intensity: 1.0,
    };

    let mut scene = Scene::new(camera, light);

    // 添加物体
    scene.add_object(create_cube(na::Vector3::new(0.0, 0.0, 0.0), false));
    scene.add_object(create_sphere(na::Vector3::new(2.0, 0.0, 0.0), false));
    scene.add_object(create_plane(na::Vector3::new(0.0, -1.0, 0.0), false));
    
    // 透明物体
    scene.add_object(create_glass_cube(na::Vector3::new(-2.0, 0.0, 0.0), true));

    // 运行
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    renderer.render(&scene).unwrap();
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
```

## 🔍 深入理解

### 性能优化

```rust
// 1. 视锥剔除
fn is_in_frustum(object: &Object3D, camera: &Camera) -> bool {
    // 检查物体是否在相机视锥内
    // 跳过不可见的物体
}

// 2. LOD（细节层次）
struct LODMesh {
    high: Mesh,    // 近距离
    medium: Mesh,  // 中距离
    low: Mesh,     // 远距离
}

fn select_lod(distance: f32) -> &Mesh {
    if distance < 10.0 {
        &self.high
    } else if distance < 50.0 {
        &self.medium
    } else {
        &self.low
    }
}

// 3. 实例化渲染
// 一次绘制多个相同物体
render_pass.draw_indexed(
    0..mesh.index_count,
    0,
    0..instance_count,  // 实例数量
);
```

## 📝 练习题

### 练习 1：实现天空盒
创建一个立方体贴图天空盒。

### 练习 2：阴影映射
实现基础的阴影映射。

### 练习 3：后处理效果
添加模糊、泛光等后处理效果。

## 🎯 学习检查清单

- [ ] 理解深度测试的工作原理
- [ ] 掌握背面剔除的配置
- [ ] 理解混合模式和透明渲染
- [ ] 实现多通道渲染
- [ ] 构建场景图系统
- [ ] 实现相机控制
- [ ] 优化渲染性能

## 🔗 延伸阅读

- [Learn OpenGL - Depth Testing](https://learnopengl.com/Advanced-OpenGL/Depth-testing)
- [Learn OpenGL - Blending](https://learnopengl.com/Advanced-OpenGL/Blending)
- [Real-Time Rendering](http://www.realtimerendering.com/)

## 🚀 下一步

完成本模块后，你可以：
1. 开始构建 3D 渲染器项目（模块 6.1-6.5）
2. 学习高级渲染技术（阴影、反射、全局光照）
3. 优化渲染性能

---

**掌握渲染管线，构建完整的 3D 场景！** 🎨

