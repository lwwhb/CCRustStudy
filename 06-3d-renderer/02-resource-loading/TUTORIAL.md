# 模块 6.2：资源加载系统 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 实现模型加载（glTF）
2. 掌握纹理加载和管理
3. 实现异步资源加载
4. 构建资源缓存系统
5. 处理资源依赖

## 🎯 为什么需要资源加载系统？

### 直接加载 vs 系统化加载

**直接加载（问题）**：
```rust
// 每次都重新加载
let mesh1 = load_obj("model.obj");
let mesh2 = load_obj("model.obj");  // 重复加载！

// 阻塞主线程
let texture = load_png("texture.png");  // 等待 I/O

问题：
- 重复加载浪费内存
- 阻塞主线程
- 难以管理
- 无法追踪依赖
```

**系统化加载（解决方案）**：
```rust
// 自动缓存
let mesh1 = resources.load("model.obj").await;
let mesh2 = resources.load("model.obj").await;  // 返回缓存

// 异步加载
let texture = resources.load_async("texture.png");

优势：
- 自动去重
- 异步加载
- 统一管理
- 依赖追踪
```

### 资源加载流程

```
请求资源
    ↓
检查缓存
    ↓
    是 → 返回缓存
    ↓
    否 → 异步加载
    ↓
解析数据
    ↓
创建 GPU 资源
    ↓
存入缓存
    ↓
返回句柄
```

## 📖 核心概念详解

### 1. glTF 模型加载

glTF 是现代 3D 模型格式。

#### glTF 结构

```
glTF 文件包含：
- 场景（Scenes）
- 节点（Nodes）- 变换层级
- 网格（Meshes）- 几何数据
- 材质（Materials）- 外观
- 纹理（Textures）- 图像
- 动画（Animations）
- 皮肤（Skins）- 骨骼动画
```

#### 加载 glTF

```rust
use gltf;
use std::path::Path;

pub struct GltfLoader {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl GltfLoader {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self { device, queue }
    }

    pub async fn load<P: AsRef<Path>>(&self, path: P) -> Result<Model, LoadError> {
        // 加载 glTF 文件
        let (document, buffers, images) = gltf::import(path)?;

        let mut meshes = Vec::new();
        let mut materials = Vec::new();
        let mut textures = Vec::new();

        // 加载纹理
        for image in images {
            let texture = self.load_texture_from_image(image).await?;
            textures.push(texture);
        }

        // 加载材质
        for material in document.materials() {
            let mat = self.load_material(&material, &textures)?;
            materials.push(mat);
        }

        // 加载网格
        for mesh in document.meshes() {
            let m = self.load_mesh(&mesh, &buffers)?;
            meshes.push(m);
        }

        Ok(Model {
            meshes,
            materials,
            textures,
        })
    }

    fn load_mesh(
        &self,
        mesh: &gltf::Mesh,
        buffers: &[gltf::buffer::Data],
    ) -> Result<Mesh, LoadError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for primitive in mesh.primitives() {
            // 读取顶点数据
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            // 位置
            if let Some(positions) = reader.read_positions() {
                for position in positions {
                    vertices.push(Vertex {
                        position,
                        ..Default::default()
                    });
                }
            }

            // 法线
            if let Some(normals) = reader.read_normals() {
                for (i, normal) in normals.enumerate() {
                    vertices[i].normal = normal;
                }
            }

            // 纹理坐标
            if let Some(tex_coords) = reader.read_tex_coords(0) {
                for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                    vertices[i].tex_coords = tex_coord;
                }
            }

            // 索引
            if let Some(indices_reader) = reader.read_indices() {
                indices.extend(indices_reader.into_u32());
            }
        }

        // 创建 GPU 缓冲区
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Ok(Mesh {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        })
    }

    fn load_material(
        &self,
        material: &gltf::Material,
        textures: &[Texture],
    ) -> Result<Material, LoadError> {
        let pbr = material.pbr_metallic_roughness();

        let base_color = pbr.base_color_factor();
        let metallic = pbr.metallic_factor();
        let roughness = pbr.roughness_factor();

        // 基础颜色纹理
        let base_color_texture = pbr
            .base_color_texture()
            .map(|info| textures[info.texture().index()].clone());

        // 法线贴图
        let normal_texture = material
            .normal_texture()
            .map(|info| textures[info.texture().index()].clone());

        Ok(Material {
            base_color,
            metallic,
            roughness,
            base_color_texture,
            normal_texture,
        })
    }
}

#[derive(Debug)]
pub enum LoadError {
    IoError(std::io::Error),
    GltfError(gltf::Error),
    ImageError(image::ImageError),
}

impl From<std::io::Error> for LoadError {
    fn from(err: std::io::Error) -> Self {
        LoadError::IoError(err)
    }
}

impl From<gltf::Error> for LoadError {
    fn from(err: gltf::Error) -> Self {
        LoadError::GltfError(err)
    }
}
```

### 2. 纹理加载

#### 从文件加载纹理

```rust
use image;

pub struct TextureLoader {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl TextureLoader {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self { device, queue }
    }

    pub async fn load<P: AsRef<Path>>(&self, path: P) -> Result<Texture, LoadError> {
        // 加载图像
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        self.create_texture_from_bytes(
            &rgba,
            dimensions.0,
            dimensions.1,
        )
    }

    pub fn create_texture_from_bytes(
        &self,
        bytes: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Texture, LoadError> {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // 创建纹理
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 写入数据
        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        // 创建视图和采样器
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(Texture {
            texture,
            view,
            sampler,
        })
    }
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
```

#### Mipmap 生成

```rust
impl TextureLoader {
    pub fn create_texture_with_mipmaps(
        &self,
        bytes: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Texture, LoadError> {
        // 计算 mipmap 层数
        let mip_level_count = (width.max(height) as f32).log2().floor() as u32 + 1;

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // 创建纹理
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture with Mipmaps"),
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        // 写入基础层
        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        // 生成 mipmaps
        self.generate_mipmaps(&texture, mip_level_count);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(Texture {
            texture,
            view,
            sampler,
        })
    }

    fn generate_mipmaps(&self, texture: &wgpu::Texture, mip_level_count: u32) {
        // 使用 blit 生成 mipmaps
        // 实现略...
    }
}
```

### 3. 异步资源加载

#### 异步加载器

```rust
use tokio::sync::mpsc;
use std::sync::Arc;

pub struct AsyncResourceLoader {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    sender: mpsc::UnboundedSender<LoadRequest>,
}

enum LoadRequest {
    Texture {
        path: String,
        callback: Box<dyn FnOnce(Result<Texture, LoadError>) + Send>,
    },
    Model {
        path: String,
        callback: Box<dyn FnOnce(Result<Model, LoadError>) + Send>,
    },
}

impl AsyncResourceLoader {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();

        // 启动加载线程
        let device_clone = device.clone();
        let queue_clone = queue.clone();

        tokio::spawn(async move {
            while let Some(request) = receiver.recv().await {
                match request {
                    LoadRequest::Texture { path, callback } => {
                        let loader = TextureLoader::new(
                            device_clone.clone(),
                            queue_clone.clone(),
                        );
                        let result = loader.load(&path).await;
                        callback(result);
                    }
                    LoadRequest::Model { path, callback } => {
                        let loader = GltfLoader::new(
                            device_clone.clone(),
                            queue_clone.clone(),
                        );
                        let result = loader.load(&path).await;
                        callback(result);
                    }
                }
            }
        });

        Self {
            device,
            queue,
            sender,
        }
    }

    pub fn load_texture<F>(&self, path: String, callback: F)
    where
        F: FnOnce(Result<Texture, LoadError>) + Send + 'static,
    {
        self.sender.send(LoadRequest::Texture {
            path,
            callback: Box::new(callback),
        }).unwrap();
    }

    pub fn load_model<F>(&self, path: String, callback: F)
    where
        F: FnOnce(Result<Model, LoadError>) + Send + 'static,
    {
        self.sender.send(LoadRequest::Model {
            path,
            callback: Box::new(callback),
        }).unwrap();
    }
}
```

### 4. 资源缓存系统

#### 缓存管理器

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ResourceCache {
    textures: Arc<RwLock<HashMap<String, Arc<Texture>>>>,
    models: Arc<RwLock<HashMap<String, Arc<Model>>>>,
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            textures: Arc::new(RwLock::new(HashMap::new())),
            models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_texture(&self, path: &str) -> Option<Arc<Texture>> {
        self.textures.read().unwrap().get(path).cloned()
    }

    pub fn insert_texture(&self, path: String, texture: Texture) -> Arc<Texture> {
        let texture = Arc::new(texture);
        self.textures.write().unwrap().insert(path, texture.clone());
        texture
    }

    pub fn get_model(&self, path: &str) -> Option<Arc<Model>> {
        self.models.read().unwrap().get(path).cloned()
    }

    pub fn insert_model(&self, path: String, model: Model) -> Arc<Model> {
        let model = Arc::new(model);
        self.models.write().unwrap().insert(path, model.clone());
        model
    }

    pub fn clear(&self) {
        self.textures.write().unwrap().clear();
        self.models.write().unwrap().clear();
    }

    pub fn memory_usage(&self) -> usize {
        // 计算内存使用
        let textures = self.textures.read().unwrap();
        let models = self.models.read().unwrap();

        let texture_mem: usize = textures.values()
            .map(|t| t.memory_size())
            .sum();

        let model_mem: usize = models.values()
            .map(|m| m.memory_size())
            .sum();

        texture_mem + model_mem
    }
}
```

### 5. 完整资源管理器

#### 统一资源管理

```rust
pub struct ResourceManager {
    cache: ResourceCache,
    loader: AsyncResourceLoader,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl ResourceManager {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            cache: ResourceCache::new(),
            loader: AsyncResourceLoader::new(device.clone(), queue.clone()),
            device,
            queue,
        }
    }

    pub async fn load_texture(&self, path: &str) -> Result<Arc<Texture>, LoadError> {
        // 检查缓存
        if let Some(texture) = self.cache.get_texture(path) {
            return Ok(texture);
        }

        // 加载纹理
        let loader = TextureLoader::new(self.device.clone(), self.queue.clone());
        let texture = loader.load(path).await?;

        // 存入缓存
        Ok(self.cache.insert_texture(path.to_string(), texture))
    }

    pub async fn load_model(&self, path: &str) -> Result<Arc<Model>, LoadError> {
        // 检查缓存
        if let Some(model) = self.cache.get_model(path) {
            return Ok(model);
        }

        // 加载模型
        let loader = GltfLoader::new(self.device.clone(), self.queue.clone());
        let model = loader.load(path).await?;

        // 存入缓存
        Ok(self.cache.insert_model(path.to_string(), model))
    }

    pub fn load_texture_async<F>(&self, path: String, callback: F)
    where
        F: FnOnce(Result<Arc<Texture>, LoadError>) + Send + 'static,
    {
        // 检查缓存
        if let Some(texture) = self.cache.get_texture(&path) {
            callback(Ok(texture));
            return;
        }

        // 异步加载
        let cache = self.cache.clone();
        let path_clone = path.clone();

        self.loader.load_texture(path, move |result| {
            let result = result.map(|texture| {
                cache.insert_texture(path_clone, texture)
            });
            callback(result);
        });
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn memory_usage(&self) -> usize {
        self.cache.memory_usage()
    }
}
```

## 💻 实战项目：资源加载演示

### 项目需求

构建一个资源加载演示程序，支持：
1. 加载 glTF 模型
2. 加载纹理
3. 异步加载
4. 资源缓存
5. 内存监控

### 完整示例

```rust
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct App {
    renderer: Renderer,
    resources: ResourceManager,
    models: Vec<Arc<Model>>,
}

impl App {
    async fn new(window: &Window) -> Self {
        let renderer = Renderer::new(window).await;
        let resources = ResourceManager::new(
            renderer.device.clone(),
            renderer.queue.clone(),
        );

        Self {
            renderer,
            resources,
            models: Vec::new(),
        }
    }

    async fn load_scene(&mut self) {
        println!("加载场景...");

        // 加载模型
        match self.resources.load_model("assets/models/scene.gltf").await {
            Ok(model) => {
                println!("模型加载成功");
                self.models.push(model);
            }
            Err(e) => {
                eprintln!("模型加载失败: {:?}", e);
            }
        }

        // 打印内存使用
        let mem = self.resources.memory_usage();
        println!("内存使用: {} MB", mem / 1024 / 1024);
    }

    fn render(&mut self) {
        // 渲染所有模型
        for model in &self.models {
            // 渲染模型...
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("资源加载演示")
        .build(&event_loop)
        .unwrap();

    let mut app = pollster::block_on(App::new(&window));

    // 加载场景
    pollster::block_on(app.load_scene());

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                app.render();
            }
            _ => {}
        }
    }).unwrap();
}
```

## 🔍 深入理解

### 资源加载优化

```
1. 批量加载
   - 一次加载多个资源
   - 减少 I/O 次数

2. 流式加载
   - 边加载边显示
   - 提升用户体验

3. 预加载
   - 提前加载可能需要的资源
   - 减少等待时间

4. 延迟加载
   - 需要时才加载
   - 节省内存

5. 压缩
   - 减少文件大小
   - 加快加载速度
```

### 内存管理

```
1. 引用计数
   - Arc 自动管理生命周期
   - 不再使用时自动释放

2. 弱引用
   - 避免循环引用
   - 缓存可以被清理

3. LRU 缓存
   - 最近最少使用
   - 自动淘汰旧资源

4. 内存限制
   - 设置最大内存使用
   - 超过限制时清理
```

## 📝 练习题

### 练习 1：实现 OBJ 加载器
```rust
// 实现简单的 OBJ 文件加载器
pub struct ObjLoader;

impl ObjLoader {
    pub fn load(path: &str) -> Result<Mesh, LoadError> {
        // 你的代码
    }
}
```

### 练习 2：实现 LRU 缓存
```rust
// 实现最近最少使用缓存
pub struct LruCache<K, V> {
    // 你的代码
}
```

### 练习 3：实现进度追踪
```rust
// 实现加载进度追踪
pub struct LoadingProgress {
    total: usize,
    loaded: usize,
}
```

## 🎯 学习检查清单

- [ ] 理解 glTF 格式
- [ ] 实现模型加载
- [ ] 实现纹理加载
- [ ] 实现异步加载
- [ ] 实现资源缓存
- [ ] 处理加载错误
- [ ] 监控内存使用
- [ ] 优化加载性能

## 🔗 延伸阅读

- [glTF 规范](https://www.khronos.org/gltf/)
- [image 库文档](https://docs.rs/image/)
- [tokio 异步编程](https://tokio.rs/)

## 🚀 下一步

完成本模块后，继续学习：
- 模块 6.3：相机与输入系统
- 模块 6.4：光照与材质系统

---

**掌握资源加载，构建完整的 3D 应用！** 🚀
