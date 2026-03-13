# 模块 3.2：Unsafe Rust 与 FFI

## 🎯 学习目标

- 理解 unsafe 代码块的使用场景
- 掌握原始指针操作
- 学习 FFI（Foreign Function Interface）
- 实现 C 语言互操作
- 构建安全抽象

## 📚 核心概念

### 1. Unsafe 代码块

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        *r2 = 10;
        println!("r2: {}", *r2);
    }
}
```

### 2. 原始指针

```rust
// 不可变原始指针
let x = 5;
let raw = &x as *const i32;

// 可变原始指针
let mut y = 10;
let raw_mut = &mut y as *mut i32;

unsafe {
    *raw_mut = 20;
}
```

### 3. Unsafe 函数

```rust
unsafe fn dangerous() {
    // 不安全操作
}

fn main() {
    unsafe {
        dangerous();
    }
}
```

### 4. FFI 基础

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}
```

### 5. 导出 Rust 函数给 C

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Called from C!");
}
```

## 💻 实战项目：C 库绑定

为简单的 C 库创建安全的 Rust 绑定。

### 功能需求

1. 原始指针操作
2. C 函数调用
3. 安全抽象封装
4. 内存管理

### 项目结构

```
unsafe-ffi/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── raw_pointers.rs  # 原始指针
│   └── ffi.rs           # FFI 示例
└── README.md
```

## 🧪 练习题

### 练习 1：原始指针操作

```rust
fn swap_values(a: *mut i32, b: *mut i32) {
    unsafe {
        // 交换两个值
    }
}
```

### 练习 2：实现安全抽象

```rust
// 为 unsafe 代码创建安全接口
```

## 📖 深入阅读

- [The Rust Book - Chapter 19.1: Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)

## ✅ 检查清单

- [ ] 理解 unsafe 的五种超能力
- [ ] 使用原始指针
- [ ] 调用 unsafe 函数
- [ ] 实现 FFI 绑定
- [ ] 构建安全抽象
- [ ] 理解内存安全保证

## 🚀 下一步

完成本模块后，继续学习 [模块 3.3：并发编程](../03-concurrency/)。
