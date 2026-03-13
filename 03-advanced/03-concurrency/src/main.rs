mod threads;
mod channels;
mod shared_state;
mod thread_pool;

use threads::*;
use channels::*;
use shared_state::*;
use thread_pool::ThreadPool;

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 并发编程演示 ===\n");

    // 演示 1：线程基础
    println!("=== 演示 1：线程基础 ===");
    create_simple_thread();
    thread_with_move();
    multiple_threads();
    thread_builder();
    thread_local_storage();
    thread_panic_handling();
    println!();

    // 演示 2：消息传递
    println!("=== 演示 2：消息传递 ===");
    simple_channel();
    multiple_messages();
    multiple_producers();
    sync_channel();
    producer_consumer();
    timeout_receive();
    try_receive();
    println!();

    // 演示 3：共享状态
    println!("=== 演示 3：共享状态 ===");
    mutex_example();
    rwlock_example();
    atomic_example();
    atomic_bool_example();
    compare_and_swap();
    avoid_deadlock();
    println!();

    // 演示 4：线程池
    println!("=== 演示 4：线程池 ===");
    demonstrate_thread_pool();
    println!();

    // 演示 5：并发模式
    println!("=== 演示 5：并发模式 ===");
    demonstrate_parallel_computation();
    println!();
}

/// 演示线程池
fn demonstrate_thread_pool() {
    println!("创建线程池（4 个工作线程）：");

    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("  任务 {} 开始执行", i);
            thread::sleep(Duration::from_millis(100));
            println!("  任务 {} 完成", i);
        });
    }

    println!("所有任务已提交，等待完成...");
    thread::sleep(Duration::from_millis(500));
    println!("线程池演示完成");
}

/// 演示并行计算
fn demonstrate_parallel_computation() {
    println!("并行计算数组和：");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = data.len() / 4;

    let mut handles = vec![];

    for i in 0..4 {
        let start = i * chunk_size;
        let end = if i == 3 { data.len() } else { (i + 1) * chunk_size };
        let chunk: Vec<i32> = data[start..end].to_vec();

        let handle = thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            println!("  线程 {} 计算范围 [{}, {}): sum = {}", i, start, end, sum);
            sum
        });

        handles.push(handle);
    }

    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("  总和: {}", total);
    println!("  预期: {}", data.iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool() {
        let pool = ThreadPool::new(4);

        for i in 0..10 {
            pool.execute(move || {
                println!("任务 {}", i);
            });
        }

        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_parallel_sum() {
        use std::sync::{Arc, Mutex};

        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        let chunk_size = data.len() / 2;

        for i in 0..2 {
            let start = i * chunk_size;
            let end = (i + 1) * chunk_size;
            let chunk: Vec<i32> = data[start..end].to_vec();
            let result = Arc::clone(&result);

            let handle = thread::spawn(move || {
                let sum: i32 = chunk.iter().sum();
                let mut r = result.lock().unwrap();
                *r += sum;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*result.lock().unwrap(), 36);
    }
}
