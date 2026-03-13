mod raw_pointers;
mod ffi;

use raw_pointers::*;
use ffi::*;

fn main() {
    println!("=== Unsafe Rust 与 FFI 演示 ===\n");

    // 演示 1：原始指针操作
    println!("=== 演示 1：原始指针操作 ===");
    demonstrate_raw_pointers();
    println!();

    // 演示 2：手动内存管理
    println!("=== 演示 2：手动内存管理 ===");
    demonstrate_manual_memory();
    println!();

    // 演示 3：调用 C 标准库函数
    println!("=== 演示 3：调用 C 标准库函数 ===");
    demonstrate_c_functions();
    println!();

    // 演示 4：字符串转换
    println!("=== 演示 4：字符串转换 ===");
    demonstrate_string_conversion();
    println!();

    // 演示 5：安全抽象
    println!("=== 演示 5：安全抽象 ===");
    demonstrate_safe_abstractions();
    println!();

    // 演示 6：数组操作
    println!("=== 演示 6：原始指针数组操作 ===");
    demonstrate_array_operations();
    println!();

    // 演示 7：导出给 C 的函数
    println!("=== 演示 7：导出给 C 的 Rust 函数 ===");
    demonstrate_exported_functions();
    println!();
}

/// 演示 C 函数调用
fn demonstrate_c_functions() {
    // 调用 C 的 abs 函数
    let result = call_c_abs(-42);
    println!("abs(-42) = {}", result);

    let result = call_c_abs(15);
    println!("abs(15) = {}", result);

    // 调用 C 的 strlen 函数
    let text = "Hello, Rust!";
    let len = call_c_strlen(text);
    println!("strlen(\"{}\") = {}", text, len);

    // 调用 C 的 strcmp 函数
    let s1 = "apple";
    let s2 = "banana";
    let cmp = call_c_strcmp(s1, s2);
    println!("strcmp(\"{}\", \"{}\") = {} ({})",
        s1, s2, cmp,
        if cmp < 0 { "s1 < s2" } else if cmp > 0 { "s1 > s2" } else { "s1 == s2" }
    );

    let s3 = "test";
    let s4 = "test";
    let cmp = call_c_strcmp(s3, s4);
    println!("strcmp(\"{}\", \"{}\") = {} ({})",
        s3, s4, cmp,
        if cmp < 0 { "s3 < s4" } else if cmp > 0 { "s3 > s4" } else { "s3 == s4" }
    );
}

/// 演示字符串转换
fn demonstrate_string_conversion() {
    let rust_str = "Hello from Rust!";
    println!("原始 Rust 字符串: {}", rust_str);

    // 转换为 C 字符串
    let c_ptr = rust_string_to_c(rust_str);
    println!("已转换为 C 字符串指针: {:?}", c_ptr);

    unsafe {
        // 从 C 字符串转回 Rust String
        let converted = c_string_to_rust(c_ptr);
        println!("转换回 Rust String: {}", converted);

        // 验证内容相同
        assert_eq!(converted, rust_str);
        println!("✓ 字符串转换成功！");

        // 释放 C 字符串
        free_c_string(c_ptr);
        println!("✓ C 字符串已释放");
    }
}

/// 演示安全抽象
fn demonstrate_safe_abstractions() {
    // 使用安全的 FFI 包装器
    println!("使用 SafeFFI 包装器：");

    let result = SafeFFI::abs(-100);
    println!("SafeFFI::abs(-100) = {}", result);

    let len = SafeFFI::strlen("Rust is awesome!");
    println!("SafeFFI::strlen(\"Rust is awesome!\") = {}", len);

    let ordering = SafeFFI::strcmp("alpha", "beta");
    println!("SafeFFI::strcmp(\"alpha\", \"beta\") = {:?}", ordering);

    let ordering = SafeFFI::strcmp("same", "same");
    println!("SafeFFI::strcmp(\"same\", \"same\") = {:?}", ordering);
}

/// 演示数组操作
fn demonstrate_array_operations() {
    let numbers = [10, 20, 30, 40, 50];
    println!("数组: {:?}", numbers);

    unsafe {
        let sum = sum_array(numbers.as_ptr(), numbers.len());
        println!("数组总和: {}", sum);
        println!("预期总和: {}", numbers.iter().sum::<i32>());
    }

    // 使用安全的交换函数
    let mut a = 100;
    let mut b = 200;
    println!("\n交换前: a = {}, b = {}", a, b);
    safe_swap(&mut a, &mut b);
    println!("交换后: a = {}, b = {}", a, b);
}

/// 演示导出的函数
fn demonstrate_exported_functions() {
    println!("这些函数可以从 C 代码调用：");
    println!("- rust_add(2, 3) = {}", rust_add(2, 3));
    println!("- rust_multiply(4, 5) = {}", rust_multiply(4, 5));
    print!("- rust_hello() = ");
    rust_hello();

    println!("\n在 C 代码中可以这样调用：");
    println!("  extern int rust_add(int a, int b);");
    println!("  extern int rust_multiply(int a, int b);");
    println!("  extern void rust_hello(void);");
    println!("  ");
    println!("  int result = rust_add(10, 20);");
    println!("  printf(\"Result: %d\\n\", result);");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_raw_pointers() {
        let mut x = 10;
        let mut y = 20;

        unsafe {
            swap_raw(&mut x as *mut i32, &mut y as *mut i32);
        }

        assert_eq!(x, 20);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_integration_ffi() {
        assert_eq!(call_c_abs(-50), 50);
        assert_eq!(call_c_strlen("test"), 4);
        assert_eq!(call_c_strcmp("a", "a"), 0);
    }

    #[test]
    fn test_integration_safe_abstractions() {
        assert_eq!(SafeFFI::abs(-99), 99);
        assert_eq!(SafeFFI::strlen("hello"), 5);
        assert_eq!(SafeFFI::strcmp("x", "x"), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_integration_string_conversion() {
        let original = "Integration test string";
        let c_ptr = rust_string_to_c(original);

        unsafe {
            let converted = c_string_to_rust(c_ptr);
            assert_eq!(converted, original);
            free_c_string(c_ptr);
        }
    }

    #[test]
    fn test_integration_array_sum() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        unsafe {
            let sum = sum_array(arr.as_ptr(), arr.len());
            assert_eq!(sum, 55);
        }
    }

    #[test]
    fn test_exported_functions() {
        assert_eq!(rust_add(100, 200), 300);
        assert_eq!(rust_multiply(7, 8), 56);
        // rust_hello() 只是打印，无需断言
        rust_hello();
    }
}
