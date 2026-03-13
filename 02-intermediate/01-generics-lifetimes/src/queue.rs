/// 泛型队列实现
///
/// 演示泛型和 FIFO 数据结构

#[derive(Debug, Clone)]
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    /// 创建新的空队列
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    /// 入队
    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    /// 出队
    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    /// 查看队首元素
    pub fn front(&self) -> Option<&T> {
        self.items.first()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 获取队列大小
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 清空队列
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 泛型 Pair 结构体
///
/// 演示多个泛型参数
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// 创建新的 Pair
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    /// 交换两个元素（消耗 self）
    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }

    /// 获取第一个元素的引用
    pub fn first(&self) -> &T {
        &self.first
    }

    /// 获取第二个元素的引用
    pub fn second(&self) -> &U {
        &self.second
    }
}

// 为相同类型的 Pair 实现额外方法
impl<T: Clone> Pair<T, T> {
    /// 创建两个相同元素的 Pair
    pub fn duplicate(value: T) -> Self {
        Pair {
            first: value.clone(),
            second: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_basic() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_queue_front() {
        let mut queue = Queue::new();
        assert_eq!(queue.front(), None);

        queue.enqueue(42);
        assert_eq!(queue.front(), Some(&42));
        assert_eq!(queue.len(), 1); // front 不消耗
    }

    #[test]
    fn test_queue_generic() {
        let mut queue: Queue<String> = Queue::new();
        queue.enqueue("first".to_string());
        queue.enqueue("second".to_string());

        assert_eq!(queue.dequeue(), Some("first".to_string()));
    }

    #[test]
    fn test_pair_basic() {
        let pair = Pair::new(1, "hello");
        assert_eq!(pair.first, 1);
        assert_eq!(pair.second, "hello");
    }

    #[test]
    fn test_pair_swap() {
        let pair = Pair::new(1, "hello");
        let swapped = pair.swap();
        assert_eq!(swapped.first, "hello");
        assert_eq!(swapped.second, 1);
    }

    #[test]
    fn test_pair_duplicate() {
        let pair = Pair::duplicate(42);
        assert_eq!(pair.first, 42);
        assert_eq!(pair.second, 42);
    }

    #[test]
    fn test_pair_different_types() {
        let pair = Pair::new(vec![1, 2, 3], "numbers");
        assert_eq!(pair.first().len(), 3);
        assert_eq!(*pair.second(), "numbers");
    }
}
