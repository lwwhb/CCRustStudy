# CCRustStudy - Rust 学习课程

欢迎来到 CCRustStudy！这是一个全面的 Rust 学习课程，旨在帮助你从零基础快速掌握 Rust 编程，并具备构建实战项目的能力。

## 🎯 课程目标

完成本课程后，你将能够：

- ✅ 熟练掌握 Rust 语言核心概念和高级特性
- ✅ 构建高性能、内存安全的系统级应用
- ✅ 开发跨平台的实时 3D 图形应用
- ✅ 构建生产级的异步 Web 服务
- ✅ 实现复杂的 AI Agent 系统

## 📚 课程结构

本课程分为 7 个阶段，共 35 个模块，每个模块都包含理论学习和实战项目。

### 阶段 1：基础篇 ★☆☆☆☆

掌握 Rust 的核心概念和基础语法。

| 模块 | 主题 | 实战项目 |
|------|------|----------|
| [1.1](01-foundation/01-ownership-basics/) | 所有权与借用 | 命令行计算器 |
| [1.2](01-foundation/02-structs-traits/) | 结构体、枚举与 Trait | 图形库（多态） |
| [1.3](01-foundation/03-collections-iterators/) | 集合与迭代器 | 文本分析工具 |
| [1.4](01-foundation/04-error-handling/) | 错误处理 | 文件处理工具 |
| [1.5](01-foundation/05-modules-cargo/) | 模块系统与 Cargo | 多模块库项目 |
| [1.6](01-foundation/06-testing-docs/) | 测试与文档 | 完整测试套件 |

### 阶段 2：中级篇 ★★☆☆☆

深入学习 Rust 的高级特性和异步编程。

| 模块 | 主题 | 实战项目 |
|------|------|----------|
| [2.1](02-intermediate/01-generics-lifetimes/) | 泛型与生命周期 | 泛型数据结构库 |
| [2.2](02-intermediate/02-smart-pointers/) | 智能指针 | 树形数据结构 |
| [2.3](02-intermediate/03-closures-functional/) | 闭包与函数式编程 | 数据处理管道 |
| [2.4](02-intermediate/04-async-basics/) | 异步编程基础 | 异步 HTTP 客户端 |
| [2.5](02-intermediate/05-serde/) | 序列化与反序列化 | 配置文件管理器 |

### 阶段 3：高级篇 ★★★☆☆

掌握 Rust 的高级编程技术。

| 模块 | 主题 | 实战项目 |
|------|------|----------|
| [3.1](03-advanced/01-macros/) | 宏编程 | 自定义 derive 宏 |
| [3.2](03-advanced/02-unsafe-ffi/) | Unsafe Rust 与 FFI | C 库绑定 |
| [3.3](03-advanced/03-concurrency/) | 并发编程 | 多线程任务调度器 |
| [3.4](03-advanced/04-performance/) | 性能优化 | 性能基准测试套件 |

### 阶段 4：图形编程基础 ★★★☆☆

学习 3D 图形编程的基础知识。

| 模块 | 主题 | 实战项目 |
|------|------|----------|
| [4.1](04-graphics-foundation/01-math/) | 线性代数与图形数学 | 3D 数学库 |
| [4.2](04-graphics-foundation/02-windowing/) | 窗口与事件处理 | 交互式窗口应用 |
| [4.3](04-graphics-foundation/03-wgpu-basics/) | wgpu 基础 | 简单三角形渲染 |
| [4.4](04-graphics-foundation/04-shaders/) | 着色器编程（WGSL） | 纹理映射立方体 |
| [4.5](04-graphics-foundation/05-rendering-pipeline/) | 3D 渲染管线 | 3D 场景渲染器 |

### 阶段 5：Web 服务与异步系统 ★★★☆☆

构建高性能的异步 Web 服务。

| 模块 | 主题 | 实战项目 |
|------|------|----------|
| [5.1](05-web-services/01-axum-basics/) | Axum Web 框架 | RESTful API 服务 |
| [5.2](05-web-services/02-http-clients/) | 异步 HTTP 客户端 | API 聚合器 |
| [5.3](05-web-services/03-streaming/) | 流式处理与 SSE | 实时数据流服务 |
| [5.4](05-web-services/04-database/) | 数据库集成 | 持久化 API 服务 |
| [5.5](05-web-services/05-observability/) | 可观测性 | 可观测的 Web 服务 |

### 阶段 6：最终项目 - 3D 渲染器 ★★★★★

构建一个完整的跨平台 3D 渲染器。

| 模块 | 主题 |
|------|------|
| [6.1](06-3d-renderer/01-architecture/) | 渲染器架构设计 |
| [6.2](06-3d-renderer/02-resource-loading/) | 资源加载系统 |
| [6.3](06-3d-renderer/03-camera-input/) | 相机与输入系统 |
| [6.4](06-3d-renderer/04-lighting-materials/) | 光照与材质系统 |
| [6.5](06-3d-renderer/05-advanced-features/) | 高级特性与优化 |

### 阶段 7：最终项目 - AI Gateway ★★★★★

构建一个生产级的 AI Gateway 与 Agent 系统。

| 模块 | 主题 |
|------|------|
| [7.1](07-ai-gateway/01-architecture/) | Gateway 架构设计 |
| [7.2](07-ai-gateway/02-ai-clients/) | AI Provider 集成 |
| [7.3](07-ai-gateway/03-agent-core/) | Agent 系统核心 |
| [7.4](07-ai-gateway/04-tools-plugins/) | 工具与插件系统 |
| [7.5](07-ai-gateway/05-production/) | 生产特性 |

## 🛤️ 学习路径

### 通往 3D 渲染器的路径

```
基础篇 (1.1-1.6) → 中级篇 (2.1-2.3) → 高级篇 (3.2, 3.4) →
图形基础 (4.1-4.5) → 最终渲染器 (6.1-6.5)
```

**关键模块：**
- 1.1: 所有权（GPU 缓冲区管理）
- 1.2: Trait（wgpu API 模式）
- 2.2: 智能指针（场景图）
- 3.2: Unsafe（底层 GPU 操作）
- 3.4: 性能优化（实时渲染）

### 通往 AI Gateway 的路径

```
基础篇 (1.1-1.6) → 中级篇 (2.4-2.5) → 高级篇 (3.3) →
Web 服务 (5.1-5.5) → 最终 Gateway (7.1-7.5)
```

**关键模块：**
- 1.4: 错误处理（API 失败）
- 2.4: Async/await（核心架构）
- 2.5: 序列化（API 请求/响应）
- 3.3: 并发（多请求处理）
- 5.1-5.5: 所有 Web 服务模块

## 🚀 快速开始

### 前置要求

- Rust 工具链（rustc, cargo）
- 代码编辑器（推荐 VS Code + rust-analyzer）
- Git

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 克隆仓库

```bash
git clone <repository-url>
cd CCRustStudy
```

### 开始学习

从第一个模块开始：

```bash
cd 01-foundation/01-ownership-basics
cargo run
```

## 📖 学习建议

1. **循序渐进**：严格按照模块顺序学习，不要跳过基础模块
2. **动手实践**：每个模块都要完成实战项目，不要只看代码
3. **深入理解**：遇到不懂的概念，查阅官方文档和 Rust Book
4. **测试驱动**：养成编写测试的习惯
5. **代码审查**：定期回顾之前的代码，重构改进

## 🔧 常用命令

```bash
cargo build              # 构建项目
cargo run                # 运行项目
cargo test               # 运行测试
cargo check              # 快速检查编译错误
cargo clippy             # 运行 linter
cargo fmt                # 格式化代码
cargo doc --open         # 生成并打开文档
```

## 📦 关键依赖库

### 图形渲染
- `wgpu` - 跨平台 GPU API
- `winit` - 窗口管理
- `nalgebra` - 线性代数
- `image` - 图像加载
- `gltf` - 3D 模型加载

### Web 服务
- `axum` - Web 框架
- `tokio` - 异步运行时
- `reqwest` - HTTP 客户端
- `serde` - 序列化
- `sqlx` - 数据库
- `tracing` - 日志追踪

### AI 相关
- `async-openai` - OpenAI 客户端
- `tokio-stream` - 流式处理

## 🎓 学习资源

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 📝 许可证

本项目仅用于学习目的。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**开始你的 Rust 学习之旅吧！** 🦀