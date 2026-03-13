/// 泛型栈实现
///
/// 演示泛型类型参数和方法实现

use std::fmt::Display;

/// 泛型栈
#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// 创建新的空栈
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// 压入元素
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// 弹出元素
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// 查看栈顶元素
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 获取栈的大小
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 清空栈
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

// 为实现了 Display 的类型提供额外方法
impl<T: Display> Stack<T> {
    /// 打印栈中所有元素
    pub fn print_all(&self) {
        println!("Stack contents:");
        for (i, item) in self.items.iter().enumerate() {
            println!("  [{}]: {}", i, item);
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 泛型函数：找出切片中的最大值
///
/// # 示例
///
/// ```
/// use generics_lifetimes::stack::find_max;
/// let numbers = vec![3, 7, 2, 9, 1];
/// assert_eq!(find_max(&numbers), Some(&9));
/// ```
pub fn find_max<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut max = &list[0];
    for item in &list[1..] {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

/// 泛型函数：找出切片中的最小值
pub fn find_min<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut min = &list[0];
    for item in &list[1..] {
        if item < min {
            min = item;
        }
    }
    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_basic() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1); // peek 不消耗
    }

    #[test]
    fn test_stack_generic_string() {
        let mut stack: Stack<String> = Stack::new();
        stack.push("hello".to_string());
        stack.push("world".to_string());

        assert_eq!(stack.pop(), Some("world".to_string()));
    }

    #[test]
    fn test_find_max() {
        let numbers = vec![3, 7, 2, 9, 1];
        assert_eq!(find_max(&numbers), Some(&9));

        let strings = vec!["apple", "zebra", "banana"];
        assert_eq!(find_max(&strings), Some(&"zebra"));

        let empty: Vec<i32> = vec![];
        assert_eq!(find_max(&empty), None);
    }

    #[test]
    fn test_find_min() {
        let numbers = vec![3, 7, 2, 9, 1];
        assert_eq!(find_min(&numbers), Some(&1));
    }
}
