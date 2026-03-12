# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目概述

这是一个 Rust 学习仓库（CCRustStudy），用于在 Claude Code 的辅助下学习 Rust 编程。

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

## 开发注意事项

- 这是一个学习仓库，代码可能具有实验性或探索性
- 编写示例时，优先考虑清晰、有教育意义的代码，而非生产优化代码
- 对可能不熟悉的 Rust 概念，添加注释加以说明
