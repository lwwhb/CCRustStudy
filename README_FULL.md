# CCRustStudy - 完整的 Rust 学习课程

一个从零基础到实战项目的完整 Rust 学习路径。

## 🎯 课程概览

本课程包含 **35 个模块**，分为 **7 个阶段**，涵盖从基础语法到复杂项目的完整学习路径。

### 📊 当前进度：17/35 模块 (48.6%)

## 🗺️ 学习路线图

```
基础篇 (6 模块) ✅
    ↓
中级篇 (5 模块) ✅
    ↓
高级篇 (4 模块) ✅
    ↓
    ├─→ 图形编程 (5 模块) → 3D 渲染器 (5 模块)
    │
    └─→ Web 服务 (5 模块) → AI Gateway (5 模块)
```

## 📚 课程内容

### ✅ 阶段 1：基础篇 (100% 完成)
掌握 Rust 核心概念和基础语法

- [x] 1.1 所有权与借用 - 命令行计算器
- [x] 1.2 结构体、枚举与 Trait - 图形库
- [x] 1.3 集合与迭代器 - 文本分析工具
- [x] 1.4 错误处理 - 文件处理工具
- [x] 1.5 模块系统与 Cargo - 多模块库
- [x] 1.6 测试与文档 - 完整测试套件

### ✅ 阶段 2：中级篇 (100% 完成)
深入学习 Rust 高级特性

- [x] 2.1 泛型与生命周期 - 泛型数据结构
- [x] 2.2 智能指针 - 树形数据结构
- [x] 2.3 闭包与函数式编程 - 数据处理管道
- [x] 2.4 异步编程基础 - 异步 HTTP 客户端
- [x] 2.5 序列化与反序列化 - 配置管理器

### ✅ 阶段 3：高级篇 (100% 完成)
掌握 Rust 高级编程技术

- [x] 3.1 宏编程 - 声明宏实现
- [x] 3.2 Unsafe Rust 与 FFI - C 库绑定
- [x] 3.3 并发编程 - 线程池实现
- [x] 3.4 性能优化 - 基准测试套件

### 🔄 阶段 4：图形编程基础 (20% 完成)
学习 3D 图形编程基础

- [x] 4.1 线性代数与图形数学 - 3D 数学库
- [x] 4.2 窗口与事件处理 - 交互式窗口
- [ ] 4.3 wgpu 基础 - 三角形渲染
- [ ] 4.4 着色器编程 - 纹理映射
- [ ] 4.5 3D 渲染管线 - 场景渲染器

### 🔄 阶段 5：Web 服务 (40% 完成)
构建高性能异步 Web 服务

- [x] 5.1 Axum Web 框架 - RESTful API
- [x] 5.2 异步 HTTP 客户端 - API 聚合器
- [ ] 5.3 流式处理与 SSE - 实时数据流
- [ ] 5.4 数据库集成 - 持久化 API
- [ ] 5.5 可观测性 - 日志与监控

### ⏳ 阶段 6：3D 渲染器项目 (0% 完成)
构建完整的 3D 渲染器

- [ ] 6.1 渲染器架构设计
- [ ] 6.2 资源加载系统
- [ ] 6.3 相机与输入系统
- [ ] 6.4 光照与材质系统
- [ ] 6.5 高级特性与优化

### ⏳ 阶段 7：AI Gateway 项目 (0% 完成)
构建生产级 AI Gateway

- [ ] 7.1 Gateway 架构设计
- [ ] 7.2 AI Provider 集成
- [ ] 7.3 Agent 系统核心
- [ ] 7.4 工具与插件系统
- [ ] 7.5 生产特性

## 🚀 快速开始

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 克隆仓库

```bash
git clone <repository-url>
cd CCRustStudy
```

### 3. 开始学习

```bash
# 从第一个模块开始
cd 01-foundation/01-ownership-basics
cargo test    # 运行测试
cargo run     # 运行程序
```

## 📖 学习建议

### 推荐学习路径

**Web 开发方向：**
```
阶段 1 → 阶段 2 → 阶段 3 → 阶段 5 → 阶段 7
```

**系统编程方向：**
```
阶段 1 → 阶段 2 → 阶段 3 → 深入并发和性能
```

**图形编程方向：**
```
阶段 1 → 阶段 2 (部分) → 阶段 4 → 阶段 6
```

### 学习技巧

1. **循序渐进** - 严格按模块顺序学习
2. **动手实践** - 完成每个实战项目
3. **理解原理** - 不要死记硬背
4. **编写测试** - 养成 TDD 习惯
5. **查阅文档** - 善用官方文档

## 🛠️ 开发工具

### 推荐 IDE
- **VS Code** + rust-analyzer
- **IntelliJ IDEA** + Rust 插件
- **Vim/Neovim** + rust.vim

### 常用命令

```bash
cargo build              # 构建项目
cargo run                # 运行项目
cargo test               # 运行测试
cargo check              # 快速检查
cargo clippy             # 代码检查
cargo fmt                # 格式化代码
cargo doc --open         # 生成文档
```

## 📦 核心依赖

### 已使用的库

- **tokio** - 异步运行时
- **serde** - 序列化框架
- **axum** - Web 框架
- **reqwest** - HTTP 客户端
- **nalgebra** - 线性代数
- **winit** - 窗口管理
- **thiserror/anyhow** - 错误处理

### 将使用的库

- **wgpu** - GPU 编程
- **sqlx** - 数据库访问
- **tracing** - 日志追踪
- **async-openai** - OpenAI 客户端

## 📊 项目统计

- **总模块数**: 35
- **已完成**: 17 (48.6%)
- **代码文件**: 85+
- **测试数量**: 320+
- **文档页面**: 35

## 🎓 学习资源

### 官方资源
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### 社区资源
- [Rust 中文社区](https://rustcc.cn/)
- [Rust 官方论坛](https://users.rust-lang.org/)
- [r/rust](https://www.reddit.com/r/rust/)

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

### 贡献方式
1. Fork 本仓库
2. 创建特性分支
3. 提交更改
4. 发起 Pull Request

## 📝 许可证

本项目采用 MIT 许可证。

## 📧 联系方式

如有问题，请提交 Issue。

---

**开始你的 Rust 学习之旅！** 🦀

*最后更新：2026-03-13*
