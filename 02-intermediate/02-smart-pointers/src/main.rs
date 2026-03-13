mod custom;
mod list;
mod tree;

use custom::{MyBox, Resource, SimpleRc};
use list::DoublyLinkedList;
use tree::BinarySearchTree;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("=== 智能指针演示 ===\n");

    // 演示 1：Box - 堆分配
    println!("=== 演示 1：Box<T> ===");
    let b = Box::new(5);
    println!("Box 中的值: {}", b);
    println!("Box 大小: {} 字节\n", std::mem::size_of_val(&b));

    // 演示 2：Rc - 引用计数
    println!("=== 演示 2：Rc<T> ===");
    let rc1 = Rc::new(42);
    println!("rc1 引用计数: {}", Rc::strong_count(&rc1));

    let rc2 = Rc::clone(&rc1);
    println!("克隆后 rc1 引用计数: {}", Rc::strong_count(&rc1));
    println!("rc2 引用计数: {}", Rc::strong_count(&rc2));
    println!("rc1 值: {}, rc2 值: {}\n", rc1, rc2);

    // 演示 3：RefCell - 内部可变性
    println!("=== 演示 3：RefCell<T> ===");
    let value = RefCell::new(5);
    println!("初始值: {}", value.borrow());

    *value.borrow_mut() += 10;
    println!("修改后: {}\n", value.borrow());

    // 演示 4：Rc<RefCell<T>> - 共享可变数据
    println!("=== 演示 4：Rc<RefCell<T>> ===");
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let shared_clone = Rc::clone(&shared);

    shared_clone.borrow_mut().push(4);
    println!("通过 clone 修改后: {:?}", shared.borrow());
    println!("引用计数: {}\n", Rc::strong_count(&shared));

    // 演示 5：二叉搜索树
    println!("=== 演示 5：二叉搜索树 ===");
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    println!("中序遍历: {:?}", tree.inorder());
    println!("树高度: {}", tree.height());
    println!("搜索 7: {}", tree.search(&7));
    println!("搜索 10: {}\n", tree.search(&10));

    // 演示 6：双向链表
    println!("=== 演示 6：双向链表（Weak 避免循环引用）===");
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("链表内容: {:?}", list.to_vec());
    println!("链表大小: {}", list.len());

    list.pop_front();
    println!("弹出头部后: {:?}\n", list.to_vec());

    // 演示 7：自定义智能指针
    println!("=== 演示 7：自定义智能指针 ===");
    let my_box = MyBox::new(String::from("Hello"));
    println!("MyBox 解引用: {}", *my_box);
    println!("MyBox 长度: {}\n", my_box.len()); // Deref 强制转换

    // 演示 8：SimpleRc
    println!("=== 演示 8：SimpleRc（自定义引用计数）===");
    let simple_rc1 = SimpleRc::new(100);
    println!("引用计数: {}", simple_rc1.strong_count());

    {
        let simple_rc2 = simple_rc1.clone();
        println!("克隆后引用计数: {}", simple_rc1.strong_count());
        println!("值: {}", *simple_rc2);
    }
    println!("作用域结束后引用计数: {}\n", simple_rc1.strong_count());

    // 演示 9：Resource（自动清理）
    println!("=== 演示 9：Resource（Drop trait）===");
    {
        let _resource = Resource::new(vec![1, 2, 3], "数据".to_string());
        println!("资源在使用中...");
    }
    println!("资源已自动清理\n");

    // 演示 10：循环引用问题
    println!("=== 演示 10：避免循环引用 ===");
    demonstrate_weak_reference();
}

/// 演示 Weak 引用避免循环引用
fn demonstrate_weak_reference() {
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf 引用计数: {}", Rc::strong_count(&leaf));
    println!("leaf 弱引用计数: {}", Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch 引用计数: {}", Rc::strong_count(&branch));
        println!("branch 弱引用计数: {}", Rc::weak_count(&branch));
        println!("leaf 引用计数: {}", Rc::strong_count(&leaf));
    }

    println!("作用域结束后 leaf 引用计数: {}", Rc::strong_count(&leaf));
    println!("leaf 的父节点: {:?}", leaf.parent.borrow().upgrade());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let b = Box::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn test_rc() {
        let rc1 = Rc::new(5);
        let rc2 = Rc::clone(&rc1);
        assert_eq!(Rc::strong_count(&rc1), 2);
        assert_eq!(*rc1, *rc2);
    }

    #[test]
    fn test_refcell() {
        let value = RefCell::new(10);
        *value.borrow_mut() += 5;
        assert_eq!(*value.borrow(), 15);
    }

    #[test]
    fn test_tree() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        assert!(tree.search(&5));
        assert!(!tree.search(&10));
    }

    #[test]
    fn test_list() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
    }
}

