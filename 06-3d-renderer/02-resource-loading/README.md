# 模块 6.2：资源加载系统

## 🎯 学习目标

- 理解资源加载流程
- 实现异步资源加载
- 学习 glTF 模型格式
- 掌握纹理加载和管理
- 实现资源缓存机制

## 📚 核心概念

### 1. 资源类型

- **Mesh**: 3D 模型网格
- **Texture**: 纹理图像
- **Material**: 材质定义
- **Shader**: 着色器程序

### 2. glTF 格式

glTF (GL Transmission Format) 是一种高效的 3D 资源格式：
- 支持网格、材质、动画
- 二进制和 JSON 格式
- 广泛的工具支持

### 3. 异步加载

```rust
async fn load_model(path: &str) -> Result<Model> {
    let data = tokio::fs::read(path).await?;
    parse_gltf(&data)
}
```

### 4. 资源缓存

```rust
struct ResourceCache {
    models: HashMap<String, Arc<Model>>,
    textures: HashMap<String, Arc<Texture>>,
}
```

## 💻 实战项目：资源加载器

实现完整的资源加载系统：
1. 模型加载（简化的 OBJ 格式）
2. 纹理加载
3. 异步加载支持
4. 资源缓存
5. 错误处理

## 🧪 测试

```bash
cargo test
```

## 📝 练习

1. 添加对更多模型格式的支持
2. 实现纹理压缩
3. 添加资源预加载
4. 实现资源卸载机制

## 🔗 深入阅读

- [glTF Specification](https://www.khronos.org/gltf/)
- [Image Crate Documentation](https://docs.rs/image/)
- [Async Programming in Rust](https://rust-lang.github.io/async-book/)

## ✅ 检查清单

- [ ] 理解资源加载流程
- [ ] 实现模型加载
- [ ] 实现纹理加载
- [ ] 添加异步支持
- [ ] 实现资源缓存
- [ ] 完成所有测试
