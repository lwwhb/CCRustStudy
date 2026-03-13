/// 泛型缓存系统
///
/// 演示泛型、trait 约束和生命周期的结合使用

use std::collections::HashMap;
use std::hash::Hash;

/// 泛型缓存
///
/// K 必须实现 Eq 和 Hash，V 可以是任意类型
pub struct Cache<K, V>
where
    K: Eq + Hash,
{
    store: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
{
    /// 创建新的缓存，指定最大容量
    pub fn new(max_size: usize) -> Self {
        Cache {
            store: HashMap::new(),
            max_size,
        }
    }

    /// 插入键值对
    ///
    /// 如果缓存已满，返回 false
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.store.len() >= self.max_size && !self.store.contains_key(&key) {
            return false;
        }
        self.store.insert(key, value);
        true
    }

    /// 获取值的引用
    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }

    /// 获取值的可变引用
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.store.get_mut(key)
    }

    /// 检查键是否存在
    pub fn contains(&self, key: &K) -> bool {
        self.store.contains_key(key)
    }

    /// 移除键值对
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key)
    }

    /// 获取缓存大小
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// 检查是否已满
    pub fn is_full(&self) -> bool {
        self.store.len() >= self.max_size
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

/// 带过期时间的缓存项
#[derive(Debug, Clone)]
pub struct CacheItem<T> {
    pub value: T,
    pub timestamp: u64,
}

impl<T> CacheItem<T> {
    /// 创建新的缓存项
    pub fn new(value: T, timestamp: u64) -> Self {
        CacheItem { value, timestamp }
    }

    /// 检查是否过期
    pub fn is_expired(&self, current_time: u64, ttl: u64) -> bool {
        current_time - self.timestamp > ttl
    }
}

/// 带 TTL 的缓存
pub struct TtlCache<K, V>
where
    K: Eq + Hash,
{
    store: HashMap<K, CacheItem<V>>,
    max_size: usize,
    ttl: u64, // 生存时间（秒）
}

impl<K, V> TtlCache<K, V>
where
    K: Eq + Hash,
{
    /// 创建新的 TTL 缓存
    pub fn new(max_size: usize, ttl: u64) -> Self {
        TtlCache {
            store: HashMap::new(),
            max_size,
            ttl,
        }
    }

    /// 插入键值对
    pub fn insert(&mut self, key: K, value: V, timestamp: u64) -> bool {
        if self.store.len() >= self.max_size && !self.store.contains_key(&key) {
            return false;
        }
        self.store.insert(key, CacheItem::new(value, timestamp));
        true
    }

    /// 获取值（检查过期）
    pub fn get(&self, key: &K, current_time: u64) -> Option<&V> {
        self.store.get(key).and_then(|item| {
            if item.is_expired(current_time, self.ttl) {
                None
            } else {
                Some(&item.value)
            }
        })
    }

    /// 清理过期项
    pub fn cleanup(&mut self, current_time: u64) -> usize {
        let before = self.store.len();
        self.store.retain(|_, item| !item.is_expired(current_time, self.ttl));
        before - self.store.len()
    }

    /// 获取缓存大小
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = Cache::new(3);
        assert!(cache.insert("key1", 100));
        assert!(cache.insert("key2", 200));
        assert!(cache.insert("key3", 300));

        assert_eq!(cache.get(&"key1"), Some(&100));
        assert_eq!(cache.get(&"key2"), Some(&200));
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_cache_full() {
        let mut cache = Cache::new(2);
        assert!(cache.insert("key1", 100));
        assert!(cache.insert("key2", 200));
        assert!(!cache.insert("key3", 300)); // 缓存已满

        assert!(cache.is_full());
    }

    #[test]
    fn test_cache_update() {
        let mut cache = Cache::new(2);
        cache.insert("key1", 100);
        cache.insert("key1", 200); // 更新

        assert_eq!(cache.get(&"key1"), Some(&200));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = Cache::new(3);
        cache.insert("key1", 100);
        cache.insert("key2", 200);

        assert_eq!(cache.remove(&"key1"), Some(100));
        assert_eq!(cache.get(&"key1"), None);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_generic_types() {
        let mut cache: Cache<String, Vec<i32>> = Cache::new(5);
        cache.insert("numbers".to_string(), vec![1, 2, 3]);

        assert_eq!(cache.get(&"numbers".to_string()), Some(&vec![1, 2, 3]));
    }

    #[test]
    fn test_ttl_cache() {
        let mut cache = TtlCache::new(10, 60); // 60 秒 TTL

        cache.insert("key1", "value1", 0);
        cache.insert("key2", "value2", 10);

        // 在 TTL 内
        assert_eq!(cache.get(&"key1", 30), Some(&"value1"));
        assert_eq!(cache.get(&"key2", 30), Some(&"value2"));

        // 超过 TTL
        assert_eq!(cache.get(&"key1", 70), None);
        assert_eq!(cache.get(&"key2", 80), None);
    }

    #[test]
    fn test_ttl_cache_cleanup() {
        let mut cache = TtlCache::new(10, 60);

        cache.insert("key1", "value1", 0);
        cache.insert("key2", "value2", 10);
        cache.insert("key3", "value3", 50);

        assert_eq!(cache.len(), 3);

        // 清理过期项（当前时间 100）
        let removed = cache.cleanup(100);
        assert_eq!(removed, 2); // key1 和 key2 过期
        assert_eq!(cache.len(), 1);
    }
}
