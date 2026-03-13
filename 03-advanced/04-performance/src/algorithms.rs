/// 算法优化
///
/// 演示常见算法的性能优化技巧

/// 斐波那契数列 - 递归版本（慢）
pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// 斐波那契数列 - 迭代版本（快）
pub fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

/// 斐波那契数列 - 带缓存的版本
pub fn fibonacci_memoized(n: u64, cache: &mut Vec<Option<u64>>) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    if let Some(result) = cache.get(n as usize).and_then(|&x| x) {
        return result;
    }

    let result = fibonacci_memoized(n - 1, cache) + fibonacci_memoized(n - 2, cache);

    if cache.len() <= n as usize {
        cache.resize(n as usize + 1, None);
    }
    cache[n as usize] = Some(result);

    result
}

/// 查找元素 - 线性搜索（慢）
pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

/// 查找元素 - 二分搜索（快，需要排序数组）
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }

    None
}

/// 字符串反转 - 使用 Vec（慢）
pub fn reverse_string_vec(s: &str) -> String {
    s.chars().rev().collect::<Vec<_>>().into_iter().collect()
}

/// 字符串反转 - 直接收集（快）
pub fn reverse_string_direct(s: &str) -> String {
    s.chars().rev().collect()
}

/// 求和 - 使用循环
pub fn sum_loop(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in arr {
        sum += item;
    }
    sum
}

/// 求和 - 使用迭代器
pub fn sum_iterator(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

/// 过滤和映射 - 多次遍历（慢）
pub fn filter_map_slow(arr: &[i32]) -> Vec<i32> {
    let filtered: Vec<_> = arr.iter().filter(|&&x| x > 0).collect();
    filtered.iter().map(|&&x| x * 2).collect()
}

/// 过滤和映射 - 单次遍历（快）
pub fn filter_map_fast(arr: &[i32]) -> Vec<i32> {
    arr.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}

/// 去重 - 使用 Vec（慢）
pub fn dedup_vec(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in arr {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}

/// 去重 - 使用 HashSet（快）
pub fn dedup_hashset(arr: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let set: HashSet<_> = arr.iter().copied().collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_recursive(10), 55);
        assert_eq!(fibonacci_iterative(10), 55);

        let mut cache = vec![None; 11];
        assert_eq!(fibonacci_memoized(10, &mut cache), 55);
    }

    #[test]
    fn test_search() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];

        assert_eq!(linear_search(&arr, 7), Some(3));
        assert_eq!(binary_search(&arr, 7), Some(3));

        assert_eq!(linear_search(&arr, 8), None);
        assert_eq!(binary_search(&arr, 8), None);
    }

    #[test]
    fn test_reverse_string() {
        let s = "hello";
        assert_eq!(reverse_string_vec(s), "olleh");
        assert_eq!(reverse_string_direct(s), "olleh");
    }

    #[test]
    fn test_sum() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_loop(&arr), 15);
        assert_eq!(sum_iterator(&arr), 15);
    }

    #[test]
    fn test_filter_map() {
        let arr = vec![-2, -1, 0, 1, 2, 3];
        let expected = vec![2, 4, 6];

        assert_eq!(filter_map_slow(&arr), expected);
        assert_eq!(filter_map_fast(&arr), expected);
    }

    #[test]
    fn test_dedup() {
        let arr = vec![1, 2, 2, 3, 1, 4, 3];

        let result1 = dedup_vec(&arr);
        let result2 = dedup_hashset(&arr);

        // HashSet 不保证顺序，所以只检查长度
        assert_eq!(result1.len(), 4);
        assert_eq!(result2.len(), 4);
    }

    #[test]
    fn test_fibonacci_performance() {
        // 迭代版本应该比递归版本快得多
        let n = 20;

        let start = std::time::Instant::now();
        let _ = fibonacci_recursive(n);
        let recursive_time = start.elapsed();

        let start = std::time::Instant::now();
        let _ = fibonacci_iterative(n);
        let iterative_time = start.elapsed();

        println!("递归: {:?}, 迭代: {:?}", recursive_time, iterative_time);
    }
}
