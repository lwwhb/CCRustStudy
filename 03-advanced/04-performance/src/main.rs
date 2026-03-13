mod memory_layout;
mod algorithms;
mod iterators;

use memory_layout::*;
use algorithms::*;
use iterators::*;

fn main() {
    println!("=== 性能优化演示 ===\n");

    // 演示 1：内存布局
    println!("=== 演示 1：内存布局优化 ===");
    demonstrate_memory_layout();
    demonstrate_cache_alignment();
    demonstrate_zero_sized_types();
    demonstrate_enum_layout();
    demonstrate_box_layout();
    demonstrate_slice_layout();
    println!();

    // 演示 2：算法优化
    println!("=== 演示 2：算法优化 ===");
    demonstrate_fibonacci();
    demonstrate_search();
    demonstrate_string_operations();
    println!();

    // 演示 3：迭代器优化
    println!("=== 演示 3：迭代器优化 ===");
    demonstrate_iterator_optimization();
    demonstrate_cow();
    demonstrate_allocation();
    println!();

    // 演示 4：性能对比
    println!("=== 演示 4：性能对比 ===");
    benchmark_fibonacci();
    benchmark_string_concat();
    benchmark_vec_building();
    println!();
}

/// 演示斐波那契数列
fn demonstrate_fibonacci() {
    println!("\n斐波那契数列（n=20）：");

    let n = 20;

    let result = fibonacci_iterative(n);
    println!("  迭代版本结果: {}", result);

    let mut cache = vec![None; (n + 1) as usize];
    let result = fibonacci_memoized(n, &mut cache);
    println!("  缓存版本结果: {}", result);
}

/// 演示搜索算法
fn demonstrate_search() {
    println!("\n搜索算法：");

    let arr = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let target = 13;

    if let Some(idx) = linear_search(&arr, target) {
        println!("  线性搜索找到 {} 在索引 {}", target, idx);
    }

    if let Some(idx) = binary_search(&arr, target) {
        println!("  二分搜索找到 {} 在索引 {}", target, idx);
    }
}

/// 演示字符串操作
fn demonstrate_string_operations() {
    println!("\n字符串操作：");

    let s = "hello world";
    let reversed = reverse_string_direct(s);
    println!("  反转 '{}': '{}'", s, reversed);
}

/// 演示迭代器优化
fn demonstrate_iterator_optimization() {
    println!("\n迭代器优化：");

    let data = vec![-5, 10, 20, 50, 100, 200];
    let result = process_fast(&data);
    println!("  处理结果: {:?}", result);
}

/// 演示 Cow
fn demonstrate_cow() {
    println!("\nCow（写时复制）：");

    let s1 = "This is good";
    let result1 = process_string_fast(s1);
    println!("  '{}' -> '{}' (借用)", s1, result1);

    let s2 = "This is bad";
    let result2 = process_string_fast(s2);
    println!("  '{}' -> '{}' (拥有)", s2, result2);
}

/// 演示内存分配
fn demonstrate_allocation() {
    println!("\n避免不必要的分配：");

    let strings = vec!["Hello".to_string(), " ".to_string(), "World".to_string()];
    let result = concat_fast(&strings);
    println!("  连接字符串: '{}'", result);
}

/// 基准测试：斐波那契
fn benchmark_fibonacci() {
    use std::time::Instant;

    println!("\n斐波那契性能对比（n=30）：");

    let n = 30;

    let start = Instant::now();
    let _ = fibonacci_iterative(n);
    let iterative_time = start.elapsed();
    println!("  迭代版本: {:?}", iterative_time);

    let start = Instant::now();
    let mut cache = vec![None; (n + 1) as usize];
    let _ = fibonacci_memoized(n, &mut cache);
    let memoized_time = start.elapsed();
    println!("  缓存版本: {:?}", memoized_time);
}

/// 基准测试：字符串连接
fn benchmark_string_concat() {
    use std::time::Instant;

    println!("\n字符串连接性能对比（1000 个字符串）：");

    let strings: Vec<String> = (0..1000).map(|i| i.to_string()).collect();

    let start = Instant::now();
    let _ = concat_fast(&strings);
    let fast_time = start.elapsed();
    println!("  优化版本: {:?}", fast_time);
}

/// 基准测试：Vec 构建
fn benchmark_vec_building() {
    use std::time::Instant;

    println!("\nVec 构建性能对比（10000 个元素）：");

    let start = Instant::now();
    let _ = (0..10000).collect::<Vec<i32>>();
    let collect_time = start.elapsed();
    println!("  collect 版本: {:?}", collect_time);

    let start = Instant::now();
    let mut v = Vec::with_capacity(10000);
    for i in 0..10000 {
        v.push(i);
    }
    let capacity_time = start.elapsed();
    println!("  预分配版本: {:?}", capacity_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_demonstrations() {
        demonstrate_fibonacci();
        demonstrate_search();
        demonstrate_string_operations();
        demonstrate_iterator_optimization();
        demonstrate_cow();
        demonstrate_allocation();
    }

    #[test]
    fn test_benchmarks() {
        benchmark_fibonacci();
        benchmark_string_concat();
        benchmark_vec_building();
    }
}
