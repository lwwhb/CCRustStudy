# 模块 1.4：错误处理

## 🎯 学习目标

- 深入理解 `Result<T, E>` 和 `Option<T>` 的使用
- 掌握 `?` 操作符进行错误传播
- 学习自定义错误类型
- 使用 `thiserror` 和 `anyhow` 简化错误处理
- 理解错误转换和错误链

## 📚 核心概念

### 1. Result 和 Option

```rust
// Option - 值可能存在或不存在
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some("Alice".to_string()) } else { None }
}

// Result - 操作可能成功或失败
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}
```

### 2. ? 操作符

```rust
use std::fs;
use std::io;

fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;  // 错误自动传播
    Ok(content)
}
```

### 3. 自定义错误类型

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(String),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO 错误: {}", e),
            AppError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            AppError::NotFound(item) => write!(f, "未找到: {}", item),
        }
    }
}

impl std::error::Error for AppError {}
```

### 4. thiserror（简化自定义错误）

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("解析错误: {0}")]
    Parse(String),

    #[error("未找到: {0}")]
    NotFound(String),
}
```

### 5. anyhow（简化错误传播）

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取文件: {}", path))?;
    Ok(content)
}
```

## 💻 实战项目：文件处理工具

构建一个 CSV 文件处理工具，演示完整的错误处理模式。

### 功能需求

1. 读取 CSV 文件
2. 解析数据行
3. 数据验证
4. 错误报告（带行号和上下文）
5. 写入处理结果

### 项目结构

```
error-handling/
├── Cargo.toml
├── src/
│   ├── main.rs       # 主程序
│   ├── errors.rs     # 自定义错误类型
│   ├── csv.rs        # CSV 解析器
│   └── validator.rs  # 数据验证器
└── README.md
```

## 🧪 练习题

### 练习 1：错误转换

```rust
// 实现 From trait 将 ParseIntError 转换为自定义错误
#[derive(Debug)]
enum MyError {
    Parse(String),
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        // 你的代码
    }
}
```

### 练习 2：链式错误处理

```rust
// 使用 ? 操作符链式处理多个可能失败的操作
fn process(input: &str) -> Result<i32, MyError> {
    // 解析字符串 -> 检查范围 -> 计算结果
    // 你的代码
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror crate](https://docs.rs/thiserror)
- [anyhow crate](https://docs.rs/anyhow)

## ✅ 检查清单

- [ ] 使用 `?` 传播错误
- [ ] 定义自定义错误类型
- [ ] 实现 `Display` 和 `Error` trait
- [ ] 使用 `thiserror` 简化错误定义
- [ ] 使用 `anyhow` 处理应用层错误
- [ ] 理解错误转换（`From` trait）

## 🚀 下一步

完成本模块后，继续学习 [模块 1.5：模块系统与 Cargo](../05-modules-cargo/)。
