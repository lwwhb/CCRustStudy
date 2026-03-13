/// 数据验证模块
///
/// 提供各种数据验证功能

/// 验证邮箱格式
///
/// # 示例
///
/// ```
/// use testing_docs::validator::is_valid_email;
/// assert!(is_valid_email("user@example.com"));
/// assert!(!is_valid_email("invalid-email"));
/// ```
pub fn is_valid_email(email: &str) -> bool {
    if email.len() <= 5 {
        return false;
    }

    let at_count = email.matches('@').count();
    if at_count != 1 {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

/// 验证密码强度
///
/// 密码必须满足：
/// - 至少 8 个字符
/// - 包含至少一个数字
/// - 包含至少一个字母
///
/// # 示例
///
/// ```
/// use testing_docs::validator::is_strong_password;
/// assert!(is_strong_password("Password123"));
/// assert!(!is_strong_password("weak"));
/// ```
pub fn is_strong_password(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|c| c.is_numeric())
        && password.chars().any(|c| c.is_alphabetic())
}

/// 验证年龄范围
///
/// # 示例
///
/// ```
/// use testing_docs::validator::is_valid_age;
/// assert!(is_valid_age(25));
/// assert!(!is_valid_age(150));
/// ```
pub fn is_valid_age(age: u32) -> bool {
    age > 0 && age < 150
}

/// 验证用户名
///
/// 用户名规则：
/// - 3-20 个字符
/// - 只能包含字母、数字和下划线
/// - 必须以字母开头
pub fn is_valid_username(username: &str) -> bool {
    let len = username.len();
    if len < 3 || len > 20 {
        return false;
    }

    let mut chars = username.chars();
    if let Some(first) = chars.next() {
        if !first.is_alphabetic() {
            return false;
        }
    }

    username.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// 验证 URL 格式
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// 验证手机号（简化版，仅检查格式）
///
/// # 示例
///
/// ```
/// use testing_docs::validator::is_valid_phone;
/// assert!(is_valid_phone("13812345678"));
/// assert!(!is_valid_phone("123"));
/// ```
pub fn is_valid_phone(phone: &str) -> bool {
    phone.len() == 11 && phone.chars().all(|c| c.is_numeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 邮箱验证测试
    mod email_validation {
        use super::*;

        #[test]
        fn test_valid_emails() {
            assert!(is_valid_email("user@example.com"));
            assert!(is_valid_email("test.user@domain.co.uk"));
            assert!(is_valid_email("name+tag@company.org"));
        }

        #[test]
        fn test_invalid_emails() {
            assert!(!is_valid_email("invalid"));
            assert!(!is_valid_email("@example.com"));
            assert!(!is_valid_email("user@"));
            assert!(!is_valid_email("user"));
        }

        #[test]
        fn test_edge_cases() {
            assert!(!is_valid_email(""));
            assert!(!is_valid_email("a@b.c")); // 太短
        }
    }

    // 密码强度测试
    mod password_validation {
        use super::*;

        #[test]
        fn test_strong_passwords() {
            assert!(is_strong_password("Password123"));
            assert!(is_strong_password("MyP@ssw0rd"));
            assert!(is_strong_password("Secure1234"));
        }

        #[test]
        fn test_weak_passwords() {
            assert!(!is_strong_password("weak"));
            assert!(!is_strong_password("12345678")); // 无字母
            assert!(!is_strong_password("password")); // 无数字
            assert!(!is_strong_password("Pass1")); // 太短
        }
    }

    // 年龄验证测试
    #[test]
    fn test_valid_ages() {
        assert!(is_valid_age(1));
        assert!(is_valid_age(25));
        assert!(is_valid_age(100));
    }

    #[test]
    fn test_invalid_ages() {
        assert!(!is_valid_age(0));
        assert!(!is_valid_age(150));
        assert!(!is_valid_age(200));
    }

    // 用户名验证测试
    mod username_validation {
        use super::*;

        #[test]
        fn test_valid_usernames() {
            assert!(is_valid_username("alice"));
            assert!(is_valid_username("user123"));
            assert!(is_valid_username("test_user"));
            assert!(is_valid_username("User_Name_123"));
        }

        #[test]
        fn test_invalid_usernames() {
            assert!(!is_valid_username("ab")); // 太短
            assert!(!is_valid_username("123user")); // 数字开头
            assert!(!is_valid_username("user-name")); // 包含连字符
            assert!(!is_valid_username("a".repeat(21).as_str())); // 太长
        }
    }

    // URL 验证测试
    #[test]
    fn test_valid_urls() {
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("https://www.rust-lang.org"));
    }

    #[test]
    fn test_invalid_urls() {
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("example.com"));
        assert!(!is_valid_url(""));
    }

    // 手机号验证测试
    #[test]
    fn test_valid_phones() {
        assert!(is_valid_phone("13812345678"));
        assert!(is_valid_phone("18900001111"));
    }

    #[test]
    fn test_invalid_phones() {
        assert!(!is_valid_phone("123"));
        assert!(!is_valid_phone("1381234567")); // 10 位
        assert!(!is_valid_phone("138123456789")); // 12 位
        assert!(!is_valid_phone("1381234567a")); // 包含字母
    }

    // 组合验证测试
    #[test]
    fn test_user_registration_data() {
        // 模拟用户注册数据验证
        let username = "alice123";
        let email = "alice@example.com";
        let password = "SecurePass123";
        let age = 25;

        assert!(is_valid_username(username));
        assert!(is_valid_email(email));
        assert!(is_strong_password(password));
        assert!(is_valid_age(age));
    }
}
