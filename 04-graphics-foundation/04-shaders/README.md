# 模块 4.4：着色器编程

## 🎯 学习目标

- 理解着色器的作用和类型
- 学习 WGSL 语法基础
- 掌握顶点着色器和片段着色器
- 理解 Uniform 缓冲区
- 学习纹理采样
- 实现基本光照效果

## 📚 核心概念

### 1. 着色器类型

**顶点着色器 (Vertex Shader)**
- 处理每个顶点
- 进行坐标变换
- 传递数据到片段着色器

**片段着色器 (Fragment Shader)**
- 处理每个像素
- 计算最终颜色
- 应用纹理和光照

### 2. WGSL 基础语法

```wgsl
// 变量声明
var<private> my_var: f32 = 1.0;
let my_const: f32 = 2.0;

// 函数定义
fn my_function(x: f32) -> f32 {
    return x * 2.0;
}

// 结构体
struct MyStruct {
    field1: f32,
    field2: vec3<f32>,
}
```

### 3. Uniform 缓冲区

```wgsl
struct Uniforms {
    view_proj: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;
```

### 4. 纹理采样

```wgsl
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, tex_coords);
}
```

## 💻 实战项目：着色器效果演示

实现多种着色器效果：
1. 颜色插值
2. 纹理映射
3. 简单光照（Phong）
4. 动画效果

## 🧪 测试

由于着色器需要 GPU 环境，我们提供：
- 着色器语法验证测试
- Uniform 数据结构测试
- 着色器编译测试（模拟）

```bash
cargo test
```

## 📝 练习

1. 实现渐变色着色器
2. 添加纹理坐标变换
3. 实现简单的 Phong 光照
4. 创建动画效果（基于时间）

## 🔗 深入阅读

- [WGSL Specification](https://www.w3.org/TR/WGSL/)
- [Learn wgpu - Shaders](https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/)
- [The Book of Shaders](https://thebookofshaders.com/)

## ✅ 检查清单

- [ ] 理解着色器的作用
- [ ] 学习 WGSL 基础语法
- [ ] 编写顶点着色器
- [ ] 编写片段着色器
- [ ] 使用 Uniform 缓冲区
- [ ] 实现纹理采样
- [ ] 完成练习题
