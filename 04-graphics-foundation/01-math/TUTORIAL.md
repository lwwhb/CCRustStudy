# 模块 4.1：图形数学基础 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 3D 图形的数学基础
2. 掌握向量和矩阵运算
3. 学习坐标变换
4. 理解投影矩阵
5. 使用 nalgebra 库

## 🎯 为什么需要图形数学？

### 3D 图形的本质

```
3D 图形 = 数学变换

从 3D 世界到 2D 屏幕：
1. 模型空间（Model Space）
   ↓ 模型矩阵
2. 世界空间（World Space）
   ↓ 视图矩阵
3. 相机空间（View Space）
   ↓ 投影矩阵
4. 裁剪空间（Clip Space）
   ↓ 透视除法
5. 屏幕空间（Screen Space）
```

**没有数学的困境**：
```rust
// 如何旋转一个点？
let point = (1.0, 0.0, 0.0);
// ??? 手动计算三角函数？

// 如何移动相机？
// ??? 手动更新每个顶点？

// 如何实现透视？
// ??? 完全不知道从何下手
```

**有了数学工具**：
```rust
use nalgebra as na;

// 旋转点
let point = na::Point3::new(1.0, 0.0, 0.0);
let rotation = na::Rotation3::from_axis_angle(
    &na::Vector3::z_axis(),
    std::f32::consts::PI / 4.0
);
let rotated = rotation * point;

// 移动相机
let view = na::Matrix4::look_at_rh(
    &eye, &target, &up
);

// 透视投影
let projection = na::Matrix4::new_perspective(
    aspect, fov, near, far
);
```

## 📖 核心概念详解

### 1. 向量（Vector）

向量表示方向和大小。

#### 向量基础

```rust
use nalgebra as na;

// 2D 向量
let v2 = na::Vector2::new(3.0, 4.0);

// 3D 向量
let v3 = na::Vector3::new(1.0, 2.0, 3.0);

// 4D 向量（齐次坐标）
let v4 = na::Vector4::new(1.0, 2.0, 3.0, 1.0);

// 零向量
let zero = na::Vector3::zeros();

// 单位向量
let unit_x = na::Vector3::x();  // (1, 0, 0)
let unit_y = na::Vector3::y();  // (0, 1, 0)
let unit_z = na::Vector3::z();  // (0, 0, 1)
```

#### 向量运算

```rust
let a = na::Vector3::new(1.0, 2.0, 3.0);
let b = na::Vector3::new(4.0, 5.0, 6.0);

// 加法
let sum = a + b;  // (5, 7, 9)

// 减法
let diff = a - b;  // (-3, -3, -3)

// 标量乘法
let scaled = a * 2.0;  // (2, 4, 6)

// 长度（模）
let length = a.magnitude();  // sqrt(1² + 2² + 3²) = sqrt(14)

// 归一化（单位向量）
let normalized = a.normalize();  // 长度为 1 的向量

// 点积（Dot Product）
let dot = a.dot(&b);  // 1*4 + 2*5 + 3*6 = 32

// 叉积（Cross Product）- 仅 3D
let cross = a.cross(&b);  // 垂直于 a 和 b 的向量
```

**点积的几何意义**：
```
a · b = |a| * |b| * cos(θ)

用途：
1. 计算夹角
   cos(θ) = (a · b) / (|a| * |b|)

2. 判断方向
   a · b > 0  → 夹角 < 90°（同向）
   a · b = 0  → 夹角 = 90°（垂直）
   a · b < 0  → 夹角 > 90°（反向）

3. 投影
   proj_b(a) = (a · b / |b|²) * b
```

**叉积的几何意义**：
```
a × b = 垂直于 a 和 b 的向量

|a × b| = |a| * |b| * sin(θ)

用途：
1. 计算法向量
2. 判断左右关系
3. 计算面积
```

#### 实际应用

```rust
// 计算两向量夹角
fn angle_between(a: &na::Vector3<f32>, b: &na::Vector3<f32>) -> f32 {
    let dot = a.dot(b);
    let lengths = a.magnitude() * b.magnitude();
    (dot / lengths).acos()
}

// 判断点是否在三角形内
fn point_in_triangle(
    p: &na::Point3<f32>,
    a: &na::Point3<f32>,
    b: &na::Point3<f32>,
    c: &na::Point3<f32>,
) -> bool {
    let v0 = c - a;
    let v1 = b - a;
    let v2 = p - a;

    let dot00 = v0.dot(&v0);
    let dot01 = v0.dot(&v1);
    let dot02 = v0.dot(&v2);
    let dot11 = v1.dot(&v1);
    let dot12 = v1.dot(&v2);

    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
}
```

### 2. 矩阵（Matrix）

矩阵用于表示变换。

#### 矩阵基础

```rust
// 4x4 矩阵（最常用）
let m = na::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
);

// 单位矩阵
let identity = na::Matrix4::identity();

// 零矩阵
let zero = na::Matrix4::zeros();
```

#### 矩阵运算

```rust
let a = na::Matrix4::identity();
let b = na::Matrix4::identity();

// 矩阵乘法（注意顺序！）
let product = a * b;

// 矩阵与向量相乘
let v = na::Vector4::new(1.0, 2.0, 3.0, 1.0);
let transformed = a * v;

// 转置
let transposed = a.transpose();

// 逆矩阵
if let Some(inverse) = a.try_inverse() {
    println!("逆矩阵: {}", inverse);
}
```

**矩阵乘法不可交换**：
```
A * B ≠ B * A

例如：
先旋转后平移 ≠ 先平移后旋转
```

### 3. 变换（Transformations）

#### 平移（Translation）

```rust
// 平移矩阵
let translation = na::Matrix4::new_translation(&na::Vector3::new(
    10.0,  // x 方向移动 10
    5.0,   // y 方向移动 5
    0.0,   // z 方向不动
));

// 应用平移
let point = na::Point3::new(0.0, 0.0, 0.0);
let translated = translation.transform_point(&point);
// 结果: (10, 5, 0)
```

**平移矩阵结构**：
```
[1  0  0  tx]
[0  1  0  ty]
[0  0  1  tz]
[0  0  0  1 ]

tx, ty, tz 是平移量
```

#### 缩放（Scaling）

```rust
// 缩放矩阵
let scaling = na::Matrix4::new_nonuniform_scaling(&na::Vector3::new(
    2.0,  // x 方向放大 2 倍
    2.0,  // y 方向放大 2 倍
    1.0,  // z 方向不变
));

// 均匀缩放
let uniform_scaling = na::Matrix4::new_scaling(2.0);
```

**缩放矩阵结构**：
```
[sx 0  0  0]
[0  sy 0  0]
[0  0  sz 0]
[0  0  0  1]

sx, sy, sz 是缩放因子
```

#### 旋转（Rotation）

```rust
use std::f32::consts::PI;

// 绕 Z 轴旋转 45 度
let rotation_z = na::Rotation3::from_axis_angle(
    &na::Vector3::z_axis(),
    PI / 4.0,
);

// 绕 X 轴旋转
let rotation_x = na::Rotation3::from_axis_angle(
    &na::Vector3::x_axis(),
    PI / 2.0,
);

// 绕 Y 轴旋转
let rotation_y = na::Rotation3::from_axis_angle(
    &na::Vector3::y_axis(),
    PI / 3.0,
);

// 转换为 4x4 矩阵
let rotation_matrix = rotation_z.to_homogeneous();

// 欧拉角旋转
let euler = na::Rotation3::from_euler_angles(
    PI / 4.0,  // roll (绕 X)
    PI / 6.0,  // pitch (绕 Y)
    PI / 3.0,  // yaw (绕 Z)
);
```

**旋转矩阵（绕 Z 轴）**：
```
[cos(θ)  -sin(θ)  0  0]
[sin(θ)   cos(θ)  0  0]
[0        0       1  0]
[0        0       0  1]
```

#### 组合变换

```rust
// 先缩放，再旋转，最后平移
let scale = na::Matrix4::new_scaling(2.0);
let rotation = na::Rotation3::from_axis_angle(
    &na::Vector3::z_axis(),
    PI / 4.0,
).to_homogeneous();
let translation = na::Matrix4::new_translation(&na::Vector3::new(10.0, 5.0, 0.0));

// 组合（注意顺序：从右到左）
let transform = translation * rotation * scale;

// 应用到点
let point = na::Point3::new(1.0, 0.0, 0.0);
let result = transform.transform_point(&point);
```

**变换顺序很重要**：
```
TRS (Translation * Rotation * Scale)
- 标准顺序
- 先缩放，再旋转，最后平移

SRT (Scale * Rotation * Translation)
- 不同的结果！
```

### 4. 相机变换

#### 视图矩阵（View Matrix）

```rust
// Look-at 矩阵
let eye = na::Point3::new(0.0, 0.0, 5.0);     // 相机位置
let target = na::Point3::new(0.0, 0.0, 0.0);  // 看向的点
let up = na::Vector3::new(0.0, 1.0, 0.0);     // 上方向

let view = na::Matrix4::look_at_rh(&eye, &target, &up);
```

**视图矩阵的作用**：
```
将世界坐标转换为相机坐标

相当于：
1. 将相机移动到原点
2. 将相机旋转到标准方向（看向 -Z 轴）
```

#### 投影矩阵（Projection Matrix）

**透视投影**：
```rust
let fov = PI / 4.0;        // 视场角 45 度
let aspect = 16.0 / 9.0;   // 宽高比
let near = 0.1;            // 近裁剪面
let far = 100.0;           // 远裁剪面

let projection = na::Matrix4::new_perspective(
    aspect,
    fov,
    near,
    far,
);
```

**正交投影**：
```rust
let left = -10.0;
let right = 10.0;
let bottom = -10.0;
let top = 10.0;
let near = 0.1;
let far = 100.0;

let projection = na::Matrix4::new_orthographic(
    left, right,
    bottom, top,
    near, far,
);
```

**透视 vs 正交**：
```
透视投影：
- 近大远小
- 符合人眼视觉
- 用于 3D 游戏

正交投影：
- 平行线保持平行
- 无透视效果
- 用于 CAD、2D 游戏
```

### 5. 四元数（Quaternion）

四元数用于表示旋转，避免万向锁。

```rust
// 从轴角创建
let axis = na::Unit::new_normalize(na::Vector3::new(0.0, 1.0, 0.0));
let angle = PI / 4.0;
let quat = na::UnitQuaternion::from_axis_angle(&axis, angle);

// 从欧拉角创建
let quat = na::UnitQuaternion::from_euler_angles(
    PI / 4.0,  // roll
    0.0,       // pitch
    0.0,       // yaw
);

// 四元数乘法（组合旋转）
let q1 = na::UnitQuaternion::from_axis_angle(&na::Vector3::y_axis(), PI / 4.0);
let q2 = na::UnitQuaternion::from_axis_angle(&na::Vector3::x_axis(), PI / 6.0);
let combined = q1 * q2;

// 转换为矩阵
let matrix = combined.to_homogeneous();

// 球面线性插值（SLERP）
let t = 0.5;  // 插值参数 [0, 1]
let interpolated = q1.slerp(&q2, t);
```

**四元数的优势**：
```
vs 欧拉角：
- 无万向锁
- 平滑插值

vs 矩阵：
- 更少的存储空间（4 个数 vs 9 个数）
- 更快的组合
- 易于归一化
```

## 💻 实战项目：3D 变换库

### 项目需求

实现一个 3D 变换库，支持：
1. 基本变换（平移、旋转、缩放）
2. 变换组合
3. 相机系统
4. 坐标空间转换

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
nalgebra = "0.32"
```

### 步骤 2：定义变换结构

```rust
use nalgebra as na;

pub struct Transform {
    position: na::Vector3<f32>,
    rotation: na::UnitQuaternion<f32>,
    scale: na::Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: na::Vector3::zeros(),
            rotation: na::UnitQuaternion::identity(),
            scale: na::Vector3::new(1.0, 1.0, 1.0),
        }
    }

    // 设置位置
    pub fn set_position(&mut self, pos: na::Vector3<f32>) {
        self.position = pos;
    }

    // 设置旋转（欧拉角）
    pub fn set_rotation_euler(&mut self, roll: f32, pitch: f32, yaw: f32) {
        self.rotation = na::UnitQuaternion::from_euler_angles(roll, pitch, yaw);
    }

    // 设置缩放
    pub fn set_scale(&mut self, scale: na::Vector3<f32>) {
        self.scale = scale;
    }

    // 获取变换矩阵
    pub fn matrix(&self) -> na::Matrix4<f32> {
        let translation = na::Matrix4::new_translation(&self.position);
        let rotation = self.rotation.to_homogeneous();
        let scale = na::Matrix4::new_nonuniform_scaling(&self.scale);

        translation * rotation * scale
    }

    // 变换点
    pub fn transform_point(&self, point: &na::Point3<f32>) -> na::Point3<f32> {
        self.matrix().transform_point(point)
    }

    // 变换向量
    pub fn transform_vector(&self, vector: &na::Vector3<f32>) -> na::Vector3<f32> {
        self.rotation * (self.scale.component_mul(vector))
    }
}
```

### 步骤 3：实现相机

```rust
pub struct Camera {
    position: na::Point3<f32>,
    target: na::Point3<f32>,
    up: na::Vector3<f32>,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: na::Point3::new(0.0, 0.0, 5.0),
            target: na::Point3::origin(),
            up: na::Vector3::y(),
            fov: std::f32::consts::PI / 4.0,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    // 视图矩阵
    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    // 投影矩阵
    pub fn projection_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }

    // 视图投影矩阵
    pub fn view_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    // 移动相机
    pub fn move_to(&mut self, position: na::Point3<f32>) {
        self.position = position;
    }

    // 看向目标
    pub fn look_at(&mut self, target: na::Point3<f32>) {
        self.target = target;
    }

    // 绕目标旋转
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let offset = self.position - self.target;
        let distance = offset.magnitude();

        // 计算当前角度
        let current_yaw = offset.z.atan2(offset.x);
        let current_pitch = (offset.y / distance).asin();

        // 更新角度
        let new_yaw = current_yaw + delta_yaw;
        let new_pitch = (current_pitch + delta_pitch)
            .clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);

        // 计算新位置
        let x = distance * new_pitch.cos() * new_yaw.cos();
        let y = distance * new_pitch.sin();
        let z = distance * new_pitch.cos() * new_yaw.sin();

        self.position = self.target + na::Vector3::new(x, y, z);
    }
}
```

### 步骤 4：坐标空间转换

```rust
pub struct CoordinateSpace {
    camera: Camera,
}

impl CoordinateSpace {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    // 世界空间 -> 裁剪空间
    pub fn world_to_clip(&self, point: &na::Point3<f32>) -> na::Vector4<f32> {
        let vp = self.camera.view_projection_matrix();
        vp * na::Vector4::new(point.x, point.y, point.z, 1.0)
    }

    // 裁剪空间 -> NDC（归一化设备坐标）
    pub fn clip_to_ndc(&self, clip: &na::Vector4<f32>) -> na::Vector3<f32> {
        na::Vector3::new(
            clip.x / clip.w,
            clip.y / clip.w,
            clip.z / clip.w,
        )
    }

    // NDC -> 屏幕空间
    pub fn ndc_to_screen(
        &self,
        ndc: &na::Vector3<f32>,
        width: f32,
        height: f32,
    ) -> na::Point2<f32> {
        na::Point2::new(
            (ndc.x + 1.0) * 0.5 * width,
            (1.0 - ndc.y) * 0.5 * height,
        )
    }

    // 完整转换：世界空间 -> 屏幕空间
    pub fn world_to_screen(
        &self,
        point: &na::Point3<f32>,
        width: f32,
        height: f32,
    ) -> Option<na::Point2<f32>> {
        let clip = self.world_to_clip(point);

        // 检查是否在视锥体内
        if clip.w <= 0.0 {
            return None;
        }

        let ndc = self.clip_to_ndc(&clip);

        // 检查是否在 NDC 范围内
        if ndc.x < -1.0 || ndc.x > 1.0 || ndc.y < -1.0 || ndc.y > 1.0 {
            return None;
        }

        Some(self.ndc_to_screen(&ndc, width, height))
    }
}
```

### 步骤 5：使用示例

```rust
fn main() {
    println!("=== 3D 变换库演示 ===\n");

    // 创建变换
    let mut transform = Transform::new();
    transform.set_position(na::Vector3::new(1.0, 2.0, 3.0));
    transform.set_rotation_euler(0.0, std::f32::consts::PI / 4.0, 0.0);
    transform.set_scale(na::Vector3::new(2.0, 2.0, 2.0));

    // 变换点
    let point = na::Point3::new(1.0, 0.0, 0.0);
    let transformed = transform.transform_point(&point);
    println!("变换后的点: {:?}", transformed);

    // 创建相机
    let mut camera = Camera::new(16.0 / 9.0);
    camera.move_to(na::Point3::new(0.0, 5.0, 10.0));
    camera.look_at(na::Point3::origin());

    // 获取矩阵
    let view = camera.view_matrix();
    let projection = camera.projection_matrix();
    println!("\n视图矩阵:\n{}", view);
    println!("\n投影矩阵:\n{}", projection);

    // 坐标转换
    let coord_space = CoordinateSpace::new(camera);
    let world_point = na::Point3::new(0.0, 0.0, 0.0);

    if let Some(screen_point) = coord_space.world_to_screen(&world_point, 1920.0, 1080.0) {
        println!("\n世界坐标 {:?} -> 屏幕坐标 {:?}", world_point, screen_point);
    }

    // 相机旋转
    println!("\n=== 相机旋转演示 ===");
    let mut camera = Camera::new(16.0 / 9.0);
    camera.move_to(na::Point3::new(0.0, 0.0, 5.0));
    camera.look_at(na::Point3::origin());

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        camera.orbit(angle, 0.0);
        println!("旋转 {} 度后的相机位置: {:?}", angle.to_degrees(), camera.position);
    }
}
```

## 🔍 深入理解

### 齐次坐标

```
为什么使用 4D 向量表示 3D 点？

齐次坐标：(x, y, z, w)

w = 1: 表示点
w = 0: 表示方向向量

优势：
1. 统一表示平移和其他变换
2. 可以用矩阵乘法表示所有变换
3. 支持透视除法
```

### 左手 vs 右手坐标系

```
右手坐标系（OpenGL, Vulkan）：
- X 向右
- Y 向上
- Z 向外（屏幕外）

左手坐标系（DirectX）：
- X 向右
- Y 向上
- Z 向内（屏幕内）

nalgebra 默认使用右手坐标系
```

## 📝 练习题

### 练习 1：实现 2D 变换
```rust
// 实现 2D 变换矩阵
fn create_2d_transform(
    translation: na::Vector2<f32>,
    rotation: f32,
    scale: na::Vector2<f32>,
) -> na::Matrix3<f32> {
    // 你的代码
}
```

### 练习 2：计算三角形法向量
```rust
// 给定三角形的三个顶点，计算法向量
fn triangle_normal(
    a: &na::Point3<f32>,
    b: &na::Point3<f32>,
    c: &na::Point3<f32>,
) -> na::Vector3<f32> {
    // 你的代码
}
```

### 练习 3：实现第一人称相机
```rust
// 实现 WASD 移动和鼠标旋转的第一人称相机
struct FPSCamera {
    position: na::Point3<f32>,
    yaw: f32,
    pitch: f32,
}

impl FPSCamera {
    fn move_forward(&mut self, distance: f32) {
        // 你的代码
    }

    fn move_right(&mut self, distance: f32) {
        // 你的代码
    }

    fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // 你的代码
    }
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解向量和矩阵的基本概念
- [ ] 进行向量运算（加减、点积、叉积）
- [ ] 创建和组合变换矩阵
- [ ] 实现相机系统
- [ ] 理解投影矩阵
- [ ] 使用四元数表示旋转
- [ ] 进行坐标空间转换
- [ ] 使用 nalgebra 库

## 🔗 延伸阅读

- [nalgebra 文档](https://docs.rs/nalgebra/)
- [3D Math Primer](https://gamemath.com/)
- [Learn OpenGL - Transformations](https://learnopengl.com/Getting-started/Transformations)
- [Quaternions and Spatial Rotation](https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation)

## 🚀 下一步

完成本模块后，你可以：
1. 继续学习模块 4.2（窗口与事件处理）
2. 实践：创建一个简单的 3D 场景
3. 深入学习：矩阵分解、插值算法

---

**掌握图形数学，开启 3D 编程之旅！** 🎨
