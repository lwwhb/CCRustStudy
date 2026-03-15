# 模块 5.3：流式处理与 SSE - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解流式处理的概念和优势
2. 掌握 Server-Sent Events (SSE)
3. 学习 tokio-stream 的使用
4. 实现实时数据推送
5. 处理背压和流控制

## 🎯 为什么需要流式处理？

### 传统请求 vs 流式处理

**传统 HTTP 请求**：
```
客户端 → 请求 → 服务器
         ↓
      等待处理
         ↓
客户端 ← 完整响应 ← 服务器

问题：
- 必须等待所有数据准备好
- 大数据传输延迟高
- 无法实时更新
- 内存占用大
```

**流式处理**：
```
客户端 → 请求 → 服务器
         ↓
客户端 ← 数据块 1 ← 服务器
客户端 ← 数据块 2 ← 服务器
客户端 ← 数据块 3 ← 服务器
         ↓
      持续推送

优势：
- 立即开始接收数据
- 降低延迟
- 实时更新
- 内存友好
```

### 应用场景

```
1. AI 聊天机器人
   - 逐字输出响应
   - 提升用户体验

2. 实时日志
   - 服务器日志流
   - 构建进度

3. 数据监控
   - 系统指标
   - 股票行情

4. 大文件传输
   - 视频流
   - 文件下载进度
```

## 📖 核心概念详解

### 1. Server-Sent Events (SSE)

SSE 是 HTML5 标准，用于服务器向客户端推送数据。

#### SSE vs WebSocket

```
SSE:
- 单向通信（服务器 → 客户端）
- 基于 HTTP
- 自动重连
- 文本数据
- 简单易用

WebSocket:
- 双向通信
- 独立协议
- 需要手动重连
- 二进制/文本
- 更复杂
```

#### SSE 协议格式

```
HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive

data: 第一条消息

data: 第二条消息

event: custom
data: 自定义事件
id: 123

data: 多行消息
data: 第二行
data: 第三行

```

**关键点**：
- `Content-Type: text/event-stream`
- 每条消息以 `\n\n` 结束
- `data:` 字段包含消息内容
- `event:` 指定事件类型（可选）
- `id:` 消息 ID（用于重连）

### 2. Axum 中的流式响应

```rust
use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt as _;

// 创建 SSE 流
async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // 创建一个每秒发送一次的流
    let stream = stream::repeat_with(|| Event::default().data("ping"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream)
}

let app = Router::new().route("/events", get(sse_handler));
```

**工作流程**：
```
1. 客户端连接 /events
2. 服务器返回 SSE 响应头
3. 保持连接打开
4. 定期发送事件
5. 客户端接收并处理
```

### 3. tokio-stream

tokio-stream 提供异步流的工具。

#### 创建流

```rust
use tokio_stream::{self as stream, StreamExt};

// 方式 1: 从迭代器创建
let s = stream::iter(vec![1, 2, 3]);

// 方式 2: 从 channel 创建
let (tx, rx) = tokio::sync::mpsc::channel(10);
let s = tokio_stream::wrappers::ReceiverStream::new(rx);

// 方式 3: 定时流
let s = stream::interval(Duration::from_secs(1));

// 方式 4: 自定义流
let s = stream::unfold(0, |state| async move {
    if state < 5 {
        Some((state, state + 1))
    } else {
        None
    }
});
```

#### 流操作

```rust
use tokio_stream::StreamExt;

let stream = stream::iter(vec![1, 2, 3, 4, 5]);

// map - 转换每个元素
let doubled = stream.map(|x| x * 2);

// filter - 过滤元素
let evens = stream.filter(|x| x % 2 == 0);

// take - 取前 N 个
let first_three = stream.take(3);

// throttle - 限流
let throttled = stream.throttle(Duration::from_millis(100));

// 组合操作
let result = stream
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .take(3)
    .collect::<Vec<_>>()
    .await;
```

### 4. 背压（Backpressure）

背压是流式系统中的重要概念。

```
生产者 → [缓冲区] → 消费者

问题：生产速度 > 消费速度
- 缓冲区溢出
- 内存耗尽
- 数据丢失

解决方案：
1. 限制生产速度（throttle）
2. 增大缓冲区
3. 丢弃旧数据
4. 阻塞生产者
```

**示例**：
```rust
use tokio::sync::mpsc;

// 有界 channel（自动背压）
let (tx, rx) = mpsc::channel(10);  // 缓冲区大小 10

// 当缓冲区满时，send 会等待
tx.send(data).await?;  // 如果满了会阻塞
```

## 💻 实战项目：实时日志流服务

### 项目需求

构建一个实时日志流服务，支持：
1. SSE 端点推送日志
2. 多个客户端同时订阅
3. 日志级别过滤
4. 历史日志回放
5. 优雅关闭

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
futures = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.5", features = ["cors"] }
```

### 步骤 2：定义日志结构

```rust
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, source: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            timestamp,
            level,
            message,
            source,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
```

### 步骤 3：日志广播系统

```rust
use tokio::sync::broadcast;
use std::sync::Arc;

pub struct LogBroadcaster {
    tx: broadcast::Sender<LogEntry>,
}

impl LogBroadcaster {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    // 发送日志
    pub fn send(&self, entry: LogEntry) {
        // 忽略发送错误（没有订阅者时）
        let _ = self.tx.send(entry);
    }

    // 订阅日志流
    pub fn subscribe(&self) -> broadcast::Receiver<LogEntry> {
        self.tx.subscribe()
    }

    // 获取订阅者数量
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }
}
```

**关键点**：
- 使用 `broadcast` channel 支持多个订阅者
- `send` 不会阻塞（如果没有订阅者）
- 每个订阅者都会收到消息的副本

### 步骤 4：SSE 处理器

```rust
use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

#[derive(Deserialize)]
pub struct LogFilter {
    level: Option<String>,
}

pub async fn log_stream(
    State(broadcaster): State<Arc<LogBroadcaster>>,
    Query(filter): Query<LogFilter>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // 订阅日志流
    let rx = broadcaster.subscribe();
    let stream = BroadcastStream::new(rx);

    // 过滤和转换
    let filtered_stream = stream
        .filter_map(move |result| {
            let entry = match result {
                Ok(entry) => entry,
                Err(_) => return None,  // 跳过错误
            };

            // 应用级别过滤
            if let Some(ref level_filter) = filter.level {
                let matches = match (level_filter.as_str(), &entry.level) {
                    ("debug", LogLevel::Debug) => true,
                    ("info", LogLevel::Info) => true,
                    ("warn", LogLevel::Warn) => true,
                    ("error", LogLevel::Error) => true,
                    _ => false,
                };

                if !matches {
                    return None;
                }
            }

            // 转换为 SSE Event
            Some(Event::default()
                .json_data(&entry)
                .unwrap())
        })
        .map(Ok);

    Sse::new(filtered_stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(15))
                .text("keep-alive")
        )
}
```

**关键点**：
- `BroadcastStream` 将 broadcast receiver 转换为 Stream
- `filter_map` 同时过滤和转换
- `keep_alive` 保持连接活跃

### 步骤 5：日志生成器（模拟）

```rust
use tokio::time::{interval, Duration};
use rand::Rng;

pub async fn start_log_generator(broadcaster: Arc<LogBroadcaster>) {
    let mut interval = interval(Duration::from_secs(1));
    let mut rng = rand::thread_rng();

    loop {
        interval.tick().await;

        // 随机生成日志
        let level = match rng.gen_range(0..4) {
            0 => LogLevel::Debug,
            1 => LogLevel::Info,
            2 => LogLevel::Warn,
            _ => LogLevel::Error,
        };

        let messages = vec![
            "处理请求",
            "数据库查询完成",
            "缓存命中",
            "连接超时",
            "内存使用率: 75%",
        ];

        let message = messages[rng.gen_range(0..messages.len())].to_string();

        let entry = LogEntry::new(
            level,
            message,
            "app-server".to_string(),
        );

        broadcaster.send(entry);
    }
}
```

### 步骤 6：主程序

```rust
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    println!("=== 实时日志流服务 ===\n");

    // 创建日志广播器
    let broadcaster = Arc::new(LogBroadcaster::new(100));

    // 启动日志生成器
    let broadcaster_clone = broadcaster.clone();
    tokio::spawn(async move {
        start_log_generator(broadcaster_clone).await;
    });

    // 创建路由
    let app = Router::new()
        .route("/logs", get(log_stream))
        .route("/health", get(|| async { "OK" }))
        .layer(CorsLayer::permissive())
        .with_state(broadcaster);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("服务器运行在 http://127.0.0.1:3000");
    println!("日志流: http://127.0.0.1:3000/logs");
    println!("过滤示例: http://127.0.0.1:3000/logs?level=error\n");

    axum::serve(listener, app).await.unwrap();
}
```

### 步骤 7：客户端测试

**使用 curl**：
```bash
# 订阅所有日志
curl -N http://localhost:3000/logs

# 只订阅错误日志
curl -N http://localhost:3000/logs?level=error
```

**使用 JavaScript**：
```javascript
const eventSource = new EventSource('http://localhost:3000/logs');

eventSource.onmessage = (event) => {
    const log = JSON.parse(event.data);
    console.log(`[${log.level}] ${log.message}`);
};

eventSource.onerror = (error) => {
    console.error('连接错误:', error);
};
```

**使用 Rust 客户端**：
```rust
use reqwest::Client;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut stream = client
        .get("http://localhost:3000/logs")
        .send()
        .await?
        .bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);
        print!("{}", text);
    }

    Ok(())
}
```

## 🔍 深入理解

### SSE 的工作原理

```
1. 客户端发起请求
   GET /events HTTP/1.1
   Accept: text/event-stream

2. 服务器响应
   HTTP/1.1 200 OK
   Content-Type: text/event-stream
   Cache-Control: no-cache
   Connection: keep-alive

3. 保持连接
   - TCP 连接保持打开
   - 服务器可以随时发送数据
   - 客户端持续接收

4. 发送事件
   data: 消息内容\n\n

5. 断线重连
   - 客户端自动重连
   - 使用 Last-Event-ID 恢复
```

### 流的生命周期

```rust
// 创建流
let stream = stream::iter(vec![1, 2, 3]);

// 流的状态
enum StreamState {
    Ready,      // 有数据可读
    Pending,    // 等待数据
    Completed,  // 流结束
}

// 消费流
while let Some(item) = stream.next().await {
    // 处理 item
}
// 流自动关闭
```

### 内存管理

```
问题：无限流可能导致内存泄漏

解决方案：
1. 有界 channel
   let (tx, rx) = mpsc::channel(100);  // 最多缓冲 100 个

2. 限流
   stream.throttle(Duration::from_millis(100))

3. 超时
   tokio::time::timeout(Duration::from_secs(30), stream.collect())

4. 定期清理
   stream.take(1000)  // 只取前 1000 个
```

## 📝 练习题

### 练习 1：实现计数器流

```rust
// 实现一个每秒递增的计数器流
async fn counter_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // 你的代码
}
```

### 练习 2：实现聊天室

```rust
// 实现一个简单的聊天室
// - 用户可以发送消息
// - 所有用户实时接收消息
// - 支持用户加入/离开通知

struct ChatRoom {
    broadcaster: Arc<LogBroadcaster>,
}

impl ChatRoom {
    async fn send_message(&self, user: String, message: String) {
        // 你的代码
    }

    async fn subscribe(&self) -> impl Stream<Item = Result<Event, Infallible>> {
        // 你的代码
    }
}
```

### 练习 3：实现进度追踪

```rust
// 实现一个文件上传进度追踪
// - 客户端上传文件
// - 服务器通过 SSE 推送进度
// - 支持多个并发上传

async fn upload_with_progress(
    file: Vec<u8>,
) -> (String, impl Stream<Item = Result<Event, Infallible>>) {
    // 返回 (upload_id, progress_stream)
    // 你的代码
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解流式处理的概念和优势
- [ ] 掌握 SSE 协议格式
- [ ] 使用 Axum 创建 SSE 端点
- [ ] 使用 tokio-stream 操作流
- [ ] 实现流的过滤和转换
- [ ] 处理多个订阅者（broadcast）
- [ ] 理解背压和流控制
- [ ] 实现 keep-alive 机制
- [ ] 处理客户端断线重连
- [ ] 优雅关闭流连接

## 🔗 延伸阅读

- [Server-Sent Events 规范](https://html.spec.whatwg.org/multipage/server-sent-events.html)
- [Axum SSE 文档](https://docs.rs/axum/latest/axum/response/sse/)
- [tokio-stream 文档](https://docs.rs/tokio-stream/)
- [Futures Stream Trait](https://docs.rs/futures/latest/futures/stream/trait.Stream.html)

## 🚀 下一步

完成本模块后，你可以：
1. 继续学习模块 5.4（数据库集成）
2. 学习 WebSocket 实现双向通信
3. 深入学习响应式编程模式

---

**掌握流式处理，构建实时应用！** 🚀
