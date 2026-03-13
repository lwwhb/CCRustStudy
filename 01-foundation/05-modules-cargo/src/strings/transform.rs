/// 字符串转换函数

/// 将字符串转换为驼峰命名（camelCase）
///
/// # 示例
///
/// ```
/// use modules_cargo::strings::transform::to_camel_case;
/// assert_eq!(to_camel_case("hello_world"), "helloWorld");
/// assert_eq!(to_camel_case("foo_bar_baz"), "fooBarBaz");
/// ```
pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for (i, ch) in s.chars().enumerate() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(ch.to_uppercase());
            capitalize_next = false;
        } else if i == 0 {
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
    }

    result
}

/// 将字符串转换为帕斯卡命名（PascalCase）
///
/// # 示例
///
/// ```
/// use modules_cargo::strings::transform::to_pascal_case;
/// assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
/// ```
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// 将字符串转换为蛇形命名（snake_case）
///
/// # 示例
///
/// ```
/// use modules_cargo::strings::transform::to_snake_case;
/// assert_eq!(to_snake_case("HelloWorld"), "hello_world");
/// assert_eq!(to_snake_case("camelCase"), "camel_case");
/// ```
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.extend(ch.to_lowercase());
    }

    result
}

/// 截断字符串到指定长度，超出部分用省略号替代
///
/// # 示例
///
/// ```
/// use modules_cargo::strings::transform::truncate;
/// assert_eq!(truncate("Hello, World!", 8), "Hello...");
/// assert_eq!(truncate("Hi", 8), "Hi");
/// ```
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// 统计单词数量
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// 反转字符串中的单词顺序
///
/// # 示例
///
/// ```
/// use modules_cargo::strings::transform::reverse_words;
/// assert_eq!(reverse_words("hello world rust"), "rust world hello");
/// ```
pub fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

/// 检查字符串是否为回文
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("foo_bar_baz"), "fooBarBaz");
        assert_eq!(to_camel_case("hello"), "hello");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("foo_bar_baz"), "FooBarBaz");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("camelCase"), "camel_case");
        assert_eq!(to_snake_case("hello"), "hello");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello, World!", 8), "Hello...");
        assert_eq!(truncate("Hi", 8), "Hi");
        assert_eq!(truncate("Hello", 5), "Hello");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world rust"), 3);
        assert_eq!(word_count("  spaces  "), 1);
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("hello world rust"), "rust world hello");
        assert_eq!(reverse_words("one"), "one");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }
}
