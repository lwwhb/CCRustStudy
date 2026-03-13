# 模块 2.1：泛型与生命周期

## 🎯 学习目标

- 深入理解泛型函数和泛型结构体
- 掌握生命周期标注和生命周期省略规则
- 理解生命周期与借用检查器的关系
- 学习高级生命周期场景
- 实现泛型数据结构

## 📚 核心概念

### 1. 泛型函数

```rust
// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 多个泛型参数
fn mix<T, U>(x: T, y: U) -> (T, U) {
    (x, y)
}
```

### 2. 泛型结构体

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 为特定类型实现方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### 3. 生命周期基础

```rust
// 生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### 4. 生命周期省略规则

```rust
// 规则 1：每个引用参数都有自己的生命周期
fn first_word(s: &str) -> &str { ... }
// 等价于
fn first_word<'a>(s: &'a str) -> &'a str { ... }

// 规则 2：如果只有一个输入生命周期，赋给所有输出
fn process(s: &str) -> &str { ... }

// 规则 3：如果有 &self 或 &mut self，其生命周期赋给所有输出
impl<'a> MyStruct<'a> {
    fn get_part(&self) -> &str { ... }
}
```

### 5. 高级生命周期

```rust
// 静态生命周期
let s: &'static str = "I have a static lifetime";

// 生命周期约束
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## 💻 实战项目：泛型数据结构库

实现多种泛型数据结构，演示泛型和生命周期的使用。

### 功能需求

1. 泛型栈和队列
2. 泛型链表
3. 泛型二叉搜索树
4. 带生命周期的引用包装器
5. 泛型缓存系统

### 项目结构

```
generics-lifetimes/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── stack.rs      # 泛型栈
│   ├── queue.rs      # 泛型队列
│   ├── list.rs       # 泛型链表
│   └── cache.rs      # 泛型缓存
└── README.md
```

## 🧪 练习题

### 练习 1：实现泛型函数

```rust
// 实现一个泛型函数，找出切片中的最大值
fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    // 你的代码
}
```

### 练习 2：生命周期标注

```rust
// 修复生命周期错误
fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

### 练习 3：泛型结构体

```rust
// 实现一个泛型 Pair 结构体
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    // 实现 new 和 swap 方法
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 10: Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [Rust by Example - Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)

## ✅ 检查清单

- [ ] 编写泛型函数
- [ ] 实现泛型结构体
- [ ] 理解生命周期标注语法
- [ ] 掌握生命周期省略规则
- [ ] 处理复杂的生命周期场景
- [ ] 结合泛型和生命周期
- [ ] 理解 'static 生命周期

## 🚀 下一步

完成本模块后，继续学习 [模块 2.2：智能指针](../02-smart-pointers/)。
