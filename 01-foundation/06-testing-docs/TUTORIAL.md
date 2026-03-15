# 模块 1.6：测试与文档 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 掌握单元测试和集成测试
2. 学习测试组织和最佳实践
3. 编写高质量的文档注释
4. 理解文档测试
5. 实践测试驱动开发（TDD）

## 🎯 为什么需要测试和文档？

### 测试的重要性

**没有测试的问题**：
```rust
fn divide(a: i32, b: i32) -> i32 {
    a / b  // 如果 b 是 0 会怎样？
}

// 部署到生产环境...
// 用户输入 divide(10, 0)
// 程序崩溃！💥
```

**有测试的好处**：
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }
}

// 在开发阶段就发现问题！✅
```

### 文档的价值

**没有文档**：
```rust
pub fn process(x: i32, y: bool, z: &str) -> Option<String> {
    // 这个函数做什么？
    // x、y、z 是什么意思？
    // 什么时候返回 None？
    ...
}
```

**有文档**：
```rust
/// 根据条件处理输入并返回结果
///
/// # 参数
///
/// * `x` - 处理的数值
/// * `y` - 是否启用特殊模式
/// * `z` - 配置字符串
///
/// # 返回
///
/// 成功时返回 `Some(String)`，失败时返回 `None`
///
/// # 示例
///
/// ```
/// let result = process(42, true, "config");
/// assert!(result.is_some());
/// ```
pub fn process(x: i32, y: bool, z: &str) -> Option<String> {
    ...
}
```

## 📖 核心概念详解

### 1. 单元测试

单元测试用于测试单个函数或模块的功能。

#### 基础测试

```rust
// 被测试的函数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;  // 导入父模块的所有项

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }
}
```

**关键点**：
- `#[cfg(test)]` - 只在测试时编译
- `#[test]` - 标记测试函数
- `use super::*` - 导入被测试的代码

#### 断言宏

```rust
#[test]
fn test_assertions() {
    // assert! - 断言条件为真
    assert!(2 + 2 == 4);
    assert!(true);

    // assert_eq! - 断言相等
    assert_eq!(2 + 2, 4);
    assert_eq!("hello", "hello");

    // assert_ne! - 断言不相等
    assert_ne!(2 + 2, 5);

    // 带自定义消息
    assert_eq!(
        2 + 2, 
        4, 
        "数学出错了：2 + 2 应该等于 4"
    );
}
```

#### 测试 panic

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除数不能为零");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0);  // 应该 panic
    }

    #[test]
    #[should_panic(expected = "除数不能为零")]
    fn test_divide_by_zero_with_message() {
        divide(10, 0);  // 检查 panic 消息
    }
}
```

#### 测试 Result

```rust
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| format!("无法解析 '{}'", s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() -> Result<(), String> {
        let num = parse_number("42")?;
        assert_eq!(num, 42);
        Ok(())
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_number("abc").is_err());
    }
}
```

### 2. 测试组织

#### 按功能分组

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod addition {
        use super::*;

        #[test]
        fn positive_numbers() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn negative_numbers() {
            assert_eq!(add(-2, -3), -5);
        }
    }

    mod subtraction {
        use super::*;

        #[test]
        fn positive_numbers() {
            assert_eq!(subtract(5, 3), 2);
        }

        #[test]
        fn negative_numbers() {
            assert_eq!(subtract(-5, -3), -2);
        }
    }
}
```

#### 测试属性

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 忽略测试（默认不运行）
    #[test]
    #[ignore]
    fn expensive_test() {
        // 耗时的测试
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

    // 只在特定平台运行
    #[test]
    #[cfg(target_os = "linux")]
    fn linux_only_test() {
        // 只在 Linux 上运行
    }

    // 只在启用特定 feature 时运行
    #[test]
    #[cfg(feature = "experimental")]
    fn experimental_test() {
        // 只在启用 experimental feature 时运行
    }
}
```

**运行测试**：
```bash
cargo test                    # 运行所有测试
cargo test addition           # 运行名称包含 "addition" 的测试
cargo test -- --ignored       # 运行被忽略的测试
cargo test -- --test-threads=1  # 单线程运行
cargo test -- --nocapture     # 显示 println! 输出
```

### 3. 集成测试

集成测试位于 `tests/` 目录，测试库的公共 API。

#### 目录结构

```
my-project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── integration_test.rs
    ├── common/
    │   └── mod.rs
    └── another_test.rs
```

#### 集成测试示例

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    assert_eq!(my_crate::add(2, 3), 5);
}

#[test]
fn test_complex_workflow() {
    let result = my_crate::process_data("input");
    assert!(result.is_ok());
}
```

#### 共享测试代码

```rust
// tests/common/mod.rs
pub fn setup() -> TestContext {
    TestContext::new()
}

pub struct TestContext {
    // 测试上下文
}

impl TestContext {
    pub fn new() -> Self {
        TestContext {}
    }
}

// tests/integration_test.rs
mod common;

#[test]
fn test_with_setup() {
    let ctx = common::setup();
    // 使用 ctx 进行测试
}
```

### 4. 文档注释

#### 三种文档注释

```rust
//! 模块级文档（在文件顶部）
//! 
//! 这是整个模块的文档

/// 项级文档（在函数、结构体等之前）
/// 
/// 这是单个项的文档
pub fn my_function() {}

// 普通注释（不会出现在文档中）
// 这只是代码注释
```

#### 文档注释结构

```rust
/// 简短的一句话描述
///
/// 更详细的描述，可以有多个段落。
/// 
/// 可以使用 Markdown 格式：
/// - 列表项 1
/// - 列表项 2
///
/// # 示例
///
/// ```
/// let result = my_function(42);
/// assert_eq!(result, 84);
/// ```
///
/// # 参数
///
/// * `x` - 输入值
///
/// # 返回
///
/// 返回输入值的两倍
///
/// # Panics
///
/// 当 x 为负数时会 panic
///
/// # Errors
///
/// 如果 x 超过最大值，返回错误
///
/// # Safety
///
/// 这是一个 unsafe 函数，调用者必须确保...
///
/// # 示例
///
/// ```
/// # use my_crate::my_function;
/// let result = my_function(21);
/// assert_eq!(result, 42);
/// ```
pub fn my_function(x: i32) -> i32 {
    x * 2
}
```

**常用章节**：
- `# Examples` - 使用示例
- `# Panics` - 何时会 panic
- `# Errors` - 可能的错误
- `# Safety` - unsafe 代码的安全要求
- `# Arguments` / `# Parameters` - 参数说明
- `# Returns` - 返回值说明

#### 文档测试

文档中的代码示例会被自动测试！

```rust
/// 计算两数之和
///
/// # 示例
///
/// ```
/// use my_crate::add;
/// 
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 运行 cargo test 会自动测试文档中的代码
```

**文档测试技巧**：

```rust
/// # 示例
///
/// ```
/// # // 以 # 开头的行不会显示在文档中
/// # use my_crate::setup;
/// # let ctx = setup();
/// 
/// // 这行会显示
/// let result = process(&ctx);
/// assert!(result.is_ok());
/// ```

/// 标记为 no_run（编译但不运行）
///
/// ```no_run
/// let server = start_server();  // 不会实际运行
/// ```

/// 标记为 ignore（不编译也不运行）
///
/// ```ignore
/// let x = some_undefined_function();
/// ```

/// 标记为 should_panic
///
/// ```should_panic
/// panic!("This should panic");
/// ```
```

### 5. 测试驱动开发（TDD）

TDD 的三个步骤：红 → 绿 → 重构

#### 步骤 1：红（写失败的测试）

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(calculate_average(&numbers), 3.0);
    }
}

// 此时函数还不存在，测试失败（红）
```

#### 步骤 2：绿（写最简单的实现）

```rust
pub fn calculate_average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// 测试通过（绿）
```

#### 步骤 3：重构（改进代码）

```rust
pub fn calculate_average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(calculate_average(&numbers), 3.0);
    }

    #[test]
    fn test_empty_list() {
        let numbers = vec![];
        assert_eq!(calculate_average(&numbers), 0.0);
    }
}

// 添加更多测试，确保重构没有破坏功能
```

### 6. 测试最佳实践

#### 测试命名

```rust
#[cfg(test)]
mod tests {
    // ✅ 好的命名：描述性强
    #[test]
    fn add_positive_numbers_returns_correct_sum() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn divide_by_zero_returns_error() {
        assert!(divide(10, 0).is_err());
    }

    // ❌ 不好的命名：不清楚测试什么
    #[test]
    fn test1() {
        assert_eq!(add(2, 3), 5);
    }
}
```

#### AAA 模式（Arrange-Act-Assert）

```rust
#[test]
fn test_user_creation() {
    // Arrange（准备）
    let name = "Alice";
    let age = 30;

    // Act（执行）
    let user = User::new(name, age);

    // Assert（断言）
    assert_eq!(user.name, name);
    assert_eq!(user.age, age);
}
```

#### 一个测试一个断言

```rust
// ✅ 好的做法
#[test]
fn test_add_returns_correct_sum() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add_handles_negative_numbers() {
    assert_eq!(add(-2, -3), -5);
}

// ❌ 不好的做法
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-2, -3), -5);
    assert_eq!(add(0, 0), 0);
    // 如果第一个失败，后面的都不会运行
}
```

#### 测试边界条件

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(process(&[]), vec![]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(process(&[1]), vec![1]);
    }

    #[test]
    fn test_maximum_value() {
        assert_eq!(process(&[i32::MAX]), vec![i32::MAX]);
    }

    #[test]
    fn test_minimum_value() {
        assert_eq!(process(&[i32::MIN]), vec![i32::MIN]);
    }
}
```

## 💻 实战项目：完整测试套件

### 项目需求

为一个计算器库编写完整的测试和文档。

### 步骤 1：定义 API

```rust
// src/lib.rs

//! # Calculator Library
//!
//! 一个简单但功能完整的计算器库
//!
//! # 示例
//!
//! ```
//! use calculator::Calculator;
//!
//! let calc = Calculator::new();
//! assert_eq!(calc.add(2, 3), 5);
//! ```

/// 计算器结构体
pub struct Calculator {
    memory: f64,
}

impl Calculator {
    /// 创建新的计算器实例
    ///
    /// # 示例
    ///
    /// ```
    /// use calculator::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// ```
    pub fn new() -> Self {
        Calculator { memory: 0.0 }
    }

    /// 加法运算
    ///
    /// # 参数
    ///
    /// * `a` - 第一个数
    /// * `b` - 第二个数
    ///
    /// # 示例
    ///
    /// ```
    /// use calculator::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// assert_eq!(calc.add(2.0, 3.0), 5.0);
    /// ```
    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    /// 除法运算
    ///
    /// # 参数
    ///
    /// * `a` - 被除数
    /// * `b` - 除数
    ///
    /// # 返回
    ///
    /// 成功时返回 `Ok(f64)`，除数为零时返回 `Err`
    ///
    /// # 示例
    ///
    /// ```
    /// use calculator::Calculator;
    /// 
    /// let calc = Calculator::new();
    /// assert_eq!(calc.divide(10.0, 2.0), Ok(5.0));
    /// assert!(calc.divide(10.0, 0.0).is_err());
    /// ```
    pub fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    /// 存储值到内存
    ///
    /// # 示例
    ///
    /// ```
    /// use calculator::Calculator;
    /// 
    /// let mut calc = Calculator::new();
    /// calc.store(42.0);
    /// assert_eq!(calc.recall(), 42.0);
    /// ```
    pub fn store(&mut self, value: f64) {
        self.memory = value;
    }

    /// 从内存中读取值
    pub fn recall(&self) -> f64 {
        self.memory
    }
}
```

### 步骤 2：编写单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use super::*;

        #[test]
        fn new_calculator_has_zero_memory() {
            let calc = Calculator::new();
            assert_eq!(calc.recall(), 0.0);
        }
    }

    mod addition {
        use super::*;

        #[test]
        fn add_positive_numbers() {
            let calc = Calculator::new();
            assert_eq!(calc.add(2.0, 3.0), 5.0);
        }

        #[test]
        fn add_negative_numbers() {
            let calc = Calculator::new();
            assert_eq!(calc.add(-2.0, -3.0), -5.0);
        }

        #[test]
        fn add_zero() {
            let calc = Calculator::new();
            assert_eq!(calc.add(5.0, 0.0), 5.0);
        }
    }

    mod division {
        use super::*;

        #[test]
        fn divide_normal() {
            let calc = Calculator::new();
            assert_eq!(calc.divide(10.0, 2.0), Ok(5.0));
        }

        #[test]
        fn divide_by_zero_returns_error() {
            let calc = Calculator::new();
            assert!(calc.divide(10.0, 0.0).is_err());
        }

        #[test]
        fn divide_zero_by_number() {
            let calc = Calculator::new();
            assert_eq!(calc.divide(0.0, 5.0), Ok(0.0));
        }
    }

    mod memory {
        use super::*;

        #[test]
        fn store_and_recall() {
            let mut calc = Calculator::new();
            calc.store(42.0);
            assert_eq!(calc.recall(), 42.0);
        }

        #[test]
        fn store_overwrites_previous_value() {
            let mut calc = Calculator::new();
            calc.store(10.0);
            calc.store(20.0);
            assert_eq!(calc.recall(), 20.0);
        }
    }
}
```

### 步骤 3：编写集成测试

```rust
// tests/integration_test.rs
use calculator::Calculator;

#[test]
fn test_basic_workflow() {
    let mut calc = Calculator::new();
    
    // 执行计算
    let result = calc.add(10.0, 5.0);
    
    // 存储结果
    calc.store(result);
    
    // 使用存储的值
    let final_result = calc.divide(calc.recall(), 3.0);
    
    assert_eq!(final_result, Ok(5.0));
}

#[test]
fn test_error_handling() {
    let calc = Calculator::new();
    
    match calc.divide(10.0, 0.0) {
        Ok(_) => panic!("应该返回错误"),
        Err(msg) => assert_eq!(msg, "除数不能为零"),
    }
}
```

### 步骤 4：生成文档

```bash
# 生成并打开文档
cargo doc --open

# 测试文档中的示例
cargo test --doc
```

## 🧪 练习题

### 练习 1：编写测试

为以下函数编写完整的测试：

```rust
pub fn factorial(n: u32) -> u64 {
    match n {
        0 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 你的测试
}
```

### 练习 2：TDD 实践

使用 TDD 方式实现一个函数，判断字符串是否为回文：

```rust
// 步骤 1：先写测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_palindrome() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome("hello"));
    }

    // 添加更多测试...
}

// 步骤 2：实现函数
pub fn is_palindrome(s: &str) -> bool {
    // 你的实现
}
```

### 练习 3：文档注释

为以下函数添加完整的文档注释：

```rust
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().max().copied()
}
```

## 📝 常见问题

### 1. 测试失败时如何调试？

```rust
#[test]
fn test_with_debug_output() {
    let result = complex_function();
    
    // 使用 dbg! 宏
    dbg!(&result);
    
    // 使用 println!（需要 --nocapture）
    println!("Result: {:?}", result);
    
    assert_eq!(result, expected);
}

// 运行：cargo test -- --nocapture
```

### 2. 如何测试私有函数？

```rust
// 私有函数
fn internal_helper() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_helper() {
        // 测试模块可以访问私有函数
        assert_eq!(internal_helper(), 42);
    }
}
```

### 3. 如何组织大型测试？

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 测试辅助函数
    fn setup() -> TestContext {
        TestContext::new()
    }

    fn teardown(ctx: TestContext) {
        // 清理
    }

    mod feature_a {
        use super::*;

        #[test]
        fn test_case_1() {
            let ctx = setup();
            // 测试
            teardown(ctx);
        }
    }

    mod feature_b {
        use super::*;

        #[test]
        fn test_case_1() {
            // 测试
        }
    }
}
```

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 编写单元测试和集成测试
- [ ] 使用各种断言宏
- [ ] 组织和分组测试
- [ ] 测试错误和 panic
- [ ] 编写文档注释
- [ ] 理解文档测试
- [ ] 实践 TDD
- [ ] 生成和查看文档

## 🔗 延伸阅读

- [The Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [rustdoc 文档](https://doc.rust-lang.org/rustdoc/)

---

**测试是代码质量的保证，文档是知识传递的桥梁！** 🧪📚
