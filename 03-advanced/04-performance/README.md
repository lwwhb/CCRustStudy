# 模块 3.4：性能优化

## 🎯 学习目标

- 理解 Rust 的零成本抽象
- 掌握性能分析工具
- 学习内存布局优化
- 理解编译器优化
- 使用 Criterion 进行基准测试
- 掌握常见性能优化技巧

## 📚 核心概念

### 1. 基准测试

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 2. 内存布局

```rust
use std::mem;

#[repr(C)]
struct Optimized {
    a: u64,  // 8 bytes
    b: u32,  // 4 bytes
    c: u16,  // 2 bytes
    d: u8,   // 1 byte
}

println!("Size: {}", mem::size_of::<Optimized>());
```

### 3. 迭代器优化

```rust
// 慢：创建中间 Vec
let sum: i32 = (0..1000)
    .collect::<Vec<_>>()
    .iter()
    .map(|x| x * 2)
    .sum();

// 快：零成本抽象
let sum: i32 = (0..1000)
    .map(|x| x * 2)
    .sum();
```

### 4. 避免不必要的克隆

```rust
// 慢
fn process_slow(data: Vec<i32>) -> Vec<i32> {
    data.clone().into_iter().map(|x| x * 2).collect()
}

// 快
fn process_fast(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}
```

### 5. 使用 Cow（写时复制）

```rust
use std::borrow::Cow;

fn process<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains("bad") {
        Cow::Owned(input.replace("bad", "good"))
    } else {
        Cow::Borrowed(input)
    }
}
```

## 💻 实战项目：性能优化实践

对常见算法和数据结构进行性能优化和基准测试。

### 功能需求

1. 实现多个版本的算法
2. 使用 Criterion 进行基准测试
3. 内存布局优化
4. 缓存友好的数据结构

### 项目结构

```
performance/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── memory_layout.rs    # 内存布局优化
│   ├── algorithms.rs       # 算法优化
│   └── iterators.rs        # 迭代器优化
├── benches/
│   └── benchmarks.rs       # 基准测试
└── README.md
```

## 🧪 练习题

### 练习 1：优化字符串处理

```rust
// 优化这个函数的性能
fn process_strings(strings: Vec<String>) -> Vec<String> {
    strings.iter()
        .map(|s| s.to_uppercase())
        .collect()
}
```

### 练习 2：缓存优化

```rust
// 实现一个缓存友好的矩阵乘法
```

### 练习 3：避免分配

```rust
// 重写这个函数以避免不必要的内存分配
```

## 📖 深入阅读

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Compiler Optimizations](https://doc.rust-lang.org/rustc/codegen-options/index.html)

## ✅ 检查清单

- [ ] 使用 Criterion 编写基准测试
- [ ] 理解内存布局和对齐
- [ ] 优化迭代器使用
- [ ] 避免不必要的克隆和分配
- [ ] 使用 Cow 优化字符串处理
- [ ] 理解编译器优化选项
- [ ] 使用性能分析工具

## 🚀 下一步

完成本模块后，你已经掌握了 Rust 的高级特性！接下来可以选择：
- [模块 4.1：图形编程基础](../../04-graphics-foundation/01-math/)
- [模块 5.1：Web 服务](../../05-web-services/01-axum-basics/)
