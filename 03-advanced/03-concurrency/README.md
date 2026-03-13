# 模块 3.3：并发编程

## 🎯 学习目标

- 理解 Rust 的并发模型
- 掌握线程创建和管理
- 学习消息传递（channels）
- 理解共享状态并发（Mutex、RwLock）
- 掌握原子操作
- 实现并发模式

## 📚 核心概念

### 1. 线程基础

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

### 2. 消息传递

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}
```

### 3. 共享状态

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
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

    println!("结果: {}", *counter.lock().unwrap());
}
```

### 4. 读写锁

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 多个读者
    let data1 = Arc::clone(&data);
    let reader1 = thread::spawn(move || {
        let r = data1.read().unwrap();
        println!("读者1: {:?}", *r);
    });

    // 写者
    let data2 = Arc::clone(&data);
    let writer = thread::spawn(move || {
        let mut w = data2.write().unwrap();
        w.push(4);
    });

    reader1.join().unwrap();
    writer.join().unwrap();
}
```

### 5. 原子操作

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", counter.load(Ordering::SeqCst));
}
```

## 💻 实战项目：多线程任务调度器

实现一个简单的线程池和任务调度系统。

### 功能需求

1. 线程池管理
2. 任务队列
3. 工作窃取
4. 优雅关闭

### 项目结构

```
concurrency/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── threads.rs      # 线程基础
│   ├── channels.rs     # 消息传递
│   ├── shared_state.rs # 共享状态
│   └── thread_pool.rs  # 线程池
└── README.md
```

## 🧪 练习题

### 练习 1：生产者-消费者

```rust
// 实现一个生产者-消费者模式
// 多个生产者，一个消费者
```

### 练习 2：并行计算

```rust
// 使用多线程计算数组元素的和
// 将数组分成多个部分，并行计算
```

### 练习 3：线程池

```rust
// 实现一个简单的线程池
// 支持提交任务和等待完成
```

## 📖 深入阅读

- [The Rust Book - Chapter 16: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/)
- [Crossbeam Documentation](https://docs.rs/crossbeam/)

## ✅ 检查清单

- [ ] 创建和管理线程
- [ ] 使用 channels 进行消息传递
- [ ] 使用 Mutex 共享状态
- [ ] 使用 RwLock 优化读多写少场景
- [ ] 使用原子操作
- [ ] 实现线程池
- [ ] 理解数据竞争和死锁

## 🚀 下一步

完成本模块后，继续学习 [模块 3.4：性能优化](../04-performance/)。
