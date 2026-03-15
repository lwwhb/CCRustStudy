# 模块 6.5：高级特性与优化 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 实现视锥体剔除
2. 掌握 LOD（细节层次）
3. 实现后处理效果
4. 学习性能优化技巧
5. 完成跨平台测试

## 🎯 为什么需要优化？

### 未优化 vs 优化后

**未优化（性能差）**：
```rust
// 绘制所有物体
for object in scene.objects {
    draw(object);  // 包括看不见的
}

问题：
- 绘制不可见物体
- 使用高精度模型
- 无批处理
- 帧率低
```

**优化后（性能好）**：
```rust
// 视锥体剔除
let visible = frustum_culling(scene.objects);

// LOD 选择
for object in visible {
    let lod = select_lod(object, distance);
    draw_batched(lod);
}

优势：
- 只绘制可见物体
- 根据距离选择细节
- 批量绘制
- 帧率高
```

### 优化技术

```
1. 剔除（Culling）
   - 视锥体剔除
   - 背面剔除
   - 遮挡剔除

2. LOD（Level of Detail）
   - 距离 LOD
   - 屏幕空间 LOD

3. 批处理（Batching）
   - 静态批处理
   - 动态批处理
   - 实例化渲染

4. 后处理（Post-processing）
   - 抗锯齿
   - 泛光
   - 色调映射
```

## 📖 核心概念详解

### 1. 视锥体剔除

视锥体剔除跳过相机视野外的物体。

#### 视锥体平面

```rust
use nalgebra as na;

// 视锥体的 6 个平面
pub struct Frustum {
    planes: [Plane; 6],
}

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: na::Vector3<f32>,
    distance: f32,
}

impl Plane {
    pub fn new(normal: na::Vector3<f32>, distance: f32) -> Self {
        Self { normal, distance }
    }

    // 点到平面的距离
    pub fn distance_to_point(&self, point: &na::Point3<f32>) -> f32 {
        self.normal.dot(&point.coords) + self.distance
    }
}

impl Frustum {
    // 从视图投影矩阵提取视锥体
    pub fn from_matrix(view_proj: &na::Matrix4<f32>) -> Self {
        let m = view_proj;

        // 左平面
        let left = Plane::new(
            na::Vector3::new(m[(3, 0)] + m[(0, 0)], m[(3, 1)] + m[(0, 1)], m[(3, 2)] + m[(0, 2)]),
            m[(3, 3)] + m[(0, 3)],
        );

        // 右平面
        let right = Plane::new(
            na::Vector3::new(m[(3, 0)] - m[(0, 0)], m[(3, 1)] - m[(0, 1)], m[(3, 2)] - m[(0, 2)]),
            m[(3, 3)] - m[(0, 3)],
        );

        // 底平面
        let bottom = Plane::new(
            na::Vector3::new(m[(3, 0)] + m[(1, 0)], m[(3, 1)] + m[(1, 1)], m[(3, 2)] + m[(1, 2)]),
            m[(3, 3)] + m[(1, 3)],
        );

        // 顶平面
        let top = Plane::new(
            na::Vector3::new(m[(3, 0)] - m[(1, 0)], m[(3, 1)] - m[(1, 1)], m[(3, 2)] - m[(1, 2)]),
            m[(3, 3)] - m[(1, 3)],
        );

        // 近平面
        let near = Plane::new(
            na::Vector3::new(m[(3, 0)] + m[(2, 0)], m[(3, 1)] + m[(2, 1)], m[(3, 2)] + m[(2, 2)]),
            m[(3, 3)] + m[(2, 3)],
        );

        // 远平面
        let far = Plane::new(
            na::Vector3::new(m[(3, 0)] - m[(2, 0)], m[(3, 1)] - m[(2, 1)], m[(3, 2)] - m[(2, 2)]),
            m[(3, 3)] - m[(2, 3)],
        );

        // 归一化平面
        let mut planes = [left, right, bottom, top, near, far];
        for plane in &mut planes {
            let length = plane.normal.magnitude();
            plane.normal /= length;
            plane.distance /= length;
        }

        Self { planes }
    }

    // 检查球体是否在视锥体内
    pub fn contains_sphere(&self, center: &na::Point3<f32>, radius: f32) -> bool {
        for plane in &self.planes {
            if plane.distance_to_point(center) < -radius {
                return false;  // 完全在平面外
            }
        }
        true
    }

    // 检查 AABB 是否在视锥体内
    pub fn contains_aabb(&self, min: &na::Point3<f32>, max: &na::Point3<f32>) -> bool {
        for plane in &self.planes {
            // 找到最远的顶点
            let mut p = *min;
            if plane.normal.x >= 0.0 { p.x = max.x; }
            if plane.normal.y >= 0.0 { p.y = max.y; }
            if plane.normal.z >= 0.0 { p.z = max.z; }

            if plane.distance_to_point(&p) < 0.0 {
                return false;  // 完全在平面外
            }
        }
        true
    }
}
```

#### 应用剔除

```rust
pub struct CullingSystem {
    frustum: Frustum,
}

impl CullingSystem {
    pub fn new(view_proj: &na::Matrix4<f32>) -> Self {
        Self {
            frustum: Frustum::from_matrix(view_proj),
        }
    }

    pub fn update(&mut self, view_proj: &na::Matrix4<f32>) {
        self.frustum = Frustum::from_matrix(view_proj);
    }

    pub fn cull_objects(&self, objects: &[GameObject]) -> Vec<&GameObject> {
        objects
            .iter()
            .filter(|obj| {
                // 使用包围球测试
                self.frustum.contains_sphere(
                    &obj.position,
                    obj.bounding_radius,
                )
            })
            .collect()
    }
}

// 使用示例
let culling = CullingSystem::new(&camera.view_proj_matrix());
let visible_objects = culling.cull_objects(&scene.objects);

// 只渲染可见物体
for object in visible_objects {
    renderer.draw(object);
}
```

**性能提升**：
```
场景：1000 个物体
视野内：200 个物体

未剔除：绘制 1000 个（100%）
剔除后：绘制 200 个（20%）

性能提升：5 倍
```

### 2. LOD 系统

LOD 根据距离使用不同细节的模型。

#### LOD 级别

```rust
#[derive(Debug, Clone)]
pub struct LodLevel {
    pub mesh: Handle<Mesh>,
    pub distance: f32,  // 切换距离
}

pub struct LodGroup {
    pub levels: Vec<LodLevel>,
}

impl LodGroup {
    pub fn new() -> Self {
        Self {
            levels: Vec::new(),
        }
    }

    pub fn add_level(&mut self, mesh: Handle<Mesh>, distance: f32) {
        self.levels.push(LodLevel { mesh, distance });
        // 按距离排序
        self.levels.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    }

    // 根据距离选择 LOD
    pub fn select_lod(&self, distance: f32) -> Option<Handle<Mesh>> {
        for level in &self.levels {
            if distance < level.distance {
                return Some(level.mesh);
            }
        }
        // 返回最低细节
        self.levels.last().map(|l| l.mesh)
    }
}
```

#### LOD 管理器

```rust
pub struct LodManager {
    lod_groups: HashMap<EntityId, LodGroup>,
}

impl LodManager {
    pub fn new() -> Self {
        Self {
            lod_groups: HashMap::new(),
        }
    }

    pub fn register(&mut self, entity: EntityId, lod_group: LodGroup) {
        self.lod_groups.insert(entity, lod_group);
    }

    pub fn update(&self, entity: EntityId, camera_position: &na::Point3<f32>, object_position: &na::Point3<f32>) -> Option<Handle<Mesh>> {
        let lod_group = self.lod_groups.get(&entity)?;
        let distance = na::distance(camera_position, object_position);
        lod_group.select_lod(distance)
    }
}

// 使用示例
let mut lod_manager = LodManager::new();

// 注册 LOD 组
let mut lod_group = LodGroup::new();
lod_group.add_level(high_detail_mesh, 10.0);   // 0-10m
lod_group.add_level(medium_detail_mesh, 50.0); // 10-50m
lod_group.add_level(low_detail_mesh, 100.0);   // 50-100m
lod_manager.register(entity_id, lod_group);

// 渲染时选择 LOD
for object in visible_objects {
    if let Some(mesh) = lod_manager.update(
        object.id,
        &camera.position,
        &object.position,
    ) {
        renderer.draw_mesh(mesh, &object.transform);
    }
}
```

**LOD 策略**：
```
LOD 0（高细节）：
- 距离：0-10m
- 三角形：10,000

LOD 1（中细节）：
- 距离：10-50m
- 三角形：2,500

LOD 2（低细节）：
- 距离：50-100m
- 三角形：500

LOD 3（极低细节）：
- 距离：100m+
- 三角形：100
```

### 3. 实例化渲染

实例化一次绘制多个相同物体。

#### 实例数据

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub model_matrix: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 4]; 4],
}

impl InstanceData {
    pub fn new(transform: &Transform) -> Self {
        let model = transform.matrix();
        let normal = model.try_inverse().unwrap().transpose();

        Self {
            model_matrix: model.into(),
            normal_matrix: normal.into(),
        }
    }
}
```

#### 实例化绘制

```rust
pub struct InstancedRenderer {
    instance_buffer: wgpu::Buffer,
    max_instances: usize,
}

impl InstancedRenderer {
    pub fn new(device: &wgpu::Device, max_instances: usize) -> Self {
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (max_instances * std::mem::size_of::<InstanceData>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            instance_buffer,
            max_instances,
        }
    }

    pub fn draw_instanced(
        &self,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
        mesh: &Mesh,
        instances: &[InstanceData],
    ) {
        if instances.is_empty() || instances.len() > self.max_instances {
            return;
        }

        // 更新实例缓冲区
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(instances),
        );

        // 绑定缓冲区
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        // 实例化绘制
        render_pass.draw_indexed(
            0..mesh.num_indices,
            0,
            0..instances.len() as u32,
        );
    }
}

// 使用示例
let instances: Vec<InstanceData> = trees
    .iter()
    .map(|tree| InstanceData::new(&tree.transform))
    .collect();

instanced_renderer.draw_instanced(
    &queue,
    &mut render_pass,
    &tree_mesh,
    &instances,
);
```

**性能对比**：
```
绘制 1000 棵树：

普通绘制：
- 1000 次 draw call
- CPU 开销大
- 帧率：30 FPS

实例化绘制：
- 1 次 draw call
- CPU 开销小
- 帧率：120 FPS

性能提升：4 倍
```

### 4. 后处理效果

#### 后处理框架

```rust
pub struct PostProcessor {
    render_texture: wgpu::Texture,
    render_view: wgpu::TextureView,
    pipeline: wgpu::RenderPipeline,
}

impl PostProcessor {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        // 创建渲染纹理
        let render_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let render_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 创建后处理管线
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Post Process Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("post_process.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Post Process Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Post Process Pipeline"),
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
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            render_texture,
            render_view,
            pipeline,
        }
    }

    pub fn render_view(&self) -> &wgpu::TextureView {
        &self.render_view
    }

    pub fn apply(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Post Process Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.draw(0..3, 0..1);  // 全屏三角形
    }
}
```

#### 后处理着色器

```wgsl
// post_process.wgsl

// 全屏三角形顶点着色器
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // 生成全屏三角形
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    return vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
}

@group(0) @binding(0)
var t_scene: texture_2d<f32>;

@group(0) @binding(1)
var s_scene: sampler;

// 片段着色器
@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = position.xy / vec2<f32>(textureDimensions(t_scene));
    var color = textureSample(t_scene, s_scene, uv);
    
    // 色调映射（Reinhard）
    color = color / (color + vec4<f32>(1.0));
    
    // Gamma 校正
    color = pow(color, vec4<f32>(1.0 / 2.2));
    
    return color;
}
```

### 5. 性能分析

#### 性能计数器

```rust
use std::time::{Duration, Instant};

pub struct PerformanceCounter {
    frame_times: Vec<Duration>,
    max_samples: usize,
    last_frame: Instant,
}

impl PerformanceCounter {
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
            last_frame: Instant::now(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.last_frame = Instant::now();
    }

    pub fn end_frame(&mut self) {
        let frame_time = self.last_frame.elapsed();
        
        if self.frame_times.len() >= self.max_samples {
            self.frame_times.remove(0);
        }
        self.frame_times.push(frame_time);
    }

    pub fn average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            return Duration::ZERO;
        }

        let total: Duration = self.frame_times.iter().sum();
        total / self.frame_times.len() as u32
    }

    pub fn fps(&self) -> f64 {
        let avg = self.average_frame_time();
        if avg.is_zero() {
            return 0.0;
        }
        1.0 / avg.as_secs_f64()
    }

    pub fn min_frame_time(&self) -> Duration {
        self.frame_times.iter().copied().min().unwrap_or(Duration::ZERO)
    }

    pub fn max_frame_time(&self) -> Duration {
        self.frame_times.iter().copied().max().unwrap_or(Duration::ZERO)
    }
}

// 使用示例
let mut perf = PerformanceCounter::new(60);

loop {
    perf.begin_frame();
    
    // 渲染...
    
    perf.end_frame();
    
    if frame_count % 60 == 0 {
        println!("FPS: {:.1}", perf.fps());
        println!("Avg: {:.2}ms", perf.average_frame_time().as_secs_f64() * 1000.0);
        println!("Min: {:.2}ms", perf.min_frame_time().as_secs_f64() * 1000.0);
        println!("Max: {:.2}ms", perf.max_frame_time().as_secs_f64() * 1000.0);
    }
}
```

## 💻 实战项目：完整的 3D 渲染器

### 项目整合

```rust
pub struct Renderer3D {
    // 核心
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    
    // 系统
    culling: CullingSystem,
    lod_manager: LodManager,
    instanced_renderer: InstancedRenderer,
    post_processor: PostProcessor,
    
    // 性能
    perf_counter: PerformanceCounter,
}

impl Renderer3D {
    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        self.perf_counter.begin_frame();
        
        // 1. 更新剔除
        self.culling.update(&camera.view_proj_matrix());
        
        // 2. 剔除不可见物体
        let visible = self.culling.cull_objects(&scene.objects);
        
        // 3. LOD 选择
        let mut draw_calls = Vec::new();
        for object in visible {
            if let Some(mesh) = self.lod_manager.update(
                object.id,
                &camera.position,
                &object.position,
            ) {
                draw_calls.push((mesh, object.transform));
            }
        }
        
        // 4. 批处理和实例化
        // ...
        
        // 5. 渲染
        // ...
        
        // 6. 后处理
        // ...
        
        self.perf_counter.end_frame();
    }
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 实现视锥体剔除
- [ ] 实现 LOD 系统
- [ ] 使用实例化渲染
- [ ] 实现后处理效果
- [ ] 进行性能分析和优化
- [ ] 构建完整的 3D 渲染器
- [ ] 跨平台测试和部署

## 🔗 延伸阅读

- [Real-Time Rendering](http://www.realtimerendering.com/)
- [GPU Gems](https://developer.nvidia.com/gpugems/gpugems/contributors)
- [Learn OpenGL - Advanced Topics](https://learnopengl.com/Advanced-OpenGL/Advanced-Data)

## 🚀 下一步

完成本模块后，你已经掌握了：
- 完整的 3D 渲染器开发
- 高级优化技术
- 跨平台图形编程

可以继续：
1. 添加更多高级特性（全局光照、体积雾等）
2. 优化性能（多线程渲染、GPU Driven）
3. 构建自己的游戏引擎

---

**恭喜完成 3D 渲染器开发！** 🎉

*最后更新：2026-03-15*
