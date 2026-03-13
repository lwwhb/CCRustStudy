mod cache;
mod list;
mod queue;
mod stack;

use cache::{Cache, TtlCache};
use list::{Excerpt, LinkedList};
use queue::{Pair, Queue};
use stack::{find_max, find_min, Stack};

fn main() {
    println!("=== 泛型与生命周期演示 ===\n");

    // 演示 1：泛型栈
    println!("=== 演示 1：泛型栈 ===");
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("栈大小: {}", stack.len());
    println!("弹出: {:?}", stack.pop());
    println!("栈顶: {:?}\n", stack.peek());

    // 演示 2：泛型队列
    println!("=== 演示 2：泛型队列 ===");
    let mut queue: Queue<String> = Queue::new();
    queue.enqueue("first".to_string());
    queue.enqueue("second".to_string());
    queue.enqueue("third".to_string());
    println!("队列大小: {}", queue.len());
    println!("出队: {:?}", queue.dequeue());
    println!("队首: {:?}\n", queue.front());

    // 演示 3：泛型 Pair
    println!("=== 演示 3：泛型 Pair ===");
    let pair = Pair::new(42, "answer");
    println!("Pair: {:?}", pair);
    let swapped = pair.swap();
    println!("交换后: {:?}\n", swapped);

    // 演示 4：泛型函数
    println!("=== 演示 4：泛型函数 ===");
    let numbers = vec![3, 7, 2, 9, 1, 5];
    println!("数组: {:?}", numbers);
    println!("最大值: {:?}", find_max(&numbers));
    println!("最小值: {:?}\n", find_min(&numbers));

    // 演示 5：链表
    println!("=== 演示 5：泛型链表 ===");
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("链表大小: {}", list.len());
    println!("头部元素: {:?}", list.front());
    println!("弹出: {:?}\n", list.pop_front());

    // 演示 6：生命周期
    println!("=== 演示 6：生命周期 ===");
    let string1 = String::from("long string is long");
    let string2 = "short";
    let result = list::longest(string1.as_str(), string2);
    println!("较长的字符串: {}", result);

    let text = "Hello world";
    let first = list::first_word(text);
    println!("第一个单词: {}\n", first);

    // 演示 7：带生命周期的结构体
    println!("=== 演示 7：带生命周期的结构体 ===");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt::new(&novel);
    println!("摘录: {}", excerpt.text);
    println!("第一句: {}\n", excerpt.first_sentence());

    // 演示 8：泛型缓存
    println!("=== 演示 8：泛型缓存 ===");
    let mut cache: Cache<String, i32> = Cache::new(3);
    cache.insert("one".to_string(), 1);
    cache.insert("two".to_string(), 2);
    cache.insert("three".to_string(), 3);
    println!("缓存大小: {}", cache.len());
    println!("获取 'two': {:?}", cache.get(&"two".to_string()));
    println!("缓存已满: {}\n", cache.is_full());

    // 演示 9：TTL 缓存
    println!("=== 演示 9：TTL 缓存 ===");
    let mut ttl_cache: TtlCache<String, String> = TtlCache::new(10, 60);
    ttl_cache.insert("session1".to_string(), "user_data".to_string(), 0);
    ttl_cache.insert("session2".to_string(), "admin_data".to_string(), 10);

    println!("在 TTL 内获取: {:?}", ttl_cache.get(&"session1".to_string(), 30));
    println!("超过 TTL 获取: {:?}", ttl_cache.get(&"session1".to_string(), 70));

    let removed = ttl_cache.cleanup(100);
    println!("清理过期项: {} 个\n", removed);

    // 演示 10：多种类型的泛型
    println!("=== 演示 10：多种类型的泛型 ===");
    demonstrate_generic_types();
}

/// 演示不同类型的泛型使用
fn demonstrate_generic_types() {
    // 整数栈
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    println!("整数栈: {:?}", int_stack);

    // 字符串栈
    let mut str_stack = Stack::new();
    str_stack.push("hello");
    str_stack.push("world");
    println!("字符串栈: {:?}", str_stack);

    // 向量栈
    let mut vec_stack: Stack<Vec<i32>> = Stack::new();
    vec_stack.push(vec![1, 2, 3]);
    vec_stack.push(vec![4, 5, 6]);
    println!("向量栈大小: {}", vec_stack.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn test_generic_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
    }

    #[test]
    fn test_pair_swap() {
        let pair = Pair::new(1, "hello");
        let swapped = pair.swap();
        assert_eq!(swapped.first, "hello");
        assert_eq!(swapped.second, 1);
    }

    #[test]
    fn test_find_max_min() {
        let numbers = vec![3, 7, 2, 9, 1];
        assert_eq!(find_max(&numbers), Some(&9));
        assert_eq!(find_min(&numbers), Some(&1));
    }

    #[test]
    fn test_longest() {
        let s1 = "hello";
        let s2 = "world!";
        assert_eq!(list::longest(s1, s2), "world!");
    }

    #[test]
    fn test_cache() {
        let mut cache = Cache::new(2);
        cache.insert("key1", 100);
        cache.insert("key2", 200);
        assert_eq!(cache.get(&"key1"), Some(&100));
    }
}

