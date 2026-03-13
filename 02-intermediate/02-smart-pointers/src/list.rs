/// 双向链表实现
///
/// 使用 Rc 和 Weak 避免循环引用

use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

/// 双向链表节点
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
    pub prev: WeakLink<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

/// 双向链表
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: WeakLink<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    /// 创建新的空链表
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// 在头部插入
    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }

    /// 在尾部插入
    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                if let Some(old_tail_rc) = old_tail.upgrade() {
                    old_tail_rc.borrow_mut().next = Some(Rc::clone(&new_node));
                    new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail_rc));
                    self.tail = Some(Rc::downgrade(&new_node));
                }
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }

    /// 从头部移除
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    /// 获取链表大小
    pub fn len(&self) -> usize {
        self.size
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 转换为 Vec
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        let mut current = self.head.clone();

        while let Some(node) = current {
            result.push(node.borrow().value.clone());
            current = node.borrow().next.clone();
        }

        result
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn test_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_pop_front() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_back(3);
        list.push_front(1);

        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_single_element() {
        let mut list = DoublyLinkedList::new();
        list.push_front(42);

        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(42));
        assert!(list.is_empty());
    }
}
