# 模块 1.2：结构体、枚举与 Trait

## 🎯 学习目标

- 定义和使用自定义类型（struct、enum）
- 实现方法和关联函数
- 理解 Rust 的 trait 系统
- 掌握 trait 约束和 trait 对象
- 学习多态和动态分发

## 📚 核心概念

### 1. 结构体（Structs）

结构体是创建自定义数据类型的方式。

**定义结构体：**
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);

// 单元结构体
struct AlwaysEqual;
```

**实现方法：**
```rust
impl User {
    // 关联函数（类似静态方法）
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }

    // 方法（需要 self 参数）
    fn is_active(&self) -> bool {
        self.active
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

### 2. 枚举（Enums）

枚举允许你定义一个类型，它可以是几个可能的变体之一。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
        }
    }
}
```

### 3. Option 和 Result

Rust 的标准库提供了两个重要的枚举：

```rust
// Option - 表示可能存在或不存在的值
enum Option<T> {
    Some(T),
    None,
}

// Result - 表示操作可能成功或失败
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 4. Trait（特征）

Trait 定义了类型必须实现的行为。

```rust
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn summarize_author(&self) -> String {
        String::from("(匿名)")
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}
```

### 5. Trait 约束

```rust
// 函数参数的 trait 约束
fn notify(item: &impl Summary) {
    println!("新消息: {}", item.summarize());
}

// 泛型 trait 约束
fn notify<T: Summary>(item: &T) {
    println!("新消息: {}", item.summarize());
}

// 多个 trait 约束
fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// where 子句
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

### 6. Trait 对象和动态分发

```rust
// trait 对象允许在运行时处理不同类型
fn draw(shape: &dyn Draw) {
    shape.draw();
}

// 存储不同类型的集合
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 10.0, height: 20.0 }),
];
```

## 💻 实战项目：图形库（多态）

构建一个支持多种图形的库，演示 trait 和多态。

### 功能需求

1. 定义多种图形（圆形、矩形、三角形）
2. 实现 `Area` 和 `Perimeter` trait
3. 实现 `Draw` trait 用于显示
4. 使用 trait 对象创建混合图形集合
5. 计算总面积和周长

### 项目结构

```
structs-traits/
├── Cargo.toml
├── src/
│   ├── main.rs          # 主程序
│   ├── shapes/
│   │   ├── mod.rs       # 模块定义
│   │   ├── circle.rs    # 圆形
│   │   ├── rectangle.rs # 矩形
│   │   └── triangle.rs  # 三角形
│   └── traits.rs        # Trait 定义
└── README.md
```

### 运行项目

```bash
cargo run
```

### 使用示例

```
=== 图形库演示 ===

圆形:
  半径: 5.00
  面积: 78.54
  周长: 31.42

矩形:
  宽度: 10.00, 高度: 20.00
  面积: 200.00
  周长: 60.00

三角形:
  边长: 3.00, 4.00, 5.00
  面积: 6.00
  周长: 12.00

所有图形总面积: 284.54
所有图形总周长: 103.42
```

## 🧪 练习题

### 练习 1：实现自定义结构体

```rust
// 定义一个 Book 结构体，包含标题、作者、页数
// 实现方法：new(), is_long() (页数 > 300)

struct Book {
    // 你的代码
}

impl Book {
    // 你的代码
}

#[test]
fn test_book() {
    let book = Book::new("Rust 编程".to_string(), "作者".to_string(), 500);
    assert!(book.is_long());
}
```

### 练习 2：使用枚举

```rust
// 定义一个 TrafficLight 枚举，包含 Red, Yellow, Green
// 实现方法 duration() 返回每个灯的持续时间（秒）

enum TrafficLight {
    // 你的代码
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        // 你的代码
    }
}
```

### 练习 3：实现 Trait

```rust
// 为 String 和 Vec<T> 实现 Summary trait

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for String {
    // 你的代码
}

impl<T: std::fmt::Display> Summary for Vec<T> {
    // 你的代码
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 5: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book - Chapter 6: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book - Chapter 10: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 定义和使用结构体
- [ ] 实现方法和关联函数
- [ ] 使用元组结构体和单元结构体
- [ ] 定义和使用枚举
- [ ] 使用 Option 和 Result 处理可选值和错误
- [ ] 定义和实现 trait
- [ ] 使用 trait 约束
- [ ] 理解 trait 对象和动态分发
- [ ] 使用 derive 宏自动实现常见 trait

## 🚀 下一步

完成本模块后，继续学习 [模块 1.3：集合与迭代器](../03-collections-iterators/)。
