# 模块 1.6：测试与文档

## 🎯 学习目标

- 编写单元测试和集成测试
- 使用测试组织和测试属性
- 编写文档注释和文档测试
- 使用基准测试（benchmarks）
- 理解测试驱动开发（TDD）

## 📚 核心概念

### 1. 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This should panic");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 耗时测试，默认跳过
    }
}
```

### 2. 集成测试

```rust
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    assert_eq!(my_crate::add(2, 3), 5);
}
```

### 3. 文档注释

```rust
/// 计算两数之和
///
/// # 示例
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
///
/// # 参数
///
/// * `a` - 第一个数
/// * `b` - 第二个数
///
/// # 返回
///
/// 返回两数之和
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 4. 测试组织

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
}
```

### 5. 断言宏

```rust
assert!(condition);
assert_eq!(left, right);
assert_ne!(left, right);
debug_assert!(condition);  // 仅在 debug 模式
```

## 💻 实战项目：完整测试套件

为之前的模块添加完整的测试覆盖。

### 项目结构

```
testing-docs/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── calculator.rs
│   └── validator.rs
├── tests/
│   └── integration_test.rs
└── README.md
```

## 🧪 练习题

### 练习 1：编写测试

```rust
// 为这个函数编写完整的测试
pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod tests {
    // 你的测试代码
}
```

### 练习 2：文档测试

```rust
/// 反转字符串
///
/// # 示例
///
/// ```
/// // 添加文档测试
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 11: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust API Guidelines - Documentation](https://rust-lang.github.io/api-guidelines/documentation.html)

## ✅ 检查清单

- [ ] 编写单元测试
- [ ] 编写集成测试
- [ ] 使用测试属性（#[should_panic]、#[ignore]）
- [ ] 编写文档注释
- [ ] 编写文档测试
- [ ] 组织测试模块
- [ ] 理解 TDD 流程

## 🚀 下一步

完成基础篇后，继续学习 [模块 2.1：泛型与生命周期](../../02-intermediate/01-generics-lifetimes/)。
