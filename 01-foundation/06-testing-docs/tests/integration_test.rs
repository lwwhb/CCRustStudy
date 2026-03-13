//! 集成测试
//!
//! 测试库的公共 API

use testing_docs::{calculator, validator};

#[test]
fn test_calculator_operations() {
    // 测试基本运算
    assert_eq!(calculator::add(10, 20), 30);
    assert_eq!(calculator::subtract(50, 30), 20);
    assert_eq!(calculator::multiply(6, 7), 42);
    assert_eq!(calculator::divide(100, 5), Some(20));
    assert_eq!(calculator::divide(10, 0), None);
}

#[test]
fn test_calculator_advanced() {
    // 测试高级功能
    assert_eq!(calculator::factorial(5), 120);
    assert_eq!(calculator::power(2, 10), 1024);
    assert_eq!(calculator::sqrt(16), Some(4));
}

#[test]
fn test_validator_email() {
    // 测试邮箱验证
    assert!(validator::is_valid_email("user@example.com"));
    assert!(validator::is_valid_email("test.user@domain.co.uk"));
    assert!(!validator::is_valid_email("invalid"));
    assert!(!validator::is_valid_email("@example.com"));
}

#[test]
fn test_validator_password() {
    // 测试密码验证
    assert!(validator::is_strong_password("Password123"));
    assert!(validator::is_strong_password("MySecure1Pass"));
    assert!(!validator::is_strong_password("weak"));
    assert!(!validator::is_strong_password("12345678"));
    assert!(!validator::is_strong_password("password"));
}

#[test]
fn test_validator_username() {
    // 测试用户名验证
    assert!(validator::is_valid_username("alice"));
    assert!(validator::is_valid_username("user123"));
    assert!(validator::is_valid_username("test_user"));
    assert!(!validator::is_valid_username("ab"));
    assert!(!validator::is_valid_username("123user"));
}

#[test]
fn test_user_registration_workflow() {
    // 模拟完整的用户注册流程
    let username = "alice123";
    let email = "alice@example.com";
    let password = "SecurePass123";
    let age = 25;
    let phone = "13812345678";

    // 验证所有字段
    assert!(validator::is_valid_username(username), "用户名无效");
    assert!(validator::is_valid_email(email), "邮箱无效");
    assert!(validator::is_strong_password(password), "密码强度不足");
    assert!(validator::is_valid_age(age), "年龄无效");
    assert!(validator::is_valid_phone(phone), "手机号无效");

    // 如果所有验证通过，注册成功
    println!("用户注册成功: {}", username);
}

#[test]
fn test_calculator_with_validation() {
    // 结合计算器和验证器的场景
    let age = 25;
    assert!(validator::is_valid_age(age));

    // 计算年龄相关的值
    let years_to_retirement = calculator::subtract(65, age as i64);
    assert_eq!(years_to_retirement, 40);
}

#[test]
#[should_panic(expected = "输入过大")]
fn test_factorial_overflow() {
    // 测试阶乘溢出保护
    calculator::factorial(21);
}

#[test]
fn test_edge_cases() {
    // 边界情况测试
    assert_eq!(calculator::add(i64::MAX, 0), i64::MAX);
    assert_eq!(calculator::multiply(0, 1000000), 0);
    assert_eq!(calculator::factorial(0), 1);
    assert_eq!(calculator::sqrt(0), Some(0));
}
