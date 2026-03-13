/// FFI（Foreign Function Interface）示例
///
/// 演示 Rust 与 C 语言的互操作

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// 声明 C 标准库函数
unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

/// 调用 C 的 abs 函数
pub fn call_c_abs(n: i32) -> i32 {
    unsafe { abs(n) }
}

/// 调用 C 的 strlen 函数
pub fn call_c_strlen(s: &str) -> usize {
    let c_string = CString::new(s).expect("CString::new failed");
    unsafe { strlen(c_string.as_ptr()) }
}

/// 调用 C 的 strcmp 函数
pub fn call_c_strcmp(s1: &str, s2: &str) -> i32 {
    let c_str1 = CString::new(s1).expect("CString::new failed");
    let c_str2 = CString::new(s2).expect("CString::new failed");
    unsafe { strcmp(c_str1.as_ptr(), c_str2.as_ptr()) }
}

/// 导出给 C 调用的 Rust 函数
#[unsafe(no_mangle)]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_multiply(a: c_int, b: c_int) -> c_int {
    a * b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_hello() {
    println!("Hello from Rust!");
}

/// 字符串转换辅助函数
pub fn rust_string_to_c(s: &str) -> *mut c_char {
    let c_string = CString::new(s).expect("CString::new failed");
    c_string.into_raw()
}

/// 从 C 字符串创建 Rust String
pub unsafe fn c_string_to_rust(ptr: *const c_char) -> String {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        c_str.to_string_lossy().into_owned()
    }
}

/// 释放 C 字符串
pub unsafe fn free_c_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// 安全的 FFI 包装器
pub struct SafeFFI;

impl SafeFFI {
    pub fn abs(n: i32) -> i32 {
        call_c_abs(n)
    }

    pub fn strlen(s: &str) -> usize {
        call_c_strlen(s)
    }

    pub fn strcmp(s1: &str, s2: &str) -> std::cmp::Ordering {
        let result = call_c_strcmp(s1, s2);
        match result {
            0 => std::cmp::Ordering::Equal,
            x if x < 0 => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Greater,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_c_abs() {
        assert_eq!(call_c_abs(-5), 5);
        assert_eq!(call_c_abs(10), 10);
        assert_eq!(call_c_abs(0), 0);
    }

    #[test]
    fn test_call_c_strlen() {
        assert_eq!(call_c_strlen("hello"), 5);
        assert_eq!(call_c_strlen(""), 0);
        assert_eq!(call_c_strlen("Rust"), 4);
    }

    #[test]
    fn test_call_c_strcmp() {
        assert_eq!(call_c_strcmp("abc", "abc"), 0);
        assert!(call_c_strcmp("abc", "def") < 0);
        assert!(call_c_strcmp("xyz", "abc") > 0);
    }

    #[test]
    fn test_rust_add() {
        assert_eq!(rust_add(2, 3), 5);
        assert_eq!(rust_add(-1, 1), 0);
    }

    #[test]
    fn test_rust_multiply() {
        assert_eq!(rust_multiply(4, 5), 20);
        assert_eq!(rust_multiply(0, 100), 0);
    }

    #[test]
    fn test_string_conversion() {
        let rust_str = "Hello, World!";
        let c_ptr = rust_string_to_c(rust_str);

        unsafe {
            let converted = c_string_to_rust(c_ptr);
            assert_eq!(converted, rust_str);
            free_c_string(c_ptr);
        }
    }

    #[test]
    fn test_safe_ffi() {
        assert_eq!(SafeFFI::abs(-42), 42);
        assert_eq!(SafeFFI::strlen("test"), 4);
        assert_eq!(SafeFFI::strcmp("a", "a"), std::cmp::Ordering::Equal);
        assert_eq!(SafeFFI::strcmp("a", "b"), std::cmp::Ordering::Less);
    }
}
