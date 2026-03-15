# 模块 6.1：3D 渲染器架构设计 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 设计模块化的渲染器架构
2. 实现资源管理系统
3. 构建场景图
4. 设计渲染抽象层
5. 实现插件化系统

## 🎯 为什么需要架构设计？

### 简单代码 vs 架构化设计

**简单代码（问题）**：
```rust
// 所有代码在 main.rs
fn main() {
    // 初始化 wgpu
    let device = ...;
    let queue = ...;
    
    // 加载模型
    let vertices = load_obj("model.obj");
    let buffer = device.create_buffer(...);
    
    // 渲染循环
    loop {
        // 更新
        // 渲染
        // ...
    }
}

问题：
- 代码耦合
- 难以扩展
- 难以测试
- 难以维护
```

**架构化设计（解决方案）**：
```rust
// 清晰的模块划分
mod renderer;
mod scene;
mod resources;
mod camera;

fn main() {
    let mut app = Application::new();
    app.add_system(RenderSystem::new());
    app.add_system(CameraSystem::new());
    app.run();
}

优势：
- 模块解耦
- 易于扩展
- 易于测试
- 易于维护
```

### 渲染器架构层次

```
应用层（Application）
    ↓
场景层（Scene）
- 场景图
- 实体组件
    ↓
渲染层（Renderer）
- 渲染管线
- 材质系统
    ↓
资源层（Resources）
- 模型加载
- 纹理管理
    ↓
图形 API 层（wgpu）
```

## 📖 核心概念详解

### 1. 渲染器核心架构

#### 渲染器接口

```rust
use wgpu;
use winit::window::Window;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // 创建实例
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // 创建表面
        let surface = unsafe { instance.create_surface(window) }.unwrap();

        // 请求适配器
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // 创建设备和队列
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

        // 配置表面
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Self {
            device,
            queue,
            surface,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
```

### 2. 资源管理系统

#### 资源句柄

```rust
use std::sync::Arc;
use std::collections::HashMap;

// 资源句柄（类型安全）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    id: u64,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }
}

// 资源存储
pub struct ResourceStorage<T> {
    resources: HashMap<u64, Arc<T>>,
    next_id: u64,
}

impl<T> ResourceStorage<T> {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, resource: T) -> Handle<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.resources.insert(id, Arc::new(resource));
        Handle::new(id)
    }

    pub fn get(&self, handle: Handle<T>) -> Option<Arc<T>> {
        self.resources.get(&handle.id).cloned()
    }

    pub fn remove(&mut self, handle: Handle<T>) -> Option<Arc<T>> {
        self.resources.remove(&handle.id)
    }
}
```

#### 资源管理器

```rust
pub struct ResourceManager {
    meshes: ResourceStorage<Mesh>,
    textures: ResourceStorage<Texture>,
    materials: ResourceStorage<Material>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            meshes: ResourceStorage::new(),
            textures: ResourceStorage::new(),
            materials: ResourceStorage::new(),
        }
    }

    pub fn load_mesh(&mut self, path: &str) -> Handle<Mesh> {
        // 加载网格
        let mesh = Mesh::load(path).unwrap();
        self.meshes.insert(mesh)
    }

    pub fn load_texture(&mut self, path: &str) -> Handle<Texture> {
        // 加载纹理
        let texture = Texture::load(path).unwrap();
        self.textures.insert(texture)
    }

    pub fn create_material(&mut self, material: Material) -> Handle<Material> {
        self.materials.insert(material)
    }

    pub fn get_mesh(&self, handle: Handle<Mesh>) -> Option<Arc<Mesh>> {
        self.meshes.get(handle)
    }

    pub fn get_texture(&self, handle: Handle<Texture>) -> Option<Arc<Texture>> {
        self.textures.get(handle)
    }

    pub fn get_material(&self, handle: Handle<Material>) -> Option<Arc<Material>> {
        self.materials.get(handle)
    }
}
```

### 3. 场景图

场景图组织场景中的对象。

#### 场景节点

```rust
use nalgebra as na;

pub struct Transform {
    pub position: na::Vector3<f32>,
    pub rotation: na::UnitQuaternion<f32>,
    pub scale: na::Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: na::Vector3::zeros(),
            rotation: na::UnitQuaternion::identity(),
            scale: na::Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn matrix(&self) -> na::Matrix4<f32> {
        let translation = na::Matrix4::new_translation(&self.position);
        let rotation = self.rotation.to_homogeneous();
        let scale = na::Matrix4::new_nonuniform_scaling(&self.scale);
        
        translation * rotation * scale
    }
}

pub struct SceneNode {
    pub name: String,
    pub transform: Transform,
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<Material>>,
    pub children: Vec<SceneNode>,
}

impl SceneNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            transform: Transform::new(),
            mesh: None,
            material: None,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }

    // 计算世界变换矩阵
    pub fn world_matrix(&self, parent_matrix: &na::Matrix4<f32>) -> na::Matrix4<f32> {
        parent_matrix * self.transform.matrix()
    }

    // 遍历场景图
    pub fn traverse<F>(&self, parent_matrix: &na::Matrix4<f32>, f: &mut F)
    where
        F: FnMut(&SceneNode, &na::Matrix4<f32>),
    {
        let world_matrix = self.world_matrix(parent_matrix);
        f(self, &world_matrix);

        for child in &self.children {
            child.traverse(&world_matrix, f);
        }
    }
}
```

#### 场景

```rust
pub struct Scene {
    pub root: SceneNode,
    pub camera: Camera,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            root: SceneNode::new("Root".to_string()),
            camera: Camera::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, node: SceneNode) {
        self.root.add_child(node);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    // 收集所有可渲染对象
    pub fn collect_renderables(&self) -> Vec<Renderable> {
        let mut renderables = Vec::new();
        let identity = na::Matrix4::identity();

        self.root.traverse(&identity, &mut |node, world_matrix| {
            if let (Some(mesh), Some(material)) = (node.mesh, node.material) {
                renderables.push(Renderable {
                    mesh,
                    material,
                    transform: *world_matrix,
                });
            }
        });

        renderables
    }
}

pub struct Renderable {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,
    pub transform: na::Matrix4<f32>,
}
```

### 4. 相机系统

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
    pub fn new() -> Self {
        Self {
            position: na::Point3::new(0.0, 0.0, 5.0),
            target: na::Point3::origin(),
            up: na::Vector3::y(),
            fov: std::f32::consts::FRAC_PI_4,
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn projection_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }

    pub fn view_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }
}
```

### 5. 渲染系统

```rust
pub struct RenderSystem {
    renderer: Renderer,
    resources: ResourceManager,
    pipelines: HashMap<String, wgpu::RenderPipeline>,
}

impl RenderSystem {
    pub fn new(window: &Window) -> Self {
        let renderer = pollster::block_on(Renderer::new(window));
        let resources = ResourceManager::new();
        let pipelines = HashMap::new();

        Self {
            renderer,
            resources,
            pipelines,
        }
    }

    pub fn render_scene(&mut self, scene: &Scene) -> Result<(), wgpu::SurfaceError> {
        // 收集可渲染对象
        let renderables = scene.collect_renderables();

        // 获取表面纹理
        let output = self.renderer.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 创建命令编码器
        let mut encoder = self.renderer.device.create_command_encoder(
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // 渲染每个对象
            for renderable in renderables {
                self.render_object(&mut render_pass, &renderable);
            }
        }

        // 提交命令
        self.renderer.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn render_object(
        &self,
        render_pass: &mut wgpu::RenderPass,
        renderable: &Renderable,
    ) {
        // 获取资源
        let mesh = self.resources.get_mesh(renderable.mesh).unwrap();
        let material = self.resources.get_material(renderable.material).unwrap();

        // 设置管线
        // 设置绑定组
        // 绘制
        // ...
    }
}
```

### 6. 应用框架

```rust
pub struct Application {
    window: Window,
    event_loop: EventLoop<()>,
    render_system: RenderSystem,
    scene: Scene,
}

impl Application {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("3D Renderer")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)
            .unwrap();

        let render_system = RenderSystem::new(&window);
        let scene = Scene::new();

        Self {
            window,
            event_loop,
            render_system,
            scene,
        }
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => {
                        self.render_system.renderer.resize(size);
                    }
                    WindowEvent::RedrawRequested => {
                        self.update();
                        match self.render_system.render_scene(&self.scene) {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                self.render_system.renderer.resize(self.render_system.renderer.size);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        }).unwrap();
    }

    fn update(&mut self) {
        // 更新场景
        // 更新相机
        // 更新动画
        // ...
    }
}
```

## 💻 实战项目：模块化渲染器

### 项目结构

```
src/
├── main.rs
├── renderer/
│   ├── mod.rs
│   ├── renderer.rs
│   └── pipeline.rs
├── scene/
│   ├── mod.rs
│   ├── scene.rs
│   ├── node.rs
│   └── camera.rs
├── resources/
│   ├── mod.rs
│   ├── manager.rs
│   ├── mesh.rs
│   ├── texture.rs
│   └── material.rs
└── app.rs
```

### 使用示例

```rust
fn main() {
    let mut app = Application::new();

    // 加载资源
    let cube_mesh = app.render_system.resources.load_mesh("cube.obj");
    let texture = app.render_system.resources.load_texture("texture.png");
    let material = app.render_system.resources.create_material(Material {
        albedo_texture: Some(texture),
        ..Default::default()
    });

    // 创建场景对象
    let mut cube = SceneNode::new("Cube".to_string());
    cube.mesh = Some(cube_mesh);
    cube.material = Some(material);
    cube.transform.position = na::Vector3::new(0.0, 0.0, 0.0);

    // 添加到场景
    app.scene.add_object(cube);

    // 运行
    app.run();
}
```

## 🎯 设计原则

### 1. 单一职责
每个模块只负责一件事

### 2. 依赖倒置
高层模块不依赖低层模块

### 3. 开闭原则
对扩展开放，对修改关闭

### 4. 接口隔离
使用小而专注的接口

### 5. 组合优于继承
使用组合而非继承

## 📝 练习题

### 练习 1：添加材质系统
实现完整的材质系统，支持多种材质类型

### 练习 2：实现资源热重载
实现资源文件变化时自动重新加载

### 练习 3：添加性能分析
添加帧率统计和性能分析工具

## 🎯 学习检查清单

- [ ] 理解渲染器架构层次
- [ ] 实现资源管理系统
- [ ] 构建场景图
- [ ] 设计相机系统
- [ ] 实现渲染系统
- [ ] 构建应用框架

## 🔗 延伸阅读

- [Game Engine Architecture](https://www.gameenginebook.com/)
- [Real-Time Rendering](http://www.realtimerendering.com/)
- [Bevy Engine Architecture](https://bevyengine.org/learn/book/)

---

**掌握架构设计，构建可扩展的 3D 渲染器！** 🚀
