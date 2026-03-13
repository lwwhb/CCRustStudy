/// 异步任务管理
///
/// 演示 tokio::spawn 和任务并发

use tokio::time::{sleep, Duration};

/// 简单的异步任务
pub async fn simple_task(id: u32, duration_ms: u64) -> u32 {
    println!("任务 {} 开始", id);
    sleep(Duration::from_millis(duration_ms)).await;
    println!("任务 {} 完成", id);
    id
}

/// 计算密集型任务（模拟）
pub async fn compute_task(x: i32) -> i32 {
    sleep(Duration::from_millis(100)).await;
    x * x
}

/// 并发执行多个任务
pub async fn run_concurrent_tasks(count: u32) -> Vec<u32> {
    let mut handles = Vec::new();

    for i in 0..count {
        let handle = tokio::spawn(async move {
            simple_task(i, 100).await
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

/// 使用 join! 宏并发执行
pub async fn run_with_join() -> (i32, i32, i32) {
    let task1 = compute_task(2);
    let task2 = compute_task(3);
    let task3 = compute_task(4);

    tokio::join!(task1, task2, task3)
}

/// 使用 select! 宏（先完成的任务）
pub async fn run_with_select() -> String {
    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        "任务 1"
    };

    let task2 = async {
        sleep(Duration::from_millis(200)).await;
        "任务 2"
    };

    tokio::select! {
        result = task1 => format!("{} 先完成", result),
        result = task2 => format!("{} 先完成", result),
    }
}

/// 超时控制
pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Result<T, &'static str>
where
    F: std::future::Future<Output = T>,
{
    match tokio::time::timeout(Duration::from_millis(timeout_ms), future).await {
        Ok(result) => Ok(result),
        Err(_) => Err("超时"),
    }
}

/// 异步计数器
pub struct AsyncCounter {
    count: tokio::sync::Mutex<i32>,
}

impl AsyncCounter {
    pub fn new() -> Self {
        AsyncCounter {
            count: tokio::sync::Mutex::new(0),
        }
    }

    pub async fn increment(&self) {
        let mut count = self.count.lock().await;
        *count += 1;
    }

    pub async fn get(&self) -> i32 {
        let count = self.count.lock().await;
        *count
    }
}

impl Default for AsyncCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_task() {
        let result = simple_task(1, 10).await;
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_compute_task() {
        let result = compute_task(5).await;
        assert_eq!(result, 25);
    }

    #[tokio::test]
    async fn test_concurrent_tasks() {
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
    async fn test_select() {
        let result = run_with_select().await;
        assert!(result.contains("任务"));
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let task = async {
            sleep(Duration::from_millis(10)).await;
            42
        };

        let result = with_timeout(task, 100).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_timeout_failure() {
        let task = async {
            sleep(Duration::from_millis(200)).await;
            42
        };

        let result = with_timeout(task, 50).await;
        assert_eq!(result, Err("超时"));
    }

    #[tokio::test]
    async fn test_async_counter() {
        use std::sync::Arc;
        let counter = Arc::new(AsyncCounter::new());

        let mut handles = Vec::new();
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                counter_clone.increment().await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(counter.get().await, 10);
    }
}
