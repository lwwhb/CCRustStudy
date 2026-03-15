# 模块 3.3：并发编程 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 Rust 的并发模型
2. 掌握线程创建和管理
3. 学习消息传递（channels）
4. 理解共享状态并发
5. 掌握原子操作
6. 实现线程池

## 🎯 为什么需要并发？

### 并发的价值

**串行执行的问题**：
```rust
// 处理 4 个任务，每个需要 1 秒
task1();  // 1 秒
task2();  // 1 秒
task3();  // 1 秒
task4();  // 1 秒
// 总共需要 4 秒
```

**并发执行**：
```rust
// 4 个任务同时执行
thread::spawn(|| task1());
thread::spawn(|| task2());
thread::spawn(|| task3());
thread::spawn(|| task4());
// 总共只需要 1 秒！
```

### 并发 vs 并行

```
并发（Concurrency）：
- 多个任务交替执行
- 单核 CPU 也可以
- 关注任务结构

CPU: [Task1][Task2][Task1][Task3][Task2]...

并行（Parallelism）：
- 多个任务同时执行
- 需要多核 CPU
- 关注执行效率

CPU1: [Task1][Task1][Task1]...
CPU2: [Task2][Task2][Task2]...
CPU3: [Task3][Task3][Task3]...
```

### Rust 并发的优势

**其他语言的问题**：
```
C/C++:
- 数据竞争（data race）
- 悬垂指针
- 难以调试

Java:
- 需要手动同步
- 容易死锁
- 性能开销

Python:
- GIL（全局解释器锁）
- 真正的并行受限
```

**Rust 的优势**：
```rust
// 编译时防止数据竞争
let data = vec![1, 2, 3];

thread::spawn(|| {
    println!("{:?}", data);  // 错误！data 没有 move
});

// 必须显式转移所有权
thread::spawn(move || {
    println!("{:?}", data);  // OK
});
```

## 📖 核心概念详解

### 1. 线程基础

线程是操作系统级别的并发单元。

#### 创建线程

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 创建新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程继续执行
    for i in 1..5 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待子线程完成
    handle.join().unwrap();
}
```

**执行流程**：
```
时间 →
主线程: [1][2][3][4]--------[等待]
子线程: ---[1][2][3][4][5][6][7][8][9]
```

#### 线程与所有权

```rust
let v = vec![1, 2, 3];

// 错误：闭包可能比 v 活得更久
// thread::spawn(|| {
//     println!("{:?}", v);
// });

// 正确：使用 move 转移所有权
thread::spawn(move || {
    println!("{:?}", v);
});

// v 已失效
// println!("{:?}", v);  // 错误！
```

#### 线程返回值

```rust
let handle = thread::spawn(|| {
    // 计算并返回结果
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    sum
});

// 获取返回值
let result = handle.join().unwrap();
println!("结果: {}", result);  // 5050
```

#### 线程构建器

```rust
use std::thread;

let builder = thread::Builder::new()
    .name("worker-1".to_string())
    .stack_size(4 * 1024 * 1024);  // 4MB 栈

let handle = builder.spawn(|| {
    println!("线程名: {:?}", thread::current().name());
}).unwrap();

handle.join().unwrap();
```

### 2. 消息传递

使用 channels 在线程间传递消息。

#### 基础 Channel

```rust
use std::sync::mpsc;  // multiple producer, single consumer
use std::thread;

fn main() {
    // 创建 channel
    let (tx, rx) = mpsc::channel();

    // 发送者在新线程中
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        // val 已被移动，不能再使用
    });

    // 接收者在主线程中
    let received = rx.recv().unwrap();
    println!("收到: {}", received);
}
```

**Channel 工作原理**：
```
发送线程                接收线程
   ↓                       ↑
[send] → [Channel] → [recv]
         (队列)
```

#### 发送多个消息

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
});

// 接收所有消息
for received in rx {
    println!("收到: {}", received);
}
```

#### 多个生产者

```rust
let (tx, rx) = mpsc::channel();

// 克隆发送者
let tx1 = tx.clone();

thread::spawn(move || {
    tx.send("消息 1".to_string()).unwrap();
});

thread::spawn(move || {
    tx1.send("消息 2".to_string()).unwrap();
});

// 接收两个消息
for received in rx {
    println!("收到: {}", received);
}
```

#### 同步 Channel

```rust
use std::sync::mpsc;

// 创建容量为 0 的同步 channel
let (tx, rx) = mpsc::sync_channel(0);

thread::spawn(move || {
    println!("发送前");
    tx.send(42).unwrap();  // 阻塞，直到接收者接收
    println!("发送后");
});

thread::sleep(Duration::from_secs(1));
println!("接收: {}", rx.recv().unwrap());
```

**同步 vs 异步**：
```
异步 channel (mpsc::channel):
- 无限容量
- send() 不阻塞
- 适合生产者快于消费者

同步 channel (mpsc::sync_channel):
- 有限容量
- send() 可能阻塞
- 提供背压（backpressure）
```

### 3. 共享状态并发

使用 Mutex 和 RwLock 共享数据。

#### Mutex（互斥锁）

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: 原子引用计数，线程安全
    // Mutex: 互斥锁，保护数据
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取锁
            let mut num = counter.lock().unwrap();
            *num += 1;
            // 锁在 num 离开作用域时自动释放
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", *counter.lock().unwrap());  // 10
}
```

**Mutex 工作原理**：
```
线程 1: [等待锁] → [获取锁] → [修改数据] → [释放锁]
线程 2: [等待锁] --------→ [获取锁] → [修改数据] → [释放锁]
线程 3: [等待锁] -------------------→ [获取锁] → [修改数据]
```

**为什么需要 Arc？**
```rust
// Rc 不是线程安全的
let counter = Rc::new(Mutex::new(0));
// thread::spawn(move || { ... });  // 错误！

// Arc 是线程安全的
let counter = Arc::new(Mutex::new(0));
thread::spawn(move || { ... });  // OK
```

#### RwLock（读写锁）

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 多个读者
    let mut handles = vec![];
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let r = data.read().unwrap();
            println!("读者 {}: {:?}", i, *r);
        });
        handles.push(handle);
    }

    // 一个写者
    let data_writer = Arc::clone(&data);
    let writer = thread::spawn(move || {
        let mut w = data_writer.write().unwrap();
        w.push(4);
        println!("写者: 添加了 4");
    });

    for handle in handles {
        handle.join().unwrap();
    }
    writer.join().unwrap();
}
```

**RwLock 规则**：
```
- 多个读者可以同时持有读锁
- 只有一个写者可以持有写锁
- 读锁和写锁互斥

适用场景：
- 读多写少
- 提高并发性能
```

#### 死锁避免

```rust
// ❌ 可能死锁
let lock1 = Arc::new(Mutex::new(0));
let lock2 = Arc::new(Mutex::new(0));

// 线程 1
let l1 = lock1.clone();
let l2 = lock2.clone();
thread::spawn(move || {
    let _g1 = l1.lock().unwrap();
    let _g2 = l2.lock().unwrap();  // 等待 lock2
});

// 线程 2
thread::spawn(move || {
    let _g2 = lock2.lock().unwrap();
    let _g1 = lock1.lock().unwrap();  // 等待 lock1
});
// 死锁！

// ✅ 避免死锁：统一锁顺序
thread::spawn(move || {
    let _g1 = lock1.lock().unwrap();
    let _g2 = lock2.lock().unwrap();
});

thread::spawn(move || {
    let _g1 = lock1.lock().unwrap();  // 相同顺序
    let _g2 = lock2.lock().unwrap();
});
```

### 4. 原子操作

原子操作是无锁的并发原语。

#### 原子类型

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
            for _ in 0..1000 {
                // 原子递增
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", counter.load(Ordering::SeqCst));  // 10000
}
```

**原子操作 vs Mutex**：
```
Mutex:
- 更灵活（可以保护任何数据）
- 有锁开销
- 可能阻塞

原子操作:
- 只能用于简单类型
- 无锁，更快
- 不会阻塞
```

#### 内存顺序

```rust
use std::sync::atomic::Ordering;

// 最严格，性能最低
Ordering::SeqCst  // 顺序一致性

// 中等
Ordering::Acquire  // 获取语义
Ordering::Release  // 释放语义
Ordering::AcqRel   // 获取-释放

// 最宽松，性能最高
Ordering::Relaxed  // 宽松

// 一般使用 SeqCst，除非你确切知道在做什么
```

### 5. 线程池

线程池复用线程，避免频繁创建销毁。

#### 简单线程池实现

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} 执行任务", id);
            job();
        });

        Worker { id, thread }
    }
}
```

**使用线程池**：
```rust
let pool = ThreadPool::new(4);

for i in 0..8 {
    pool.execute(move || {
        println!("任务 {} 开始", i);
        thread::sleep(Duration::from_millis(100));
        println!("任务 {} 完成", i);
    });
}
```

### 6. 并发模式

#### 生产者-消费者

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // 生产者
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 消费者
    for received in rx {
        println!("处理: {}", received);
    }
}
```

#### 并行计算

```rust
fn parallel_sum(data: Vec<i32>) -> i32 {
    let chunk_size = data.len() / 4;
    let mut handles = vec![];

    for i in 0..4 {
        let start = i * chunk_size;
        let end = if i == 3 { data.len() } else { (i + 1) * chunk_size };
        let chunk = data[start..end].to_vec();

        let handle = thread::spawn(move || {
            chunk.iter().sum::<i32>()
        });

        handles.push(handle);
    }

    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}
```

## 💻 实战项目：多线程任务调度器

### 项目需求

构建一个线程池实现，支持：
1. 固定数量的工作线程
2. 任务队列
3. 优雅关闭
4. 任务优先级

### 实现要点

```rust
// 1. 任务定义
type Job = Box<dyn FnOnce() + Send + 'static>;

// 2. 线程池结构
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// 3. 工作线程
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// 4. 优雅关闭
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

## 🔍 深入理解

### 并发安全的类型

```rust
// Send: 可以在线程间转移所有权
// Sync: 可以在线程间共享引用

// 实现 Send 的类型可以 move 到其他线程
// 实现 Sync 的类型的引用可以在线程间共享

// 大多数类型都是 Send + Sync
// 例外：
// - Rc: 不是 Send 或 Sync
// - RefCell: 不是 Sync
// - 原始指针: 不是 Send 或 Sync
```

### 性能考虑

```rust
// 1. 线程创建开销
// 创建线程很昂贵，使用线程池

// 2. 锁竞争
// 减少锁的持有时间
{
    let mut data = lock.lock().unwrap();
    // 尽快完成操作
    *data += 1;
}  // 立即释放锁

// 3. 原子操作 vs Mutex
// 简单计数器用原子操作
let counter = AtomicUsize::new(0);

// 复杂数据用 Mutex
let data = Mutex::new(HashMap::new());
```

## 📝 常见问题

### 1. 数据竞争

```rust
// Rust 编译器防止数据竞争
let mut data = vec![1, 2, 3];

thread::spawn(|| {
    data.push(4);  // 错误！
});

data.push(5);  // 错误！
```

### 2. 死锁

```rust
// 避免死锁的方法：
// 1. 统一锁顺序
// 2. 使用 try_lock
// 3. 使用超时
// 4. 避免嵌套锁
```

### 3. 线程泄漏

```rust
// 确保 join 所有线程
let handle = thread::spawn(|| { ... });
handle.join().unwrap();  // 不要忘记！
```

## ✅ 检查清单

- [ ] 理解线程创建和管理
- [ ] 掌握消息传递模式
- [ ] 使用 Mutex 和 RwLock
- [ ] 理解原子操作
- [ ] 实现线程池
- [ ] 避免死锁和数据竞争
- [ ] 选择合适的并发模式

## 🔗 延伸阅读

- [The Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/)
- [Crossbeam](https://docs.rs/crossbeam/) - 高级并发工具
- [Rayon](https://docs.rs/rayon/) - 数据并行库

---

**掌握并发编程，释放多核性能！** 🚀
