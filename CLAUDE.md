# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目概述

这是一个全面的 Rust 学习课程仓库（CCRustStudy），包含 7 个阶段共 35 个模块，旨在帮助学习者从零基础到能够构建实时 3D 图形渲染器和 AI Gateway 系统。

### 课程结构

```
CCRustStudy/
├── 01-foundation/          # 基础篇（6 个模块）
├── 02-intermediate/        # 中级篇（5 个模块）
├── 03-advanced/           # 高级篇（4 个模块）
├── 04-graphics-foundation/ # 图形编程基础（5 个模块）
├── 05-web-services/       # Web 服务（5 个模块）
├── 06-3d-renderer/        # 最终项目：3D 渲染器（5 个子项目）
└── 07-ai-gateway/         # 最终项目：AI Gateway（5 个子项目）
```

### 学习路径

**图形渲染路径：**
基础篇 → 中级篇 (2.1-2.3) → 高级篇 (3.2, 3.4) → 图形基础 → 3D 渲染器

**Web 服务路径：**
基础篇 → 中级篇 (2.4-2.5) → 高级篇 (3.3) → Web 服务 → AI Gateway

## 开发命令

### 创建新的 Rust 项目
```bash
cargo new <project_name>        # 创建新的二进制项目
cargo new --lib <project_name>  # 创建新的库项目
```

### 构建与运行
```bash
cargo build                     # 构建项目
cargo build --release           # 优化构建
cargo run                       # 构建并运行项目
cargo run -- <args>             # 带参数运行
```

### 测试
```bash
cargo test                      # 运行所有测试
cargo test <test_name>          # 运行指定测试
cargo test -- --nocapture       # 在测试中显示 println! 输出
cargo test -- --test-threads=1  # 顺序运行测试
cargo test --quiet              # 安静模式运行测试
```

### 代码质量
```bash
cargo check                     # 快速编译检查，不生成二进制文件
cargo clippy                    # 运行 linter 检查常见错误
cargo fmt                       # 按 Rust 风格格式化代码
cargo doc --open                # 生成并打开文档
```

## 仓库结构

本仓库使用 Rust 标准 `.gitignore`，排除以下内容：
- `target/` - 编译产物和构建输出
- `debug/` - 调试构建
- `**/*.rs.bk` - rustfmt 备份文件
- `*.pdb` - MSVC 调试信息
- `**/mutants.out*/` - 变异测试数据
- `Cargo.lock` - 对于库项目通常不提交（但本项目的示例会保留）

## 开发注意事项

### 代码风格

- **教育优先**：代码应清晰易懂，优先考虑教学价值而非极致优化
- **详细注释**：所有代码使用中文注释，解释关键概念和设计决策
- **渐进式复杂度**：从简单示例开始，逐步增加复杂度
- **实战导向**：每个模块都包含可运行的实战项目

### 模块开发规范

每个模块应包含：

1. **README.md**
   - 学习目标
   - 核心概念讲解
   - 实战项目说明
   - 练习题
   - 深入阅读资源
   - 检查清单

2. **src/ 目录**
   - `main.rs` 或 `lib.rs` - 主入口
   - 模块化的代码组织
   - 完整的单元测试
   - 详细的文档注释

3. **Cargo.toml**
   - 清晰的依赖声明
   - 适当的版本约束

### 测试要求

- 每个模块必须包含单元测试
- 测试覆盖核心功能
- 测试应该能够独立运行
- 使用 `cargo test` 验证所有测试通过

### 文档要求

- 所有公共 API 必须有文档注释
- 使用 `///` 进行文档注释
- 包含使用示例
- 解释参数、返回值和可能的错误

## 关键依赖库

### 已使用
- 标准库（std）

### 待使用（按阶段）

**图形渲染相关：**
- `wgpu` - 跨平台 GPU API
- `winit` - 窗口管理
- `nalgebra` - 线性代数
- `image` - 图像加载
- `gltf` - 3D 模型加载

**Web 服务相关：**
- `axum` - Web 框架
- `tokio` - 异步运行时
- `tower` - 中间件
- `reqwest` - HTTP 客户端
- `serde` / `serde_json` - 序列化
- `sqlx` - 数据库访问
- `tracing` - 日志追踪

**AI 相关：**
- `async-openai` - OpenAI 客户端
- `tokio-stream` - 流式处理

**工具库：**
- `thiserror` / `anyhow` - 错误处理
- `criterion` - 性能基准测试

## 开发工作流

### 开发新模块

1. 进入模块目录：`cd 0X-stage/0X-module-name`
2. 阅读 README.md 了解学习目标
3. 实现代码和测试
4. 运行测试：`cargo test`
5. 检查代码质量：`cargo clippy`
6. 格式化代码：`cargo fmt`
7. 运行项目：`cargo run`

### 提交代码

```bash
# 检查状态
git status

# 添加文件
git add .

# 提交（使用清晰的提交信息）
git commit -m "feat: 完成模块 X.X - 模块名称"

# 推送
git push
```

### 提交信息规范

- `feat:` - 新功能或新模块
- `fix:` - 修复 bug
- `docs:` - 文档更新
- `test:` - 添加或修改测试
- `refactor:` - 代码重构
- `style:` - 代码格式调整

## 常见问题

### 编译错误

如果遇到编译错误：
1. 仔细阅读错误信息
2. 使用 `cargo check` 快速检查
3. 查看 Rust 编译器的建议
4. 参考模块 README 中的概念讲解

### 测试失败

如果测试失败：
1. 使用 `cargo test -- --nocapture` 查看输出
2. 单独运行失败的测试：`cargo test test_name`
3. 检查测试逻辑和预期结果

### 依赖问题

如果依赖无法解析：
1. 检查 `Cargo.toml` 中的版本
2. 运行 `cargo update` 更新依赖
3. 清理并重新构建：`cargo clean && cargo build`

## 学习建议

1. **按顺序学习**：严格按照模块编号顺序，不要跳过
2. **动手实践**：每个模块都要亲自编写代码，不要只看
3. **完成练习**：README 中的练习题很重要
4. **理解概念**：不要死记硬背，理解背后的原理
5. **编写测试**：养成 TDD 的习惯
6. **查阅文档**：遇到问题先查官方文档
7. **代码审查**：定期回顾之前的代码，寻找改进空间

## 参考资源

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## 项目状态

- **当前阶段**：基础篇
- **已完成模块**：2/35 (5.7%)
  - ✅ 1.1 所有权与借用
  - ✅ 1.2 结构体、枚举与 Trait
- **下一个模块**：1.3 集合与迭代器
