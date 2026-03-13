mod client;
mod tasks;
mod utils;

use client::AsyncClient;
use tasks::{run_concurrent_tasks, run_with_join, run_with_select, simple_task, AsyncCounter};
use utils::{delay, AsyncCache, RateLimiter};

use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== 异步编程基础演示 ===\n");

    // 演示 1：基本异步函数
    println!("=== 演示 1：基本异步函数 ===");
    simple_task(1, 100).await;
    println!();

    // 演示 2：并发执行任务
    println!("=== 演示 2：并发执行任务 ===");
    let results = run_concurrent_tasks(3).await;
    println!("完成的任务: {:?}\n", results);

    // 演示 3：使用 join! 宏
    println!("=== 演示 3：使用 join! 宏 ===");
    let (r1, r2, r3) = run_with_join().await;
    println!("结果: {}, {}, {}\n", r1, r2, r3);

    // 演示 4：使用 select! 宏
    println!("=== 演示 4：使用 select! 宏 ===");
    let result = run_with_select().await;
    println!("{}\n", result);

    // 演示 5：超时控制
    println!("=== 演示 5：超时控制 ===");
    demonstrate_timeout().await;

    // 演示 6：异步计数器
    println!("\n=== 演示 6：异步计数器（共享状态）===");
    demonstrate_async_counter().await;

    // 演示 7：限流器
    println!("\n=== 演示 7：限流器 ===");
    demonstrate_rate_limiter().await;

    // 演示 8：异步缓存
    println!("\n=== 演示 8：异步缓存 ===");
    demonstrate_async_cache().await;

    // 演示 9：HTTP 客户端
    println!("\n=== 演示 9：HTTP 客户端 ===");
    demonstrate_http_client().await;

    // 演示 10：延迟执行
    println!("\n=== 演示 10：延迟执行 ===");
    println!("延迟 500ms...");
    delay(500).await;
    println!("完成！");
}

/// 演示超时控制
async fn demonstrate_timeout() {
    use tasks::with_timeout;

    // 成功的任务
    let task1 = async {
        delay(50).await;
        42
    };

    match with_timeout(task1, 100).await {
        Ok(result) => println!("任务完成: {}", result),
        Err(e) => println!("任务失败: {}", e),
    }

    // 超时的任务
    let task2 = async {
        delay(200).await;
        42
    };

    match with_timeout(task2, 100).await {
        Ok(result) => println!("任务完成: {}", result),
        Err(e) => println!("任务失败: {}", e),
    }
}

/// 演示异步计数器
async fn demonstrate_async_counter() {
    use std::sync::Arc;
    let counter = Arc::new(AsyncCounter::new());

    let mut handles = Vec::new();
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            println!("任务 {} 增加计数", i);
            counter_clone.increment().await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("最终计数: {}", counter.get().await);
}

/// 演示限流器
async fn demonstrate_rate_limiter() {
    use std::sync::Arc;
    let limiter = Arc::new(RateLimiter::new(2));

    let mut handles = Vec::new();
    for i in 0..5 {
        let limiter_clone = Arc::clone(&limiter);
        let handle = tokio::spawn(async move {
            let _permit = limiter_clone.acquire().await;
            println!("任务 {} 开始执行", i);
            delay(100).await;
            println!("任务 {} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

/// 演示异步缓存
async fn demonstrate_async_cache() {
    let cache: AsyncCache<String, i32> = AsyncCache::new();

    cache.insert("apple".to_string(), 1).await;
    cache.insert("banana".to_string(), 2).await;
    cache.insert("cherry".to_string(), 3).await;

    println!("缓存大小: {}", cache.len().await);
    println!("apple: {:?}", cache.get(&"apple".to_string()).await);
    println!("banana: {:?}", cache.get(&"banana".to_string()).await);
}

/// 演示 HTTP 客户端
async fn demonstrate_http_client() {
    let client = AsyncClient::new();

    // 尝试获取状态码
    match client.get_status("https://httpbin.org/status/200").await {
        Ok(status) => println!("HTTP 状态码: {}", status),
        Err(e) => println!("请求失败: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_async() {
        let result = simple_task(1, 10).await;
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_concurrent() {
        let results = run_concurrent_tasks(3).await;
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_join() {
        let (r1, r2, r3) = run_with_join().await;
        assert_eq!(r1, 4);
        assert_eq!(r2, 9);
        assert_eq!(r3, 16);
    }

    #[tokio::test]
    async fn test_delay() {
        let start = std::time::Instant::now();
        delay(100).await;
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 100);
    }

    #[tokio::test]
    async fn test_async_cache() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        cache.insert("key".to_string(), 42).await;
        assert_eq!(cache.get(&"key".to_string()).await, Some(42));
    }
}

