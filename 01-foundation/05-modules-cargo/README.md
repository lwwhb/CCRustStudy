# 模块 1.5：模块系统与 Cargo

## 🎯 学习目标

- 理解 Rust 的模块系统（mod、use、pub）
- 掌握包（package）、crate 和模块的关系
- 学习 Cargo 工作区（workspace）
- 使用 Cargo 管理依赖和特性（features）
- 编写文档注释和发布 crate

## 📚 核心概念

### 1. 模块系统

```rust
// 定义模块
mod garden {
    pub mod vegetables {
        pub struct Asparagus {}
    }
}

// 使用模块
use garden::vegetables::Asparagus;

// 相对路径
use self::vegetables::Asparagus;

// 父模块路径
use super::vegetables::Asparagus;
```

### 2. pub 可见性

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,       // 公开字段
        seasonal_fruit: String,  // 私有字段
    }

    pub enum Appetizer {
        Soup,   // 枚举变体默认公开
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

### 3. use 和 as

```rust
use std::collections::HashMap;
use std::io::{self, Write};
use std::fmt::Result as FmtResult;

// 重导出
pub use crate::garden::vegetables::Asparagus;
```

### 4. Cargo.toml 配置

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
description = "A sample crate"
license = "MIT"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", optional = true }

[features]
default = []
async = ["tokio"]

[dev-dependencies]
criterion = "0.5"
```

### 5. 工作区（Workspace）

```toml
# 根目录 Cargo.toml
[workspace]
members = [
    "core",
    "cli",
    "web",
]
```

## 💻 实战项目：多模块库

构建一个展示模块系统的库，包含数学工具、字符串工具和集合工具。

### 项目结构

```
modules-cargo/
├── Cargo.toml
├── src/
│   ├── lib.rs           # 库入口，重导出公共 API
│   ├── math/
│   │   ├── mod.rs       # 数学模块
│   │   ├── basic.rs     # 基本运算
│   │   └── stats.rs     # 统计函数
│   ├── strings/
│   │   ├── mod.rs       # 字符串模块
│   │   └── transform.rs # 字符串转换
│   └── collections/
│       ├── mod.rs       # 集合模块
│       └── stack.rs     # 栈实现
└── README.md
```

## 📖 深入阅读

- [The Rust Book - Chapter 7: Packages, Crates, Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## ✅ 检查清单

- [ ] 理解 package、crate、module 的区别
- [ ] 使用 mod 定义模块层次
- [ ] 使用 pub 控制可见性
- [ ] 使用 use 引入路径
- [ ] 配置 Cargo.toml 依赖
- [ ] 编写文档注释
- [ ] 理解 features 机制

## 🚀 下一步

完成本模块后，继续学习 [模块 1.6：测试与文档](../06-testing-docs/)。
