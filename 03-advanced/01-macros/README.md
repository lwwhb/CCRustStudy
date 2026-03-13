# 模块 3.1：宏编程

## 🎯 学习目标

- 理解声明宏（macro_rules!）
- 学习过程宏基础
- 实现 derive 宏
- 掌握属性宏和函数宏
- 理解宏的卫生性和作用域

## 📚 核心概念

### 1. 声明宏（macro_rules!）

```rust
// 简单宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// 带参数的宏
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// 重复模式
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}
```

### 2. 宏模式匹配

```rust
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    };
}
```

### 3. 过程宏基础

```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // 处理输入并生成代码
    input
}
```

### 4. Derive 宏

```rust
#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    // 为结构体自动实现 trait
}

// 使用
#[derive(MyTrait)]
struct MyStruct;
```

### 5. 属性宏

```rust
#[proc_macro_attribute]
pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 修改或增强代码
}

// 使用
#[my_attribute]
fn my_function() {}
```

## 💻 实战项目：自定义宏库

实现多种实用宏，演示宏编程的各种技巧。

### 功能需求

1. 声明宏（日志、断言、构建器）
2. Derive 宏（Builder 模式）
3. 属性宏（计时、缓存）
4. 函数宏（DSL）

### 项目结构

```
macros-demo/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── declarative.rs  # 声明宏
└── README.md
```

## 🧪 练习题

### 练习 1：实现 vec! 宏

```rust
macro_rules! my_vec {
    // 你的实现
}
```

### 练习 2：实现 hashmap! 宏

```rust
macro_rules! hashmap {
    // 你的实现
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 19.6: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)

## ✅ 检查清单

- [ ] 编写声明宏
- [ ] 理解宏模式匹配
- [ ] 使用重复模式
- [ ] 理解宏卫生性
- [ ] 了解过程宏基础
- [ ] 实现简单的 derive 宏

## 🚀 下一步

完成本模块后，继续学习 [模块 3.2：Unsafe Rust 与 FFI](../02-unsafe-ffi/)。
