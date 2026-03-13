/// 共享状态并发
///
/// 演示使用 Mutex、RwLock 和原子操作进行共享状态管理

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// 使用 Mutex 共享状态
pub fn mutex_example() {
    println!("使用 Mutex 共享状态：");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  线程 {} 增加计数器", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终结果: {}", *counter.lock().unwrap());
}

/// 使用 RwLock（读写锁）
pub fn rwlock_example() {
    println!("\n使用 RwLock（读写锁）：");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 多个读者
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let r = data.read().unwrap();
            println!("  读者 {} 读取: {:?}", i, *r);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // 一个写者
    let data_writer = Arc::clone(&data);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut w = data_writer.write().unwrap();
        w.push(4);
        println!("  写者添加元素 4");
    });
    handles.push(writer);

    // 再来一个读者
    let data_reader = Arc::clone(&data);
    let reader = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let r = data_reader.read().unwrap();
        println!("  最后的读者读取: {:?}", *r);
    });
    handles.push(reader);

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 原子操作
pub fn atomic_example() {
    println!("\n原子操作：");

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            println!("  线程 {} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终结果: {}", counter.load(Ordering::SeqCst));
}

/// 原子布尔值
pub fn atomic_bool_example() {
    println!("\n原子布尔值：");

    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        flag_clone.store(true, Ordering::SeqCst);
        println!("  标志已设置为 true");
    });

    while !flag.load(Ordering::SeqCst) {
        println!("  等待标志...");
        thread::sleep(Duration::from_millis(50));
    }

    println!("  标志已设置！");
    handle.join().unwrap();
}

/// 比较并交换（CAS）
pub fn compare_and_swap() {
    println!("\n比较并交换（CAS）：");

    let value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            loop {
                let current = value.load(Ordering::SeqCst);
                if current >= 10 {
                    break;
                }

                match value.compare_exchange(
                    current,
                    current + 1,
                    Ordering::SeqCst,
                    Ordering::SeqCst
                ) {
                    Ok(_) => {
                        println!("  线程 {} 成功增加: {} -> {}", i, current, current + 1);
                        break;
                    }
                    Err(_) => {
                        // 重试
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终值: {}", value.load(Ordering::SeqCst));
}

/// 死锁示例（注意：这会导致死锁）
#[allow(dead_code)]
fn deadlock_example() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);

    let handle1 = thread::spawn(move || {
        let _g1 = lock1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _g2 = lock2_clone.lock().unwrap();
    });

    let handle2 = thread::spawn(move || {
        let _g2 = lock2.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _g1 = lock1.lock().unwrap();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

/// 避免死锁：使用固定顺序获取锁
pub fn avoid_deadlock() {
    println!("\n避免死锁（固定顺序获取锁）：");

    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);

    let handle1 = thread::spawn(move || {
        let _g1 = lock1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _g2 = lock2_clone.lock().unwrap();
        println!("  线程 1 获取了两个锁");
    });

    let lock1_clone2 = Arc::clone(&lock1);
    let lock2_clone2 = Arc::clone(&lock2);

    let handle2 = thread::spawn(move || {
        // 使用相同的顺序获取锁
        let _g1 = lock1_clone2.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _g2 = lock2_clone2.lock().unwrap();
        println!("  线程 2 获取了两个锁");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("  成功避免死锁");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_example() {
        mutex_example();
    }

    #[test]
    fn test_rwlock_example() {
        rwlock_example();
    }

    #[test]
    fn test_atomic_example() {
        atomic_example();
    }

    #[test]
    fn test_atomic_bool_example() {
        atomic_bool_example();
    }

    #[test]
    fn test_compare_and_swap() {
        compare_and_swap();
    }

    #[test]
    fn test_avoid_deadlock() {
        avoid_deadlock();
    }

    #[test]
    fn test_mutex_increment() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..100 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 100);
    }

    #[test]
    fn test_atomic_increment() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..100 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
}
