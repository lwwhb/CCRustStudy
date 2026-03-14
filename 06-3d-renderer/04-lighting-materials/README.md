# 模块 6.4：光照与材质系统

## 🎯 学习目标

- 理解光照模型原理
- 实现 Phong 光照模型
- 学习 PBR（基于物理的渲染）
- 掌握材质系统设计
- 实现多光源支持

## 📚 核心概念

### 1. 光照模型

**Phong 光照模型**
- 环境光（Ambient）
- 漫反射（Diffuse）
- 镜面反射（Specular）

**PBR 光照模型**
- 金属度（Metallic）
- 粗糙度（Roughness）
- 基础颜色（Base Color）
- 能量守恒

### 2. 光源类型

- **方向光（Directional Light）**: 太阳光
- **点光源（Point Light）**: 灯泡
- **聚光灯（Spot Light）**: 手电筒

### 3. 材质属性

```rust
struct Material {
    albedo: Vec3,
    metallic: f32,
    roughness: f32,
    ao: f32, // 环境光遮蔽
}
```

### 4. 法线贴图

用于增加表面细节而不增加几何复杂度。

## 💻 实战项目：光照系统

实现完整的光照和材质系统：
1. Phong 光照模型
2. PBR 材质系统
3. 多光源支持
4. 材质管理器
5. 光照计算

## 🧪 测试

```bash
cargo test
```

## 📝 练习

1. 实现阴影映射
2. 添加环境光遮蔽
3. 实现法线贴图
4. 添加更多光源类型

## 🔗 深入阅读

- [Learn OpenGL - Lighting](https://learnopengl.com/Lighting/Basic-Lighting)
- [PBR Theory](https://learnopengl.com/PBR/Theory)
- [Real Shading in Unreal Engine 4](https://blog.selfshadow.com/publications/s2013-shading-course/)

## ✅ 检查清单

- [ ] 理解光照模型
- [ ] 实现 Phong 光照
- [ ] 实现 PBR 材质
- [ ] 支持多光源
- [ ] 完成所有测试
