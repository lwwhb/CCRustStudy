/// 基本数学运算
///
/// 提供常用的数学计算函数，演示模块组织和文档注释

/// 计算两数之和
///
/// # 示例
///
/// ```
/// use modules_cargo::math::basic::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 计算两数之差
///
/// # 示例
///
/// ```
/// use modules_cargo::math::basic::subtract;
/// assert_eq!(subtract(10, 3), 7);
/// ```
pub fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

/// 计算两数之积
pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// 计算两数之商
///
/// # 错误
///
/// 当除数为零时返回 `None`
///
/// # 示例
///
/// ```
/// use modules_cargo::math::basic::divide;
/// assert_eq!(divide(10, 2), Some(5));
/// assert_eq!(divide(10, 0), None);
/// ```
pub fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 { None } else { Some(a / b) }
}

/// 计算幂次方
///
/// # 示例
///
/// ```
/// use modules_cargo::math::basic::power;
/// assert_eq!(power(2, 10), 1024);
/// ```
pub fn power(base: i64, exp: u32) -> i64 {
    base.pow(exp)
}

/// 计算最大公约数（欧几里得算法）
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// 计算最小公倍数
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// 判断是否为质数
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt = (n as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() { assert_eq!(add(2, 3), 5); }

    #[test]
    fn test_subtract() { assert_eq!(subtract(10, 3), 7); }

    #[test]
    fn test_multiply() { assert_eq!(multiply(4, 5), 20); }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_power() { assert_eq!(power(2, 10), 1024); }

    #[test]
    fn test_gcd() { assert_eq!(gcd(48, 18), 6); }

    #[test]
    fn test_lcm() { assert_eq!(lcm(4, 6), 12); }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(1));
        assert!(!is_prime(15));
    }
}
