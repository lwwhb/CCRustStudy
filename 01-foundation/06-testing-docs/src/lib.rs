//! # testing-docs
//!
//! 演示 Rust 测试和文档的最佳实践
//!
//! 本库包含两个主要模块：
//! - `calculator` - 数学计算函数（演示单元测试）
//! - `validator` - 数据验证函数（演示测试组织）
//!
//! # 示例
//!
//! ```
//! use testing_docs::calculator;
//! use testing_docs::validator;
//!
//! // 使用计算器
//! let sum = calculator::add(2, 3);
//! assert_eq!(sum, 5);
//!
//! // 使用验证器
//! assert!(validator::is_valid_email("user@example.com"));
//! ```
//!
//! # 测试
//!
//! 运行所有测试：
//! ```bash
//! cargo test
//! ```
//!
//! 运行特定模块的测试：
//! ```bash
//! cargo test calculator
//! ```

pub mod calculator;
pub mod validator;

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    // 集成测试：测试模块间的协作
    #[test]
    fn test_modules_integration() {
        // 计算器模块
        assert_eq!(calculator::add(2, 3), 5);
        assert_eq!(calculator::multiply(4, 5), 20);

        // 验证器模块
        assert!(validator::is_valid_email("test@example.com"));
        assert!(validator::is_strong_password("Password123"));
    }
}
