# 模块 1.5：模块系统与 Cargo - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 Rust 的模块系统
2. 掌握包、crate 和模块的关系
3. 学习 pub 可见性控制
4. 使用 Cargo 管理项目
5. 理解工作区（workspace）
6. 发布和管理依赖

## 🎯 为什么需要模块系统？

### 代码组织的挑战

**没有模块系统的问题**：
```rust
// 所有代码都在一个文件中
fn add(a: i32, b: i32) -> i32 { a + b }
fn subtract(a: i32, b: i32) -> i32 { a - b }
fn to_uppercase(s: &str) -> String { s.to_uppercase() }
fn to_lowercase(s: &str) -> String { s.to_lowercase() }
// ... 数百个函数

问题：
- 难以维护
- 命名冲突
- 无法控制可见性
- 难以复用
```

**使用模块系统**：
```rust
// 清晰的组织结构
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
    pub fn subtract(a: i32, b: i32) -> i32 { a - b }
}

mod strings {
    pub fn to_uppercase(s: &str) -> String { s.to_uppercase() }
    pub fn to_lowercase(s: &str) -> String { s.to_lowercase() }
}

// 使用
use math::add;
use strings::to_uppercase;
```

### 其他语言的对比

**Java**：
```java
package com.example.math;
public class Calculator {
    public int add(int a, int b) { return a + b; }
}

// 使用
import com.example.math.Calculator;
```

**Python**：
```python
# math.py
def add(a, b):
    return a + b

# 使用
from math import add
```

**Rust**：
```rust
// math.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

// 使用
use crate::math::add;
```

## 📖 核心概念详解

### 1. 包（Package）、Crate 和模块

#### 概念层次

```
Package (包)
  └── Crate (单元)
       └── Module (模块)
            └── Item (项：函数、结构体等)
```

**Package（包）**：
- 一个 Cargo 项目
- 包含一个 `Cargo.toml` 文件
- 可以包含多个 crate

**Crate（单元）**：
- 编译的最小单位
- 两种类型：
  - 二进制 crate（有 `main.rs`）
  - 库 crate（有 `lib.rs`）

**Module（模块）**：
- 组织代码的方式
- 控制可见性
- 可以嵌套

#### 项目结构示例

```
my-project/
├── Cargo.toml          # Package 配置
├── src/
│   ├── main.rs         # 二进制 crate 根
│   ├── lib.rs          # 库 crate 根
│   ├── math/           # 模块目录
│   │   ├── mod.rs      # 模块声明
│   │   ├── add.rs      # 子模块
│   │   └── multiply.rs # 子模块
│   └── utils.rs        # 单文件模块
└── tests/              # 集成测试
    └── integration_test.rs
```

### 2. 定义模块

#### 内联模块

```rust
// 在同一文件中定义模块
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn internal_helper() -> i32 {
        42  // 私有函数
    }
}

// 使用
fn main() {
    let sum = math::add(2, 3);
    // math::internal_helper();  // 错误！私有函数
}
```

#### 文件模块

```rust
// src/lib.rs
pub mod math;  // 声明模块，对应 src/math.rs 或 src/math/mod.rs

// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

#### 目录模块

```rust
// src/lib.rs
pub mod math;  // 对应 src/math/ 目录

// src/math/mod.rs
pub mod operations;  // 对应 src/math/operations.rs
pub mod constants;   // 对应 src/math/constants.rs

// src/math/operations.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// src/math/constants.rs
pub const PI: f64 = 3.14159;
```

**新风格（Rust 2018+）**：
```rust
// src/lib.rs
pub mod math;  // 对应 src/math.rs 或 src/math/

// src/math.rs（推荐）
pub mod operations;  // 对应 src/math/operations.rs

// 目录结构：
// src/
//   lib.rs
//   math.rs
//   math/
//     operations.rs
```

### 3. 可见性控制（pub）

#### 默认私有

```rust
mod garden {
    // 私有函数（默认）
    fn water_plants() {
        println!("浇水");
    }
    
    // 公开函数
    pub fn tend_garden() {
        water_plants();  // 模块内可以访问私有函数
        println!("照料花园");
    }
}

fn main() {
    garden::tend_garden();  // OK
    // garden::water_plants();  // 错误！私有函数
}
```

#### 结构体的可见性

```rust
mod restaurant {
    pub struct Breakfast {
        pub toast: String,       // 公开字段
        seasonal_fruit: String,  // 私有字段
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

fn main() {
    let mut meal = restaurant::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");  // OK，公开字段
    // meal.seasonal_fruit = String::from("blueberries");  // 错误！私有字段
}
```

**关键点**：
- 结构体是 `pub`，但字段默认私有
- 必须显式标记字段为 `pub`

#### 枚举的可见性

```rust
mod restaurant {
    pub enum Appetizer {
        Soup,   // 枚举变体自动公开
        Salad,
    }
}

fn main() {
    let order = restaurant::Appetizer::Soup;  // OK
}
```

**关键点**：
- 枚举是 `pub`，所有变体自动公开
- 不能有私有变体

#### pub(crate) 和 pub(super)

```rust
mod outer {
    pub(crate) fn crate_visible() {
        // 在整个 crate 内可见
    }
    
    pub(super) fn parent_visible() {
        // 在父模块中可见
    }
    
    mod inner {
        pub(in crate::outer) fn outer_visible() {
            // 在 outer 模块中可见
        }
    }
}
```

**可见性级别**：
```
pub              - 完全公开
pub(crate)       - crate 内可见
pub(super)       - 父模块可见
pub(in path)     - 指定路径可见
(无修饰符)        - 当前模块可见（私有）
```

### 4. use 关键字

#### 基础用法

```rust
// 导入单个项
use std::collections::HashMap;

// 导入多个项
use std::io::{self, Write, Read};
// 等价于：
// use std::io;
// use std::io::Write;
// use std::io::Read;

// 导入所有公开项（不推荐）
use std::collections::*;
```

#### 路径类型

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 绝对路径
use crate::front_of_house::hosting;

// 相对路径
use self::front_of_house::hosting;

// 父模块路径
use super::front_of_house::hosting;
```

#### as 重命名

```rust
use std::fmt::Result;
use std::io::Result as IoResult;  // 避免命名冲突

fn function1() -> Result { ... }
fn function2() -> IoResult<()> { ... }
```

#### 重导出（Re-exporting）

```rust
// src/lib.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }
}

// 重导出，让外部可以直接使用
pub use math::add;

// 外部使用：
// use my_crate::add;  // 而不是 my_crate::math::add
```

**使用场景**：
- 简化 API
- 隐藏内部结构
- 提供便捷的导入路径

### 5. Cargo 基础

#### Cargo.toml 配置

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A sample project"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/my-project"
keywords = ["cli", "tool"]
categories = ["command-line-utilities"]

[dependencies]
serde = "1.0"
tokio = { version = "1", features = ["full"] }
log = "0.4"

[dev-dependencies]
criterion = "0.5"

[build-dependencies]
cc = "1.0"

[features]
default = ["std"]
std = []
async = ["tokio"]
```

#### 依赖版本

```toml
[dependencies]
# 精确版本
serde = "=1.0.0"

# 兼容版本（推荐）
serde = "1.0"      # >= 1.0.0, < 2.0.0
serde = "1.0.100"  # >= 1.0.100, < 1.1.0

# 范围版本
serde = ">= 1.0, < 2.0"

# 通配符
serde = "1.*"

# Git 依赖
my-lib = { git = "https://github.com/user/my-lib" }

# 本地路径
my-lib = { path = "../my-lib" }

# 可选依赖
tokio = { version = "1", optional = true }
```

#### Features（特性）

```toml
[features]
default = ["std"]
std = []
async = ["tokio"]
full = ["std", "async"]

[dependencies]
tokio = { version = "1", optional = true }
```

```rust
// 条件编译
#[cfg(feature = "async")]
pub mod async_module {
    // 只在启用 async 特性时编译
}

// 使用：
// cargo build --features async
// cargo build --features "std,async"
// cargo build --all-features
```

### 6. 工作区（Workspace）

工作区允许管理多个相关的包。

#### 工作区结构

```
my-workspace/
├── Cargo.toml          # 工作区配置
├── core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── cli/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── web/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

#### 根 Cargo.toml

```toml
[workspace]
members = [
    "core",
    "cli",
    "web",
]

# 共享依赖版本
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

# 工作区级别的配置
[profile.release]
opt-level = 3
lto = true
```

#### 成员包使用共享依赖

```toml
# cli/Cargo.toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
my-core = { path = "../core" }
serde = { workspace = true }
tokio = { workspace = true }
```

**工作区的优势**：
- 共享 `Cargo.lock`
- 统一依赖版本
- 一次性构建所有包
- 共享 `target` 目录

### 7. 文档注释

#### 文档注释类型

```rust
/// 这是一个文档注释（外部文档）
/// 
/// # Examples
/// 
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! 这是模块级文档注释（内部文档）
//! 
//! 本模块提供数学运算函数

/// 一个示例结构体
/// 
/// # Fields
/// 
/// * `x` - X 坐标
/// * `y` - Y 坐标
pub struct Point {
    pub x: i32,
    pub y: i32,
}
```

#### 文档测试

```rust
/// 计算两个数的和
/// 
/// # Examples
/// 
/// ```
/// use my_crate::add;
/// 
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
/// 
/// # Panics
/// 
/// 此函数不会 panic
/// 
/// # Errors
/// 
/// 此函数不返回错误
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 运行文档测试：
// cargo test --doc
```

## 💻 实战项目：多模块库

### 项目结构

```
modules-cargo/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── math/
│   │   ├── mod.rs
│   │   ├── basic.rs
│   │   └── stats.rs
│   ├── strings/
│   │   ├── mod.rs
│   │   └── transform.rs
│   └── collections/
│       ├── mod.rs
│       └── stack.rs
└── examples/
    └── demo.rs
```

### 实现步骤

#### 步骤 1：创建库结构

```rust
// src/lib.rs
//! # modules-cargo
//! 
//! 一个演示模块系统的库

pub mod math;
pub mod strings;
pub mod collections;

// 重导出常用类型
pub use collections::Stack;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
```

#### 步骤 2：实现 math 模块

```rust
// src/math/mod.rs
pub mod basic;
pub mod stats;

// 重导出
pub use basic::{add, subtract, multiply, divide};
pub use stats::{mean, median};

// src/math/basic.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// src/math/stats.rs
pub fn mean(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    numbers.iter().sum::<f64>() / numbers.len() as f64
}
```

#### 步骤 3：实现 strings 模块

```rust
// src/strings/mod.rs
mod transform;

pub use transform::{to_camel_case, to_snake_case};

// src/strings/transform.rs
pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for ch in s.chars() {
        if ch == '_' || ch == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    result
}
```

#### 步骤 4：使用示例

```rust
// examples/demo.rs
use modules_cargo::math;
use modules_cargo::strings;
use modules_cargo::Stack;

fn main() {
    // 使用 math 模块
    println!("2 + 3 = {}", math::add(2, 3));
    
    // 使用 strings 模块
    let camel = strings::to_camel_case("hello_world");
    println!("Camel case: {}", camel);
    
    // 使用重导出的 Stack
    let mut stack = Stack::new();
    stack.push(42);
    println!("Stack pop: {:?}", stack.pop());
}
```

## 🔍 深入理解

### 模块解析规则

```rust
// 当编译器看到 `mod foo;` 时，会按以下顺序查找：
// 1. foo.rs（内联，同一文件）
// 2. foo.rs（同级目录）
// 3. foo/mod.rs（子目录）

// Rust 2018+ 推荐：
src/
  lib.rs
  foo.rs        // mod foo;
  foo/
    bar.rs      // mod bar; (在 foo.rs 中)
```

### 路径解析

```rust
// 绝对路径
use crate::math::add;

// 相对路径
use self::math::add;

// 父模块
use super::math::add;

// 外部 crate
use std::collections::HashMap;
```

### 预导入（Prelude）

```rust
// Rust 自动导入 std::prelude::v1::*
// 包含常用类型：
// - Option, Some, None
// - Result, Ok, Err
// - String, Vec
// - Box, Rc, Arc
// - 等等

// 自定义 prelude
// src/prelude.rs
pub use crate::math::{add, subtract};
pub use crate::strings::to_camel_case;

// 用户可以：
use my_crate::prelude::*;
```

## 📝 最佳实践

### 1. 模块组织

```rust
// ✅ 好的组织
src/
  lib.rs          // 库根，声明模块
  math.rs         // 简单模块
  database/       // 复杂模块
    mod.rs
    connection.rs
    query.rs

// ❌ 避免
src/
  lib.rs
  math/
    mod.rs        // 只有一个文件，不需要目录
```

### 2. 可见性

```rust
// ✅ 最小可见性原则
mod internal {
    pub(crate) fn helper() { }  // 只在 crate 内可见
}

// ❌ 过度公开
pub mod internal {
    pub fn helper() { }  // 完全公开，可能不必要
}
```

### 3. 重导出

```rust
// ✅ 简化 API
// src/lib.rs
mod internal {
    pub struct ImportantType;
}

pub use internal::ImportantType;  // 用户可以直接 use my_crate::ImportantType

// ❌ 暴露内部结构
pub mod internal {  // 用户必须 use my_crate::internal::ImportantType
    pub struct ImportantType;
}
```

## 🧪 练习题

### 练习 1：创建模块

创建一个包含以下模块的库：
- `geometry` - 几何计算
  - `circle` - 圆形相关
  - `rectangle` - 矩形相关

### 练习 2：可见性控制

实现一个模块，其中包含公开和私有函数，测试可见性规则。

### 练习 3：工作区

创建一个工作区，包含：
- `core` - 核心库
- `cli` - 命令行工具（使用 core）
- `web` - Web 服务（使用 core）

## ✅ 检查清单

- [ ] 理解包、crate 和模块的关系
- [ ] 掌握 mod、use、pub 的使用
- [ ] 能够组织多文件项目
- [ ] 理解可见性控制
- [ ] 会使用 Cargo 管理依赖
- [ ] 理解工作区的概念
- [ ] 能够编写文档注释

---

**掌握模块系统，构建大型 Rust 项目！** 🦀🚀
