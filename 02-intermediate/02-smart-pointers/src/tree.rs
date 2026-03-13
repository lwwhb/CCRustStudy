/// 二叉搜索树实现
///
/// 使用 Rc<RefCell<Node>> 实现共享可变的树结构

use std::cell::RefCell;
use std::rc::Rc;

type TreeNode<T> = Rc<RefCell<Node<T>>>;

/// 树节点
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<TreeNode<T>>,
    pub right: Option<TreeNode<T>>,
}

impl<T> Node<T> {
    /// 创建新节点
    pub fn new(value: T) -> TreeNode<T> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

/// 二叉搜索树
pub struct BinarySearchTree<T> {
    root: Option<TreeNode<T>>,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    /// 创建新的空树
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// 插入值
    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Node::new(value));
            return;
        }

        Self::insert_node(self.root.as_ref().unwrap(), value);
    }

    /// 递归插入节点
    fn insert_node(node: &TreeNode<T>, value: T) {
        let mut node_ref = node.borrow_mut();

        if value < node_ref.value {
            if let Some(ref left) = node_ref.left {
                Self::insert_node(left, value);
            } else {
                node_ref.left = Some(Node::new(value));
            }
        } else {
            if let Some(ref right) = node_ref.right {
                Self::insert_node(right, value);
            } else {
                node_ref.right = Some(Node::new(value));
            }
        }
    }

    /// 搜索值
    pub fn search(&self, value: &T) -> bool {
        self.root
            .as_ref()
            .map_or(false, |node| Self::search_node(node, value))
    }

    /// 递归搜索节点
    fn search_node(node: &TreeNode<T>, value: &T) -> bool {
        let node_ref = node.borrow();

        if value == &node_ref.value {
            true
        } else if value < &node_ref.value {
            node_ref
                .left
                .as_ref()
                .map_or(false, |left| Self::search_node(left, value))
        } else {
            node_ref
                .right
                .as_ref()
                .map_or(false, |right| Self::search_node(right, value))
        }
    }

    /// 中序遍历
    pub fn inorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::inorder_traverse(root, &mut result);
        }
        result
    }

    /// 递归中序遍历
    fn inorder_traverse(node: &TreeNode<T>, result: &mut Vec<T>) {
        let node_ref = node.borrow();

        if let Some(ref left) = node_ref.left {
            Self::inorder_traverse(left, result);
        }

        result.push(node_ref.value.clone());

        if let Some(ref right) = node_ref.right {
            Self::inorder_traverse(right, result);
        }
    }

    /// 获取树的高度
    pub fn height(&self) -> usize {
        self.root
            .as_ref()
            .map_or(0, |node| Self::node_height(node))
    }

    /// 递归计算节点高度
    fn node_height(node: &TreeNode<T>) -> usize {
        let node_ref = node.borrow();

        let left_height = node_ref
            .left
            .as_ref()
            .map_or(0, |left| Self::node_height(left));

        let right_height = node_ref
            .right
            .as_ref()
            .map_or(0, |right| Self::node_height(right));

        1 + left_height.max(right_height)
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl<T: Ord + Clone> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);

        assert!(tree.search(&5));
        assert!(tree.search(&3));
        assert!(tree.search(&7));
        assert!(!tree.search(&10));
    }

    #[test]
    fn test_inorder() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(9);

        let result = tree.inorder();
        assert_eq!(result, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_height() {
        let mut tree = BinarySearchTree::new();
        assert_eq!(tree.height(), 0);

        tree.insert(5);
        assert_eq!(tree.height(), 1);

        tree.insert(3);
        tree.insert(7);
        assert_eq!(tree.height(), 2);

        tree.insert(1);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_empty_tree() {
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree.is_empty());
        assert!(!tree.search(&5));
        assert_eq!(tree.inorder(), Vec::<i32>::new());
    }

    #[test]
    fn test_string_tree() {
        let mut tree = BinarySearchTree::new();
        tree.insert("dog".to_string());
        tree.insert("cat".to_string());
        tree.insert("elephant".to_string());

        assert!(tree.search(&"dog".to_string()));
        assert!(tree.search(&"cat".to_string()));
        assert!(!tree.search(&"bird".to_string()));
    }
}
