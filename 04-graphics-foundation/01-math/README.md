# 模块 4.1：线性代数与图形数学

## 🎯 学习目标

- 理解向量和矩阵的基本概念
- 掌握向量运算（加减、点积、叉积）
- 学习矩阵变换（平移、旋转、缩放）
- 理解齐次坐标系
- 掌握投影矩阵（正交投影、透视投影）
- 使用 nalgebra 库进行数学计算

## 📚 核心概念

### 1. 向量基础

```rust
use nalgebra::{Vector3, Vector4};

fn main() {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    // 向量加法
    let sum = v1 + v2;
    println!("Sum: {:?}", sum);

    // 点积
    let dot = v1.dot(&v2);
    println!("Dot product: {}", dot);

    // 叉积
    let cross = v1.cross(&v2);
    println!("Cross product: {:?}", cross);
}
```

### 2. 矩阵变换

```rust
use nalgebra::{Matrix4, Vector3};

fn main() {
    // 平移矩阵
    let translation = Matrix4::new_translation(&Vector3::new(1.0, 2.0, 3.0));

    // 旋转矩阵（绕 Z 轴旋转 45 度）
    let rotation = Matrix4::from_euler_angles(0.0, 0.0, std::f32::consts::PI / 4.0);

    // 缩放矩阵
    let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(2.0, 2.0, 2.0));

    // 组合变换
    let transform = translation * rotation * scale;
}
```

### 3. 四元数

```rust
use nalgebra::{UnitQuaternion, Vector3};

fn main() {
    // 创建旋转四元数
    let axis = Vector3::z_axis();
    let angle = std::f32::consts::PI / 4.0;
    let rotation = UnitQuaternion::from_axis_angle(&axis, angle);

    // 应用旋转
    let point = Vector3::new(1.0, 0.0, 0.0);
    let rotated = rotation * point;
}
```

### 4. 投影矩阵

```rust
use nalgebra::{Matrix4, Perspective3};

fn main() {
    // 透视投影
    let perspective = Perspective3::new(
        16.0 / 9.0,  // 宽高比
        std::f32::consts::PI / 4.0,  // FOV
        0.1,  // 近平面
        100.0  // 远平面
    );

    let proj_matrix = perspective.to_homogeneous();
}
```

## 💻 实战项目：3D 数学库

实现基本的 3D 图形数学运算。

### 功能需求

1. 向量运算（2D/3D/4D）
2. 矩阵变换
3. 四元数旋转
4. 相机矩阵
5. 投影矩阵

### 项目结构

```
graphics-math/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── vectors.rs      # 向量运算
│   ├── matrices.rs     # 矩阵变换
│   ├── quaternions.rs  # 四元数
│   └── camera.rs       # 相机系统
└── README.md
```

## 🧪 练习题

### 练习 1：向量归一化

```rust
// 实现向量归一化函数
fn normalize(v: Vector3<f32>) -> Vector3<f32> {
    // 你的代码
}
```

### 练习 2：构建视图矩阵

```rust
// 实现 look_at 函数
fn look_at(eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    // 你的代码
}
```

### 练习 3：球面插值

```rust
// 实现四元数球面线性插值
fn slerp(q1: UnitQuaternion<f32>, q2: UnitQuaternion<f32>, t: f32) -> UnitQuaternion<f32> {
    // 你的代码
}
```

## 📖 深入阅读

- [nalgebra Documentation](https://docs.rs/nalgebra/)
- [3D Math Primer for Graphics and Game Development](https://gamemath.com/)
- [Essential Mathematics for Games](https://www.essentialmath.com/)

## ✅ 检查清单

- [ ] 理解向量的基本运算
- [ ] 掌握点积和叉积的应用
- [ ] 理解矩阵变换的组合
- [ ] 使用四元数进行旋转
- [ ] 构建视图矩阵和投影矩阵
- [ ] 理解齐次坐标系

## 🚀 下一步

完成本模块后，继续学习 [模块 4.2：窗口与事件处理](../02-windowing/)。
