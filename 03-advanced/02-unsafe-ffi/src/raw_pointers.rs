/// 原始指针操作
///
/// 演示 unsafe Rust 中的原始指针使用

/// 交换两个值（使用原始指针）
pub unsafe fn swap_raw(a: *mut i32, b: *mut i32) {
    unsafe {
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}

/// 读取原始指针的值
pub unsafe fn read_raw(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

/// 写入原始指针
pub unsafe fn write_raw(ptr: *mut i32, value: i32) {
    unsafe { *ptr = value; }
}

/// 原始指针数组操作
pub unsafe fn sum_array(ptr: *const i32, len: usize) -> i32 {
    let mut sum = 0;
    for i in 0..len {
        unsafe {
            sum += *ptr.add(i);
        }
    }
    sum
}

/// 安全的交换函数（封装 unsafe）
pub fn safe_swap(a: &mut i32, b: &mut i32) {
    unsafe {
        swap_raw(a as *mut i32, b as *mut i32);
    }
}

/// 创建和操作原始指针
pub fn demonstrate_raw_pointers() {
    let mut x = 5;
    let mut y = 10;

    println!("交换前: x = {}, y = {}", x, y);

    unsafe {
        swap_raw(&mut x as *mut i32, &mut y as *mut i32);
    }

    println!("交换后: x = {}, y = {}", x, y);
}

/// 原始指针与切片
pub unsafe fn slice_from_raw_parts(ptr: *const i32, len: usize) -> &'static [i32] {
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

/// 分配和释放内存
pub fn demonstrate_manual_memory() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;

        if !ptr.is_null() {
            *ptr = 42;
            println!("分配的值: {}", *ptr);

            dealloc(ptr as *mut u8, layout);
            println!("内存已释放");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_raw() {
        let mut a = 5;
        let mut b = 10;

        unsafe {
            swap_raw(&mut a as *mut i32, &mut b as *mut i32);
        }

        assert_eq!(a, 10);
        assert_eq!(b, 5);
    }

    #[test]
    fn test_read_raw() {
        let x = 42;
        let ptr = &x as *const i32;

        unsafe {
            assert_eq!(read_raw(ptr), 42);
        }
    }

    #[test]
    fn test_write_raw() {
        let mut x = 0;
        let ptr = &mut x as *mut i32;

        unsafe {
            write_raw(ptr, 100);
        }

        assert_eq!(x, 100);
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        let ptr = arr.as_ptr();

        unsafe {
            let sum = sum_array(ptr, arr.len());
            assert_eq!(sum, 15);
        }
    }

    #[test]
    fn test_safe_swap() {
        let mut a = 1;
        let mut b = 2;

        safe_swap(&mut a, &mut b);

        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_slice_from_raw_parts() {
        let arr = [10, 20, 30, 40, 50];
        let ptr = arr.as_ptr();

        unsafe {
            let slice = std::slice::from_raw_parts(ptr, arr.len());
            assert_eq!(slice, &[10, 20, 30, 40, 50]);
        }
    }
}
