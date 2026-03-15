# 模块 7.1：AI Gateway 架构设计 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 AI Gateway 的架构模式
2. 掌握微服务网关设计
3. 学习中间件模式
4. 实现请求路由系统
5. 掌握状态管理和错误处理
6. 构建可扩展的 Gateway 框架

## 🎯 为什么需要 AI Gateway？

### 问题场景

**没有 Gateway 的情况**：
```
客户端 1 → 直接调用 OpenAI API
客户端 2 → 直接调用 Anthropic API
客户端 3 → 直接调用 Google AI API

问题：
- 每个客户端需要管理多个 API 密钥
- 无法统一监控和日志
- 难以实现速率限制
- 无法切换 Provider
- 成本难以控制
```

**使用 Gateway 的情况**：
```
客户端 1 ┐
客户端 2 ├→ AI Gateway → 路由到合适的 Provider
客户端 3 ┘

优势：
- 统一接口
- 集中认证和授权
- 统一监控和日志
- 灵活的路由策略
- 成本控制和优化
```

### Gateway 的核心功能

```
┌─────────────────────────────────────────┐
│           AI Gateway                     │
├─────────────────────────────────────────┤
│  1. 认证授权 (Authentication)           │
│  2. 请求路由 (Routing)                  │
│  3. 负载均衡 (Load Balancing)           │
│  4. 速率限制 (Rate Limiting)            │
│  5. 缓存 (Caching)                      │
│  6. 日志监控 (Logging & Monitoring)     │
│  7. 错误处理 (Error Handling)           │
│  8. 协议转换 (Protocol Translation)     │
└─────────────────────────────────────────┘
```

## 📖 核心概念详解

### 1. Gateway 架构模式

#### 单体 Gateway

```
客户端请求
    ↓
┌─────────────────┐
│   AI Gateway    │
│  (单一服务)     │
└─────────────────┘
    ↓
多个 AI Providers
```

**优点**：
- 简单易部署
- 低延迟
- 易于调试

**缺点**：
- 单点故障
- 难以扩展
- 资源竞争

#### 微服务 Gateway

```
客户端请求
    ↓
┌─────────────────┐
│  负载均衡器      │
└─────────────────┘
    ↓
┌─────────────────┐
│ Gateway 实例 1  │
│ Gateway 实例 2  │
│ Gateway 实例 3  │
└─────────────────┘
    ↓
多个 AI Providers
```

**优点**：
- 高可用
- 水平扩展
- 故障隔离

**缺点**：
- 复杂度高
- 需要服务发现
- 状态同步问题

### 2. 中间件模式

中间件是一种设计模式，允许在请求处理前后插入自定义逻辑。

#### 中间件执行流程

```
请求 → 中间件 1 → 中间件 2 → 处理器 → 中间件 2 → 中间件 1 → 响应
       (前置)     (前置)                  (后置)     (后置)
```

**示例**：
```
请求
  ↓
日志中间件 (记录请求开始)
  ↓
认证中间件 (验证 Token)
  ↓
速率限制中间件 (检查配额)
  ↓
处理器 (处理业务逻辑)
  ↓
速率限制中间件 (更新配额)
  ↓
认证中间件 (无操作)
  ↓
日志中间件 (记录请求结束)
  ↓
响应
```

#### Axum 中的中间件

```rust
use axum::{
    middleware::{self, Next},
    http::Request,
    response::Response,
};

// 自定义中间件
async fn my_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 前置处理
    println!("请求开始");

    // 调用下一个中间件或处理器
    let response = next.run(request).await;

    // 后置处理
    println!("请求结束");

    response
}

// 应用中间件
let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(my_middleware));
```

### 3. 状态管理

Gateway 需要管理共享状态，如请求计数、配置等。

#### 状态的类型

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct GatewayState {
    // 只读配置（不需要锁）
    config: Arc<GatewayConfig>,

    // 读多写少（使用 RwLock）
    request_count: Arc<RwLock<u64>>,

    // 读写频繁（使用 Mutex）
    cache: Arc<Mutex<HashMap<String, String>>>,
}
```

**选择合适的同步原语**：

```
Arc (Atomic Reference Counting):
- 多个所有者共享数据
- 线程安全的引用计数
- 只读共享

RwLock (Read-Write Lock):
- 多个读者或一个写者
- 读操作不阻塞
- 适合读多写少

Mutex (Mutual Exclusion):
- 同一时间只有一个访问者
- 读写都会阻塞
- 适合读写频繁
```

#### 使用状态

```rust
use axum::extract::State;

async fn handler(
    State(state): State<GatewayState>,
) -> String {
    // 读取配置（不需要锁）
    let service_name = &state.config.service_name;

    // 读取计数（需要读锁）
    let count = *state.request_count.read().await;

    // 更新计数（需要写锁）
    let mut count_mut = state.request_count.write().await;
    *count_mut += 1;

    format!("{}: {}", service_name, count)
}
```

### 4. 路由系统

路由将请求映射到正确的处理器。

#### 基础路由

```rust
let app = Router::new()
    // 健康检查
    .route("/health", get(health_check))

    // API 路由
    .route("/api/chat", post(chat_handler))
    .route("/api/completion", post(completion_handler))

    // 管理路由
    .route("/admin/stats", get(stats_handler))
    .route("/admin/config", get(config_handler));
```

#### 嵌套路由

```rust
// API v1 路由
let api_v1 = Router::new()
    .route("/chat", post(chat_v1))
    .route("/completion", post(completion_v1));

// API v2 路由
let api_v2 = Router::new()
    .route("/chat", post(chat_v2))
    .route("/completion", post(completion_v2));

// 主应用
let app = Router::new()
    .nest("/api/v1", api_v1)
    .nest("/api/v2", api_v2)
    .route("/health", get(health_check));

// 最终路由：
// POST /api/v1/chat
// POST /api/v1/completion
// POST /api/v2/chat
// POST /api/v2/completion
// GET /health
```

#### 路由组和中间件

```rust
// 公开路由（无需认证）
let public_routes = Router::new()
    .route("/health", get(health_check))
    .route("/login", post(login));

// 受保护路由（需要认证）
let protected_routes = Router::new()
    .route("/api/chat", post(chat_handler))
    .route("/api/completion", post(completion_handler))
    .layer(middleware::from_fn(auth_middleware));

// 合并路由
let app = Router::new()
    .merge(public_routes)
    .merge(protected_routes);
```

### 5. 错误处理

统一的错误处理提高代码质量和用户体验。

#### 自定义错误类型

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

// 自定义错误类型
enum GatewayError {
    InvalidRequest(String),
    Unauthorized,
    RateLimitExceeded,
    ProviderError(String),
    InternalError,
}

// 实现 IntoResponse
impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match self {
            GatewayError::InvalidRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
                "INVALID_REQUEST".to_string(),
            ),
            GatewayError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "未授权".to_string(),
                "UNAUTHORIZED".to_string(),
            ),
            GatewayError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "超过速率限制".to_string(),
                "RATE_LIMIT_EXCEEDED".to_string(),
            ),
            GatewayError::ProviderError(msg) => (
                StatusCode::BAD_GATEWAY,
                msg,
                "PROVIDER_ERROR".to_string(),
            ),
            GatewayError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "内部错误".to_string(),
                "INTERNAL_ERROR".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_message,
            code: error_code,
        });

        (status, body).into_response()
    }
}
```

#### 使用错误类型

```rust
async fn chat_handler(
    State(state): State<GatewayState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, GatewayError> {
    // 验证请求
    if req.message.is_empty() {
        return Err(GatewayError::InvalidRequest(
            "消息不能为空".to_string()
        ));
    }

    // 检查速率限制
    if !check_rate_limit(&state).await {
        return Err(GatewayError::RateLimitExceeded);
    }

    // 调用 Provider
    let response = call_provider(&req)
        .await
        .map_err(|e| GatewayError::ProviderError(e.to_string()))?;

    Ok(Json(response))
}
```

## 💻 实战项目：基础 Gateway 框架

### 项目需求

构建一个基础的 AI Gateway 框架，支持：
1. 健康检查端点
2. 聊天 API 端点
3. 请求日志中间件
4. 请求计数统计
5. 错误处理
6. 配置管理

### 步骤 1：定义数据结构

```rust
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// Gateway 配置
#[derive(Debug, Clone)]
struct GatewayConfig {
    service_name: String,
    version: String,
    max_request_size: usize,
}

// Gateway 状态
#[derive(Clone)]
struct GatewayState {
    request_count: Arc<RwLock<u64>>,
    config: Arc<GatewayConfig>,
}

// 聊天请求
#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
    model: Option<String>,
}

// 聊天响应
#[derive(Debug, Serialize)]
struct ChatResponse {
    request_id: String,
    response: String,
    model: String,
}

// 健康检查响应
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
    total_requests: u64,
}
```

### 步骤 2：实现处理器

```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;

// 健康检查
async fn health_check(
    State(state): State<GatewayState>,
) -> Json<HealthResponse> {
    let count = *state.request_count.read().await;

    Json(HealthResponse {
        status: "ok".to_string(),
        service: state.config.service_name.clone(),
        version: state.config.version.clone(),
        total_requests: count,
    })
}

// 聊天处理器
async fn chat_handler(
    State(state): State<GatewayState>,
    Json(req): Json<ChatRequest>,
) -> Result<(StatusCode, Json<ChatResponse>), GatewayError> {
    // 验证请求
    if req.message.trim().is_empty() {
        return Err(GatewayError::InvalidRequest(
            "消息不能为空".to_string()
        ));
    }

    // 更新请求计数
    let mut count = state.request_count.write().await;
    *count += 1;

    // 生成请求 ID
    let request_id = Uuid::new_v4().to_string();

    // 模拟 AI 响应
    let model = req.model.unwrap_or_else(|| "gpt-4".to_string());
    let response = format!("收到消息: {}", req.message);

    Ok((
        StatusCode::OK,
        Json(ChatResponse {
            request_id,
            response,
            model,
        }),
    ))
}
```

### 步骤 3：实现中间件

```rust
use axum::{
    middleware::Next,
    http::Request,
    response::Response,
};
use tracing::info;

// 日志中间件
async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    info!("请求开始: {} {}", method, uri);

    let response = next.run(request).await;

    info!("请求结束: {} {} - {}", method, uri, response.status());

    response
}

// 请求 ID 中间件
async fn request_id_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    let request_id = Uuid::new_v4().to_string();

    // 将请求 ID 添加到请求头
    request.headers_mut().insert(
        "X-Request-ID",
        request_id.parse().unwrap(),
    );

    let mut response = next.run(request).await;

    // 将请求 ID 添加到响应头
    response.headers_mut().insert(
        "X-Request-ID",
        request_id.parse().unwrap(),
    );

    response
}
```

### 步骤 4：创建应用

```rust
use axum::{
    middleware,
    Router,
};
use tower_http::trace::TraceLayer;

fn create_app(state: GatewayState) -> Router {
    Router::new()
        // 健康检查（无中间件）
        .route("/health", get(health_check))

        // API 路由（带中间件）
        .route("/api/chat", post(chat_handler))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())

        // 添加状态
        .with_state(state)
}
```

### 步骤 5：主函数

```rust
#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建配置
    let config = GatewayConfig {
        service_name: "AI Gateway".to_string(),
        version: "1.0.0".to_string(),
        max_request_size: 1024 * 1024, // 1MB
    };

    // 创建状态
    let state = GatewayState {
        request_count: Arc::new(RwLock::new(0)),
        config: Arc::new(config),
    };

    // 创建应用
    let app = create_app(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 AI Gateway 运行在 http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
```

## 🔍 深入理解

### Gateway 的性能考虑

#### 1. 异步 I/O

```rust
// ❌ 同步阻塞（性能差）
fn sync_handler() -> String {
    let response = std::thread::sleep(Duration::from_secs(1));
    "Done".to_string()
}

// ✅ 异步非阻塞（性能好）
async fn async_handler() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    "Done".to_string()
}
```

#### 2. 连接池

```rust
// 为每个 Provider 维护连接池
struct ProviderPool {
    client: reqwest::Client,
}

impl ProviderPool {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(10)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client }
    }
}
```

#### 3. 缓存策略

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct Cache {
    data: HashMap<String, (String, Instant)>,
    ttl: Duration,
}

impl Cache {
    async fn get(&self, key: &str) -> Option<String> {
        if let Some((value, timestamp)) = self.data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }
}
```

### 常见陷阱

#### 陷阱 1：死锁

```rust
// ❌ 错误：可能死锁
async fn bad_handler(state: State<GatewayState>) {
    let lock1 = state.lock1.lock().await;
    let lock2 = state.lock2.lock().await;  // 如果另一个任务持有 lock2 并等待 lock1
    // 死锁！
}

// ✅ 正确：按顺序获取锁
async fn good_handler(state: State<GatewayState>) {
    // 总是按相同顺序获取锁
    let lock1 = state.lock1.lock().await;
    let lock2 = state.lock2.lock().await;
}
```

#### 陷阱 2：过度使用 Mutex

```rust
// ❌ 错误：读操作也会阻塞
let count = Arc::new(Mutex::new(0));

// ✅ 正确：使用 RwLock
let count = Arc::new(RwLock::new(0));

// 多个读者可以同时访问
let value = count.read().await;
```

#### 陷阱 3：忘记释放锁

```rust
// ❌ 错误：锁持有时间过长
async fn bad_handler(state: State<GatewayState>) {
    let mut data = state.data.lock().await;
    // 执行耗时操作
    expensive_operation().await;  // 锁一直被持有！
    *data += 1;
}

// ✅ 正确：尽快释放锁
async fn good_handler(state: State<GatewayState>) {
    // 先执行耗时操作
    expensive_operation().await;

    // 快速获取锁、修改、释放
    let mut data = state.data.lock().await;
    *data += 1;
    drop(data);  // 显式释放（可选）
}
```

## 📝 练习题

### 练习 1：添加请求耗时中间件

实现一个中间件，记录每个请求的处理时间：

```rust
async fn timing_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 你的代码
}
```

### 练习 2：实现简单的速率限制

实现一个基于令牌桶的速率限制：

```rust
struct RateLimiter {
    tokens: Arc<RwLock<u32>>,
    max_tokens: u32,
    refill_rate: u32,  // 每秒
}

impl RateLimiter {
    async fn check(&self) -> bool {
        // 你的代码
    }
}
```

### 练习 3：添加 CORS 支持

为 Gateway 添加 CORS 中间件：

```rust
use tower_http::cors::{CorsLayer, Any};

fn create_app_with_cors(state: GatewayState) -> Router {
    // 你的代码
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解 Gateway 架构模式
- [ ] 掌握中间件的实现和使用
- [ ] 实现状态管理
- [ ] 设计路由系统
- [ ] 处理错误和异常
- [ ] 实现日志和监控
- [ ] 理解性能优化要点
- [ ] 避免常见陷阱

## 🔗 延伸阅读

- [Axum 官方文档](https://docs.rs/axum/)
- [Tower 中间件](https://docs.rs/tower/)
- [API Gateway 模式](https://microservices.io/patterns/apigateway.html)
- [Tokio 异步编程](https://tokio.rs/)

## 🚀 下一步

完成本模块后，继续学习：
- [模块 7.2：AI Provider 客户端](../02-ai-clients/)
- 深入学习流式响应处理
- 实现 Agent 系统

---

**掌握 Gateway 架构，构建强大的 AI 服务！** 🚀

## 💻 实战项目：AI Gateway 基础框架

### 项目需求

构建一个基础的 AI Gateway 框架，支持：
1. 健康检查端点
2. 聊天 API 端点
3. 请求日志中间件
4. 请求计数统计
5. 错误处理

### 步骤 1：定义数据结构

```rust
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// Gateway 状态
#[derive(Clone)]
struct GatewayState {
    request_count: Arc<RwLock<u64>>,
    config: Arc<GatewayConfig>,
}

// 配置
#[derive(Debug, Clone)]
struct GatewayConfig {
    service_name: String,
    version: String,
    max_request_size: usize,
}

// 聊天请求
#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
    model: Option<String>,
}

// 聊天响应
#[derive(Debug, Serialize)]
struct ChatResponse {
    request_id: String,
    response: String,
    model: String,
}
```

### 步骤 2：实现处理器

```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;

// 健康检查
async fn health_check(
    State(state): State<GatewayState>,
) -> Json<serde_json::Value> {
    let count = *state.request_count.read().await;
    
    Json(serde_json::json!({
        "status": "ok",
        "service": state.config.service_name,
        "version": state.config.version,
        "total_requests": count,
    }))
}

// 聊天处理器
async fn chat_handler(
    State(state): State<GatewayState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    // 验证请求
    if req.message.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 生成请求 ID
    let request_id = Uuid::new_v4().to_string();

    // 模拟 AI 响应
    let response = format!("Echo: {}", req.message);
    let model = req.model.unwrap_or_else(|| "default".to_string());

    Ok(Json(ChatResponse {
        request_id,
        response,
        model,
    }))
}
```

### 步骤 3：实现中间件

```rust
use axum::{
    middleware::Next,
    http::Request,
    response::Response,
};
use tracing::info;

// 日志中间件
async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    info!("收到请求: {} {}", method, uri);

    let response = next.run(request).await;

    info!("响应状态: {}", response.status());

    response
}

// 请求计数中间件
async fn request_counter_middleware<B>(
    State(state): State<GatewayState>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 增加计数
    let mut count = state.request_count.write().await;
    *count += 1;
    drop(count);

    next.run(request).await
}
```

### 步骤 4：创建应用

```rust
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

fn create_app(state: GatewayState) -> Router {
    Router::new()
        // 路由
        .route("/health", get(health_check))
        .route("/api/chat", post(chat_handler))
        
        // 中间件
        .layer(middleware::from_fn_with_state(
            state.clone(),
            request_counter_middleware,
        ))
        .layer(middleware::from_fn(logging_middleware))
        .layer(TraceLayer::new_for_http())
        
        // 状态
        .with_state(state)
}
```

### 步骤 5：主函数

```rust
#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建配置
    let config = GatewayConfig {
        service_name: "AI Gateway".to_string(),
        version: "1.0.0".to_string(),
        max_request_size: 1024 * 1024, // 1MB
    };

    // 创建状态
    let state = GatewayState {
        request_count: Arc::new(RwLock::new(0)),
        config: Arc::new(config),
    };

    // 创建应用
    let app = create_app(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 AI Gateway 运行在 http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
```

## 🧪 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn create_test_state() -> GatewayState {
        let config = GatewayConfig {
            service_name: "Test Gateway".to_string(),
            version: "1.0.0".to_string(),
            max_request_size: 1024,
        };

        GatewayState {
            request_count: Arc::new(RwLock::new(0)),
            config: Arc::new(config),
        }
    }

    #[tokio::test]
    async fn test_health_check() {
        let state = create_test_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_chat_endpoint() {
        let state = create_test_state();
        let app = create_app(state);

        let request_body = serde_json::json!({
            "message": "Hello"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_string(&request_body).unwrap()
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

### 集成测试

```bash
# 启动服务器
cargo run

# 在另一个终端测试

# 健康检查
curl http://localhost:3000/health

# 聊天请求
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, AI!"}'

# 无效请求
curl -X POST http://localhost:3000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": ""}'
```

## 🔍 深入理解

### Gateway 的性能考虑

**1. 异步 I/O**
```rust
// ❌ 阻塞 I/O（不要这样做）
fn blocking_handler() -> String {
    std::thread::sleep(Duration::from_secs(1));
    "Done".to_string()
}

// ✅ 异步 I/O
async fn async_handler() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    "Done".to_string()
}
```

**2. 连接池**
```rust
// 为每个 Provider 维护连接池
struct ProviderPool {
    client: reqwest::Client,
}

impl ProviderPool {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(10)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client }
    }
}
```

**3. 缓存策略**
```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct Cache {
    data: HashMap<String, (String, Instant)>,
    ttl: Duration,
}

impl Cache {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).and_then(|(value, timestamp)| {
            if timestamp.elapsed() < self.ttl {
                Some(value.clone())
            } else {
                None
            }
        })
    }
}
```

### 错误处理最佳实践

**1. 使用 Result 类型**
```rust
async fn handler() -> Result<Json<Response>, GatewayError> {
    let data = fetch_data().await?;
    let processed = process_data(data)?;
    Ok(Json(processed))
}
```

**2. 错误转换**
```rust
impl From<reqwest::Error> for GatewayError {
    fn from(err: reqwest::Error) -> Self {
        GatewayError::ProviderError(err.to_string())
    }
}
```

**3. 错误日志**
```rust
use tracing::error;

async fn handler() -> Result<Json<Response>, GatewayError> {
    match risky_operation().await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("操作失败: {:?}", e);
            Err(GatewayError::InternalError)
        }
    }
}
```

## 📝 练习题

### 练习 1：添加请求耗时中间件

实现一个中间件，记录每个请求的处理时间：

```rust
async fn timing_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 你的代码
    // 提示：使用 std::time::Instant
}
```

### 练习 2：实现请求 ID 追踪

为每个请求生成唯一 ID，并在响应头中返回：

```rust
async fn request_id_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    // 你的代码
    // 提示：使用 uuid::Uuid 和 request.extensions_mut()
}
```

### 练习 3：添加 CORS 支持

实现 CORS 中间件，允许跨域请求：

```rust
use axum::http::header;

async fn cors_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // 你的代码
    // 提示：添加 Access-Control-Allow-Origin 等头
}
```

### 练习 4：实现简单的速率限制

基于 IP 地址的简单速率限制：

```rust
use std::collections::HashMap;
use std::net::IpAddr;

struct RateLimiter {
    requests: HashMap<IpAddr, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn check(&mut self, ip: IpAddr) -> bool {
        // 你的代码
    }
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解 Gateway 架构模式
- [ ] 掌握 Axum 路由系统
- [ ] 实现自定义中间件
- [ ] 使用 Arc 和 RwLock 管理状态
- [ ] 实现统一的错误处理
- [ ] 编写单元测试和集成测试
- [ ] 理解异步 I/O 的优势
- [ ] 掌握性能优化技巧

## 🔗 延伸阅读

- [Axum 官方文档](https://docs.rs/axum/)
- [Tower 中间件](https://docs.rs/tower/)
- [API Gateway 模式](https://microservices.io/patterns/apigateway.html)
- [Tokio 异步编程](https://tokio.rs/)
- [微服务架构](https://martinfowler.com/articles/microservices.html)

## 🚀 下一步

完成本模块后，你可以：
1. 继续学习模块 7.2（AI Provider 客户端）
2. 深入学习中间件开发
3. 研究负载均衡策略
4. 学习分布式追踪

---

**掌握 Gateway 架构，构建强大的 AI 服务！** 🚀
