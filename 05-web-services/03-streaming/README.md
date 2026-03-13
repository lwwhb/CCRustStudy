# 模块 5.3：流式处理与 SSE

## 🎯 学习目标

- 理解流式数据处理
- 实现 Server-Sent Events (SSE)
- 使用 tokio-stream
- 处理背压
- 实现实时数据推送

## 📚 核心概念

### 1. Server-Sent Events (SSE)

```rust
use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use std::time::Duration;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("ping"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream)
}
```

### 2. 流式响应

```rust
use tokio_stream::StreamExt;

let stream = tokio_stream::iter(vec![1, 2, 3, 4, 5])
    .map(|n| format!("Number: {}", n))
    .throttle(Duration::from_millis(100));
```

### 3. 异步迭代器

```rust
use futures::stream::StreamExt;

let mut stream = futures::stream::iter(vec![1, 2, 3]);

while let Some(item) = stream.next().await {
    println!("Item: {}", item);
}
```

## 💻 实战项目：实时数据流服务

实现一个支持 SSE 的实时数据推送服务。

### 功能需求

1. SSE 端点实现
2. 实时数据推送
3. 多客户端支持
4. 背压处理

## 🧪 练习题

### 练习 1：实现进度推送

```rust
// 实现一个推送任务进度的 SSE 端点
```

### 练习 2：聊天室

```rust
// 使用 SSE 实现简单的聊天室
```

## 📖 深入阅读

- [Axum SSE Documentation](https://docs.rs/axum/latest/axum/response/sse/)
- [tokio-stream Documentation](https://docs.rs/tokio-stream/)

## ✅ 检查清单

- [ ] 实现 SSE 端点
- [ ] 流式数据处理
- [ ] 多客户端管理
- [ ] 背压处理
- [ ] 错误处理

## 🚀 下一步

完成本模块后，继续学习 [模块 5.4：数据库集成](../04-database/)。

## 注意事项

SSE 是单向通信（服务器到客户端），如果需要双向通信，考虑使用 WebSocket。
