# 模块 2.4：异步编程 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解异步编程的概念和优势
2. 掌握 async/await 语法
3. 学习 Tokio 运行时
4. 实现异步 HTTP 客户端
5. 处理并发和错误

## 🎯 为什么需要异步编程？

### 同步 vs 异步

**同步编程（阻塞）**：
```rust
// 同步代码 - 每个操作都会阻塞
fn fetch_data() {
    let data1 = download("url1");  // 等待 1 秒
    let data2 = download("url2");  // 等待 1 秒
    let data3 = download("url3");  // 等待 1 秒
    // 总共需要 3 秒
}
```

**异步编程（非阻塞）**：
```rust
// 异步代码 - 并发执行
async fn fetch_data() {
    let (data1, data2, data3) = tokio::join!(
        download("url1"),  // 同时开始
        download("url2"),  // 同时开始
        download("url3"),  // 同时开始
    );
    // 总共只需要 1 秒！
}
```

### 性能对比

```
同步模型：
线程 1: [====下载1====][====下载2====][====下载3====]
时间:    0s    1s    2s    3s

异步模型：
任务 1: [====下载1====]
任务 2: [====下载2====]
任务 3: [====下载3====]
时间:    0s    1s
```

## 📖 核心概念详解

### 1. Future Trait

Future 是异步计算的核心：

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Future 的定义（简化版）
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Self::Output>;
}

// Poll 的两种状态
enum Poll<T> {
    Ready(T),      // 完成，返回结果
    Pending,       // 未完成，稍后再试
}
```

**工作原理**：
```
1. 创建 Future
2. 运行时调用 poll()
3. 如果 Ready -> 返回结果
4. 如果 Pending -> 等待唤醒
5. 被唤醒后再次 poll()
6. 重复直到 Ready
```

### 2. async/await 语法

**async 函数**：
```rust
// 普通函数
fn sync_function() -> String {
    "hello".to_string()
}

// 异步函数
async fn async_function() -> String {
    "hello".to_string()
}

// async 函数实际返回 Future
// async fn foo() -> T  等价于  fn foo() -> impl Future<Output = T>
```

**await 关键字**：
```rust
async fn example() {
    // await 等待 Future 完成
    let result = async_function().await;

    // 可以链式调用
    let data = fetch_data()
        .await
        .process()
        .await;
}
```

### 3. Tokio 运行时

Tokio 是 Rust 最流行的异步运行时：

```rust
use tokio;

// 方式 1: 使用 #[tokio::main] 宏
#[tokio::main]
async fn main() {
    println!("Hello from async!");
}

// 方式 2: 手动创建运行时
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        println!("Hello from async!");
    });
}
```

**运行时的工作**：
1. 调度异步任务
2. 管理线程池
3. 处理 I/O 事件
4. 唤醒等待的 Future

### 4. 任务（Task）

任务是异步执行的单元：

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // 生成新任务（类似线程）
    let handle = task::spawn(async {
        // 这里的代码在后台运行
        println!("Running in background");
        42
    });

    // 等待任务完成
    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
```

**任务 vs 线程**：
```
线程：
- 操作系统级别
- 重量级（MB 级内存）
- 上下文切换昂贵
- 数量有限（通常几千个）

任务：
- 运行时级别
- 轻量级（KB 级内存）
- 切换成本低
- 可以有数百万个
```

### 5. 并发模式

**并发执行多个任务**：

```rust
use tokio;

#[tokio::main]
async fn main() {
    // 方式 1: join! - 等待所有任务完成
    let (r1, r2, r3) = tokio::join!(
        task1(),
        task2(),
        task3(),
    );

    // 方式 2: select! - 等待第一个完成
    tokio::select! {
        r1 = task1() => println!("Task 1 finished first: {:?}", r1),
        r2 = task2() => println!("Task 2 finished first: {:?}", r2),
    }

    // 方式 3: spawn - 后台运行
    let handle1 = tokio::spawn(task1());
    let handle2 = tokio::spawn(task2());

    let r1 = handle1.await.unwrap();
    let r2 = handle2.await.unwrap();
}
```

### 6. 错误处理

异步代码中的错误处理：

```rust
use tokio;

async fn may_fail() -> Result<String, Box<dyn std::error::Error>> {
    // 使用 ? 操作符传播错误
    let data = fetch_data().await?;
    let processed = process(data).await?;
    Ok(processed)
}

#[tokio::main]
async fn main() {
    match may_fail().await {
        Ok(result) => println!("Success: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## 💻 实战项目：异步 HTTP 客户端

### 项目需求

构建一个异步 HTTP 客户端，支持：
1. 发送 GET/POST 请求
2. 并发请求多个 URL
3. 超时控制
4. 错误处理和重试
5. 响应解析

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### 步骤 2：基础 HTTP 客户端

```rust
use reqwest;
use serde::{Deserialize, Serialize};

// 定义响应数据结构
#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    data: serde_json::Value,
}

// 基础 GET 请求
async fn get_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

// 使用示例
#[tokio::main]
async fn main() {
    match get_request("https://api.example.com/data").await {
        Ok(body) => println!("Response: {}", body),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**关键点**：
- `reqwest::get()` 返回 Future
- `.await` 等待请求完成
- `?` 传播错误

### 步骤 3：带超时的请求

```rust
use tokio::time::{timeout, Duration};

async fn get_with_timeout(
    url: &str,
    timeout_secs: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    // 设置超时
    let result = timeout(
        Duration::from_secs(timeout_secs),
        reqwest::get(url)
    ).await??;  // 注意两个 ?

    let body = result.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    match get_with_timeout("https://slow-api.com", 5).await {
        Ok(body) => println!("Got response"),
        Err(e) => eprintln!("Timeout or error: {}", e),
    }
}
```

**超时处理**：
```
时间线：
0s -------- 请求开始
1s
2s
3s
4s
5s -------- 超时！返回错误
```

### 步骤 4：并发请求

```rust
use tokio;

async fn fetch_multiple(urls: Vec<&str>) -> Vec<Result<String, reqwest::Error>> {
    // 创建所有请求的 Future
    let futures: Vec<_> = urls
        .into_iter()
        .map(|url| async move {
            reqwest::get(url).await?.text().await
        })
        .collect();

    // 并发执行所有请求
    futures::future::join_all(futures).await
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://api1.com/data",
        "https://api2.com/data",
        "https://api3.com/data",
    ];

    let results = fetch_multiple(urls).await;

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(body) => println!("URL {}: {} bytes", i, body.len()),
            Err(e) => eprintln!("URL {}: Error - {}", i, e),
        }
    }
}
```

**并发执行图**：
```
请求 1: [========]
请求 2:   [=======]
请求 3:     [======]
时间:    0s  1s  2s  3s

而不是：
请求 1: [========]
请求 2:          [=======]
请求 3:                   [======]
```

### 步骤 5：完整的 HTTP 客户端

```rust
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

pub struct HttpClient {
    client: Client,
    default_timeout: Duration,
}

impl HttpClient {
    pub fn new(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .unwrap();

        Self {
            client,
            default_timeout: Duration::from_secs(timeout_secs),
        }
    }

    // GET 请求
    pub async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    // POST 请求（JSON）
    pub async fn post_json<T: Serialize>(
        &self,
        url: &str,
        data: &T,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.client
            .post(url)
            .json(data)
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }

    // 带重试的请求
    pub async fn get_with_retry(
        &self,
        url: &str,
        max_retries: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut attempts = 0;

        loop {
            match self.get(url).await {
                Ok(body) => return Ok(body),
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_retries {
                        return Err(e);
                    }

                    // 指数退避
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempts));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    // 并发请求多个 URL
    pub async fn get_all(
        &self,
        urls: Vec<String>,
    ) -> Vec<Result<String, Box<dyn std::error::Error>>> {
        let futures: Vec<_> = urls
            .into_iter()
            .map(|url| async move {
                self.get(&url).await
            })
            .collect();

        futures::future::join_all(futures).await
    }
}
```

### 步骤 6：使用示例

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new(10);

    // 示例 1: 简单 GET 请求
    println!("=== 示例 1: GET 请求 ===");
    let body = client.get("https://httpbin.org/get").await?;
    println!("Response: {}", body);

    // 示例 2: POST 请求
    println!("\n=== 示例 2: POST 请求 ===");
    #[derive(Serialize)]
    struct PostData {
        name: String,
        value: i32,
    }

    let data = PostData {
        name: "test".to_string(),
        value: 42,
    };

    let response = client.post_json("https://httpbin.org/post", &data).await?;
    println!("Response: {}", response);

    // 示例 3: 带重试
    println!("\n=== 示例 3: 带重试 ===");
    let body = client.get_with_retry("https://httpbin.org/get", 3).await?;
    println!("Got response after retries");

    // 示例 4: 并发请求
    println!("\n=== 示例 4: 并发请求 ===");
    let urls = vec![
        "https://httpbin.org/delay/1".to_string(),
        "https://httpbin.org/delay/2".to_string(),
        "https://httpbin.org/delay/1".to_string(),
    ];

    let start = std::time::Instant::now();
    let results = client.get_all(urls).await;
    let elapsed = start.elapsed();

    println!("完成 {} 个请求，耗时: {:?}", results.len(), elapsed);
    println!("（如果是串行，需要 4 秒；并发只需 2 秒）");

    Ok(())
}
```

## 🔍 深入理解

### 异步运行时的工作原理

```
1. 任务队列
   ┌─────────┐
   │ Task 1  │
   │ Task 2  │
   │ Task 3  │
   └─────────┘

2. 工作线程池
   Thread 1: [执行 Task 1]
   Thread 2: [执行 Task 2]
   Thread 3: [空闲]

3. I/O 事件循环
   监听: 网络、文件、定时器
   就绪时唤醒对应的 Task

4. 调度器
   - 选择下一个要执行的 Task
   - 在线程间分配工作
   - 处理任务完成
```

### 常见陷阱

**陷阱 1: 忘记 .await**
```rust
// 错误：Future 不会执行
async fn wrong() {
    fetch_data();  // 只创建 Future，不执行！
}

// 正确
async fn correct() {
    fetch_data().await;  // 执行并等待
}
```

**陷阱 2: 阻塞运行时**
```rust
// 错误：阻塞整个运行时
async fn wrong() {
    std::thread::sleep(Duration::from_secs(1));  // 阻塞！
}

// 正确：使用异步 sleep
async fn correct() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

**陷阱 3: 过度并发**
```rust
// 错误：可能创建太多连接
async fn wrong(urls: Vec<String>) {
    for url in urls {
        tokio::spawn(fetch(url));  // 可能数千个并发！
    }
}

// 正确：限制并发数
use tokio::sync::Semaphore;

async fn correct(urls: Vec<String>) {
    let semaphore = Arc::new(Semaphore::new(10));  // 最多 10 个并发

    let futures: Vec<_> = urls.into_iter().map(|url| {
        let sem = semaphore.clone();
        async move {
            let _permit = sem.acquire().await.unwrap();
            fetch(&url).await
        }
    }).collect();

    futures::future::join_all(futures).await;
}
```

## 📝 练习题

### 练习 1: 基础异步函数
编写一个异步函数，模拟下载文件：
```rust
async fn download_file(url: &str) -> Result<Vec<u8>, String> {
    // 模拟网络延迟
    tokio::time::sleep(Duration::from_secs(1)).await;

    // 返回模拟数据
    Ok(vec![1, 2, 3, 4, 5])
}
```

### 练习 2: 并发下载
实现一个函数，并发下载多个文件：
```rust
async fn download_all(urls: Vec<&str>) -> Vec<Result<Vec<u8>, String>> {
    // 你的代码
}
```

### 练习 3: 超时和重试
实现带超时和重试的下载函数：
```rust
async fn download_with_retry(
    url: &str,
    timeout_secs: u64,
    max_retries: u32,
) -> Result<Vec<u8>, String> {
    // 你的代码
}
```

### 练习 4: 进度追踪
实现一个能报告进度的并发下载器：
```rust
async fn download_with_progress(
    urls: Vec<&str>,
    progress_callback: impl Fn(usize, usize),
) -> Vec<Result<Vec<u8>, String>> {
    // 你的代码
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解 Future 和 async/await 的工作原理
- [ ] 使用 Tokio 运行时
- [ ] 编写异步函数
- [ ] 使用 tokio::spawn 创建任务
- [ ] 使用 tokio::join! 并发执行
- [ ] 使用 tokio::select! 选择第一个完成的任务
- [ ] 处理异步代码中的错误
- [ ] 实现超时控制
- [ ] 实现重试逻辑
- [ ] 限制并发数量
- [ ] 理解阻塞 vs 非阻塞操作

## 🔗 延伸阅读

- [Tokio 官方教程](https://tokio.rs/tokio/tutorial)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Reqwest 文档](https://docs.rs/reqwest/)
- [Futures 库](https://docs.rs/futures/)

## 🚀 下一步

完成本模块后，你可以：
1. 继续学习模块 2.5（序列化）
2. 跳到阶段 5（Web 服务）深入异步 Web 开发
3. 学习模块 3.3（并发编程）了解多线程

---

**掌握异步编程，开启高性能 Rust 之旅！** 🚀
