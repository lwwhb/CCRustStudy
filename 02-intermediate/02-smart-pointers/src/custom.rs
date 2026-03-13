/// 自定义智能指针
///
/// 演示 Deref 和 Drop trait 的实现

use std::ops::Deref;

/// 自定义 Box 实现
pub struct MyBox<T> {
    data: T,
}

impl<T> MyBox<T> {
    /// 创建新的 MyBox
    pub fn new(data: T) -> Self {
        MyBox { data }
    }

    /// 获取内部值的引用
    pub fn get(&self) -> &T {
        &self.data
    }
}

// 实现 Deref trait，允许解引用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// 实现 Drop trait，自定义清理逻辑
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox 被销毁");
    }
}

/// 引用计数智能指针（简化版）
pub struct SimpleRc<T> {
    data: *mut RcInner<T>,
}

struct RcInner<T> {
    value: T,
    ref_count: usize,
}

impl<T> SimpleRc<T> {
    /// 创建新的 SimpleRc
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            ref_count: 1,
        });
        SimpleRc {
            data: Box::into_raw(inner),
        }
    }

    /// 获取引用计数
    pub fn strong_count(&self) -> usize {
        unsafe { (*self.data).ref_count }
    }

    /// 获取内部值的引用
    pub fn get(&self) -> &T {
        unsafe { &(*self.data).value }
    }
}

impl<T> Clone for SimpleRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.data).ref_count += 1;
        }
        SimpleRc { data: self.data }
    }
}

impl<T> Drop for SimpleRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.data).ref_count -= 1;
            if (*self.data).ref_count == 0 {
                let _ = Box::from_raw(self.data);
                println!("SimpleRc 内部数据被释放");
            }
        }
    }
}

impl<T> Deref for SimpleRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.data).value }
    }
}

/// 带自动清理的资源包装器
pub struct Resource<T> {
    data: T,
    name: String,
}

impl<T> Resource<T> {
    /// 创建新的资源
    pub fn new(data: T, name: String) -> Self {
        println!("资源 '{}' 被创建", name);
        Resource { data, name }
    }

    /// 获取数据引用
    pub fn get(&self) -> &T {
        &self.data
    }
}

impl<T> Drop for Resource<T> {
    fn drop(&mut self) {
        println!("资源 '{}' 被清理", self.name);
    }
}

/// 演示 Deref 强制转换
pub fn print_string(s: &str) {
    println!("字符串: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox_deref() {
        let x = MyBox::new(5);
        assert_eq!(*x, 5);
        assert_eq!(*x.get(), 5);
    }

    #[test]
    fn test_mybox_with_string() {
        let s = MyBox::new(String::from("hello"));
        // Deref 强制转换：MyBox<String> -> String -> &str
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_simple_rc() {
        let rc1 = SimpleRc::new(42);
        assert_eq!(rc1.strong_count(), 1);

        let rc2 = rc1.clone();
        assert_eq!(rc1.strong_count(), 2);
        assert_eq!(rc2.strong_count(), 2);

        assert_eq!(*rc1, 42);
        assert_eq!(*rc2, 42);
    }

    #[test]
    fn test_simple_rc_drop() {
        let rc1 = SimpleRc::new(100);
        {
            let rc2 = rc1.clone();
            assert_eq!(rc1.strong_count(), 2);
            assert_eq!(*rc2, 100);
        }
        // rc2 被销毁，引用计数减 1
        assert_eq!(rc1.strong_count(), 1);
    }

    #[test]
    fn test_resource() {
        let _resource = Resource::new(vec![1, 2, 3], "test_data".to_string());
        // 资源在作用域结束时自动清理
    }

    #[test]
    fn test_deref_coercion() {
        let s = MyBox::new(String::from("hello world"));
        // Deref 强制转换允许我们传递 &MyBox<String> 给需要 &str 的函数
        print_string(&s);
    }

    #[test]
    fn test_nested_mybox() {
        let x = MyBox::new(MyBox::new(42));
        assert_eq!(**x, 42);
    }
}
