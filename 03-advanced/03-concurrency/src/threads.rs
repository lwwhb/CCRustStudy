/// 线程基础操作
///
/// 演示线程创建、join、sleep 等基本操作

use std::thread;
use std::time::Duration;

/// 创建简单线程
pub fn create_simple_thread() {
    println!("创建简单线程：");

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("  子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..3 {
        println!("  主线程: {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
    println!("  线程已结束");
}

/// 线程间传递数据
pub fn thread_with_move() {
    println!("\n线程间传递数据：");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("  子线程中的向量: {:?}", v);
    });

    handle.join().unwrap();
}

/// 多个线程
pub fn multiple_threads() {
    println!("\n创建多个线程：");

    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("  线程 {} 开始", i);
            thread::sleep(Duration::from_millis(100));
            println!("  线程 {} 结束", i);
            i * 2
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("  线程返回值: {}", result);
    }
}

/// 线程构建器
pub fn thread_builder() {
    println!("\n使用线程构建器：");

    let builder = thread::Builder::new()
        .name("worker-thread".into())
        .stack_size(4 * 1024 * 1024);

    let handle = builder.spawn(|| {
        println!("  线程名称: {:?}", thread::current().name());
        println!("  线程 ID: {:?}", thread::current().id());
    }).unwrap();

    handle.join().unwrap();
}

/// 线程局部存储
thread_local! {
    static COUNTER: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

pub fn thread_local_storage() {
    println!("\n线程局部存储：");

    let mut handles = vec![];

    for i in 0..3 {
        let handle = thread::spawn(move || {
            COUNTER.with(|c| {
                *c.borrow_mut() = i * 10;
                println!("  线程 {} 的计数器: {}", i, *c.borrow());
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 线程 panic 处理
pub fn thread_panic_handling() {
    println!("\n线程 panic 处理：");

    let handle = thread::spawn(|| {
        panic!("子线程 panic!");
    });

    match handle.join() {
        Ok(_) => println!("  线程正常结束"),
        Err(e) => println!("  线程 panic: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_simple_thread() {
        create_simple_thread();
    }

    #[test]
    fn test_thread_with_move() {
        thread_with_move();
    }

    #[test]
    fn test_multiple_threads() {
        multiple_threads();
    }

    #[test]
    fn test_thread_builder() {
        thread_builder();
    }

    #[test]
    fn test_thread_local_storage() {
        thread_local_storage();
    }

    #[test]
    fn test_thread_panic_handling() {
        thread_panic_handling();
    }

    #[test]
    fn test_thread_return_value() {
        let handle = thread::spawn(|| {
            42
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }
}
