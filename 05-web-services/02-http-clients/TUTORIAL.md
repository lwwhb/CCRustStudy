# 模块 5.2：异步 HTTP 客户端 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 掌握 reqwest 库的使用
2. 处理 JSON 请求和响应
3. 实现并发 HTTP 请求
4. 学习错误处理和重试机制
5. 管理超时和连接池

## 🎯 为什么需要 HTTP 客户端？

### HTTP 客户端的应用场景

**常见需求**：
```
- 调用第三方 API
- 微服务间通信
- 数据抓取和爬虫
- Webhook 调用
- 服务健康检查
```

**reqwest 的优势**：
```
其他语言：
Python requests: 简单但同步
Node.js axios: 异步但 JS 生态

Rust reqwest:
- 异步高性能
- 类型安全
- 零成本抽象
- 完整的 HTTP/2 支持
```

## 📖 核心概念详解

### 1. 基础 HTTP 请求

#### GET 请求

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // 最简单的 GET 请求
    let response = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;

    println!("Response: {}", response);
    Ok(())
}
```

**请求流程**：
```
1. reqwest::get() → 创建请求
2. .await → 发送请求，等待响应
3. .text() → 读取响应体
4. .await → 等待读取完成
```

#### 使用 Client

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // 创建可复用的客户端
    let client = Client::new();

    // 发送请求
    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    let body = response.text().await?;
    println!("Body: {}", body);

    Ok(())
}
```

**为什么使用 Client？**
```
reqwest::get():
- 每次创建新连接
- 适合单次请求

Client:
- 复用连接（连接池）
- 共享配置
- 更高性能
```

### 2. JSON 处理

#### 反序列化响应

```rust
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user: User = reqwest::get("https://api.example.com/user/1")
        .await?
        .json()
        .await?;

    println!("User: {:?}", user);
    Ok(())
}
```

#### 序列化请求

```rust
use serde::Serialize;

#[derive(Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_user = CreateUser {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.example.com/users")
        .json(&new_user)  // 自动序列化为 JSON
        .send()
        .await?;

    println!("Status: {}", response.status());
    Ok(())
}
```

### 3. HTTP 方法

#### GET、POST、PUT、DELETE

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    // GET
    let response = client
        .get("https://api.example.com/users")
        .send()
        .await?;

    // POST
    let response = client
        .post("https://api.example.com/users")
        .json(&new_user)
        .send()
        .await?;

    // PUT
    let response = client
        .put("https://api.example.com/users/1")
        .json(&updated_user)
        .send()
        .await?;

    // DELETE
    let response = client
        .delete("https://api.example.com/users/1")
        .send()
        .await?;

    Ok(())
}
```

#### 自定义请求头

```rust
let response = client
    .get("https://api.example.com/data")
    .header("Authorization", "Bearer token123")
    .header("User-Agent", "MyApp/1.0")
    .send()
    .await?;
```

#### 查询参数

```rust
// 方式 1：URL 中包含
let response = client
    .get("https://api.example.com/search?q=rust&limit=10")
    .send()
    .await?;

// 方式 2：使用 query 方法
let response = client
    .get("https://api.example.com/search")
    .query(&[("q", "rust"), ("limit", "10")])
    .send()
    .await?;

// 方式 3：使用结构体
#[derive(Serialize)]
struct SearchParams {
    q: String,
    limit: u32,
}

let params = SearchParams {
    q: "rust".to_string(),
    limit: 10,
};

let response = client
    .get("https://api.example.com/search")
    .query(&params)
    .send()
    .await?;
```

### 4. 并发请求

#### 使用 tokio::join!

```rust
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 并发执行 3 个请求
    let (resp1, resp2, resp3) = join!(
        reqwest::get("https://api.example.com/1"),
        reqwest::get("https://api.example.com/2"),
        reqwest::get("https://api.example.com/3"),
    );

    let body1 = resp1?.text().await?;
    let body2 = resp2?.text().await?;
    let body3 = resp3?.text().await?;

    println!("Got {} responses", 3);
    Ok(())
}
```

**性能对比**：
```
串行执行：
请求 1: [====] 1s
请求 2:       [====] 1s
请求 3:             [====] 1s
总计: 3s

并发执行：
请求 1: [====] 1s
请求 2: [====] 1s
请求 3: [====] 1s
总计: 1s
```

#### 使用 tokio::spawn

```rust
async fn fetch_multiple(urls: Vec<String>) -> Vec<Result<String, String>> {
    let mut handles = vec![];

    for url in urls {
        let handle = tokio::spawn(async move {
            reqwest::get(&url)
                .await
                .map_err(|e| e.to_string())?
                .text()
                .await
                .map_err(|e| e.to_string())
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => results.push(Err(e.to_string())),
        }
    }

    results
}
```

### 5. 超时控制

#### 全局超时

```rust
use std::time::Duration;

let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()?;

// 所有请求都有 10 秒超时
let response = client.get("https://slow-api.com").send().await?;
```

#### 单个请求超时

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(5),
    reqwest::get("https://slow-api.com")
).await;

match result {
    Ok(Ok(response)) => {
        println!("Got response");
    }
    Ok(Err(e)) => {
        println!("Request error: {}", e);
    }
    Err(_) => {
        println!("Timeout!");
    }
}
```

### 6. 错误处理和重试

#### 基础错误处理

```rust
match reqwest::get("https://api.example.com/data").await {
    Ok(response) => {
        if response.status().is_success() {
            let body = response.text().await?;
            println!("Success: {}", body);
        } else {
            println!("HTTP error: {}", response.status());
        }
    }
    Err(e) => {
        if e.is_timeout() {
            println!("Request timeout");
        } else if e.is_connect() {
            println!("Connection error");
        } else {
            println!("Other error: {}", e);
        }
    }
}
```

#### 重试机制

```rust
async fn fetch_with_retry(
    url: &str,
    max_retries: u32,
) -> Result<String, reqwest::Error> {
    let mut retries = 0;

    loop {
        match reqwest::get(url).await {
            Ok(response) => {
                return response.text().await;
            }
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }

                println!("Retry {}/{}", retries, max_retries);

                // 指数退避
                let delay = Duration::from_millis(100 * 2_u64.pow(retries));
                tokio::time::sleep(delay).await;
            }
        }
    }
}
```

**指数退避**：
```
重试 1: 等待 200ms
重试 2: 等待 400ms
重试 3: 等待 800ms
重试 4: 等待 1600ms
```

### 7. 连接池管理

```rust
let client = reqwest::Client::builder()
    .pool_max_idle_per_host(10)  // 每个主机最多 10 个空闲连接
    .pool_idle_timeout(Duration::from_secs(90))  // 空闲连接超时
    .build()?;
```

**连接池的好处**：
```
没有连接池：
请求 1: [建立连接][发送][接收][关闭]
请求 2: [建立连接][发送][接收][关闭]
请求 3: [建立连接][发送][接收][关闭]

有连接池：
请求 1: [建立连接][发送][接收]
请求 2: [复用连接][发送][接收]
请求 3: [复用连接][发送][接收]

节省时间和资源！
```

## 💻 实战项目：API 聚合器

### 项目需求

构建一个聚合多个 API 的服务：
1. 并发调用多个 API
2. 合并响应数据
3. 错误处理和降级
4. 超时控制
5. 重试机制

### 实现示例

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio;

#[derive(Debug, Deserialize)]
struct WeatherData {
    temperature: f64,
    condition: String,
}

#[derive(Debug, Deserialize)]
struct NewsData {
    title: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct AggregatedData {
    weather: Option<WeatherData>,
    news: Vec<NewsData>,
    timestamp: u64,
}

struct ApiAggregator {
    client: Client,
}

impl ApiAggregator {
    fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .pool_max_idle_per_host(10)
            .build()
            .unwrap();

        Self { client }
    }

    async fn fetch_weather(&self) -> Option<WeatherData> {
        match self.client
            .get("https://api.weather.com/current")
            .send()
            .await
        {
            Ok(resp) => resp.json().await.ok(),
            Err(_) => None,
        }
    }

    async fn fetch_news(&self) -> Vec<NewsData> {
        match self.client
            .get("https://api.news.com/top")
            .send()
            .await
        {
            Ok(resp) => resp.json().await.unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    async fn aggregate(&self) -> AggregatedData {
        // 并发获取数据
        let (weather, news) = tokio::join!(
            self.fetch_weather(),
            self.fetch_news(),
        );

        AggregatedData {
            weather,
            news,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
```

## 🔍 最佳实践

### 1. 复用 Client

```rust
// ❌ 不好：每次创建新 Client
async fn bad_example() {
    for _ in 0..100 {
        let client = Client::new();
        client.get("https://api.com").send().await;
    }
}

// ✅ 好：复用 Client
async fn good_example() {
    let client = Client::new();
    for _ in 0..100 {
        client.get("https://api.com").send().await;
    }
}
```

### 2. 设置合理的超时

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(30))  // 总超时
    .connect_timeout(Duration::from_secs(10))  // 连接超时
    .build()?;
```

### 3. 处理所有错误情况

```rust
match client.get(url).send().await {
    Ok(response) => {
        match response.status() {
            status if status.is_success() => {
                // 处理成功
            }
            status if status.is_client_error() => {
                // 4xx 错误
            }
            status if status.is_server_error() => {
                // 5xx 错误
            }
            _ => {}
        }
    }
    Err(e) => {
        // 网络错误
    }
}
```

### 4. 限制并发数

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

async fn fetch_with_limit(urls: Vec<String>, max_concurrent: usize) {
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut handles = vec![];

    for url in urls {
        let sem = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            reqwest::get(&url).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
```

## 📝 练习题

### 练习 1：实现带缓存的 HTTP 客户端

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

struct CachedClient {
    client: Client,
    cache: Arc<Mutex<HashMap<String, String>>>,
}

impl CachedClient {
    async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        // 检查缓存
        // 如果没有，发起请求并缓存
        // 你的代码
        todo!()
    }
}
```

### 练习 2：实现批量请求

```rust
async fn batch_fetch(urls: Vec<String>, batch_size: usize) -> Vec<Result<String, String>> {
    // 将 URLs 分批
    // 每批并发执行
    // 你的代码
    todo!()
}
```

## ✅ 检查清单

- [ ] 理解 reqwest 的基本用法
- [ ] 掌握 JSON 序列化和反序列化
- [ ] 实现并发 HTTP 请求
- [ ] 处理超时和错误
- [ ] 实现重试机制
- [ ] 管理连接池
- [ ] 完成练习题

---

**掌握 HTTP 客户端，构建强大的网络应用！** 🚀
