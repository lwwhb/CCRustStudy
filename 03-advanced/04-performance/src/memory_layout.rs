/// 内存布局优化
///
/// 演示结构体内存布局、对齐和优化技巧

use std::mem;

/// 未优化的结构体（有内存填充）
#[derive(Debug)]
struct Unoptimized {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes (需要 8 字节对齐)
    c: u16,  // 2 bytes
    d: u32,  // 4 bytes
}

/// 优化后的结构体（减少内存填充）
#[derive(Debug)]
struct Optimized {
    b: u64,  // 8 bytes
    d: u32,  // 4 bytes
    c: u16,  // 2 bytes
    a: u8,   // 1 byte
}

/// 使用 #[repr(C)] 固定布局
#[repr(C)]
#[derive(Debug)]
struct CLayout {
    a: u8,
    b: u16,
    c: u32,
}

/// 使用 #[repr(packed)] 紧凑布局（无填充）
#[repr(packed)]
#[derive(Debug)]
struct Packed {
    a: u8,
    b: u64,
    c: u16,
}

/// 演示内存布局
pub fn demonstrate_memory_layout() {
    println!("=== 内存布局对比 ===\n");

    println!("未优化结构体:");
    println!("  大小: {} bytes", mem::size_of::<Unoptimized>());
    println!("  对齐: {} bytes", mem::align_of::<Unoptimized>());

    println!("\n优化后结构体:");
    println!("  大小: {} bytes", mem::size_of::<Optimized>());
    println!("  对齐: {} bytes", mem::align_of::<Optimized>());

    println!("\nC 布局结构体:");
    println!("  大小: {} bytes", mem::size_of::<CLayout>());
    println!("  对齐: {} bytes", mem::align_of::<CLayout>());

    println!("\n紧凑布局结构体:");
    println!("  大小: {} bytes", mem::size_of::<Packed>());
    println!("  对齐: {} bytes", mem::align_of::<Packed>());
}

/// 缓存行大小（通常是 64 字节）
const CACHE_LINE_SIZE: usize = 64;

/// 缓存行对齐的结构体
#[repr(align(64))]
#[derive(Debug)]
struct CacheAligned {
    data: [u8; 64],
}

/// 演示缓存行对齐
pub fn demonstrate_cache_alignment() {
    println!("\n=== 缓存行对齐 ===\n");

    println!("缓存行大小: {} bytes", CACHE_LINE_SIZE);
    println!("CacheAligned 大小: {} bytes", mem::size_of::<CacheAligned>());
    println!("CacheAligned 对齐: {} bytes", mem::align_of::<CacheAligned>());
}

/// 零大小类型（ZST）
#[derive(Debug)]
struct ZeroSized;

/// 演示零大小类型
pub fn demonstrate_zero_sized_types() {
    println!("\n=== 零大小类型 ===\n");

    println!("ZeroSized 大小: {} bytes", mem::size_of::<ZeroSized>());
    println!("() 大小: {} bytes", mem::size_of::<()>());
    println!("PhantomData<i32> 大小: {} bytes", mem::size_of::<std::marker::PhantomData<i32>>());
}

/// 枚举的内存布局
#[derive(Debug)]
enum MyEnum {
    A(u8),
    B(u32),
    C(u64),
}

/// 演示枚举布局
pub fn demonstrate_enum_layout() {
    println!("\n=== 枚举内存布局 ===\n");

    println!("MyEnum 大小: {} bytes", mem::size_of::<MyEnum>());
    println!("MyEnum 对齐: {} bytes", mem::align_of::<MyEnum>());

    println!("\nOption<u8> 大小: {} bytes", mem::size_of::<Option<u8>>());
    println!("Option<u32> 大小: {} bytes", mem::size_of::<Option<u32>>());
}

/// Box 的内存布局
pub fn demonstrate_box_layout() {
    println!("\n=== Box 内存布局 ===\n");

    println!("Box<u8> 大小: {} bytes", mem::size_of::<Box<u8>>());
    println!("Box<u64> 大小: {} bytes", mem::size_of::<Box<u64>>());
    println!("Box<[u8; 1000]> 大小: {} bytes", mem::size_of::<Box<[u8; 1000]>>());
}

/// 切片和字符串的内存布局
pub fn demonstrate_slice_layout() {
    println!("\n=== 切片和字符串布局 ===\n");

    println!("&[u8] 大小: {} bytes", mem::size_of::<&[u8]>());
    println!("&str 大小: {} bytes", mem::size_of::<&str>());
    println!("String 大小: {} bytes", mem::size_of::<String>());
    println!("Vec<u8> 大小: {} bytes", mem::size_of::<Vec<u8>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_layout() {
        demonstrate_memory_layout();
    }

    #[test]
    fn test_cache_alignment() {
        demonstrate_cache_alignment();
    }

    #[test]
    fn test_zero_sized_types() {
        demonstrate_zero_sized_types();
    }

    #[test]
    fn test_enum_layout() {
        demonstrate_enum_layout();
    }

    #[test]
    fn test_box_layout() {
        demonstrate_box_layout();
    }

    #[test]
    fn test_slice_layout() {
        demonstrate_slice_layout();
    }

    #[test]
    fn test_optimized_smaller() {
        assert!(mem::size_of::<Optimized>() <= mem::size_of::<Unoptimized>());
    }

    #[test]
    fn test_packed_smallest() {
        // Packed 应该是最小的（11 bytes: 1 + 8 + 2）
        assert_eq!(mem::size_of::<Packed>(), 11);
    }
}
