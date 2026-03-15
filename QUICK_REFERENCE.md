# CCRustStudy 快速参考

## 🚀 快速开始

```bash
# 1. 克隆仓库
git clone <repository-url>
cd CCRustStudy

# 2. 开始第一个模块
cd 01-foundation/01-ownership-basics
cat TUTORIAL.md  # 阅读教程
cargo run        # 运行代码
cargo test       # 运行测试
```

---

## 📚 学习路径选择

### 我想学 Web 开发
```
路径：基础篇 → 中级篇 → Web 服务 → AI Gateway
时间：105-120 小时
模块：21 个
```
**开始**：[1.1 所有权与借用](01-foundation/01-ownership-basics/TUTORIAL.md)

### 我想学系统编程
```
路径：基础篇 → 中级篇 → 高级篇
时间：75-85 小时
模块：15 个
```
**开始**：[1.1 所有权与借用](01-foundation/01-ownership-basics/TUTORIAL.md)

### 我想学图形编程
```
路径：基础篇 → 中级篇(部分) → 图形基础 → 3D 渲染器
时间：95-110 小时
模块：19 个
```
**开始**：[1.1 所有权与借用](01-foundation/01-ownership-basics/TUTORIAL.md)

---

## 🔧 常用命令

### Cargo 基础
```bash
cargo new <name>        # 创建新项目
cargo build             # 构建项目
cargo run               # 运行项目
cargo test              # 运行测试
cargo check             # 快速检查
```

### 代码质量
```bash
cargo clippy            # 代码检查
cargo fmt               # 格式化代码
cargo doc --open        # 生成文档
```

### 依赖管理
```bash
cargo add <crate>       # 添加依赖
cargo update            # 更新依赖
cargo tree              # 查看依赖树
```

---

## 📖 模块速查

### 基础篇 (1.1-1.6)
- **1.1** 所有权与借用 - Rust 的核心概念
- **1.2** 结构体与 Trait - 自定义类型和多态
- **1.3** 集合与迭代器 - 数据结构和函数式编程
- **1.4** 错误处理 - Result 和 Option
- **1.5** 模块系统 - 代码组织
- **1.6** 测试与文档 - 质量保证

### 中级篇 (2.1-2.5)
- **2.1** 泛型与生命周期 - 类型参数化
- **2.2** 智能指针 - Box、Rc、Arc
- **2.3** 闭包与函数式 - 高阶函数
- **2.4** 异步编程 - async/await、tokio
- **2.5** 序列化 - serde、JSON

### 高级篇 (3.1-3.4)
- **3.1** 宏编程 - 元编程
- **3.2** Unsafe 与 FFI - 底层操作
- **3.3** 并发编程 - 多线程
- **3.4** 性能优化 - 基准测试

### 图形编程 (4.1-4.5)
- **4.1** 图形数学 - 向量、矩阵
- **4.2** 窗口与事件 - winit
- **4.3** wgpu 基础 - GPU 编程
- **4.4** 着色器编程 - WGSL
- **4.5** 渲染管线 - 深度测试、混合

### Web 服务 (5.1-5.5)
- **5.1** Axum 框架 - Web 框架
- **5.2** HTTP 客户端 - reqwest
- **5.3** 流式处理 - SSE
- **5.4** 数据库 - sqlx
- **5.5** 可观测性 - tracing

### 3D 渲染器 (6.1-6.5)
- **6.1** 架构设计 - 模块化设计
- **6.2** 资源加载 - glTF、纹理
- **6.3** 相机与输入 - FPS 相机
- **6.4** 光照与材质 - PBR
- **6.5** 高级特性 - 剔除、LOD

### AI Gateway (7.1-7.5)
- **7.1** 架构设计 - Gateway 模式
- **7.2** AI 客户端 - OpenAI、Claude
- **7.3** Agent 核心 - 推理循环
- **7.4** 工具插件 - 插件系统
- **7.5** 生产特性 - 认证、限流

---

## 🎯 学习技巧

### 高效学习
1. **按顺序学习** - 不要跳过基础模块
2. **动手实践** - 完成所有实战项目
3. **理解原理** - 不要死记硬背
4. **编写测试** - 养成 TDD 习惯
5. **定期复习** - 巩固知识

### 遇到问题
1. **阅读错误信息** - Rust 编译器很友好
2. **查阅文档** - 官方文档很详细
3. **搜索问题** - Stack Overflow、Reddit
4. **提问** - Rust 社区很友好

### 进阶建议
1. **阅读源码** - 学习优秀项目
2. **参与开源** - 贡献代码
3. **写博客** - 分享经验
4. **做项目** - 实践是最好的老师

---

## 📦 关键依赖

### 异步运行时
```toml
tokio = { version = "1", features = ["full"] }
```

### Web 框架
```toml
axum = "0.7"
serde = { version = "1", features = ["derive"] }
```

### 图形编程
```toml
wgpu = "0.19"
winit = "0.29"
nalgebra = "0.32"
```

### 数据库
```toml
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
```

### 日志追踪
```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 🔗 有用链接

### 官方资源
- [Rust 官网](https://www.rust-lang.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [标准库文档](https://doc.rust-lang.org/std/)

### 学习资源
- [Rustlings](https://github.com/rust-lang/rustlings) - 交互式练习
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - 实用示例
- [This Week in Rust](https://this-week-in-rust.org/) - 周报

### 社区
- [Rust 用户论坛](https://users.rust-lang.org/)
- [Rust Reddit](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)

---

## 📊 进度追踪

使用检查清单追踪学习进度：

### 基础篇
- [ ] 1.1 所有权与借用
- [ ] 1.2 结构体与 Trait
- [ ] 1.3 集合与迭代器
- [ ] 1.4 错误处理
- [ ] 1.5 模块系统
- [ ] 1.6 测试与文档

### 中级篇
- [ ] 2.1 泛型与生命周期
- [ ] 2.2 智能指针
- [ ] 2.3 闭包与函数式
- [ ] 2.4 异步编程
- [ ] 2.5 序列化

### 高级篇
- [ ] 3.1 宏编程
- [ ] 3.2 Unsafe 与 FFI
- [ ] 3.3 并发编程
- [ ] 3.4 性能优化

### 专业方向
- [ ] Web 服务 (5.1-5.5)
- [ ] AI Gateway (7.1-7.5)
- [ ] 图形编程 (4.1-4.5)
- [ ] 3D 渲染器 (6.1-6.5)

---

**开始你的 Rust 学习之旅！** 🦀🚀
