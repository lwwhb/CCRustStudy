/// 泛型栈实现
///
/// 演示泛型数据结构和模块组织

/// 栈数据结构
///
/// # 示例
///
/// ```
/// use modules_cargo::collections::stack::Stack;
///
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
///
/// assert_eq!(stack.pop(), Some(3));
/// assert_eq!(stack.peek(), Some(&2));
/// assert_eq!(stack.size(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// 创建新的空栈
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    /// 将元素压入栈顶
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// 弹出栈顶元素
    ///
    /// 如果栈为空，返回 `None`
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// 查看栈顶元素（不弹出）
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// 返回栈的大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// 清空栈
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// 将栈转换为 Vec（消耗栈）
    pub fn into_vec(self) -> Vec<T> {
        self.elements
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 使用栈检查括号是否匹配
///
/// # 示例
///
/// ```
/// use modules_cargo::collections::stack::is_balanced;
/// assert!(is_balanced("()[]{}"));
/// assert!(!is_balanced("([)]"));
/// ```
pub fn is_balanced(s: &str) -> bool {
    let mut stack = Stack::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.size(), 1); // peek 不消耗元素
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());

        stack.push(1);
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_clear() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_generic() {
        let mut stack: Stack<String> = Stack::new();
        stack.push("hello".to_string());
        stack.push("world".to_string());
        assert_eq!(stack.pop(), Some("world".to_string()));
    }

    #[test]
    fn test_is_balanced() {
        assert!(is_balanced("()[]{}"));
        assert!(is_balanced("({[]})"));
        assert!(is_balanced(""));
        assert!(!is_balanced("([)]"));
        assert!(!is_balanced("("));
        assert!(!is_balanced(")"));
    }
}
