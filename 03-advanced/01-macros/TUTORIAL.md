# 模块 3.1：宏编程 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解宏的概念和用途
2. 掌握声明宏（macro_rules!）
3. 学习宏模式匹配
4. 了解过程宏基础
5. 实现实用的自定义宏

## 🎯 为什么需要宏？

### 宏的价值

**没有宏的问题**：
```rust
// 想要创建一个 HashMap，需要写很多代码
let mut map = HashMap::new();
map.insert("key1", "value1");
map.insert("key2", "value2");
map.insert("key3", "value3");
// 繁琐、重复
```

**使用宏**：
```rust
// 一行搞定
let map = hashmap! {
    "key1" => "value1",
    "key2" => "value2",
    "key3" => "value3",
};
```

### 宏 vs 函数

**函数的局限**：
```rust
// 函数不能做到的事情：
// 1. 可变数量的参数（不同类型）
// 2. 在编译时生成代码
// 3. 实现自定义语法
// 4. 自动实现 trait
```

**宏的能力**：
```rust
// 1. 可变参数
println!("Hello");
println!("Hello, {}", name);
println!("x = {}, y = {}", x, y);

// 2. 编译时代码生成
vec![1, 2, 3];  // 生成创建 Vec 的代码

// 3. 自定义语法
assert_eq!(a, b);  // 看起来像语言特性

// 4. 自动实现 trait
#[derive(Debug, Clone)]  // 自动生成代码
struct Point { x: i32, y: i32 }
```

### 其他语言的对比

**C/C++ 宏**：
```c
#define MAX(a, b) ((a) > (b) ? (a) : (b))

// 问题：
// - 不卫生（可能有命名冲突）
// - 没有类型检查
// - 难以调试
```

**Rust 宏**：
```rust
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

// 优势：
// - 卫生宏（避免命名冲突）
// - 类型安全
// - 更好的错误信息
```

## 📖 核心概念详解

### 1. 声明宏基础

声明宏使用 `macro_rules!` 定义。

#### 最简单的宏

```rust
// 定义宏
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

// 使用宏
say_hello!();  // 输出: Hello, World!
```

**宏展开**：
```rust
// 宏调用
say_hello!();

// 展开后的代码
println!("Hello, World!");
```

#### 带参数的宏

```rust
macro_rules! say_hello {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

say_hello!("Alice");  // 输出: Hello, Alice!
say_hello!("Bob");    // 输出: Hello, Bob!
```

**参数类型**：
```rust
$name:ident  // 标识符（变量名、函数名等）
$name:expr   // 表达式
$name:ty     // 类型
$name:pat    // 模式
$name:stmt   // 语句
$name:block  // 代码块
$name:item   // 项（函数、结构体等）
$name:meta   // 元数据
$name:tt     // token tree（任何 token）
```

### 2. 宏模式匹配

宏可以有多个匹配分支。

#### 多个模式

```rust
macro_rules! say_hello {
    // 无参数
    () => {
        println!("Hello!");
    };
    
    // 一个参数
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    
    // 两个参数
    ($name:expr, $greeting:expr) => {
        println!("{}, {}!", $greeting, $name);
    };
}

say_hello!();                    // Hello!
say_hello!("Alice");             // Hello, Alice!
say_hello!("Bob", "Hi");         // Hi, Bob!
```

#### 重复模式

```rust
// 使用 $(...),* 表示重复
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = my_vec![1, 2, 3, 4, 5];
// 等价于：
// let mut temp_vec = Vec::new();
// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);
// temp_vec.push(4);
// temp_vec.push(5);
```

**重复符号**：
```rust
$(...)*   // 0 次或多次
$(...)+   // 1 次或多次
$(...)?   // 0 次或 1 次

// 分隔符
$(...),*  // 逗号分隔
$(...);*  // 分号分隔
```

### 3. 实用宏示例

#### HashMap 构建宏

```rust
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// 使用
let scores = hashmap! {
    "Alice" => 95,
    "Bob" => 87,
    "Carol" => 92,
};
```

**关键点**：
- `$(,)?` - 允许尾随逗号
- `$(...)*` - 重复模式
- 返回创建的 HashMap

#### 日志宏

```rust
macro_rules! log {
    ($level:expr, $msg:expr) => {
        println!("[{}] {}", $level, $msg);
    };
    
    ($level:expr, $fmt:expr, $($arg:expr),+) => {
        println!("[{}] {}", $level, format!($fmt, $($arg),+));
    };
}

// 使用
log!("INFO", "Application started");
log!("DEBUG", "Processing {} items", 42);
log!("ERROR", "Connection failed: {}", "timeout");
```

#### 调试打印宏

```rust
macro_rules! debug_print {
    ($val:expr) => {
        println!("{} = {:?}", stringify!($val), $val);
    };
}

let x = 42;
let y = vec![1, 2, 3];

debug_print!(x);  // x = 42
debug_print!(y);  // y = [1, 2, 3]
debug_print!(x + 10);  // x + 10 = 52
```

**stringify!**：
- 将表达式转换为字符串字面量
- 在编译时执行

### 4. 高级宏技巧

#### 递归宏

```rust
macro_rules! count {
    () => (0);
    ($x:tt $($xs:tt)*) => (1 + count!($($xs)*));
}

let n = count!(a b c d e);  // 5
```

#### 内部规则

```rust
macro_rules! my_macro {
    // 公开接口
    ($x:expr) => {
        my_macro!(@internal $x, 0)
    };
    
    // 内部规则（以 @ 开头）
    (@internal $x:expr, $y:expr) => {
        $x + $y
    };
}
```

#### TT Muncher 模式

```rust
macro_rules! parse {
    // 基础情况
    () => {};
    
    // 递归处理
    (+ $($rest:tt)*) => {
        println!("Found +");
        parse!($($rest)*);
    };
    
    (- $($rest:tt)*) => {
        println!("Found -");
        parse!($($rest)*);
    };
    
    ($num:literal $($rest:tt)*) => {
        println!("Found number: {}", $num);
        parse!($($rest)*);
    };
}

parse!(1 + 2 - 3);
// 输出:
// Found number: 1
// Found +
// Found number: 2
// Found -
// Found number: 3
```

### 5. 宏的卫生性

Rust 宏是卫生的，避免命名冲突。

```rust
macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42;  // 宏内部的 a
            $e
        }
    };
}

let a = 10;  // 外部的 a
let result = using_a!(a + 1);  // 使用外部的 a
println!("{}", result);  // 11，不是 43
```

**卫生性规则**：
- 宏内部定义的变量不会影响外部
- 宏外部的变量可以在宏内使用
- 避免意外的命名冲突

### 6. 实用宏模式

#### 测量执行时间

```rust
macro_rules! time_it {
    ($name:expr, $code:block) => {
        {
            let start = std::time::Instant::now();
            let result = $code;
            let duration = start.elapsed();
            println!("{} took {:?}", $name, duration);
            result
        }
    };
}

// 使用
let result = time_it!("计算总和", {
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    sum
});
```

#### 重复执行

```rust
macro_rules! repeat {
    ($n:expr, $code:block) => {
        for _ in 0..$n {
            $code
        }
    };
}

// 使用
repeat!(5, {
    println!("Hello!");
});
```

#### 创建函数

```rust
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// 使用
create_function!(foo);
create_function!(bar);

foo();  // You called "foo"()
bar();  // You called "bar"()
```

## 💻 实战项目：实用宏库

### 项目需求

创建一个包含多种实用宏的库：
1. 集合构建宏（vec、hashmap、hashset）
2. 日志宏
3. 断言宏
4. 性能测量宏
5. DSL 宏

### 实现示例

#### 1. 增强的 Vec 宏

```rust
macro_rules! my_vec {
    // 空 Vec
    () => {
        Vec::new()
    };
    
    // 重复元素
    ($elem:expr; $n:expr) => {
        vec![$elem; $n]
    };
    
    // 元素列表
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )+
            temp_vec
        }
    };
}
```

#### 2. 增强的日志宏

```rust
macro_rules! log {
    (INFO, $($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
    
    (DEBUG, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
    
    (ERROR, $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
}
```

#### 3. 自定义断言宏

```rust
macro_rules! assert_between {
    ($val:expr, $min:expr, $max:expr) => {
        if !($min <= $val && $val <= $max) {
            panic!(
                "assertion failed: {} <= {} <= {} (actual: {})",
                $min,
                stringify!($val),
                $max,
                $val
            );
        }
    };
}

// 使用
let x = 5;
assert_between!(x, 0, 10);  // OK
// assert_between!(x, 6, 10);  // panic!
```

## 🔍 深入理解

### 宏展开过程

```rust
// 1. 宏定义
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// 2. 宏调用
let result = add!(2, 3);

// 3. 宏展开（编译时）
let result = 2 + 3;

// 4. 编译
// 生成机器码
```

### 宏的限制

```rust
// ❌ 不能做的事情：

// 1. 不能在宏内定义宏
macro_rules! outer {
    () => {
        macro_rules! inner {  // 错误！
            () => {};
        }
    };
}

// 2. 不能导出宏内部的项
macro_rules! define_struct {
    () => {
        struct MyStruct;  // 这个结构体在宏外部不可见
    };
}

// 3. 不能访问宏外部的私有项
mod private {
    fn secret() {}
}

macro_rules! call_secret {
    () => {
        private::secret();  // 错误！
    };
}
```

### 调试宏

```rust
// 使用 cargo expand 查看宏展开
// cargo install cargo-expand
// cargo expand

// 使用 trace_macros! 跟踪宏展开
#![feature(trace_macros)]

trace_macros!(true);
let v = vec![1, 2, 3];
trace_macros!(false);
```

## 🧪 练习题

### 练习 1：实现 min/max 宏

```rust
// 实现一个宏，找出多个值中的最小/最大值
macro_rules! min {
    // 你的代码
}

macro_rules! max {
    // 你的代码
}

#[test]
fn test_min_max() {
    assert_eq!(min!(1, 2, 3), 1);
    assert_eq!(max!(1, 2, 3), 3);
}
```

### 练习 2：实现 unless 宏

```rust
// 实现一个 unless 宏（if 的反向）
macro_rules! unless {
    // 你的代码
}

// 使用
unless!(false, {
    println!("This should print");
});
```

### 练习 3：实现 enum_str 宏

```rust
// 实现一个宏，为枚举生成 to_string 方法
macro_rules! enum_str {
    // 你的代码
}

enum_str! {
    enum Color {
        Red,
        Green,
        Blue,
    }
}

#[test]
fn test_enum_str() {
    assert_eq!(Color::Red.to_string(), "Red");
}
```

## 📝 常见问题

### 1. 宏卫生性问题

```rust
// 问题：想在宏中使用外部变量
macro_rules! use_x {
    () => {
        println!("{}", x);  // 错误！x 未定义
    };
}

// 解决：将变量作为参数传入
macro_rules! use_var {
    ($var:ident) => {
        println!("{}", $var);
    };
}

let x = 42;
use_var!(x);  // OK
```

### 2. 类型推导问题

```rust
// 问题：宏生成的代码类型不明确
macro_rules! make_vec {
    () => {
        Vec::new()  // 类型未知
    };
}

// 解决：显式指定类型
let v: Vec<i32> = make_vec!();
```

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 理解宏的概念和用途
- [ ] 使用 macro_rules! 定义声明宏
- [ ] 使用宏模式匹配
- [ ] 实现重复模式
- [ ] 理解宏的卫生性
- [ ] 创建实用的自定义宏
- [ ] 调试宏展开问题

## 🔗 延伸阅读

- [The Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [macro_rules! documentation](https://doc.rust-lang.org/reference/macros-by-example.html)

---

**掌握宏编程，提升代码复用能力！** 🚀
