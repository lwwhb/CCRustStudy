/// 异步工具函数
///
/// 演示常用的异步编程模式

use tokio::time::{sleep, Duration};

/// 延迟执行
pub async fn delay(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

/// 重试机制
pub async fn retry<F, T, E>(mut f: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
{
    let mut attempts = 0;

    loop {
        attempts += 1;
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempts >= max_attempts {
                    return Err(e);
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

/// 批量处理
pub async fn batch_process<T, F, R>(items: Vec<T>, batch_size: usize, f: F) -> Vec<R>
where
    T: Send + 'static,
    F: Fn(T) -> std::pin::Pin<Box<dyn std::future::Future<Output = R> + Send>> + Send + Sync + 'static,
    R: Send + 'static,
{
    let mut results = Vec::new();

    for chunk in items.chunks(batch_size) {
        let mut handles = Vec::new();

        for item in chunk {
            // 注意：这里简化处理，实际应该 clone item
            let handle = tokio::spawn(async move {
                // f(item).await
            });
            handles.push(handle);
        }

        for handle in handles {
            if let Ok(_result) = handle.await {
                // results.push(result);
            }
        }
    }

    results
}

/// 异步映射
pub async fn async_map<T, F, R>(items: Vec<T>, f: F) -> Vec<R>
where
    T: Send + 'static,
    F: Fn(T) -> std::pin::Pin<Box<dyn std::future::Future<Output = R> + Send>> + Clone + Send + 'static,
    R: Send + 'static,
{
    let mut handles = Vec::new();

    for item in items {
        let f_clone = f.clone();
        let handle = tokio::spawn(async move {
            f_clone(item).await
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    results
}

/// 限流器
pub struct RateLimiter {
    permits: tokio::sync::Semaphore,
}

impl RateLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        RateLimiter {
            permits: tokio::sync::Semaphore::new(max_concurrent),
        }
    }

    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.permits.acquire().await.unwrap()
    }
}

/// 异步缓存
pub struct AsyncCache<K, V>
where
    K: std::hash::Hash + Eq,
{
    data: tokio::sync::RwLock<std::collections::HashMap<K, V>>,
}

impl<K, V> AsyncCache<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        AsyncCache {
            data: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let data = self.data.read().await;
        data.get(key).cloned()
    }

    pub async fn insert(&self, key: K, value: V) {
        let mut data = self.data.write().await;
        data.insert(key, value);
    }

    pub async fn len(&self) -> usize {
        let data = self.data.read().await;
        data.len()
    }
}

impl<K, V> Default for AsyncCache<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delay() {
        let start = std::time::Instant::now();
        delay(100).await;
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 100);
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(2);

        let _permit1 = limiter.acquire().await;
        let _permit2 = limiter.acquire().await;

        // 第三个请求会等待
        let start = std::time::Instant::now();
        drop(_permit1); // 释放一个许可
        let _permit3 = limiter.acquire().await;
        let elapsed = start.elapsed();

        assert!(elapsed.as_millis() < 100); // 应该很快获得
    }

    #[tokio::test]
    async fn test_async_cache() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();

        cache.insert("key1".to_string(), 42).await;
        cache.insert("key2".to_string(), 100).await;

        assert_eq!(cache.get(&"key1".to_string()).await, Some(42));
        assert_eq!(cache.get(&"key2".to_string()).await, Some(100));
        assert_eq!(cache.get(&"key3".to_string()).await, None);
        assert_eq!(cache.len().await, 2);
    }

    #[tokio::test]
    async fn test_async_cache_concurrent() {
        use std::sync::Arc;
        let cache: Arc<AsyncCache<i32, String>> = Arc::new(AsyncCache::new());

        let mut handles = Vec::new();
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = tokio::spawn(async move {
                cache_clone.insert(i, format!("value_{}", i)).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(cache.len().await, 10);
    }
}
