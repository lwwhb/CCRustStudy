# 模块 2.4：异步编程基础

## 🎯 学习目标

- 理解 async/await 语法
- 掌握 Future trait
- 使用 Tokio 运行时
- 实现异步任务和并发
- 处理异步错误

## 📚 核心概念

### 1. async/await 基础

```rust
async fn fetch_data() -> String {
    // 异步操作
    "data".to_string()
}

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{}", result);
}
```

### 2. Future trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42)
    }
}
```

### 3. Tokio 运行时

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("开始");
    sleep(Duration::from_secs(1)).await;
    println!("1 秒后");
}
```

### 4. 并发执行

```rust
use tokio::join;

async fn task1() -> i32 { 1 }
async fn task2() -> i32 { 2 }

#[tokio::main]
async fn main() {
    let (r1, r2) = join!(task1(), task2());
    println!("{}, {}", r1, r2);
}
```

### 5. 异步 HTTP 请求

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("{}", body);
    Ok(())
}
```

## 💻 实战项目：异步 HTTP 客户端

构建一个异步 HTTP 客户端，演示异步编程的各种模式。

### 功能需求

1. 异步 HTTP GET/POST 请求
2. 并发请求多个 URL
3. 超时和重试机制
4. 异步文件下载
5. 请求池和限流

### 项目结构

```
async-basics/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── client.rs     # HTTP 客户端
│   ├── tasks.rs      # 异步任务
│   └── utils.rs      # 工具函数
└── README.md
```

## 🧪 练习题

### 练习 1：基本异步函数

```rust
async fn compute(x: i32) -> i32 {
    // 模拟异步计算
    tokio::time::sleep(Duration::from_millis(100)).await;
    x * 2
}
```

### 练习 2：并发执行

```rust
// 并发执行多个任务
async fn run_concurrent() {
    let tasks = vec![task1(), task2(), task3()];
    let results = futures::future::join_all(tasks).await;
}
```

## 📖 深入阅读

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Book](https://rust-lang.github.io/async-book/)
- [reqwest Documentation](https://docs.rs/reqwest)

## ✅ 检查清单

- [ ] 理解 async/await 语法
- [ ] 使用 Tokio 运行时
- [ ] 实现异步函数
- [ ] 并发执行多个任务
- [ ] 处理异步错误
- [ ] 使用 tokio::spawn
- [ ] 理解 Future trait

## 🚀 下一步

完成本模块后，继续学习 [模块 2.5：序列化与反序列化](../05-serde/)。
