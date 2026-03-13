# 模块 5.2：异步 HTTP 客户端

## 🎯 学习目标

- 使用 reqwest 发起 HTTP 请求
- 处理 JSON 响应
- 实现并发请求
- 错误处理和重试机制
- 超时和连接池管理

## 📚 核心概念

### 1. 基本 GET 请求

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://api.example.com/data")
        .await?
        .text()
        .await?;

    println!("Response: {}", response);
    Ok(())
}
```

### 2. JSON 处理

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct User {
    id: u64,
    name: String,
}

let user: User = reqwest::get("https://api.example.com/user/1")
    .await?
    .json()
    .await?;
```

### 3. POST 请求

```rust
#[derive(Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

let new_user = CreateUser {
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let client = reqwest::Client::new();
let response = client
    .post("https://api.example.com/users")
    .json(&new_user)
    .send()
    .await?;
```

### 4. 并发请求

```rust
use tokio::join;

let (resp1, resp2, resp3) = join!(
    reqwest::get("https://api.example.com/1"),
    reqwest::get("https://api.example.com/2"),
    reqwest::get("https://api.example.com/3"),
);
```

### 5. 超时设置

```rust
use std::time::Duration;

let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(10))
    .build()?;
```

## 💻 实战项目：API 聚合器

实现一个聚合多个 API 的服务。

### 功能需求

1. 并发调用多个 API
2. 响应合并
3. 错误处理和重试
4. 超时控制
5. 连接池管理

## 🧪 练习题

### 练习 1：实现重试机制

```rust
// 实现一个带重试的 HTTP 请求函数
async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, Error>
```

### 练习 2：并行请求

```rust
// 并行请求多个 URL 并收集结果
async fn fetch_all(urls: Vec<String>) -> Vec<Result<String, Error>>
```

## 📖 深入阅读

- [reqwest Documentation](https://docs.rs/reqwest/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## ✅ 检查清单

- [ ] 发起 GET/POST 请求
- [ ] 处理 JSON 响应
- [ ] 实现并发请求
- [ ] 错误处理
- [ ] 超时控制
- [ ] 重试机制

## 🚀 下一步

完成本模块后，继续学习 [模块 5.3：流式处理与 SSE](../03-streaming/)。
