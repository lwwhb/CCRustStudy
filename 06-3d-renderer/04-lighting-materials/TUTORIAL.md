# 模块 6.4：光照与材质系统 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 实现 PBR（基于物理的渲染）
2. 掌握多光源系统
3. 实现材质系统
4. 学习阴影映射
5. 实现环境光遮蔽

## 🎯 为什么需要光照系统？

### 简单着色 vs 真实光照

**简单着色（不真实）**：
```wgsl
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);  // 纯红色
}

问题：
- 没有深度感
- 没有立体感
- 不真实
```

**真实光照（逼真）**：
```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ambient = calculate_ambient();
    let diffuse = calculate_diffuse();
    let specular = calculate_specular();
    
    let color = ambient + diffuse + specular;
    return vec4<f32>(color, 1.0);
}

优势：
- 有深度感
- 有立体感
- 真实感强
```

### 光照模型对比

```
Phong（传统）:
- 环境光 + 漫反射 + 镜面反射
- 简单快速
- 不够真实

PBR（现代）:
- 基于物理
- 能量守恒
- 更真实
- 行业标准
```

## 📖 核心概念详解

### 1. PBR 基础

#### PBR 材质参数

```rust
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PbrMaterial {
    // 基础颜色（反照率）
    pub base_color: [f32; 4],
    
    // 金属度（0 = 非金属，1 = 金属）
    pub metallic: f32,
    
    // 粗糙度（0 = 光滑，1 = 粗糙）
    pub roughness: f32,
    
    // 环境光遮蔽
    pub ao: f32,
    
    // 自发光
    pub emissive: [f32; 3],
    
    _padding: f32,
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            base_color: [1.0, 1.0, 1.0, 1.0],
            metallic: 0.0,
            roughness: 0.5,
            ao: 1.0,
            emissive: [0.0, 0.0, 0.0],
            _padding: 0.0,
        }
    }
}
```

**材质参数说明**：
```
Base Color（基础颜色）:
- 物体的固有颜色
- 非金属：漫反射颜色
- 金属：反射颜色

Metallic（金属度）:
- 0.0 = 绝缘体（塑料、木头）
- 1.0 = 金属（铁、金、铜）
- 中间值很少使用

Roughness（粗糙度）:
- 0.0 = 完全光滑（镜面）
- 1.0 = 完全粗糙（哑光）
- 控制高光大小

AO（环境光遮蔽）:
- 模拟间接光照
- 缝隙和凹陷处较暗
```

#### PBR 着色器

```wgsl
// 材质 Uniform
struct Material {
    base_color: vec4<f32>,
    metallic: f32,
    roughness: f32,
    ao: f32,
    _padding: f32,
}

@group(1) @binding(0)
var<uniform> material: Material;

// 光照 Uniform
struct Light {
    position: vec3<f32>,
    _padding1: f32,
    color: vec3<f32>,
    intensity: f32,
}

@group(2) @binding(0)
var<uniform> light: Light;

// 相机位置
@group(0) @binding(1)
var<uniform> camera_position: vec3<f32>;

// PBR 着色函数
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 归一化向量
    let N = normalize(in.normal);
    let V = normalize(camera_position - in.world_position);
    let L = normalize(light.position - in.world_position);
    let H = normalize(V + L);
    
    // 计算角度
    let NdotL = max(dot(N, L), 0.0);
    let NdotV = max(dot(N, V), 0.0);
    let NdotH = max(dot(N, H), 0.0);
    let VdotH = max(dot(V, H), 0.0);
    
    // 基础颜色
    let albedo = material.base_color.rgb;
    
    // F0（0度角的菲涅尔反射率）
    var F0 = vec3<f32>(0.04);  // 非金属默认值
    F0 = mix(F0, albedo, material.metallic);
    
    // Cook-Torrance BRDF
    let D = distribution_ggx(NdotH, material.roughness);
    let F = fresnel_schlick(VdotH, F0);
    let G = geometry_smith(NdotV, NdotL, material.roughness);
    
    // 镜面反射
    let numerator = D * F * G;
    let denominator = 4.0 * NdotV * NdotL + 0.0001;
    let specular = numerator / denominator;
    
    // 能量守恒
    let kS = F;  // 镜面反射比例
    var kD = vec3<f32>(1.0) - kS;  // 漫反射比例
    kD *= 1.0 - material.metallic;  // 金属没有漫反射
    
    // 漫反射
    let diffuse = kD * albedo / PI;
    
    // 最终颜色
    let radiance = light.color * light.intensity * NdotL;
    let Lo = (diffuse + specular) * radiance;
    
    // 环境光
    let ambient = vec3<f32>(0.03) * albedo * material.ao;
    
    let color = ambient + Lo;
    
    // HDR 色调映射
    let mapped = color / (color + vec3<f32>(1.0));
    
    // Gamma 校正
    let gamma_corrected = pow(mapped, vec3<f32>(1.0 / 2.2));
    
    return vec4<f32>(gamma_corrected, 1.0);
}

// 法线分布函数（GGX/Trowbridge-Reitz）
fn distribution_ggx(NdotH: f32, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH2 = NdotH * NdotH;
    
    let num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
    
    return num / denom;
}

// 菲涅尔方程（Schlick 近似）
fn fresnel_schlick(cosTheta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

// 几何函数（Smith）
fn geometry_smith(NdotV: f32, NdotL: f32, roughness: f32) -> f32 {
    let ggx2 = geometry_schlick_ggx(NdotV, roughness);
    let ggx1 = geometry_schlick_ggx(NdotL, roughness);
    return ggx1 * ggx2;
}

fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    
    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    
    return num / denom;
}

const PI: f32 = 3.14159265359;
```

### 2. 多光源系统

#### 光源类型

```rust
#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Directional,  // 方向光（太阳）
    Point,        // 点光源（灯泡）
    Spot,         // 聚光灯（手电筒）
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    // 位置（点光源和聚光灯）
    pub position: [f32; 3],
    pub light_type: u32,
    
    // 方向（方向光和聚光灯）
    pub direction: [f32; 3],
    pub intensity: f32,
    
    // 颜色
    pub color: [f32; 3],
    pub range: f32,  // 点光源范围
    
    // 聚光灯参数
    pub inner_cutoff: f32,  // 内圆锥角
    pub outer_cutoff: f32,  // 外圆锥角
    
    _padding: [f32; 2],
}

impl Light {
    // 方向光
    pub fn directional(direction: na::Vector3<f32>, color: na::Vector3<f32>) -> Self {
        Self {
            position: [0.0; 3],
            light_type: 0,
            direction: direction.normalize().into(),
            intensity: 1.0,
            color: color.into(),
            range: 0.0,
            inner_cutoff: 0.0,
            outer_cutoff: 0.0,
            _padding: [0.0; 2],
        }
    }

    // 点光源
    pub fn point(position: na::Point3<f32>, color: na::Vector3<f32>, range: f32) -> Self {
        Self {
            position: position.coords.into(),
            light_type: 1,
            direction: [0.0; 3],
            intensity: 1.0,
            color: color.into(),
            range,
            inner_cutoff: 0.0,
            outer_cutoff: 0.0,
            _padding: [0.0; 2],
        }
    }

    // 聚光灯
    pub fn spot(
        position: na::Point3<f32>,
        direction: na::Vector3<f32>,
        color: na::Vector3<f32>,
        inner_angle: f32,
        outer_angle: f32,
    ) -> Self {
        Self {
            position: position.coords.into(),
            light_type: 2,
            direction: direction.normalize().into(),
            intensity: 1.0,
            color: color.into(),
            range: 50.0,
            inner_cutoff: inner_angle.cos(),
            outer_cutoff: outer_angle.cos(),
            _padding: [0.0; 2],
        }
    }
}
```

#### 多光源着色器

```wgsl
struct Light {
    position: vec3<f32>,
    light_type: u32,
    direction: vec3<f32>,
    intensity: f32,
    color: vec3<f32>,
    range: f32,
    inner_cutoff: f32,
    outer_cutoff: f32,
}

@group(2) @binding(0)
var<storage, read> lights: array<Light>;

@group(2) @binding(1)
var<uniform> num_lights: u32;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.normal);
    let V = normalize(camera_position - in.world_position);
    
    var Lo = vec3<f32>(0.0);
    
    // 遍历所有光源
    for (var i = 0u; i < num_lights; i++) {
        let light = lights[i];
        
        var L: vec3<f32>;
        var radiance: vec3<f32>;
        var attenuation = 1.0;
        
        if light.light_type == 0u {
            // 方向光
            L = normalize(-light.direction);
            radiance = light.color * light.intensity;
        } else if light.light_type == 1u {
            // 点光源
            L = normalize(light.position - in.world_position);
            let distance = length(light.position - in.world_position);
            attenuation = 1.0 / (distance * distance);
            attenuation = min(attenuation, 1.0 / (light.range * light.range));
            radiance = light.color * light.intensity * attenuation;
        } else {
            // 聚光灯
            L = normalize(light.position - in.world_position);
            let distance = length(light.position - in.world_position);
            
            let theta = dot(L, normalize(-light.direction));
            let epsilon = light.inner_cutoff - light.outer_cutoff;
            let spot_intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);
            
            attenuation = 1.0 / (distance * distance);
            radiance = light.color * light.intensity * attenuation * spot_intensity;
        }
        
        // 计算该光源的贡献
        let H = normalize(V + L);
        let NdotL = max(dot(N, L), 0.0);
        
        // PBR 计算...
        let contribution = calculate_pbr(N, V, L, H, NdotL);
        Lo += contribution * radiance;
    }
    
    // 环境光
    let ambient = vec3<f32>(0.03) * material.base_color.rgb * material.ao;
    
    let color = ambient + Lo;
    
    // 色调映射和 Gamma 校正
    let mapped = tone_mapping(color);
    let gamma_corrected = pow(mapped, vec3<f32>(1.0 / 2.2));
    
    return vec4<f32>(gamma_corrected, 1.0);
}
```

### 3. 阴影映射

#### 阴影贴图创建

```rust
pub struct ShadowMap {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: u32,
}

impl ShadowMap {
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Map"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Shadow Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            size,
        }
    }
}
```

#### 阴影渲染

```rust
pub fn render_shadow_map(
    &mut self,
    encoder: &mut wgpu::CommandEncoder,
    light: &Light,
    scene: &Scene,
) {
    // 计算光源的视图投影矩阵
    let light_view = na::Matrix4::look_at_rh(
        &na::Point3::from(light.position),
        &(na::Point3::from(light.position) + na::Vector3::from(light.direction)),
        &na::Vector3::y(),
    );

    let light_proj = na::Matrix4::new_orthographic(
        -10.0, 10.0,  // left, right
        -10.0, 10.0,  // bottom, top
        0.1, 100.0,   // near, far
    );

    let light_space_matrix = light_proj * light_view;

    // 渲染到阴影贴图
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Shadow Pass"),
        color_attachments: &[],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &self.shadow_map.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    render_pass.set_pipeline(&self.shadow_pipeline);
    
    // 渲染场景中的所有物体
    for object in &scene.objects {
        // 设置变换矩阵
        // 渲染物体
    }
}
```

#### 阴影采样

```wgsl
@group(3) @binding(0)
var shadow_map: texture_depth_2d;

@group(3) @binding(1)
var shadow_sampler: sampler_comparison;

@group(3) @binding(2)
var<uniform> light_space_matrix: mat4x4<f32>;

fn calculate_shadow(world_pos: vec3<f32>) -> f32 {
    // 转换到光源空间
    let light_space_pos = light_space_matrix * vec4<f32>(world_pos, 1.0);
    
    // 透视除法
    var proj_coords = light_space_pos.xyz / light_space_pos.w;
    
    // 转换到 [0, 1] 范围
    proj_coords = proj_coords * 0.5 + 0.5;
    
    // 超出阴影贴图范围
    if proj_coords.x < 0.0 || proj_coords.x > 1.0 ||
       proj_coords.y < 0.0 || proj_coords.y > 1.0 ||
       proj_coords.z > 1.0 {
        return 1.0;  // 无阴影
    }
    
    // PCF（百分比渐近过滤）
    var shadow = 0.0;
    let texel_size = 1.0 / 2048.0;  // 阴影贴图大小
    
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let uv = proj_coords.xy + offset;
            shadow += textureSampleCompare(
                shadow_map,
                shadow_sampler,
                uv,
                proj_coords.z - 0.005  // 偏移避免阴影痤疮
            );
        }
    }
    
    shadow /= 9.0;  // 平均
    
    return shadow;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // ... PBR 计算
    
    // 应用阴影
    let shadow = calculate_shadow(in.world_position);
    let final_color = color * shadow;
    
    return vec4<f32>(final_color, 1.0);
}
```

### 4. 材质系统

#### 材质管理器

```rust
pub struct MaterialManager {
    materials: HashMap<String, Handle<Material>>,
    storage: ResourceStorage<Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            storage: ResourceStorage::new(),
        }
    }

    pub fn create_material(&mut self, name: String, material: Material) -> Handle<Material> {
        let handle = self.storage.insert(material);
        self.materials.insert(name, handle);
        handle
    }

    pub fn get(&self, name: &str) -> Option<Handle<Material>> {
        self.materials.get(name).copied()
    }

    pub fn get_material(&self, handle: Handle<Material>) -> Option<Arc<Material>> {
        self.storage.get(handle)
    }
}

pub struct Material {
    pub pbr_params: PbrMaterial,
    pub base_color_texture: Option<Handle<Texture>>,
    pub normal_texture: Option<Handle<Texture>>,
    pub metallic_roughness_texture: Option<Handle<Texture>>,
    pub ao_texture: Option<Handle<Texture>>,
    pub emissive_texture: Option<Handle<Texture>>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            pbr_params: PbrMaterial::default(),
            base_color_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
            ao_texture: None,
            emissive_texture: None,
        }
    }

    // 预设材质
    pub fn plastic(color: [f32; 3]) -> Self {
        Self {
            pbr_params: PbrMaterial {
                base_color: [color[0], color[1], color[2], 1.0],
                metallic: 0.0,
                roughness: 0.5,
                ..Default::default()
            },
            ..Self::new()
        }
    }

    pub fn metal(color: [f32; 3], roughness: f32) -> Self {
        Self {
            pbr_params: PbrMaterial {
                base_color: [color[0], color[1], color[2], 1.0],
                metallic: 1.0,
                roughness,
                ..Default::default()
            },
            ..Self::new()
        }
    }

    pub fn glass() -> Self {
        Self {
            pbr_params: PbrMaterial {
                base_color: [1.0, 1.0, 1.0, 0.1],
                metallic: 0.0,
                roughness: 0.0,
                ..Default::default()
            },
            ..Self::new()
        }
    }
}
```

## 💻 实战项目：PBR 材质球

### 项目需求

创建一个材质球展示程序，展示不同的 PBR 材质效果。

### 实现步骤

```rust
// 创建多个材质球
let materials = vec![
    Material::plastic([1.0, 0.0, 0.0]),  // 红色塑料
    Material::metal([0.8, 0.8, 0.8], 0.2),  // 光滑金属
    Material::metal([0.8, 0.8, 0.8], 0.8),  // 粗糙金属
    Material::glass(),  // 玻璃
];

// 排列材质球
for (i, material) in materials.iter().enumerate() {
    let x = (i as f32 - 1.5) * 3.0;
    let sphere = create_sphere(1.0);
    scene.add_object(Object {
        mesh: sphere,
        material: material.clone(),
        transform: Transform::from_translation(x, 0.0, 0.0),
    });
}

// 添加光源
scene.add_light(Light::directional(
    na::Vector3::new(-1.0, -1.0, -1.0),
    na::Vector3::new(1.0, 1.0, 1.0),
));

scene.add_light(Light::point(
    na::Point3::new(5.0, 5.0, 5.0),
    na::Vector3::new(1.0, 0.8, 0.6),
    20.0,
));
```

## 📝 练习题

### 练习 1：实现不同的材质预设
创建木头、石头、布料等材质预设。

### 练习 2：实现 IBL（基于图像的光照）
使用环境贴图提供更真实的环境光照。

### 练习 3：实现级联阴影贴图
改善大场景的阴影质量。

## 🎯 学习检查清单

- [ ] 理解 PBR 的基本原理
- [ ] 掌握材质参数的含义
- [ ] 实现多光源系统
- [ ] 实现阴影映射
- [ ] 创建材质系统
- [ ] 理解能量守恒
- [ ] 掌握色调映射和 Gamma 校正

## 🔗 延伸阅读

- [PBR 理论](https://learnopengl.com/PBR/Theory)
- [Real Shading in Unreal Engine 4](https://blog.selfshadow.com/publications/s2013-shading-course/)
- [Physically Based Rendering Book](https://www.pbr-book.org/)

---

**掌握 PBR，创造真实的 3D 世界！** 🌟
