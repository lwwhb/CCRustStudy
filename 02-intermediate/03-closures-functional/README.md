# 模块 2.3：闭包与函数式编程

## 🎯 学习目标

- 理解闭包语法和捕获机制
- 掌握 Fn、FnMut、FnOnce trait
- 学习高阶函数和函数组合
- 实践函数式编程模式
- 使用迭代器适配器

## 📚 核心概念

### 1. 闭包基础

```rust
// 闭包语法
let add = |x, y| x + y;
let result = add(2, 3); // 5

// 类型标注
let multiply: fn(i32, i32) -> i32 = |x, y| x * y;

// 捕获环境
let x = 10;
let add_x = |y| x + y;
println!("{}", add_x(5)); // 15
```

### 2. 闭包捕获模式

```rust
// 不可变借用
let list = vec![1, 2, 3];
let print = || println!("{:?}", list);

// 可变借用
let mut list = vec![1, 2, 3];
let mut add = || list.push(4);

// 获取所有权
let list = vec![1, 2, 3];
let consume = move || list;
```

### 3. Fn trait 家族

```rust
// Fn - 不可变借用捕获
fn apply_fn<F>(f: F) where F: Fn(i32) -> i32 {
    println!("{}", f(5));
}

// FnMut - 可变借用捕获
fn apply_fn_mut<F>(mut f: F) where F: FnMut() {
    f();
}

// FnOnce - 获取所有权
fn apply_fn_once<F>(f: F) where F: FnOnce() {
    f();
}
```

### 4. 高阶函数

```rust
// 接受函数作为参数
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// 返回闭包
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

### 5. 函数式编程模式

```rust
let numbers = vec![1, 2, 3, 4, 5];

// map, filter, fold
let result: i32 = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();

// 函数组合
let add_one = |x| x + 1;
let double = |x| x * 2;
let composed = |x| double(add_one(x));
```

## 💻 实战项目：数据处理管道

构建可组合的数据转换管道，演示函数式编程。

### 功能需求

1. 数据过滤器（filter、map、reduce）
2. 函数组合器
3. 惰性求值管道
4. 自定义迭代器适配器

### 项目结构

```
closures-functional/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── pipeline.rs   # 数据管道
│   ├── combinators.rs # 函数组合
│   └── iterators.rs  # 自定义迭代器
└── README.md
```

## 🧪 练习题

### 练习 1：闭包捕获

```rust
// 实现一个计数器闭包
fn make_counter() -> impl FnMut() -> i32 {
    // 你的代码
}
```

### 练习 2：高阶函数

```rust
// 实现 compose 函数
fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    // 你的代码
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 13: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)

## ✅ 检查清单

- [ ] 理解闭包语法和捕获
- [ ] 区分 Fn、FnMut、FnOnce
- [ ] 编写高阶函数
- [ ] 使用函数组合
- [ ] 实践函数式编程模式
- [ ] 理解惰性求值

## 🚀 下一步

完成本模块后，继续学习 [模块 2.4：异步编程基础](../04-async-basics/)。
