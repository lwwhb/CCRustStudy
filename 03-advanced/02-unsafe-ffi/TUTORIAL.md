# 模块 3.2：Unsafe Rust 与 FFI - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 unsafe 的必要性和使用场景
2. 掌握原始指针操作
3. 学习 FFI（外部函数接口）
4. 实现 C 库绑定
5. 构建安全的抽象层

## 🎯 为什么需要 Unsafe？

### Safe Rust vs Unsafe Rust

**Safe Rust（默认）**：
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
// v.push(4);  // 编译错误！不能同时有可变和不可变引用

优势：
- 编译器保证内存安全
- 无数据竞争
- 无悬垂指针
- 无缓冲区溢出

限制：
- 某些底层操作无法实现
- 与 C 库交互困难
- 性能优化受限
```

**Unsafe Rust**：
```rust
unsafe {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    *ptr = 10;  // 直接修改内存
}

能力：
- 解引用原始指针
- 调用 unsafe 函数
- 访问可变静态变量
- 实现 unsafe trait
- 访问 union 字段

责任：
- 程序员保证安全性
- 编译器不再检查
```

### Unsafe 的使用场景

```
1. 底层系统编程
   - 操作系统内核
   - 设备驱动
   - 内存分配器

2. 性能优化
   - 跳过边界检查
   - SIMD 指令
   - 内存布局控制

3. FFI（外部函数接口）
   - 调用 C 库
   - 与其他语言交互
   - 系统调用

4. 实现安全抽象
   - Vec、String 等标准库类型
   - 智能指针
   - 并发原语
```

## 📖 核心概念详解

### 1. 原始指针

Rust 有两种原始指针类型。

```rust
// 不可变原始指针
let x = 5;
let raw_ptr: *const i32 = &x;

// 可变原始指针
let mut y = 10;
let raw_mut_ptr: *mut i32 = &mut y;
```

**原始指针 vs 引用**：

```
引用（&T, &mut T）：
- 总是有效
- 遵循借用规则
- 编译器检查生命周期
- 自动解引用

原始指针（*const T, *mut T）：
- 可能无效（悬垂）
- 可以同时存在多个可变指针
- 无生命周期检查
- 需要 unsafe 解引用
```

#### 创建原始指针

```rust
// 从引用创建（安全）
let x = 42;
let r1: *const i32 = &x;
let r2: *const i32 = &x as *const i32;

// 从可变引用创建
let mut y = 10;
let r3: *mut i32 = &mut y;

// 从地址创建（危险！）
let address = 0x012345usize;
let r4 = address as *const i32;

// 空指针
let null_ptr: *const i32 = std::ptr::null();
let null_mut_ptr: *mut i32 = std::ptr::null_mut();
```

#### 解引用原始指针

```rust
let mut num = 5;

// 创建原始指针（安全）
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// 解引用（需要 unsafe）
unsafe {
    println!("r1: {}", *r1);
    *r2 = 10;
    println!("r2: {}", *r2);
}
```

**为什么需要 unsafe？**
```
原始指针可能：
1. 指向无效内存
2. 悬垂（指向已释放的内存）
3. 未对齐
4. 为空

编译器无法验证这些，所以需要程序员保证安全
```

### 2. Unsafe 函数

```rust
// 定义 unsafe 函数
unsafe fn dangerous() {
    // 可以包含 unsafe 操作
}

// 调用 unsafe 函数
unsafe {
    dangerous();
}
```

**示例：实现自己的切片索引**

```rust
use std::slice;

// 不安全的切片分割
unsafe fn split_at_mut_unsafe<T>(
    slice: &mut [T],
    mid: usize,
) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
}

// 使用
let mut v = vec![1, 2, 3, 4, 5, 6];
let (left, right) = unsafe {
    split_at_mut_unsafe(&mut v, 3)
};

left[0] = 10;
right[0] = 20;
println!("{:?}", v);  // [10, 2, 3, 20, 5, 6]
```

**为什么标准库的 split_at_mut 是安全的？**
```rust
// 标准库实现（简化）
pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    // 内部使用 unsafe，但对外提供安全接口
    unsafe {
        // ... unsafe 操作
    }
}

// 安全使用
let mut v = vec![1, 2, 3, 4];
let (left, right) = v.split_at_mut(2);  // 无需 unsafe
```

### 3. 外部函数接口（FFI）

FFI 允许 Rust 调用其他语言的代码。

#### 调用 C 函数

```rust
// 声明外部函数
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

// 调用（需要 unsafe）
fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
        println!("sqrt(9.0) = {}", sqrt(9.0));
    }
}
```

#### 从 Rust 导出函数给 C

```rust
// 导出函数给 C 调用
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

**编译为动态库**：
```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]
```

```bash
# 编译
cargo build --release

# 生成的库文件
# Linux: target/release/libmylib.so
# macOS: target/release/libmylib.dylib
# Windows: target/release/mylib.dll
```

### 4. 类型表示

C 和 Rust 的类型对应关系。

```rust
// C 类型 -> Rust 类型
// int -> i32
// unsigned int -> u32
// long -> i64 (64位系统)
// float -> f32
// double -> f64
// char -> i8
// void* -> *mut c_void
// const char* -> *const c_char

use std::os::raw::{c_int, c_char, c_void};

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}
```

#### 结构体布局

```rust
// Rust 默认布局（可能重排字段）
struct RustStruct {
    a: u8,
    b: u32,
    c: u16,
}

// C 兼容布局
#[repr(C)]
struct CStruct {
    a: u8,
    b: u32,
    c: u16,
}

// 打包布局（无填充）
#[repr(packed)]
struct PackedStruct {
    a: u8,
    b: u32,
    c: u16,
}
```

**内存布局对比**：
```
Rust 默认（可能优化）：
[a:1][pad:3][b:4][c:2][pad:2] = 12 字节

#[repr(C)]（C 兼容）：
[a:1][pad:3][b:4][c:2][pad:2] = 12 字节

#[repr(packed)]（无填充）：
[a:1][b:4][c:2] = 7 字节
```

### 5. 字符串处理

C 和 Rust 的字符串不同。

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Rust String -> C 字符串
fn rust_to_c() {
    let rust_str = String::from("Hello, C!");
    
    // 创建 C 字符串（以 \0 结尾）
    let c_str = CString::new(rust_str).unwrap();
    
    // 获取原始指针
    let c_ptr: *const c_char = c_str.as_ptr();
    
    unsafe {
        // 传递给 C 函数
        some_c_function(c_ptr);
    }
}

// C 字符串 -> Rust String
unsafe fn c_to_rust(c_ptr: *const c_char) -> String {
    // 从 C 字符串创建 CStr
    let c_str = CStr::from_ptr(c_ptr);
    
    // 转换为 Rust String
    c_str.to_string_lossy().into_owned()
}

extern "C" {
    fn some_c_function(s: *const c_char);
}
```

**注意事项**：
```
1. C 字符串以 \0 结尾
2. Rust String 是 UTF-8，可能包含 \0
3. CString 确保没有内部 \0
4. 需要管理内存所有权
```

## 💻 实战项目：C 库绑定

### 项目需求

为 C 的 zlib 压缩库创建安全的 Rust 绑定。

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
libc = "0.2"

[build-dependencies]
cc = "1.0"
```

### 步骤 2：声明 C 函数

```rust
// src/ffi.rs
use std::os::raw::{c_int, c_ulong, c_void};

// C 函数声明
extern "C" {
    // 压缩
    pub fn compress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;

    // 解压
    pub fn uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;

    // 计算压缩后的最大大小
    pub fn compressBound(source_len: c_ulong) -> c_ulong;
}

// 错误码
pub const Z_OK: c_int = 0;
pub const Z_MEM_ERROR: c_int = -4;
pub const Z_BUF_ERROR: c_int = -5;
pub const Z_DATA_ERROR: c_int = -3;
```

### 步骤 3：创建安全抽象

```rust
// src/lib.rs
mod ffi;

use std::error::Error;
use std::fmt;

// 错误类型
#[derive(Debug)]
pub enum CompressionError {
    MemoryError,
    BufferError,
    DataError,
    UnknownError(i32),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompressionError::MemoryError => write!(f, "内存不足"),
            CompressionError::BufferError => write!(f, "缓冲区太小"),
            CompressionError::DataError => write!(f, "数据损坏"),
            CompressionError::UnknownError(code) => {
                write!(f, "未知错误: {}", code)
            }
        }
    }
}

impl Error for CompressionError {}

// 安全的压缩函数
pub fn compress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    unsafe {
        // 计算压缩后的最大大小
        let max_len = ffi::compressBound(data.len() as u64);
        
        // 分配缓冲区
        let mut compressed = vec![0u8; max_len as usize];
        let mut compressed_len = max_len;

        // 调用 C 函数
        let result = ffi::compress(
            compressed.as_mut_ptr(),
            &mut compressed_len,
            data.as_ptr(),
            data.len() as u64,
        );

        // 检查结果
        match result {
            ffi::Z_OK => {
                // 调整大小到实际压缩后的大小
                compressed.truncate(compressed_len as usize);
                Ok(compressed)
            }
            ffi::Z_MEM_ERROR => Err(CompressionError::MemoryError),
            ffi::Z_BUF_ERROR => Err(CompressionError::BufferError),
            code => Err(CompressionError::UnknownError(code)),
        }
    }
}

// 安全的解压函数
pub fn uncompress(
    compressed: &[u8],
    original_size: usize,
) -> Result<Vec<u8>, CompressionError> {
    unsafe {
        // 分配缓冲区
        let mut uncompressed = vec![0u8; original_size];
        let mut uncompressed_len = original_size as u64;

        // 调用 C 函数
        let result = ffi::uncompress(
            uncompressed.as_mut_ptr(),
            &mut uncompressed_len,
            compressed.as_ptr(),
            compressed.len() as u64,
        );

        // 检查结果
        match result {
            ffi::Z_OK => {
                uncompressed.truncate(uncompressed_len as usize);
                Ok(uncompressed)
            }
            ffi::Z_MEM_ERROR => Err(CompressionError::MemoryError),
            ffi::Z_BUF_ERROR => Err(CompressionError::BufferError),
            ffi::Z_DATA_ERROR => Err(CompressionError::DataError),
            code => Err(CompressionError::UnknownError(code)),
        }
    }
}
```

### 步骤 4：使用示例

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = b"Hello, World! This is a test of compression.";
    
    println!("原始数据: {} 字节", original.len());
    println!("内容: {:?}", String::from_utf8_lossy(original));

    // 压缩
    let compressed = compress(original)?;
    println!("\n压缩后: {} 字节", compressed.len());
    println!("压缩率: {:.2}%", 
        (1.0 - compressed.len() as f64 / original.len() as f64) * 100.0
    );

    // 解压
    let uncompressed = uncompress(&compressed, original.len())?;
    println!("\n解压后: {} 字节", uncompressed.len());
    println!("内容: {:?}", String::from_utf8_lossy(&uncompressed));

    // 验证
    assert_eq!(original, &uncompressed[..]);
    println!("\n✅ 验证成功！");

    Ok(())
}
```

### 步骤 5：构建脚本

```rust
// build.rs
fn main() {
    // 链接 zlib 库
    println!("cargo:rustc-link-lib=z");
}
```

## 🔍 深入理解

### Unsafe 的最佳实践

```rust
// ❌ 错误：过大的 unsafe 块
unsafe {
    let ptr = get_pointer();
    do_something();
    do_something_else();
    *ptr = 10;
    more_operations();
}

// ✅ 正确：最小化 unsafe 块
let ptr = get_pointer();
do_something();
do_something_else();
unsafe {
    *ptr = 10;  // 只有这里需要 unsafe
}
more_operations();

// ✅ 更好：封装为安全函数
fn safe_set_value(ptr: *mut i32, value: i32) {
    unsafe {
        *ptr = value;
    }
}
```

### 内存安全检查清单

```
使用 unsafe 时，确保：

1. 指针有效性
   ✓ 指针不为空
   ✓ 指针指向有效内存
   ✓ 指针未悬垂

2. 内存对齐
   ✓ 指针正确对齐
   ✓ 使用 align_of 检查

3. 生命周期
   ✓ 数据在使用期间有效
   ✓ 无 use-after-free

4. 数据竞争
   ✓ 无并发访问冲突
   ✓ 正确使用同步原语

5. 不变量
   ✓ 维护类型不变量
   ✓ 不破坏 Rust 的假设
```

### Union 类型

```rust
// Union（所有字段共享内存）
#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
}

fn main() {
    let mut u = MyUnion { i: 42 };

    // 读取需要 unsafe
    unsafe {
        println!("as i32: {}", u.i);
        
        // 写入不同字段
        u.f = 3.14;
        println!("as f32: {}", u.f);
    }
}
```

**使用场景**：
- 类型双关（type punning）
- FFI 与 C union 交互
- 节省内存

## 📝 练习题

### 练习 1：实现自己的 Box

```rust
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout) as *mut T;
            ptr::write(ptr, value);
            MyBox { ptr }
        }
    }

    fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr);
            let layout = Layout::new::<T>();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

#[test]
fn test_my_box() {
    let b = MyBox::new(42);
    assert_eq!(*b.get(), 42);
}
```

### 练习 2：实现 C 字符串工具

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// 计算 C 字符串长度
unsafe fn c_strlen(s: *const c_char) -> usize {
    let mut len = 0;
    let mut ptr = s;
    while *ptr != 0 {
        len += 1;
        ptr = ptr.add(1);
    }
    len
}

// 复制 C 字符串
unsafe fn c_strcpy(dest: *mut c_char, src: *const c_char) {
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_c_string_utils() {
    let s = CString::new("Hello").unwrap();
    unsafe {
        assert_eq!(c_strlen(s.as_ptr()), 5);
    }
}
```

### 练习 3：实现简单的内存池

```rust
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct MemoryPool {
    ptr: *mut u8,
    size: usize,
    used: usize,
}

impl MemoryPool {
    fn new(size: usize) -> Self {
        unsafe {
            let layout = Layout::from_size_align(size, 8).unwrap();
            let ptr = alloc(layout);
            MemoryPool { ptr, size, used: 0 }
        }
    }

    fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        if self.used + size > self.size {
            return None;
        }
        unsafe {
            let ptr = self.ptr.add(self.used);
            self.used += size;
            Some(ptr)
        }
    }

    fn reset(&mut self) {
        self.used = 0;
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.size, 8).unwrap();
            dealloc(self.ptr, layout);
        }
    }
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解 unsafe 的必要性和限制
- [ ] 创建和使用原始指针
- [ ] 编写 unsafe 函数
- [ ] 声明和调用外部函数
- [ ] 处理 C 字符串
- [ ] 使用 #[repr(C)] 定义兼容结构体
- [ ] 为 C 库创建安全的 Rust 绑定
- [ ] 理解内存布局和对齐
- [ ] 遵循 unsafe 最佳实践
- [ ] 构建安全的抽象层

## 🔗 延伸阅读

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust 圣经
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [bindgen](https://rust-lang.github.io/rust-bindgen/) - 自动生成 FFI 绑定
- [cbindgen](https://github.com/eqrion/cbindgen) - 生成 C/C++ 头文件

## 🚀 下一步

完成本模块后，你可以：
1. 学习图形编程（阶段 4）
2. 深入系统编程
3. 为现有 C 库创建 Rust 绑定

---

**掌握 Unsafe，解锁 Rust 的全部能力！** 🦀⚡
