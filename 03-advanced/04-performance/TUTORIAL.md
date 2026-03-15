# 模块 3.4：性能优化 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 Rust 的零成本抽象
2. 掌握性能分析工具
3. 学习内存布局优化
4. 理解编译器优化
5. 使用 Criterion 进行基准测试
6. 掌握常见性能优化技巧

## 🎯 为什么需要性能优化？

### 性能的重要性

**性能差的影响**：
```
慢 1 秒 = 用户流失 7%
慢 3 秒 = 用户流失 40%
慢 5 秒 = 用户流失 90%

对于服务器：
- 更高的成本
- 更多的资源
- 更差的用户体验
```

**Rust 的性能优势**：
```
C/C++: 快，但不安全
Java/Go: 安全，但有 GC 开销
Python: 易用，但慢

Rust: 快 + 安全 + 零成本抽象
```

### 零成本抽象

**概念**：使用高级抽象不会带来运行时开销。

```rust
// 高级抽象
let sum: i32 = (0..1000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum();

// 编译后等价于手写循环
let mut sum = 0;
for i in 0..1000 {
    if i % 2 == 0 {
        sum += i * 2;
    }
}

// 性能相同！
```

## 📖 核心概念详解

### 1. 基准测试

使用 Criterion 进行准确的性能测试。

#### 安装 Criterion

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

#### 基础基准测试

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    b
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib recursive 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });

    c.bench_function("fib iterative 20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

**运行基准测试**：
```bash
cargo bench
```

**为什么使用 black_box？**
```rust
// 没有 black_box，编译器可能优化掉
b.iter(|| fibonacci(20));  // 编译器可能计算常量

// 使用 black_box，防止编译器优化
b.iter(|| fibonacci(black_box(20)));  // 强制运行时计算
```

#### 对比多个实现

```rust
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_concat");
    
    // 方法 1：使用 +
    group.bench_function("plus", |b| {
        b.iter(|| {
            let mut s = String::new();
            for i in 0..100 {
                s = s + &i.to_string();
            }
            s
        })
    });
    
    // 方法 2：使用 push_str
    group.bench_function("push_str", |b| {
        b.iter(|| {
            let mut s = String::new();
            for i in 0..100 {
                s.push_str(&i.to_string());
            }
            s
        })
    });
    
    // 方法 3：使用 format!
    group.bench_function("format", |b| {
        b.iter(|| {
            let mut s = String::new();
            for i in 0..100 {
                s = format!("{}{}", s, i);
            }
            s
        })
    });
    
    group.finish();
}
```

### 2. 内存布局优化

理解数据在内存中的布局可以提高性能。

#### 结构体大小和对齐

```rust
use std::mem;

// 未优化的布局
struct Unoptimized {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes
    c: u8,   // 1 byte
    d: u32,  // 4 bytes
}

// 优化后的布局
struct Optimized {
    b: u64,  // 8 bytes
    d: u32,  // 4 bytes
    a: u8,   // 1 byte
    c: u8,   // 1 byte
}

println!("Unoptimized: {} bytes", mem::size_of::<Unoptimized>());  // 24
println!("Optimized: {} bytes", mem::size_of::<Optimized>());      // 16
```

**内存布局**：
```
Unoptimized (24 bytes):
[a][pad][pad][pad][pad][pad][pad][pad]  8 bytes
[b][b][b][b][b][b][b][b]                 8 bytes
[c][pad][pad][pad][d][d][d][d]           8 bytes

Optimized (16 bytes):
[b][b][b][b][b][b][b][b]                 8 bytes
[d][d][d][d][a][c][pad][pad]             8 bytes
```

#### 使用 #[repr(C)]

```rust
// Rust 默认布局（可能重排）
struct RustLayout {
    a: u8,
    b: u64,
    c: u8,
}

// C 布局（保持顺序）
#[repr(C)]
struct CLayout {
    a: u8,
    b: u64,
    c: u8,
}

// 紧凑布局（无填充）
#[repr(packed)]
struct PackedLayout {
    a: u8,
    b: u64,
    c: u8,
}
```

#### 零大小类型（ZST）

```rust
struct ZeroSized;

println!("Size: {}", mem::size_of::<ZeroSized>());  // 0

// ZST 的应用
struct PhantomData<T>;  // 标记类型，不占空间

// Vec 的优化
let v: Vec<()> = vec![(); 1000];
println!("Size: {}", mem::size_of_val(&v));  // 只存储长度
```

### 3. 迭代器优化

迭代器是零成本抽象的典范。

#### 避免中间集合

```rust
// 慢：创建中间 Vec
let result: Vec<_> = data.iter()
    .collect::<Vec<_>>()  // 不必要的分配
    .iter()
    .map(|x| x * 2)
    .collect();

// 快：直接链式操作
let result: Vec<_> = data.iter()
    .map(|x| x * 2)
    .collect();
```

#### 使用迭代器而非索引

```rust
// 慢：索引访问
let mut sum = 0;
for i in 0..data.len() {
    sum += data[i];  // 每次都要边界检查
}

// 快：迭代器
let sum: i32 = data.iter().sum();  // 编译器优化掉边界检查
```

#### 提前分配容量

```rust
// 慢：多次重新分配
let mut v = Vec::new();
for i in 0..1000 {
    v.push(i);  // 可能多次重新分配
}

// 快：预分配
let mut v = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);  // 不需要重新分配
}
```

### 4. 避免不必要的克隆

克隆是昂贵的操作。

#### 使用引用

```rust
// 慢：克隆整个 Vec
fn process_slow(data: Vec<i32>) -> i32 {
    data.clone().iter().sum()  // 不必要的克隆
}

// 快：使用引用
fn process_fast(data: &[i32]) -> i32 {
    data.iter().sum()
}
```

#### 使用 Cow（写时复制）

```rust
use std::borrow::Cow;

fn process_string(input: &str) -> Cow<str> {
    if input.contains("bad") {
        // 需要修改，分配新字符串
        Cow::Owned(input.replace("bad", "good"))
    } else {
        // 不需要修改，借用原字符串
        Cow::Borrowed(input)
    }
}

let s1 = "This is good";
let result1 = process_string(s1);  // 借用，无分配

let s2 = "This is bad";
let result2 = process_string(s2);  // 分配新字符串
```

### 5. 字符串操作优化

字符串操作是常见的性能瓶颈。

#### 字符串拼接

```rust
// 最慢：使用 +
let mut s = String::new();
for i in 0..100 {
    s = s + &i.to_string();  // 每次都重新分配
}

// 慢：使用 format!
let mut s = String::new();
for i in 0..100 {
    s = format!("{}{}", s, i);  // 每次都分配
}

// 快：使用 push_str
let mut s = String::with_capacity(300);  // 预分配
for i in 0..100 {
    s.push_str(&i.to_string());
}

// 最快：使用 write!
use std::fmt::Write;
let mut s = String::with_capacity(300);
for i in 0..100 {
    write!(&mut s, "{}", i).unwrap();
}
```

#### 避免不必要的 to_string()

```rust
// 慢
let s = 42.to_string();
println!("{}", s);

// 快
println!("{}", 42);  // 直接格式化
```

### 6. 算法优化

选择正确的算法比微优化更重要。

#### 斐波那契数列

```rust
// O(2^n) - 指数时间
fn fib_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

// O(n) - 线性时间，使用缓存
fn fib_memoized(n: u64, cache: &mut Vec<Option<u64>>) -> u64 {
    if let Some(result) = cache[n as usize] {
        return result;
    }
    
    let result = match n {
        0 | 1 => 1,
        n => fib_memoized(n - 1, cache) + fib_memoized(n - 2, cache),
    };
    
    cache[n as usize] = Some(result);
    result
}

// O(n) - 线性时间，迭代
fn fib_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    b
}
```

**性能对比**：
```
fib(30):
递归:   ~1000 ms
缓存:   ~0.001 ms
迭代:   ~0.0001 ms
```

### 7. 编译器优化

#### 发布模式

```toml
[profile.release]
opt-level = 3        # 最高优化级别
lto = true           # 链接时优化
codegen-units = 1    # 更好的优化
panic = 'abort'      # 更小的二进制
```

#### 内联

```rust
// 建议内联
#[inline]
fn small_function(x: i32) -> i32 {
    x * 2
}

// 强制内联
#[inline(always)]
fn critical_function(x: i32) -> i32 {
    x * 2
}

// 禁止内联
#[inline(never)]
fn large_function() {
    // 大量代码
}
```

### 8. 性能分析工具

#### 使用 perf

```bash
# 编译带调试信息的发布版本
cargo build --release

# 运行性能分析
perf record --call-graph=dwarf ./target/release/my_program
perf report
```

#### 使用 flamegraph

```bash
# 安装
cargo install flamegraph

# 生成火焰图
cargo flamegraph
```

#### 使用 valgrind

```bash
# 内存分析
valgrind --tool=cachegrind ./target/release/my_program

# 查看结果
cg_annotate cachegrind.out.<pid>
```

## 💻 实战项目：性能优化实践

### 项目：优化数据处理管道

```rust
// 未优化版本
fn process_data_slow(data: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for item in data.clone() {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}

// 优化版本
fn process_data_fast(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}

// 进一步优化
fn process_data_fastest(data: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(data.len());
    for &item in data {
        if item > 0 {
            result.push(item * 2);
        }
    }
    result
}
```

## 🔍 性能优化清单

### 1. 算法层面
- [ ] 选择正确的算法（O(n) vs O(n²)）
- [ ] 使用缓存避免重复计算
- [ ] 考虑空间换时间

### 2. 数据结构
- [ ] 选择合适的集合类型
- [ ] 预分配容量
- [ ] 优化内存布局

### 3. 迭代器
- [ ] 使用迭代器而非索引
- [ ] 避免中间集合
- [ ] 利用零成本抽象

### 4. 内存管理
- [ ] 避免不必要的克隆
- [ ] 使用引用而非所有权
- [ ] 使用 Cow 延迟分配

### 5. 编译器
- [ ] 使用发布模式
- [ ] 启用 LTO
- [ ] 适当使用内联

## 📝 常见性能陷阱

### 1. 过早优化
```rust
// 不要这样做
// 在没有测量的情况下进行复杂优化

// 应该这样
// 1. 先写清晰的代码
// 2. 测量性能
// 3. 找到瓶颈
// 4. 针对性优化
```

### 2. 忽略算法复杂度
```rust
// 微优化 O(n²) 算法不如换成 O(n log n) 算法
```

### 3. 过度分配
```rust
// 不好
let mut v = Vec::new();
for i in 0..1000000 {
    v.push(i);  // 多次重新分配
}

// 好
let mut v = Vec::with_capacity(1000000);
for i in 0..1000000 {
    v.push(i);  // 一次分配
}
```

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 使用 Criterion 进行基准测试
- [ ] 理解内存布局和对齐
- [ ] 优化迭代器使用
- [ ] 避免不必要的克隆
- [ ] 选择合适的算法
- [ ] 使用性能分析工具
- [ ] 配置编译器优化
- [ ] 识别性能瓶颈

---

**记住：先测量，再优化！** 🚀
