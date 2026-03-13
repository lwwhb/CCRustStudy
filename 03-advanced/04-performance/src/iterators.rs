/// 迭代器优化
///
/// 演示迭代器的性能优化技巧

use std::borrow::Cow;

/// 避免中间集合 - 慢版本
pub fn process_slow(data: &[i32]) -> Vec<i32> {
    let filtered: Vec<_> = data.iter().filter(|&&x| x > 0).collect();
    let mapped: Vec<_> = filtered.iter().map(|&&x| x * 2).collect();
    mapped.iter().filter(|&&x| x < 100).copied().collect()
}

/// 避免中间集合 - 快版本
pub fn process_fast(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .filter(|&x| x < 100)
        .collect()
}

/// 使用 Cow 避免不必要的克隆
pub fn process_string_slow(s: &str) -> String {
    if s.contains("bad") {
        s.replace("bad", "good")
    } else {
        s.to_string()
    }
}

pub fn process_string_fast(s: &str) -> Cow<'_, str> {
    if s.contains("bad") {
        Cow::Owned(s.replace("bad", "good"))
    } else {
        Cow::Borrowed(s)
    }
}

/// 避免不必要的分配 - 慢版本
pub fn concat_slow(strings: &[String]) -> String {
    let mut result = String::new();
    for s in strings {
        result = result + s;  // 每次都创建新的 String
    }
    result
}

/// 避免不必要的分配 - 快版本
pub fn concat_fast(strings: &[String]) -> String {
    let mut result = String::new();
    for s in strings {
        result.push_str(s);  // 直接追加，不创建新 String
    }
    result
}

/// 预分配容量
pub fn build_vec_slow() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
    }
    v
}

pub fn build_vec_fast() -> Vec<i32> {
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000 {
        v.push(i);
    }
    v
}

/// 使用 collect 预分配
pub fn build_vec_fastest() -> Vec<i32> {
    (0..1000).collect()
}

/// 避免克隆 - 慢版本
pub fn transform_slow(data: Vec<i32>) -> Vec<i32> {
    data.clone().into_iter().map(|x| x * 2).collect()
}

/// 避免克隆 - 快版本
pub fn transform_fast(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().map(|x| x * 2).collect()
}

/// 使用引用避免移动
pub fn sum_slow(data: Vec<i32>) -> i32 {
    let sum = data.iter().sum();
    // data 已经被移动，无法再使用
    sum
}

pub fn sum_fast(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// 链式迭代器
pub fn chain_slow(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = a.to_vec();
    result.extend_from_slice(b);
    result
}

pub fn chain_fast(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().chain(b.iter()).copied().collect()
}

/// 扁平化嵌套结构
pub fn flatten_slow(data: &[Vec<i32>]) -> Vec<i32> {
    let mut result = Vec::new();
    for v in data {
        for &item in v {
            result.push(item);
        }
    }
    result
}

pub fn flatten_fast(data: &[Vec<i32>]) -> Vec<i32> {
    data.iter().flatten().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let data = vec![-5, -1, 0, 10, 20, 50, 100, 200];
        let expected = vec![20, 40];

        assert_eq!(process_slow(&data), expected);
        assert_eq!(process_fast(&data), expected);
    }

    #[test]
    fn test_process_string() {
        let s1 = "This is good";
        let s2 = "This is bad";

        assert_eq!(process_string_slow(s1), "This is good");
        assert_eq!(process_string_slow(s2), "This is good");

        assert_eq!(process_string_fast(s1), "This is good");
        assert_eq!(process_string_fast(s2), "This is good");
    }

    #[test]
    fn test_concat() {
        let strings = vec![
            "Hello".to_string(),
            " ".to_string(),
            "World".to_string(),
        ];

        assert_eq!(concat_slow(&strings), "Hello World");
        assert_eq!(concat_fast(&strings), "Hello World");
    }

    #[test]
    fn test_build_vec() {
        assert_eq!(build_vec_slow().len(), 1000);
        assert_eq!(build_vec_fast().len(), 1000);
        assert_eq!(build_vec_fastest().len(), 1000);
    }

    #[test]
    fn test_transform() {
        let data = vec![1, 2, 3, 4, 5];
        let expected = vec![2, 4, 6, 8, 10];

        assert_eq!(transform_slow(data.clone()), expected);
        assert_eq!(transform_fast(data), expected);
    }

    #[test]
    fn test_sum() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_fast(&data), 15);
    }

    #[test]
    fn test_chain() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let expected = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(chain_slow(&a, &b), expected);
        assert_eq!(chain_fast(&a, &b), expected);
    }

    #[test]
    fn test_flatten() {
        let data = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        let expected = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(flatten_slow(&data), expected);
        assert_eq!(flatten_fast(&data), expected);
    }

    #[test]
    fn test_cow_no_allocation() {
        let s = "this is good";
        let result = process_string_fast(s);

        // 检查是否是借用
        if let Cow::Borrowed(borrowed) = result {
            assert_eq!(borrowed, s);
        } else {
            panic!("Expected Cow::Borrowed, got Cow::Owned");
        }
    }

    #[test]
    fn test_cow_with_allocation() {
        let s = "this is bad";
        let result = process_string_fast(s);

        // 检查是否是拥有
        if let Cow::Owned(owned) = result {
            assert_eq!(owned, "this is good");
        } else {
            panic!("Expected Cow::Owned, got Cow::Borrowed");
        }
    }
}
