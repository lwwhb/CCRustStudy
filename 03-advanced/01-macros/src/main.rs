#[macro_use]
mod declarative;

use std::collections::HashMap;

fn main() {
    println!("=== 宏编程演示 ===\n");

    // 演示 1：简单宏
    println!("=== 演示 1：简单宏 ===");
    say_hello!();
    say_hello!("Rust");
    println!();

    // 演示 2：创建函数
    println!("=== 演示 2：创建函数宏 ===");
    create_function!(greet);
    greet();
    println!();

    // 演示 3：计算表达式
    println!("=== 演示 3：计算表达式 ===");
    calculate!(eval 2 + 3);
    calculate!(eval 10 * 5);
    println!();

    // 演示 4：创建 Vec
    println!("=== 演示 4：my_vec! 宏 ===");
    let numbers = my_vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    let strings = my_vec!["hello", "world"];
    println!("strings: {:?}", strings);
    println!();

    // 演示 5：创建 HashMap
    println!("=== 演示 5：hashmap! 宏 ===");
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,
    };
    println!("scores: {:?}", scores);
    println!();

    // 演示 6：日志宏
    println!("=== 演示 6：日志宏 ===");
    log!("INFO", "Application started");
    log!("DEBUG", "Processing {} items", 42);
    log!("ERROR", "Connection failed: {}", "timeout");
    println!();

    // 演示 7：调试打印
    println!("=== 演示 7：调试打印 ===");
    let x = 42;
    let y = "hello";
    debug_print!(x);
    debug_print!(y);
    debug_print!(x + 10);
    println!();

    // 演示 8：重复执行
    println!("=== 演示 8：重复执行 ===");
    let mut counter = 0;
    repeat!(5, {
        counter += 1;
        println!("  迭代 {}", counter);
    });
    println!();

    // 演示 9：测量执行时间
    println!("=== 演示 9：测量执行时间 ===");
    let result = time_it!("计算总和", {
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    });
    println!("结果: {}\n", result);

    // 演示 10：宏的组合使用
    println!("=== 演示 10：宏的组合使用 ===");
    demonstrate_macro_composition();
}

/// 演示宏的组合使用
fn demonstrate_macro_composition() {
    // 使用多个宏创建和处理数据
    let data = my_vec![1, 2, 3, 4, 5];
    debug_print!(data);

    let map = hashmap! {
        "count" => data.len(),
        "sum" => data.iter().sum::<usize>(),
    };
    debug_print!(map);

    log!("INFO", "Data processing complete");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_vec_macro() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_hashmap_macro() {
        let map = hashmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_calculate_macro() {
        let result = calculate!(eval 5 * 5);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_repeat_macro() {
        let mut sum = 0;
        repeat!(10, {
            sum += 1;
        });
        assert_eq!(sum, 10);
    }
}

