/// 带生命周期的链表实现
///
/// 演示生命周期标注和引用管理

/// 链表节点
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

/// 泛型链表
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    /// 创建新的空链表
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    /// 在链表头部插入元素
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    /// 从链表头部移除元素
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }

    /// 查看头部元素
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 获取链表大小
    pub fn len(&self) -> usize {
        self.size
    }

    /// 清空链表
    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 带生命周期的引用包装器
///
/// 演示生命周期标注
#[derive(Debug)]
pub struct RefWrapper<'a, T> {
    value: &'a T,
}

impl<'a, T> RefWrapper<'a, T> {
    /// 创建新的引用包装器
    pub fn new(value: &'a T) -> Self {
        RefWrapper { value }
    }

    /// 获取引用
    pub fn get(&self) -> &T {
        self.value
    }
}

/// 比较两个字符串切片，返回较长的那个
///
/// 演示生命周期标注
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 获取字符串的第一个单词
///
/// 演示生命周期省略
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// 带生命周期的结构体
///
/// 演示结构体中的生命周期
#[derive(Debug)]
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    /// 创建新的摘录
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }

    /// 获取摘录的第一个句子
    pub fn first_sentence(&self) -> &str {
        if let Some(pos) = self.text.find('.') {
            &self.text[..=pos]
        } else {
            self.text
        }
    }

    /// 获取文本长度
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_basic() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_linked_list_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.front(), None);

        list.push_front(42);
        assert_eq!(list.front(), Some(&42));
    }

    #[test]
    fn test_ref_wrapper() {
        let value = 42;
        let wrapper = RefWrapper::new(&value);
        assert_eq!(*wrapper.get(), 42);
    }

    #[test]
    fn test_longest() {
        let s1 = "hello";
        let s2 = "world!";
        assert_eq!(longest(s1, s2), "world!");

        let s3 = "short";
        let s4 = "longer string";
        assert_eq!(longest(s3, s4), "longer string");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_excerpt() {
        let text = "Hello world. This is Rust.";
        let excerpt = Excerpt::new(text);
        assert_eq!(excerpt.first_sentence(), "Hello world.");
        assert_eq!(excerpt.len(), 26);
    }

    #[test]
    fn test_excerpt_no_period() {
        let text = "No period here";
        let excerpt = Excerpt::new(text);
        assert_eq!(excerpt.first_sentence(), "No period here");
    }

    #[test]
    fn test_lifetime_scope() {
        let string1 = String::from("long string");
        let result;
        {
            let string2 = String::from("short");
            result = longest(string1.as_str(), string2.as_str());
            // result 在这里有效
            assert_eq!(result, "long string");
        }
        // result 在这里仍然有效，因为它引用 string1
    }
}
