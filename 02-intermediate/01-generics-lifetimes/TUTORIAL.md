# 模块 2.1：泛型与生命周期 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 深入理解泛型编程
2. 掌握生命周期标注
3. 理解借用检查器的工作原理
4. 学习生命周期省略规则
5. 实现泛型数据结构

## 🎯 为什么需要泛型和生命周期？

### 泛型的价值

**没有泛型的代码**：
```rust
// 为每种类型写重复代码
fn find_max_i32(list: &[i32]) -> Option<&i32> {
    if list.is_empty() { return None; }
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    Some(max)
}

fn find_max_f64(list: &[f64]) -> Option<&f64> {
    // 完全相同的逻辑，只是类型不同
    if list.is_empty() { return None; }
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    Some(max)
}

// 需要为每种类型都写一遍！
```

**使用泛型**：
```rust
// 一次编写，适用所有类型
fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() { return None; }
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    Some(max)
}

// 可以用于任何可比较的类型
let numbers = vec![1, 5, 3, 9, 2];
let max = find_max(&numbers);  // 适用于 i32

let floats = vec![1.5, 3.2, 2.1];
let max = find_max(&floats);   // 适用于 f64
```

### 生命周期的必要性

**问题场景**：
```rust
// 这段代码有什么问题？
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// 编译器的困惑：
// - 返回的引用来自 x 还是 y？
// - 返回的引用能存活多久？
// - 调用者如何知道返回值的有效期？
```

**使用生命周期标注**：
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 明确告诉编译器：
// - 返回值的生命周期与 x 和 y 中较短的那个相同
// - 调用者必须确保 x 和 y 在返回值使用期间都有效
```

## 📖 核心概念详解

### 1. 泛型函数

泛型函数可以处理多种类型的数据。

#### 基础泛型函数

```rust
// 单个泛型参数
fn print_value<T: std::fmt::Display>(value: T) {
    println!("值: {}", value);
}

// 使用
print_value(42);           // T = i32
print_value("hello");      // T = &str
print_value(3.14);         // T = f64

// 多个泛型参数
fn swap<T, U>(pair: (T, U)) -> (U, T) {
    (pair.1, pair.0)
}

let result = swap((1, "hello"));  // (String, i32)
```

#### Trait 约束

泛型参数通常需要 trait 约束来指定类型必须具备的能力。

```rust
// 使用 trait 约束
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {  // 需要 PartialOrd trait
            largest = item;
        }
    }
    largest
}

// 多个 trait 约束
fn print_and_compare<T: std::fmt::Display + PartialOrd>(a: T, b: T) {
    println!("a = {}, b = {}", a, b);
    if a > b {
        println!("a 更大");
    } else {
        println!("b 更大或相等");
    }
}

// 使用 where 子句（更清晰）
fn complex_function<T, U>(t: T, u: U) -> String
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone,
{
    format!("t = {}, u = {:?}", t, u)
}
```

**常用 trait 约束**：
```
Display    - 可以格式化输出
Debug      - 可以调试输出
Clone      - 可以克隆
Copy       - 可以复制
PartialEq  - 可以比较相等
PartialOrd - 可以比较大小
Default    - 有默认值
```

### 2. 泛型结构体

结构体也可以使用泛型。

#### 基础泛型结构体

```rust
// 单个泛型参数
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// 使用
let int_point = Point::new(5, 10);      // Point<i32>
let float_point = Point::new(1.0, 4.0); // Point<f64>

// 多个泛型参数
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

let pair = Pair::new(1, "hello");
let swapped = pair.swap();  // Pair<&str, i32>
```

#### 为特定类型实现方法

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 为所有类型实现
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 只为 f64 实现
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 只为实现了 Display 的类型实现
impl<T: std::fmt::Display> Point<T> {
    fn print(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}
```

### 3. 泛型枚举

Rust 标准库中的 Option 和 Result 就是泛型枚举。

```rust
// Option 的定义
enum Option<T> {
    Some(T),
    None,
}

// Result 的定义
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 自定义泛型枚举
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
}
```

### 4. 生命周期基础

生命周期是 Rust 最独特的特性之一。

#### 为什么需要生命周期？

```rust
// 这段代码为什么不能编译？
fn dangling_reference() -> &String {
    let s = String::from("hello");
    &s  // 错误！s 将被释放，返回悬垂引用
}

// 生命周期防止悬垂引用
fn valid_reference<'a>(s: &'a String) -> &'a String {
    s  // OK，返回的引用与输入的生命周期相同
}
```

#### 生命周期标注语法

```rust
// 生命周期参数以 ' 开头
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 读作：
// "函数 longest 有一个生命周期参数 'a"
// "参数 x 和 y 都是字符串切片的引用，生命周期为 'a"
// "返回值也是字符串切片的引用，生命周期为 'a"
```

**生命周期的含义**：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 使用示例
let string1 = String::from("long string");
let result;
{
    let string2 = String::from("short");
    result = longest(string1.as_str(), string2.as_str());
    // result 的生命周期 = min(string1 的生命周期, string2 的生命周期)
    // result 的生命周期 = string2 的生命周期
}
// println!("{}", result);  // 错误！string2 已失效
```

#### 结构体中的生命周期

```rust
// 结构体持有引用，需要生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}

// 使用
let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().expect("找不到 '.'");
let excerpt = ImportantExcerpt {
    part: first_sentence,
};
// excerpt 的生命周期不能超过 novel
```

### 5. 生命周期省略规则

编译器可以在某些情况下自动推导生命周期。

#### 三条省略规则

```rust
// 规则 1：每个引用参数都有自己的生命周期
fn foo(x: &i32, y: &i32) { }
// 等价于
fn foo<'a, 'b>(x: &'a i32, y: &'b i32) { }

// 规则 2：如果只有一个输入生命周期参数，
// 它被赋给所有输出生命周期参数
fn first_word(s: &str) -> &str { }
// 等价于
fn first_word<'a>(s: &'a str) -> &'a str { }

// 规则 3：如果有多个输入生命周期参数，
// 但其中一个是 &self 或 &mut self，
// self 的生命周期被赋给所有输出生命周期参数
impl<'a> ImportantExcerpt<'a> {
    fn get_part(&self) -> &str { }
    // 等价于
    fn get_part(&self) -> &'a str { }
}
```

**何时需要显式标注**：

```rust
// 需要显式标注（规则无法推导）
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 不需要显式标注（规则 2 可以推导）
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// 不需要显式标注（规则 3 可以推导）
impl<'a> MyStruct<'a> {
    fn get_data(&self) -> &str {
        self.data
    }
}
```

### 6. 静态生命周期

'static 是一个特殊的生命周期，表示整个程序运行期间。

```rust
// 字符串字面量有 'static 生命周期
let s: &'static str = "I have a static lifetime";

// 静态变量
static HELLO: &str = "Hello, world!";

// 何时使用 'static
fn needs_static() -> &'static str {
    "这个字符串存活于整个程序"
}

// 注意：不要滥用 'static
// ❌ 错误的使用
fn bad_example<T: 'static>(x: T) -> T {
    x  // 要求 T 必须是 'static，过于严格
}

// ✅ 正确的使用
fn good_example<T>(x: T) -> T {
    x  // 不需要 'static 约束
}
```

### 7. 生命周期约束

生命周期可以有约束关系。

```rust
// 'b 的生命周期至少和 'a 一样长
struct Ref<'a, 'b: 'a> {
    x: &'a i32,
    y: &'b i32,
}

// 泛型类型的生命周期约束
struct Parser<'a, T: 'a> {
    data: &'a T,
}

// 多个生命周期约束
fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,  // 'b 至少和 'a 一样长
{
    x
}
```

## 💻 实战项目：泛型数据结构库

### 项目需求

实现多个泛型数据结构：
1. 泛型栈（Stack）
2. 泛型队列（Queue）
3. 泛型链表（LinkedList）
4. 泛型缓存（Cache）
5. 带生命周期的结构体

### 步骤 1：泛型栈

```rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

// 使用
let mut stack = Stack::new();
stack.push(1);
stack.push(2);
stack.push(3);
assert_eq!(stack.pop(), Some(3));
```

### 步骤 2：泛型队列

```rust
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }
    
    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    
    pub fn front(&self) -> Option<&T> {
        self.items.first()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
```

### 步骤 3：带生命周期的结构体

```rust
pub struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }
    
    pub fn first_sentence(&self) -> &str {
        self.text
            .split('.')
            .next()
            .unwrap_or(self.text)
    }
    
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

// 使用
let novel = String::from("Call me Ishmael. Some years ago...");
let excerpt = Excerpt::new(&novel);
println!("第一句: {}", excerpt.first_sentence());
```

### 步骤 4：泛型缓存

```rust
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K, V> {
    map: HashMap<K, V>,
    capacity: usize,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Cache {
            map: HashMap::new(),
            capacity,
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.map.len() >= self.capacity && !self.map.contains_key(&key) {
            // 缓存已满，这里简单地拒绝插入
            // 实际应用中可以实现 LRU 等策略
            return None;
        }
        self.map.insert(key, value)
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    pub fn is_full(&self) -> bool {
        self.map.len() >= self.capacity
    }
}
```

## 🔍 深入理解

### 泛型的零成本抽象

Rust 的泛型是零成本的，编译时会进行单态化。

```rust
// 源代码
fn print<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

print(5);
print("hello");

// 编译后（单态化）
fn print_i32(value: i32) {
    println!("{}", value);
}

fn print_str(value: &str) {
    println!("{}", value);
}

print_i32(5);
print_str("hello");

// 没有运行时开销！
```

### 生命周期的本质

生命周期不是运行时的概念，而是编译时的约束。

```rust
// 生命周期标注不改变实际生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 'a 只是告诉编译器：
// "返回值的生命周期不会超过 x 和 y 中较短的那个"

// 编译器使用这个信息来验证代码的安全性
```

### 常见陷阱

**陷阱 1：过度约束生命周期**

```rust
// ❌ 错误：过度约束
fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

// ✅ 正确：只约束需要的
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

**陷阱 2：混淆生命周期和作用域**

```rust
// 生命周期 ≠ 作用域
let r;
{
    let x = 5;
    r = &x;  // 错误！x 的生命周期太短
}
// println!("{}", r);
```

**陷阱 3：不必要的 'static**

```rust
// ❌ 不好：要求 'static
fn process<T: 'static>(x: T) -> T {
    x
}

// ✅ 更好：不需要 'static
fn process<T>(x: T) -> T {
    x
}
```

## 📝 练习题

### 练习 1：实现泛型函数

```rust
// 实现一个泛型函数，找出切片中的最小值
fn find_min<T: PartialOrd>(list: &[T]) -> Option<&T> {
    // 你的代码
}

#[test]
fn test_find_min() {
    assert_eq!(find_min(&[3, 1, 4, 1, 5]), Some(&1));
    assert_eq!(find_min(&[1.5, 2.3, 0.5]), Some(&0.5));
}
```

### 练习 2：实现泛型结构体

```rust
// 实现一个泛型 Pair 结构体，支持交换和比较
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        // 你的代码
    }
    
    fn swap(self) -> Self {
        // 你的代码
    }
}

impl<T: PartialOrd> Pair<T> {
    fn max(&self) -> &T {
        // 你的代码
    }
}
```

### 练习 3：生命周期标注

```rust
// 为这个函数添加正确的生命周期标注
fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// 为这个结构体添加生命周期标注
struct Book {
    title: &str,
    author: &str,
}
```

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 编写泛型函数和结构体
- [ ] 使用 trait 约束限制泛型类型
- [ ] 理解单态化和零成本抽象
- [ ] 正确标注生命周期
- [ ] 理解生命周期省略规则
- [ ] 在结构体中使用生命周期
- [ ] 理解 'static 生命周期
- [ ] 避免常见的生命周期陷阱

## 🚀 下一步

完成本模块后，继续学习 [模块 2.2：智能指针](../02-smart-pointers/)。

---

**掌握泛型和生命周期，你就掌握了 Rust 的精髓！** 🦀
