//! # modules-cargo
//!
//! 一个演示 Rust 模块系统的库
//!
//! 本库提供三个主要模块：
//! - `math` - 数学运算和统计函数
//! - `strings` - 字符串转换工具
//! - `collections` - 自定义集合类型
//!
//! # 示例
//!
//! ```
//! use modules_cargo::math;
//! use modules_cargo::strings;
//! use modules_cargo::collections::Stack;
//!
//! // 使用数学函数
//! let sum = math::add(2, 3);
//! assert_eq!(sum, 5);
//!
//! // 使用字符串转换
//! let camel = strings::to_camel_case("hello_world");
//! assert_eq!(camel, "helloWorld");
//!
//! // 使用栈
//! let mut stack = Stack::new();
//! stack.push(42);
//! assert_eq!(stack.pop(), Some(42));
//! ```

// 公开模块
pub mod collections;
pub mod math;
pub mod strings;

// 重导出常用类型和函数，提供更简洁的 API
pub use collections::Stack;

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库的名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_name() {
        assert_eq!(NAME, "modules-cargo");
    }

    #[test]
    fn test_math_module() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!(math::multiply(4, 5), 20);
    }

    #[test]
    fn test_strings_module() {
        assert_eq!(strings::to_camel_case("hello_world"), "helloWorld");
    }

    #[test]
    fn test_collections_module() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
    }
}
