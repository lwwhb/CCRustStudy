# 模块 5.5：可观测性 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解可观测性的三大支柱
2. 掌握 tracing 框架
3. 学习结构化日志
4. 实现分布式追踪
5. 掌握指标收集

## 🎯 为什么需要可观测性？

### 传统日志 vs 可观测性

**传统日志**：
```rust
println!("User {} logged in", user_id);
println!("Processing request...");
println!("Error: {}", error);

问题：
- 难以搜索和分析
- 缺乏上下文
- 无法关联请求
- 性能影响未知
```

**可观测性**：
```rust
tracing::info!(user_id = %user_id, "User logged in");
tracing::debug!("Processing request");
tracing::error!(error = %error, "Request failed");

优势：
- 结构化数据
- 丰富的上下文
- 请求追踪
- 性能分析
```

### 可观测性三大支柱

```
1. 日志（Logs）
   - 离散的事件记录
   - 用于调试和审计
   - 示例：用户登录、错误信息

2. 指标（Metrics）
   - 数值型数据
   - 用于监控和告警
   - 示例：请求数、延迟、CPU 使用率

3. 追踪（Traces）
   - 请求的完整路径
   - 用于性能分析
   - 示例：请求经过的所有服务和函数
```

## 📖 核心概念详解

### 1. tracing 框架

tracing 是 Rust 的应用级追踪框架。

#### 核心概念

```rust
// Span - 表示一段时间
let span = tracing::info_span!("processing_request", request_id = %id);
let _enter = span.enter();  // 进入 span

// Event - 表示一个时间点
tracing::info!("Request received");
tracing::error!(error = %e, "Failed to process");

// Field - 结构化数据
tracing::info!(
    user_id = %user_id,
    action = "login",
    "User action"
);
```

**Span 的层级关系**：
```
root_span
├── database_query_span
│   ├── connect_span
│   └── execute_span
└── http_request_span
    ├── parse_span
    └── validate_span
```

#### 日志级别

```rust
tracing::error!("严重错误");    // ERROR - 错误
tracing::warn!("警告信息");     // WARN  - 警告
tracing::info!("一般信息");     // INFO  - 信息
tracing::debug!("调试信息");    // DEBUG - 调试
tracing::trace!("详细追踪");    // TRACE - 追踪
```

**使用建议**：
```
ERROR: 需要立即处理的错误
WARN:  可能的问题，需要关注
INFO:  重要的业务事件
DEBUG: 开发调试信息
TRACE: 非常详细的追踪信息
```

### 2. Subscriber（订阅者）

Subscriber 决定如何处理追踪数据。

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 方式 1: 简单的控制台输出
tracing_subscriber::fmt::init();

// 方式 2: 自定义格式
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_target(false)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .init();

// 方式 3: 多层订阅者
tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

**输出格式**：
```
2024-03-15T10:30:45.123Z INFO  myapp: User logged in user_id=123
2024-03-15T10:30:45.456Z DEBUG myapp: Processing request request_id="abc"
2024-03-15T10:30:45.789Z ERROR myapp: Database error error="connection timeout"
```

### 3. 结构化日志

结构化日志使用键值对而非纯文本。

```rust
// 非结构化（难以解析）
println!("User 123 logged in from 192.168.1.1");

// 结构化（易于解析和查询）
tracing::info!(
    user_id = 123,
    ip_address = "192.168.1.1",
    "User logged in"
);
```

**JSON 输出**：
```json
{
  "timestamp": "2024-03-15T10:30:45.123Z",
  "level": "INFO",
  "message": "User logged in",
  "fields": {
    "user_id": 123,
    "ip_address": "192.168.1.1"
  }
}
```

**好处**：
- 易于搜索（如：查找所有 user_id=123 的日志）
- 易于聚合（如：统计每个用户的登录次数）
- 易于可视化

### 4. 仪表化（Instrumentation）

为代码添加追踪信息。

#### 函数级仪表化

```rust
use tracing::instrument;

// 自动创建 span
#[instrument]
async fn process_user(user_id: i32) -> Result<User, Error> {
    // 函数执行时自动记录 user_id
    tracing::info!("Processing user");
    // ...
}

// 自定义 span 名称和字段
#[instrument(
    name = "user_processing",
    skip(db),  // 跳过不需要记录的参数
    fields(user_id = %user_id)
)]
async fn process_user_advanced(
    user_id: i32,
    db: &Database,
) -> Result<User, Error> {
    // ...
}
```

**生成的追踪**：
```
TRACE user_processing{user_id=123}: enter
DEBUG user_processing{user_id=123}: Processing user
TRACE user_processing{user_id=123}: exit
```

#### 手动 Span

```rust
async fn complex_operation() {
    let span = tracing::info_span!("complex_op", operation_id = %uuid);
    let _enter = span.enter();

    // 在 span 内执行操作
    tracing::debug!("Step 1");
    do_step1().await;

    tracing::debug!("Step 2");
    do_step2().await;

    // span 在 _enter 被 drop 时自动结束
}
```

### 5. 分布式追踪

追踪跨服务的请求。

```
客户端 → 服务 A → 服务 B → 数据库
         ↓         ↓
      trace_id  trace_id
      span_1    span_2
```

**实现方式**：
```rust
use tracing_opentelemetry::OpenTelemetryLayer;
use opentelemetry::global;

// 初始化 OpenTelemetry
let tracer = opentelemetry_jaeger::new_pipeline()
    .with_service_name("my-service")
    .install_simple()?;

// 添加到订阅者
tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(OpenTelemetryLayer::new(tracer))
    .init();
```

## 💻 实战项目：可观测的 Web 服务

### 项目需求

为之前的用户管理 API 添加完整的可观测性：
1. 结构化日志
2. 请求追踪
3. 性能指标
4. 错误追踪
5. 健康检查

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tower-http = { version = "0.5", features = ["trace"] }
uuid = { version = "1", features = ["v4", "serde"] }
serde = { version = "1", features = ["derive"] }
```

### 步骤 2：初始化追踪

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing() {
    // 从环境变量读取日志级别（默认 info）
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .json()  // JSON 格式输出
        )
        .init();

    tracing::info!("Tracing initialized");
}
```

### 步骤 3：请求追踪中间件

```rust
use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

// 自定义请求 ID 中间件
async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // 生成或提取请求 ID
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // 添加到请求扩展
    request.extensions_mut().insert(request_id.clone());

    // 创建 span
    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
    );

    let _enter = span.enter();
    tracing::info!("Request started");

    // 处理请求
    let response = next.run(request).await;

    tracing::info!(
        status = %response.status(),
        "Request completed"
    );

    response
}

// 应用中间件
let app = Router::new()
    .route("/users", get(list_users))
    .layer(middleware::from_fn(request_id_middleware))
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                tracing::info_span!(
                    "request",
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
    );
```

### 步骤 4：仪表化处理器

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::instrument;

#[instrument(
    skip(pool),
    fields(user_id = %id)
)]
async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    tracing::debug!("Fetching user from database");

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Database query failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or_else(|| {
        tracing::warn!("User not found");
        StatusCode::NOT_FOUND
    })?;

    tracing::info!("User fetched successfully");
    Ok(Json(user))
}

#[instrument(skip(pool, req))]
async fn create_user(
    State(pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    tracing::info!(
        name = %req.name,
        email = %req.email,
        "Creating new user"
    );

    // 验证
    if req.name.is_empty() {
        tracing::warn!("Invalid user name");
        return Err(StatusCode::BAD_REQUEST);
    }

    // 创建用户
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, age) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&req.name)
    .bind(&req.email)
    .bind(req.age)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Failed to create user");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(user_id = user.id, "User created successfully");
    Ok((StatusCode::CREATED, Json(user)))
}
```

### 步骤 5：数据库查询追踪

```rust
use sqlx::{PgPool, Postgres, Transaction};
use tracing::instrument;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        tracing::debug!("Querying database");

        let start = std::time::Instant::now();

        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await;

        let duration = start.elapsed();
        tracing::debug!(
            duration_ms = duration.as_millis(),
            "Query completed"
        );

        result
    }

    #[instrument(skip(self, req))]
    pub async fn create(&self, req: CreateUserRequest) -> Result<User, sqlx::Error> {
        tracing::info!("Creating user in database");

        let mut tx = self.pool.begin().await?;

        let user = self.create_in_tx(&mut tx, req).await?;

        tx.commit().await?;
        tracing::info!("Transaction committed");

        Ok(user)
    }

    #[instrument(skip(self, tx, req))]
    async fn create_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        req: CreateUserRequest,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, age) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&req.name)
        .bind(&req.email)
        .bind(req.age)
        .fetch_one(&mut **tx)
        .await
    }
}
```

### 步骤 6：错误追踪

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

// 自定义错误类型
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!(error = %e, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::NotFound => {
                tracing::warn!("Resource not found");
                (StatusCode::NOT_FOUND, "Not found".to_string())
            }
            AppError::ValidationError(msg) => {
                tracing::warn!(message = %msg, "Validation error");
                (StatusCode::BAD_REQUEST, msg)
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

// 使用自定义错误
#[instrument(skip(pool))]
async fn get_user_with_error(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(AppError::Database)?
    .ok_or(AppError::NotFound)?;

    Ok(Json(user))
}
```

### 步骤 7：健康检查

```rust
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    database: String,
    uptime_seconds: u64,
}

#[instrument(skip(pool))]
async fn health_check(
    State(pool): State<PgPool>,
) -> Json<HealthResponse> {
    tracing::debug!("Health check requested");

    // 检查数据库连接
    let db_status = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => {
            tracing::debug!("Database connection OK");
            "healthy"
        }
        Err(e) => {
            tracing::error!(error = %e, "Database connection failed");
            "unhealthy"
        }
    };

    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Json(HealthResponse {
        status: if db_status == "healthy" {
            "ok".to_string()
        } else {
            "degraded".to_string()
        },
        database: db_status.to_string(),
        uptime_seconds: uptime,
    })
}
```

### 步骤 8：主程序

```rust
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化追踪
    init_tracing();

    tracing::info!("Starting application");

    // 创建数据库连接池
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/mydb".to_string());

    tracing::info!(url = %database_url, "Connecting to database");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("Database connected");

    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .layer(middleware::from_fn(request_id_middleware))
        .with_state(pool);

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!(address = %addr, "Server starting");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

## 🔍 深入理解

### 日志输出示例

```json
{
  "timestamp": "2024-03-15T10:30:45.123Z",
  "level": "INFO",
  "fields": {
    "message": "Request started",
    "request_id": "abc-123",
    "method": "GET",
    "uri": "/users/1"
  },
  "target": "myapp::handlers",
  "span": {
    "name": "http_request",
    "request_id": "abc-123"
  }
}
```

### 性能考虑

```rust
// 避免：在热路径中创建大量 span
for item in items {
    let span = tracing::info_span!("process_item");
    // ...
}

// 推荐：使用更高级别的 span
let span = tracing::info_span!("process_items", count = items.len());
let _enter = span.enter();
for item in items {
    // ...
}
```

### 环境变量配置

```bash
# 设置日志级别
export RUST_LOG=info

# 按模块设置
export RUST_LOG=myapp=debug,sqlx=warn

# 详细追踪
export RUST_LOG=trace
```

## 📝 练习题

### 练习 1：添加自定义字段
为所有请求添加用户 ID 追踪：
```rust
#[instrument(skip(pool), fields(user_id))]
async fn handler(
    State(pool): State<PgPool>,
    // 你的代码
) {
    // 从认证中提取 user_id
    // 添加到当前 span
}
```

### 练习 2：性能监控
实现请求耗时统计：
```rust
async fn timing_middleware(
    request: Request,
    next: Next,
) -> Response {
    // 记录开始时间
    // 执行请求
    // 计算耗时
    // 记录到日志
}
```

### 练习 3：错误率监控
统计错误率：
```rust
// 实现一个中间件
// 记录成功和失败的请求数
// 定期输出错误率
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解可观测性的重要性
- [ ] 使用 tracing 框架
- [ ] 创建和管理 span
- [ ] 记录结构化日志
- [ ] 为函数添加仪表化
- [ ] 实现请求追踪
- [ ] 处理错误追踪
- [ ] 配置日志级别
- [ ] 理解性能影响
- [ ] 实现健康检查

## 🔗 延伸阅读

- [tracing 官方文档](https://docs.rs/tracing/)
- [tracing-subscriber 文档](https://docs.rs/tracing-subscriber/)
- [OpenTelemetry](https://opentelemetry.io/)
- [可观测性最佳实践](https://sre.google/books/)

## 🚀 下一步

完成 Web 服务阶段后，你可以：
1. 继续学习阶段 7（AI Gateway）
2. 回顾并完善之前的项目
3. 学习更高级的可观测性工具（Prometheus、Grafana）

---

**掌握可观测性，构建可靠的生产系统！** 🚀
