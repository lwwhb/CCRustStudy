# 模块 2.3：闭包与函数式编程 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解闭包的语法和捕获机制
2. 掌握 Fn、FnMut、FnOnce trait
3. 学习高阶函数和函数组合
4. 实践函数式编程模式
5. 构建数据处理管道

## 🎯 为什么需要闭包和函数式编程？

### 闭包的价值

**没有闭包的问题**：
```rust
// 想要创建一个带状态的函数
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
    
    fn increment(&mut self) -> i32 {
        self.count += 1;
        self.count
    }
}

// 使用起来很繁琐
let mut counter = Counter::new();
println!("{}", counter.increment());
println!("{}", counter.increment());
```

**使用闭包**：
```rust
// 简洁优雅
fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

let mut counter = make_counter();
println!("{}", counter());  // 1
println!("{}", counter());  // 2
```

### 函数式编程的优势

**命令式风格**：
```rust
let numbers = vec![1, 2, 3, 4, 5];
let mut result = Vec::new();

for num in &numbers {
    if num % 2 == 0 {
        result.push(num * num);
    }
}

// 冗长、容易出错
```

**函数式风格**：
```rust
let result: Vec<_> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .collect();

// 简洁、清晰、不易出错
```

## 📖 核心概念详解

### 1. 闭包基础

闭包是可以捕获环境的匿名函数。

#### 闭包语法

```rust
// 最简单的闭包
let add = |x, y| x + y;
println!("{}", add(2, 3));  // 5

// 带类型标注
let multiply: fn(i32, i32) -> i32 = |x, y| x * y;

// 带代码块
let complex = |x: i32| {
    let y = x * 2;
    let z = y + 1;
    z
};

// 无参数闭包
let say_hello = || println!("Hello!");
say_hello();
```

**闭包 vs 函数**：
```rust
// 函数
fn add_function(x: i32, y: i32) -> i32 {
    x + y
}

// 闭包
let add_closure = |x: i32, y: i32| -> i32 { x + y };

// 简化的闭包（类型推导）
let add_simple = |x, y| x + y;
```

#### 类型推导

```rust
// 编译器会推导闭包的类型
let add = |x, y| x + y;

// 第一次调用确定类型
let result = add(1, 2);  // 推导为 i32

// 后续调用必须使用相同类型
// let result2 = add(1.0, 2.0);  // 错误！类型不匹配
```

### 2. 闭包捕获环境

闭包可以捕获其定义环境中的变量。

#### 不可变借用捕获

```rust
let x = 10;
let print_x = || println!("x = {}", x);

print_x();  // x = 10
println!("x = {}", x);  // x 仍然有效
```

**内存布局**：
```
栈
┌─────────┐
│ x = 10  │
└─────────┘
     ↑
     │ 不可变借用
┌─────────┐
│ print_x │
└─────────┘
```

#### 可变借用捕获

```rust
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};

println!("{}", increment());  // 1
println!("{}", increment());  // 2
// println!("{}", count);  // 错误！count 被可变借用
```

**关键点**：
- 闭包可变借用 `count`
- 在闭包存在期间，不能直接访问 `count`

#### 获取所有权（move）

```rust
let data = vec![1, 2, 3];
let consume = move || {
    println!("{:?}", data);
    data  // 返回 data，转移所有权
};

let result = consume();
// println!("{:?}", data);  // 错误！data 已被移动
// consume();  // 错误！consume 只能调用一次
```

**何时使用 move**：
```rust
use std::thread;

let data = vec![1, 2, 3];

// 必须使用 move，因为线程可能比当前作用域活得更久
thread::spawn(move || {
    println!("{:?}", data);
});
```

### 3. Fn trait 家族

Rust 有三个闭包 trait，形成继承关系。

#### Fn - 不可变借用

```rust
// Fn 可以被多次调用，不修改捕获的变量
fn apply_fn<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let multiplier = 2;
let double = |x| x * multiplier;  // 不可变借用 multiplier

println!("{}", apply_fn(double, 5));  // 10
println!("{}", apply_fn(double, 10)); // 20
```

#### FnMut - 可变借用

```rust
// FnMut 可以修改捕获的变量
fn apply_fn_mut<F>(mut f: F, times: usize)
where
    F: FnMut(),
{
    for _ in 0..times {
        f();
    }
}

let mut count = 0;
let mut increment = || {
    count += 1;
    println!("count = {}", count);
};

apply_fn_mut(increment, 3);
// 输出：
// count = 1
// count = 2
// count = 3
```

#### FnOnce - 获取所有权

```rust
// FnOnce 只能调用一次，会消费捕获的变量
fn apply_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

let data = vec![1, 2, 3];
let consume = move || {
    println!("{:?}", data);
    drop(data);  // 消费 data
};

apply_fn_once(consume);
// apply_fn_once(consume);  // 错误！已经被消费
```

#### Trait 继承关系

```
FnOnce
  ↑
FnMut (实现了 FnOnce)
  ↑
Fn (实现了 FnMut 和 FnOnce)
```

**规则**：
- 所有闭包都实现 `FnOnce`
- 不移动捕获变量的闭包实现 `FnMut`
- 不修改捕获变量的闭包实现 `Fn`

```rust
// 示例
let x = 5;

// 实现 Fn（不修改）
let f1 = || println!("{}", x);

// 实现 FnMut（修改但不移动）
let mut y = 5;
let mut f2 = || y += 1;

// 只实现 FnOnce（移动）
let data = vec![1, 2, 3];
let f3 = move || drop(data);
```

### 4. 高阶函数

高阶函数是接受函数作为参数或返回函数的函数。

#### 接受函数作为参数

```rust
// 接受闭包作为参数
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let double = |x| x * 2;
let result = apply(double, 5);  // 10

// 接受多个函数
fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| f(g(x))
}

let add_one = |x| x + 1;
let double = |x| x * 2;
let f = compose(double, add_one);
println!("{}", f(5));  // (5 + 1) * 2 = 12
```

#### 返回闭包

```rust
// 返回闭包
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

let add_10 = make_adder(10);
println!("{}", add_10(5));   // 15
println!("{}", add_10(20));  // 30

// 返回不同类型的闭包（使用 Box）
fn make_operation(op: char) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        '+' => Box::new(|a, b| a + b),
        '-' => Box::new(|a, b| a - b),
        '*' => Box::new(|a, b| a * b),
        '/' => Box::new(|a, b| a / b),
        _ => Box::new(|a, b| a + b),
    }
}

let add = make_operation('+');
println!("{}", add(10, 5));  // 15
```

### 5. 函数式编程模式

#### map - 转换

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 每个元素乘以 2
let doubled: Vec<_> = numbers.iter()
    .map(|&x| x * 2)
    .collect();

println!("{:?}", doubled);  // [2, 4, 6, 8, 10]
```

#### filter - 过滤

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// 只保留偶数
let evens: Vec<_> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();

println!("{:?}", evens);  // [2, 4, 6]
```

#### fold/reduce - 累积

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 求和
let sum = numbers.iter()
    .fold(0, |acc, &x| acc + x);

println!("{}", sum);  // 15

// 求积
let product = numbers.iter()
    .fold(1, |acc, &x| acc * x);

println!("{}", product);  // 120
```

#### 链式操作

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: i32 = numbers.iter()
    .filter(|&&x| x % 2 == 0)  // 过滤偶数
    .map(|&x| x * x)            // 计算平方
    .take(3)                    // 取前 3 个
    .sum();                     // 求和

println!("{}", result);  // 4 + 16 + 36 = 56
```

### 6. 函数组合

#### compose - 从右到左

```rust
fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

let add_one = |x: i32| x + 1;
let double = |x: i32| x * 2;

// compose(f, g)(x) = f(g(x))
let f = compose(double, add_one);
println!("{}", f(5));  // double(add_one(5)) = double(6) = 12
```

#### pipe - 从左到右

```rust
fn pipe<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// pipe(f, g)(x) = g(f(x))
let f = pipe(add_one, double);
println!("{}", f(5));  // double(add_one(5)) = double(6) = 12
```

#### 部分应用（柯里化）

```rust
fn partial<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

let add = |x, y| x + y;
let add_10 = partial(add, 10);

println!("{}", add_10(5));   // 15
println!("{}", add_10(20));  // 30
```

## 💻 实战项目：数据处理管道

### 项目需求

构建一个可组合的数据处理管道：
1. 支持 filter、map、reduce 操作
2. 惰性求值
3. 可链式调用
4. 自定义迭代器

### 步骤 1：定义管道结构

```rust
pub struct Pipeline<T> {
    data: Vec<T>,
}

impl<T> Pipeline<T> {
    pub fn new(data: Vec<T>) -> Self {
        Pipeline { data }
    }
    
    pub fn filter<F>(self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        let data = self.data
            .into_iter()
            .filter(|x| predicate(x))
            .collect();
        Pipeline { data }
    }
    
    pub fn map<U, F>(self, f: F) -> Pipeline<U>
    where
        F: Fn(T) -> U,
    {
        let data = self.data
            .into_iter()
            .map(f)
            .collect();
        Pipeline { data }
    }
    
    pub fn collect(self) -> Vec<T> {
        self.data
    }
}
```

### 步骤 2：使用管道

```rust
let result = Pipeline::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    .filter(|&x| x % 2 == 0)  // 过滤偶数
    .map(|x| x * x)            // 计算平方
    .collect();

println!("{:?}", result);  // [4, 16, 36, 64, 100]
```

### 步骤 3：自定义迭代器

```rust
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// 使用
let fib: Vec<u64> = Fibonacci::new()
    .take(10)
    .collect();

println!("{:?}", fib);  // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

### 步骤 4：函数组合器

```rust
pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

pub fn repeat<F, T>(f: F, n: usize) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    T: Clone,
{
    move |x| {
        let mut result = x;
        for _ in 0..n {
            result = f(result.clone());
        }
        result
    }
}

// 使用
let double = |x| x * 2;
let f = repeat(double, 3);
println!("{}", f(1));  // 1 * 2 * 2 * 2 = 8
```

## 🔍 深入理解

### 闭包的性能

```rust
// 闭包是零成本抽象
let add = |x, y| x + y;

// 编译后等价于：
struct AddClosure;
impl Fn<(i32, i32)> for AddClosure {
    extern "rust-call" fn call(&self, (x, y): (i32, i32)) -> i32 {
        x + y
    }
}

// 没有运行时开销！
```

### 闭包的大小

```rust
use std::mem::size_of_val;

// 不捕获变量的闭包
let f1 = || 42;
println!("f1 size: {}", size_of_val(&f1));  // 0 字节

// 捕获一个 i32
let x = 42;
let f2 = || x;
println!("f2 size: {}", size_of_val(&f2));  // 4 字节

// 捕获一个 String
let s = String::from("hello");
let f3 = || s.len();
println!("f3 size: {}", size_of_val(&f3));  // 24 字节（指针+长度+容量）
```

## 📝 练习题

### 练习 1：实现 map 函数

```rust
fn my_map<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    // 你的代码
}

#[test]
fn test_my_map() {
    let numbers = vec![1, 2, 3];
    let doubled = my_map(numbers, |x| x * 2);
    assert_eq!(doubled, vec![2, 4, 6]);
}
```

### 练习 2：实现 filter 函数

```rust
fn my_filter<T, F>(vec: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    // 你的代码
}

#[test]
fn test_my_filter() {
    let numbers = vec![1, 2, 3, 4, 5];
    let evens = my_filter(numbers, |&x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4]);
}
```

### 练习 3：实现缓存函数

```rust
fn memoize<F>(f: F) -> impl FnMut(i32) -> i32
where
    F: Fn(i32) -> i32,
{
    // 实现一个带缓存的函数
    // 你的代码
}
```

## ✅ 检查清单

- [ ] 理解闭包的三种捕获方式
- [ ] 掌握 Fn、FnMut、FnOnce 的区别
- [ ] 能够编写高阶函数
- [ ] 理解函数组合
- [ ] 熟练使用迭代器适配器
- [ ] 实现自定义迭代器
- [ ] 理解零成本抽象

## 🔗 延伸阅读

- [The Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

---

**掌握闭包和函数式编程，写出优雅的 Rust 代码！** 🦀✨
