# 模块 1.2：结构体、枚举与 Trait - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 掌握结构体的定义和使用
2. 理解枚举的强大功能
3. 深入学习 Trait 系统
4. 掌握多态和动态分发
5. 构建一个图形库项目

## 🎯 为什么需要结构体和 Trait？

### 其他语言的对比

**C 语言的结构体**：
```c
struct Point {
    int x;
    int y;
};

// 函数分离，没有封装
void move_point(struct Point* p, int dx, int dy) {
    p->x += dx;
    p->y += dy;
}
```

**Java 的类和接口**：
```java
interface Drawable {
    void draw();
}

class Circle implements Drawable {
    private double radius;

    public void draw() {
        // 实现
    }
}
```

**Rust 的结构体和 Trait**：
```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("绘制圆形");
    }
}
```

**Rust 的优势**：
- 零成本抽象
- 编译时多态（静态分发）
- 运行时多态（动态分发）
- 没有继承，使用组合
- 更灵活的代码复用

## 📖 核心概念详解

### 1. 结构体（Structs）

结构体是创建自定义数据类型的基础。

#### 三种结构体类型

```rust
// 1. 命名字段结构体（最常用）
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 2. 元组结构体（字段没有名字）
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 3. 单元结构体（没有字段）
struct AlwaysEqual;
```

**为什么有三种？**
- 命名字段：清晰表达意图，适合复杂数据
- 元组结构体：简洁，适合简单数据包装
- 单元结构体：标记类型，实现 trait

#### 创建和使用结构体

```rust
// 创建实例
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
    active: true,
};

// 访问字段
println!("用户名: {}", user1.username);

// 可变实例
let mut user2 = User {
    username: String::from("bob"),
    email: String::from("bob@example.com"),
    sign_in_count: 0,
    active: true,
};

// 修改字段（整个实例必须是可变的）
user2.sign_in_count += 1;

// 字段初始化简写
let username = String::from("charlie");
let email = String::from("charlie@example.com");

let user3 = User {
    username,  // 等同于 username: username
    email,     // 等同于 email: email
    sign_in_count: 0,
    active: true,
};

// 结构体更新语法
let user4 = User {
    email: String::from("david@example.com"),
    ..user3  // 其他字段从 user3 复制
};
```

**注意**：使用 `..` 语法会移动数据！

```rust
let user3 = User { /* ... */ };
let user4 = User {
    email: String::from("new@example.com"),
    ..user3  // username 被移动到 user4
};

// println!("{}", user3.username);  // 错误！username 已被移动
println!("{}", user3.active);  // OK，bool 实现了 Copy
```

### 2. 方法和关联函数

```rust
impl User {
    // 关联函数（类似静态方法）
    // 不需要 self 参数
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }

    // 方法（需要 self 参数）
    // &self - 不可变借用
    fn is_active(&self) -> bool {
        self.active
    }

    // &mut self - 可变借用
    fn increment_sign_in(&mut self) {
        self.sign_in_count += 1;
    }

    // self - 获取所有权（消费 self）
    fn into_email(self) -> String {
        self.email
    }
}

// 使用
let mut user = User::new(
    String::from("alice"),
    String::from("alice@example.com")
);

user.increment_sign_in();
println!("活跃: {}", user.is_active());

let email = user.into_email();  // user 被消费
// println!("{}", user.username);  // 错误！user 已失效
```

**self 的三种形式**：
```
&self       -> 借用，只读访问
&mut self   -> 可变借用，可以修改
self        -> 获取所有权，消费实例
```

### 3. 枚举（Enums）

枚举比其他语言更强大，每个变体可以携带不同类型的数据。

#### 基础枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

#### 携带数据的枚举

```rust
// 每个变体可以有不同的数据
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// 更复杂的例子
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },   // 命名字段
    Write(String),              // 单个值
    ChangeColor(i32, i32, i32), // 元组
}
```

**为什么这样设计？**

传统方式（使用结构体）：
```rust
struct QuitMessage;
struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// 需要不同的类型，难以统一处理
```

使用枚举：
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 统一类型，易于处理
fn process_message(msg: Message) {
    match msg {
        Message::Quit => { /* ... */ },
        Message::Move { x, y } => { /* ... */ },
        Message::Write(text) => { /* ... */ },
        Message::ChangeColor(r, g, b) => { /* ... */ },
    }
}
```

#### 枚举的方法

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("颜色: RGB({}, {}, {})", r, g, b)
            }
        }
    }
}

let msg = Message::Write(String::from("hello"));
msg.call();  // 输出: 写入: hello
```

### 4. Option 和 Result

Rust 没有 null，使用 `Option` 表示可能不存在的值。

#### Option<T>

```rust
enum Option<T> {
    Some(T),
    None,
}

// 使用示例
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

// 处理 Option
match find_user(1) {
    Some(name) => println!("找到用户: {}", name),
    None => println!("用户不存在"),
}

// 使用 if let
if let Some(name) = find_user(1) {
    println!("找到用户: {}", name);
}

// 使用方法
let name = find_user(1).unwrap_or(String::from("Guest"));
```

**为什么没有 null？**

```
其他语言的问题：
let user = find_user(1);
user.name;  // 如果 user 是 null，运行时崩溃！

Rust 的解决方案：
let user = find_user(1);  // 类型是 Option<User>
user.name;  // 编译错误！必须先处理 None 的情况
```

#### Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 使用示例
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

// 处理 Result
match divide(10.0, 2.0) {
    Ok(result) => println!("结果: {}", result),
    Err(e) => println!("错误: {}", e),
}

// 使用 ? 操作符传播错误
fn calculate() -> Result<f64, String> {
    let x = divide(10.0, 2.0)?;  // 如果是 Err，直接返回
    let y = divide(x, 3.0)?;
    Ok(y)
}
```

### 5. Trait（特征）

Trait 定义共享的行为，类似其他语言的接口。

#### 定义和实现 Trait

```rust
// 定义 trait
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn summarize_author(&self) -> String {
        String::from("(匿名作者)")
    }
}

// 为类型实现 trait
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, 作者: {}", self.title, self.author)
    }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
    // 使用默认的 summarize_author
}
```

#### Trait 作为参数

```rust
// 方式 1: impl Trait 语法
fn notify(item: &impl Summary) {
    println!("新消息: {}", item.summarize());
}

// 方式 2: Trait 约束语法
fn notify<T: Summary>(item: &T) {
    println!("新消息: {}", item.summarize());
}

// 多个 trait 约束
fn notify<T: Summary + Display>(item: &T) {
    println!("{}", item);
    println!("{}", item.summarize());
}

// where 子句（更清晰）
fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    format!("{:?}", u)
}
```

#### 返回实现 Trait 的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，你可能已经知道了"),
    }
}

// 注意：只能返回单一类型
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Article { /* ... */ }  // 错误！
    } else {
        Tweet { /* ... */ }    // 不能返回不同类型
    }
}
```

### 6. Trait 对象和动态分发

当需要在运行时处理不同类型时，使用 trait 对象。

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("绘制圆形，半径: {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("绘制矩形，{}x{}", self.width, self.height);
    }
}

// 使用 trait 对象
fn draw_shape(shape: &dyn Draw) {
    shape.draw();
}

// 存储不同类型的集合
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 10.0, height: 20.0 }),
];

for shape in shapes.iter() {
    shape.draw();
}
```

**静态分发 vs 动态分发**：

```rust
// 静态分发（编译时确定）
fn draw_static<T: Draw>(shape: &T) {
    shape.draw();
}
// 优点：性能好，编译器内联优化
// 缺点：不能存储不同类型的集合

// 动态分发（运行时确定）
fn draw_dynamic(shape: &dyn Draw) {
    shape.draw();
}
// 优点：灵活，可以处理不同类型
// 缺点：有轻微性能开销（虚函数表查找）
```

## 💻 实战项目：图形库

### 项目需求

构建一个图形库，支持：
1. 多种图形（圆形、矩形、三角形）
2. 计算面积和周长
3. 绘制图形
4. 使用 trait 对象实现多态

### 步骤 1：定义 Trait

```rust
// traits.rs
pub trait Area {
    fn area(&self) -> f64;
}

pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

pub trait Draw {
    fn draw(&self);
}

// 组合 trait
pub trait Shape: Area + Perimeter + Draw {}

// 自动为实现了三个 trait 的类型实现 Shape
impl<T> Shape for T where T: Area + Perimeter + Draw {}
```

**关键点**：
- `Shape` 是一个 supertrait，要求实现三个基础 trait
- 使用 blanket implementation 自动实现

### 步骤 2：实现圆形

```rust
// shapes/circle.rs
use crate::traits::{Area, Draw, Perimeter};

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Draw for Circle {
    fn draw(&self) {
        println!("圆形:");
        println!("  半径: {:.2}", self.radius);
        println!("  面积: {:.2}", self.area());
        println!("  周长: {:.2}", self.perimeter());
    }
}
```

### 步骤 3：实现矩形

```rust
// shapes/rectangle.rs
use crate::traits::{Area, Draw, Perimeter};

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("矩形:");
        println!("  宽度: {:.2}, 高度: {:.2}", self.width, self.height);
        println!("  面积: {:.2}", self.area());
        println!("  周长: {:.2}", self.perimeter());
    }
}
```

### 步骤 4：实现三角形

```rust
// shapes/triangle.rs
use crate::traits::{Area, Draw, Perimeter};

pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Result<Self, String> {
        // 验证三角形不等式
        if a + b <= c || b + c <= a || a + c <= b {
            return Err(String::from("无效的三角形边长"));
        }
        Ok(Self { a, b, c })
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        // 使用海伦公式
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("三角形:");
        println!("  边长: {:.2}, {:.2}, {:.2}", self.a, self.b, self.c);
        println!("  面积: {:.2}", self.area());
        println!("  周长: {:.2}", self.perimeter());
    }
}
```

### 步骤 5：主程序

```rust
// main.rs
mod shapes;
mod traits;

use shapes::{Circle, Rectangle, Triangle};
use traits::Shape;

fn main() {
    println!("=== 图形库演示 ===\n");

    // 创建图形
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(10.0, 20.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0).expect("无效的三角形");

    // 绘制每个图形
    circle.draw();
    println!();
    rectangle.draw();
    println!();
    triangle.draw();
    println!();

    // 使用 trait 对象实现多态
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(rectangle),
        Box::new(triangle),
    ];

    // 计算总面积和总周长
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    let total_perimeter: f64 = shapes.iter().map(|s| s.perimeter()).sum();

    println!("=== 统计信息 ===");
    println!("图形总数: {}", shapes.len());
    println!("所有图形总面积: {:.2}", total_area);
    println!("所有图形总周长: {:.2}", total_perimeter);

    // 演示泛型函数
    println!("\n=== 泛型函数演示 ===");
    print_shape_info(&Circle::new(3.0));
    print_shape_info(&Rectangle::new(5.0, 5.0));
}

/// 泛型函数：打印任何实现了 Shape trait 的图形信息
fn print_shape_info<T: Shape>(shape: &T) {
    println!(
        "面积: {:.2}, 周长: {:.2}",
        shape.area(),
        shape.perimeter()
    );
}
```

## 🔍 深入理解

### Trait 对象的内存布局

```rust
// 静态分发
let circle = Circle::new(5.0);
// 内存: 只有 Circle 的数据（8 字节的 f64）

// 动态分发
let shape: Box<dyn Draw> = Box::new(Circle::new(5.0));
// 内存:
// - 指向数据的指针（8 字节）
// - 指向虚函数表的指针（8 字节）
// 总共 16 字节（胖指针）
```

### 为什么需要 Box？

```rust
// 错误：trait 对象大小未知
let shapes: Vec<dyn Draw> = vec![/* ... */];

// 正确：Box 提供固定大小
let shapes: Vec<Box<dyn Draw>> = vec![/* ... */];

// 也可以使用引用
let circle = Circle::new(5.0);
let shape: &dyn Draw = &circle;
```

### Trait 的孤儿规则

```rust
// 可以：为自己的类型实现标准库的 trait
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Circle({})", self.radius)
    }
}

// 可以：为标准库类型实现自己的 trait
impl Area for String {
    fn area(&self) -> f64 {
        self.len() as f64
    }
}

// 不可以：为标准库类型实现标准库的 trait
// impl Display for Vec<i32> { }  // 错误！
```

**为什么？** 防止不同 crate 之间的冲突。

## 📝 练习题

### 练习 1：实现自定义结构体

```rust
// 定义一个 Book 结构体
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Self {
        Self { title, author, pages }
    }

    fn is_long(&self) -> bool {
        self.pages > 300
    }
}

#[test]
fn test_book() {
    let book = Book::new(
        "Rust 编程".to_string(),
        "作者".to_string(),
        500
    );
    assert!(book.is_long());
}
```

### 练习 2：使用枚举

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 45,
        }
    }
}

#[test]
fn test_traffic_light() {
    assert_eq!(TrafficLight::Red.duration(), 60);
    assert_eq!(TrafficLight::Yellow.duration(), 3);
    assert_eq!(TrafficLight::Green.duration(), 45);
}
```

### 练习 3：实现 Trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for String {
    fn summarize(&self) -> String {
        if self.len() > 20 {
            format!("{}...", &self[..20])
        } else {
            self.clone()
        }
    }
}

impl<T: std::fmt::Display> Summary for Vec<T> {
    fn summarize(&self) -> String {
        format!("Vec with {} items", self.len())
    }
}

#[test]
fn test_summary() {
    let s = String::from("这是一个很长的字符串，需要被截断");
    assert_eq!(s.summarize().len(), 23);  // 20 + "..."

    let v = vec![1, 2, 3];
    assert_eq!(v.summarize(), "Vec with 3 items");
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 定义和使用三种类型的结构体
- [ ] 实现方法和关联函数
- [ ] 理解 self、&self、&mut self 的区别
- [ ] 定义和使用枚举
- [ ] 使用 Option 和 Result 处理可选值和错误
- [ ] 定义和实现 trait
- [ ] 使用 trait 约束编写泛型函数
- [ ] 理解静态分发和动态分发
- [ ] 使用 trait 对象实现多态
- [ ] 理解孤儿规则

## 🔗 下一步

掌握了结构体和 Trait 后，继续学习：
- [模块 1.3：集合与迭代器](../03-collections-iterators/)
- 深入学习泛型和生命周期
- 了解高级 trait 特性

## 📚 参考资源

- [The Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust by Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)

---

**记住**：Trait 是 Rust 实现抽象和代码复用的核心机制。掌握 trait 系统，你就掌握了 Rust 的精髓！🦀
