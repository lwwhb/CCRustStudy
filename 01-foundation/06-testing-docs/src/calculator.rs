/// 计算器模块
///
/// 提供基本的数学运算功能，演示完整的测试覆盖

/// 加法运算
///
/// # 示例
///
/// ```
/// use testing_docs::calculator::add;
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 减法运算
///
/// # 示例
///
/// ```
/// use testing_docs::calculator::subtract;
/// assert_eq!(subtract(10, 3), 7);
/// ```
pub fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

/// 乘法运算
pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// 除法运算
///
/// # 错误
///
/// 当除数为零时返回 `None`
///
/// # 示例
///
/// ```
/// use testing_docs::calculator::divide;
/// assert_eq!(divide(10, 2), Some(5));
/// assert_eq!(divide(10, 0), None);
/// ```
pub fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// 计算阶乘
///
/// # 示例
///
/// ```
/// use testing_docs::calculator::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # Panics
///
/// 当输入大于 20 时会 panic（防止溢出）
pub fn factorial(n: u32) -> u64 {
    assert!(n <= 20, "输入过大，可能导致溢出");
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

/// 计算幂次方
pub fn power(base: i64, exp: u32) -> i64 {
    base.pow(exp)
}

/// 计算平方根（整数部分）
///
/// # 示例
///
/// ```
/// use testing_docs::calculator::sqrt;
/// assert_eq!(sqrt(16), Some(4));
/// assert_eq!(sqrt(20), Some(4));
/// assert_eq!(sqrt(0), Some(0));
/// ```
pub fn sqrt(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    Some((n as f64).sqrt() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 基本运算测试
    mod basic_operations {
        use super::*;

        #[test]
        fn test_add_positive() {
            assert_eq!(add(2, 3), 5);
            assert_eq!(add(100, 200), 300);
        }

        #[test]
        fn test_add_negative() {
            assert_eq!(add(-2, -3), -5);
            assert_eq!(add(-10, 5), -5);
        }

        #[test]
        fn test_add_zero() {
            assert_eq!(add(0, 0), 0);
            assert_eq!(add(5, 0), 5);
        }

        #[test]
        fn test_subtract() {
            assert_eq!(subtract(10, 3), 7);
            assert_eq!(subtract(0, 5), -5);
        }

        #[test]
        fn test_multiply() {
            assert_eq!(multiply(4, 5), 20);
            assert_eq!(multiply(-2, 3), -6);
            assert_eq!(multiply(0, 100), 0);
        }
    }

    // 除法测试
    mod division {
        use super::*;

        #[test]
        fn test_divide_normal() {
            assert_eq!(divide(10, 2), Some(5));
            assert_eq!(divide(15, 3), Some(5));
        }

        #[test]
        fn test_divide_by_zero() {
            assert_eq!(divide(10, 0), None);
            assert_eq!(divide(0, 0), None);
        }

        #[test]
        fn test_divide_negative() {
            assert_eq!(divide(-10, 2), Some(-5));
            assert_eq!(divide(10, -2), Some(-5));
        }
    }

    // 阶乘测试
    mod factorial_tests {
        use super::*;

        #[test]
        fn test_factorial_base_cases() {
            assert_eq!(factorial(0), 1);
            assert_eq!(factorial(1), 1);
        }

        #[test]
        fn test_factorial_small_numbers() {
            assert_eq!(factorial(5), 120);
            assert_eq!(factorial(10), 3628800);
        }

        #[test]
        #[should_panic(expected = "输入过大")]
        fn test_factorial_overflow() {
            factorial(21); // 应该 panic
        }
    }

    // 幂次方测试
    #[test]
    fn test_power() {
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(5, 3), 125);
        assert_eq!(power(10, 0), 1);
    }

    // 平方根测试
    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(0), Some(0));
        assert_eq!(sqrt(1), Some(1));
        assert_eq!(sqrt(16), Some(4));
        assert_eq!(sqrt(100), Some(10));
        assert_eq!(sqrt(20), Some(4)); // 整数部分
    }

    // 属性测试：交换律
    #[test]
    fn test_add_commutative() {
        assert_eq!(add(3, 5), add(5, 3));
        assert_eq!(multiply(4, 7), multiply(7, 4));
    }

    // 属性测试：结合律
    #[test]
    fn test_add_associative() {
        let a = 2;
        let b = 3;
        let c = 4;
        assert_eq!(add(add(a, b), c), add(a, add(b, c)));
    }
}
