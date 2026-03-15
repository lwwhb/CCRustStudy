# 模块 1.1：所有权与借用 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 深入理解 Rust 的所有权系统
2. 掌握借用和引用的使用
3. 理解可变性和不可变性
4. 学会使用生命周期基础
5. 构建一个命令行计算器项目

## 🎯 为什么要学习所有权？

所有权是 Rust 最独特和最重要的特性，它让 Rust 能够在没有垃圾回收器的情况下保证内存安全。

### 其他语言的内存管理

**C/C++**: 手动管理内存
```c
int* ptr = malloc(sizeof(int));  // 手动分配
*ptr = 42;
free(ptr);  // 必须记得释放，否则内存泄漏
```

**Java/Python**: 垃圾回收器
```java
String s = new String("hello");  // 自动管理，但有性能开销
// 垃圾回收器会在某个时候回收
```

**Rust**: 所有权系统
```rust
let s = String::from("hello");  // 自动管理，零开销
// 离开作用域时自动释放，编译时保证安全
```

## 📖 核心概念详解

### 1. 所有权规则

Rust 的所有权有三条基本规则：

```rust
// 规则 1: Rust 中的每个值都有一个所有者
let s = String::from("hello");  // s 是这个字符串的所有者

// 规则 2: 值在任一时刻只能有一个所有者
let s1 = String::from("hello");
let s2 = s1;  // 所有权转移给 s2，s1 不再有效
// println!("{}", s1);  // 错误！s1 已经无效

// 规则 3: 当所有者离开作用域，值将被丢弃
{
    let s = String::from("hello");
    // s 在这里有效
}  // s 离开作用域，内存被自动释放
```

### 2. 移动 (Move) 语义

```rust
// 栈上的数据：复制
let x = 5;
let y = x;  // 复制值
println!("x = {}, y = {}", x, y);  // 都有效

// 堆上的数据：移动
let s1 = String::from("hello");
let s2 = s1;  // 移动所有权
// println!("{}", s1);  // 错误！s1 已失效
println!("{}", s2);  // 正确
```

**为什么要这样设计？**

```
栈上 (x = 5):
┌─────┐
│  5  │ x
└─────┘
┌─────┐
│  5  │ y (复制)
└─────┘

堆上 (String):
栈              堆
┌─────┐      ┌───────────┐
│ ptr │─────>│  "hello"  │
└─────┘      └───────────┘
  s1

移动后：
┌─────┐      ┌───────────┐
│ ptr │─────>│  "hello"  │
└─────┘      └───────────┘
  s2

s1 失效，避免双重释放！
```

### 3. 克隆 (Clone)

如果确实需要深拷贝堆数据：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 显式克隆

println!("s1 = {}, s2 = {}", s1, s2);  // 都有效
```

**性能考虑**：
- `clone()` 会复制堆数据，可能很昂贵
- 只在必要时使用
- 编译器会提示你何时需要 clone

### 4. 借用 (Borrowing)

借用允许你引用值而不获取所有权：

```rust
fn main() {
    let s1 = String::from("hello");

    // 不可变借用
    let len = calculate_length(&s1);

    println!("'{}' 的长度是 {}", s1, len);  // s1 仍然有效
}

fn calculate_length(s: &String) -> usize {
    s.len()  // 可以读取，但不能修改
}
```

**借用规则**：
1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
2. 引用必须总是有效的

```rust
let mut s = String::from("hello");

// 多个不可变引用 - OK
let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);

// 一个可变引用 - OK
let r3 = &mut s;
r3.push_str(", world");

// 不可变和可变引用同时存在 - 错误！
let r1 = &s;
let r2 = &mut s;  // 错误！
```

### 5. 可变借用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);  // 可变借用

    println!("{}", s);  // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**为什么限制可变引用？**

防止数据竞争：
```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;  // 错误！不能同时有两个可变引用

// 如果允许，可能导致：
// 线程 1 修改 s
// 线程 2 同时修改 s
// 数据损坏！
```

### 6. 悬垂引用

Rust 编译器防止悬垂引用：

```rust
fn dangle() -> &String {  // 错误！
    let s = String::from("hello");
    &s  // s 将被释放，返回悬垂引用
}  // s 离开作用域

// 正确的做法：
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 转移所有权
}
```

## 💻 实战项目：命令行计算器

### 项目结构

```
01-ownership-basics/
├── Cargo.toml
├── src/
│   ├── main.rs          # 主程序
│   ├── calculator.rs    # 计算器逻辑
│   └── history.rs       # 历史记录
└── README.md
```

### 步骤 1：理解项目需求

我们要构建一个计算器，它需要：
1. 解析用户输入的表达式
2. 执行计算
3. 保存历史记录
4. 支持查看历史

### 步骤 2：设计数据结构

```rust
// history.rs
pub struct History {
    records: Vec<String>,  // 所有权：History 拥有 Vec
}

impl History {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    // 借用：不获取所有权
    pub fn add(&mut self, record: String) {
        self.records.push(record);
    }

    // 不可变借用：只读访问
    pub fn display(&self) {
        for (i, record) in self.records.iter().enumerate() {
            println!("{}: {}", i + 1, record);
        }
    }
}
```

**所有权分析**：
- `History` 拥有 `Vec<String>`
- `add` 需要 `&mut self`（可变借用）来修改
- `display` 只需要 `&self`（不可变借用）来读取

### 步骤 3：实现计算器

```rust
// calculator.rs
pub struct Calculator {
    history: History,  // Calculator 拥有 History
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: History::new(),
        }
    }

    // 借用输入，不获取所有权
    pub fn evaluate(&mut self, expression: &str) -> Result<f64, String> {
        // 解析表达式
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() != 3 {
            return Err("格式错误".to_string());
        }

        // 解析数字
        let a: f64 = parts[0].parse()
            .map_err(|_| "无效的数字".to_string())?;
        let b: f64 = parts[2].parse()
            .map_err(|_| "无效的数字".to_string())?;

        // 执行运算
        let result = match parts[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return Err("除数不能为零".to_string());
                }
                a / b
            }
            _ => return Err("不支持的运算符".to_string()),
        };

        // 保存历史（需要拥有 String）
        self.history.add(format!("{} = {}", expression, result));

        Ok(result)
    }

    // 借用 history
    pub fn show_history(&self) {
        self.history.display();
    }
}
```

**关键点**：
1. `expression: &str` - 借用输入，不需要所有权
2. `self.history.add(...)` - 传递 `String` 所有权给 history
3. `&self` vs `&mut self` - 根据是否修改状态选择

### 步骤 4：主程序

```rust
// main.rs
use std::io::{self, Write};

mod calculator;
mod history;

use calculator::Calculator;

fn main() {
    let mut calc = Calculator::new();  // calc 拥有 Calculator

    println!("简单计算器");
    println!("输入格式: 数字 运算符 数字");
    println!("例如: 5 + 3");
    println!("输入 'history' 查看历史");
    println!("输入 'quit' 退出");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();  // 借用 input

        if input == "quit" {
            break;
        }

        if input == "history" {
            calc.show_history();  // 借用 calc
            continue;
        }

        // 借用 input 给 evaluate
        match calc.evaluate(input) {
            Ok(result) => println!("结果: {}", result),
            Err(e) => println!("错误: {}", e),
        }
    }
}
```

## 🔍 深入理解

### 为什么需要所有权？

**问题场景**：
```rust
// 如果没有所有权系统
let s = String::from("hello");
let s2 = s;  // 两个变量指向同一内存

// 当 s 和 s2 都离开作用域时
// 会尝试释放同一块内存两次 -> 双重释放 bug!
```

**Rust 的解决方案**：
```rust
let s = String::from("hello");
let s2 = s;  // 移动所有权，s 失效

// 只有 s2 会释放内存，安全！
```

### 借用的实际应用

```rust
// 场景 1: 函数需要读取数据
fn print_length(s: &String) {  // 借用
    println!("长度: {}", s.len());
}  // 不释放 s

// 场景 2: 函数需要修改数据
fn append_world(s: &mut String) {  // 可变借用
    s.push_str(", world");
}  // 不释放 s

// 场景 3: 函数需要获取所有权
fn consume(s: String) {  // 获取所有权
    println!("{}", s);
}  // 释放 s
```

### 生命周期基础

```rust
// 编译器自动推导生命周期
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 显式生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 🧪 练习题

### 练习 1：修复所有权错误

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);  // 错误！修复它
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

**提示**：有三种解决方案
1. 使用借用
2. 克隆
3. 返回所有权

### 练习 2：实现字符串反转

```rust
// 实现一个函数，反转字符串但不获取所有权
fn reverse_string(s: &mut String) {
    // 你的代码
}

fn main() {
    let mut s = String::from("hello");
    reverse_string(&mut s);
    println!("{}", s);  // 应该输出 "olleh"
}
```

### 练习 3：理解借用规则

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // 错误！为什么？

    println!("{}, {}, {}", r1, r2, r3);
}
```

## 📝 常见错误和解决方案

### 错误 1：使用已移动的值

```rust
// 错误代码
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // 错误！

// 解决方案 1：使用借用
let s = String::from("hello");
let s2 = &s;
println!("{}", s);  // OK

// 解决方案 2：克隆
let s = String::from("hello");
let s2 = s.clone();
println!("{}", s);  // OK
```

### 错误 2：可变借用冲突

```rust
// 错误代码
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // 错误！

// 解决方案：使用作用域分离
let mut s = String::from("hello");
{
    let r1 = &mut s;
}  // r1 离开作用域
let r2 = &mut s;  // OK
```

### 错误 3：悬垂引用

```rust
// 错误代码
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // 错误！
}

// 解决方案：返回所有权
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // OK
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 解释 Rust 的三条所有权规则
- [ ] 区分移动和复制语义
- [ ] 正确使用不可变借用和可变借用
- [ ] 理解借用检查器的作用
- [ ] 避免常见的所有权错误
- [ ] 运行并理解计算器项目
- [ ] 完成所有练习题

## 🔗 下一步

掌握了所有权后，继续学习：
- [模块 1.2：结构体与 Trait](../02-structs-traits/TUTORIAL.md)
- 深入学习生命周期
- 了解智能指针

## 📚 参考资源

- [The Rust Book - 所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - 所有权](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [所有权可视化工具](https://github.com/cognitive-engineering-lab/aquascope)

---

**记住**：所有权是 Rust 的核心，一开始可能觉得困难，但它是 Rust 安全性和性能的基础。多练习，多思考，你会逐渐掌握它！🦀
