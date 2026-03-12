# 模块 1.1：所有权与借用

## 🎯 学习目标

- 理解 Rust 的所有权模型和内存管理机制
- 掌握借用规则（可变借用和不可变借用）
- 学习生命周期的基本概念
- 熟悉基本语法、控制流和数据类型
- 掌握模式匹配和错误处理基础

## 📚 核心概念

### 1. 所有权规则

Rust 的所有权系统是其最独特的特性，它在编译时保证内存安全，无需垃圾回收器。

**三大规则：**
1. Rust 中的每个值都有一个所有者（owner）
2. 值在任一时刻只能有一个所有者
3. 当所有者离开作用域时，值将被丢弃

```rust
fn main() {
    let s1 = String::from("hello");  // s1 是所有者
    let s2 = s1;                      // 所有权转移给 s2，s1 不再有效
    // println!("{}", s1);            // 错误！s1 已失效
    println!("{}", s2);               // 正确
}
```

### 2. 借用（Borrowing）

借用允许你引用某个值而不获取其所有权。

**不可变借用：**
```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // 可以读取，但不能修改
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 借用 s1
    println!("'{}' 的长度是 {}", s1, len);  // s1 仍然有效
}
```

**可变借用：**
```rust
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);  // 输出: hello, world!
}
```

**借用规则：**
- 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
- 引用必须总是有效的

### 3. 切片（Slices）

切片是对集合中一段连续元素的引用。

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### 4. 基本数据类型

**标量类型：**
- 整数：`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 浮点数：`f32`, `f64`
- 布尔值：`bool`
- 字符：`char`

**复合类型：**
- 元组：`(i32, f64, u8)`
- 数组：`[i32; 5]`

### 5. 控制流

```rust
// if 表达式
let number = 6;
if number % 2 == 0 {
    println!("偶数");
} else {
    println!("奇数");
}

// loop 循环
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};

// while 循环
while counter > 0 {
    counter -= 1;
}

// for 循环
for i in 0..5 {
    println!("{}", i);
}
```

### 6. 模式匹配

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## 💻 实战项目：命令行计算器

构建一个支持历史记录的命令行计算器，练习所有权和借用。

### 功能需求

1. 支持基本算术运算（+、-、*、/）
2. 存储计算历史
3. 查看历史记录
4. 清除历史记录
5. 优雅的错误处理

### 项目结构

```
ownership-basics/
├── Cargo.toml
├── src/
│   ├── main.rs          # 主程序入口
│   ├── calculator.rs    # 计算器逻辑
│   └── history.rs       # 历史记录管理
└── README.md
```

### 运行项目

```bash
cargo run
```

### 使用示例

```
欢迎使用 Rust 计算器！
支持的命令：
  <数字> <运算符> <数字>  - 执行计算（例如：5 + 3）
  history                  - 查看历史记录
  clear                    - 清除历史记录
  quit                     - 退出程序

> 10 + 5
结果: 15

> 20 * 3
结果: 60

> history
计算历史:
1. 10 + 5 = 15
2. 20 * 3 = 60

> quit
再见！
```

## 🧪 练习题

### 练习 1：理解所有权转移

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // 问题：下面这行代码会发生什么？为什么？
    // println!("{}", s1);
}
```

### 练习 2：修复借用错误

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // 错误！

    println!("{}, {}, {}", r1, r2, r3);
}
```

### 练习 3：实现字符串反转

编写一个函数，接受字符串的不可变引用，返回反转后的新字符串。

```rust
fn reverse_string(s: &str) -> String {
    // 你的代码
}

#[test]
fn test_reverse() {
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("rust"), "tsur");
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Visualizing Memory Layout of Rust's Data Types](https://www.youtube.com/watch?v=rDoqT-a6UFg)

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 解释 Rust 的所有权规则
- [ ] 区分值的移动（move）和复制（copy）
- [ ] 正确使用可变和不可变借用
- [ ] 理解借用检查器的错误信息
- [ ] 使用切片引用集合的部分数据
- [ ] 编写基本的 Rust 程序
- [ ] 使用 match 进行模式匹配
- [ ] 处理基本的错误情况

## 🚀 下一步

完成本模块后，继续学习 [模块 1.2：结构体、枚举与 Trait](../02-structs-traits/)。
