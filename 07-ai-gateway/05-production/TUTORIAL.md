# 模块 7.5：生产特性 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 实现认证和授权
2. 添加速率限制
3. 配置管理和环境变量
4. 实现健康检查和优雅关闭
5. Docker 容器化部署

## 🎯 为什么需要生产特性？

### 开发环境 vs 生产环境

**开发环境（简单）**：
```rust
#[tokio::main]
async fn main() {
    let app = create_app();
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

问题：
- 无认证（任何人都能访问）
- 无速率限制（易被滥用）
- 硬编码配置
- 无监控
- 无优雅关闭
```

**生产环境（完善）**：
```rust
#[tokio::main]
async fn main() {
    // 加载配置
    let config = Config::from_env()?;
    
    // 初始化日志
    init_tracing(&config);
    
    // 创建应用（带认证、限流等）
    let app = create_app_with_middleware(config.clone());
    
    // 健康检查
    tokio::spawn(health_check_server());
    
    // 优雅关闭
    let server = axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());
    
    server.await?;
}

优势：
- 安全（认证授权）
- 稳定（限流保护）
- 灵活（配置管理）
- 可观测（监控日志）
- 可靠（优雅关闭）
```

## 📖 核心概念详解

### 1. 认证与授权

#### API Key 认证

```rust
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

// API Key 中间件
pub async fn api_key_auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从 header 获取 API key
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 验证 API key
    if !is_valid_api_key(api_key).await {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 继续处理请求
    Ok(next.run(request).await)
}

async fn is_valid_api_key(key: &str) -> bool {
    // 从数据库或配置中验证
    // 这里简化为硬编码
    key == "secret-api-key-123"
}
```

#### JWT 认证

```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // 用户 ID
    exp: usize,   // 过期时间
    role: String, // 角色
}

// 生成 JWT
pub fn generate_token(user_id: &str, role: &str) -> Result<String, String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| e.to_string())
}

// 验证 JWT
pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}

// JWT 中间件
pub async fn jwt_auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 将用户信息添加到请求扩展中
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
```

### 2. 速率限制

防止 API 滥用。

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

// 速率限制器
pub struct RateLimiter {
    // 用户 -> (请求次数, 窗口开始时间)
    requests: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub async fn check(&self, user_id: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();

        let entry = requests.entry(user_id.to_string())
            .or_insert((0, now));

        // 检查窗口是否过期
        if now.duration_since(entry.1) > self.window {
            // 重置窗口
            entry.0 = 0;
            entry.1 = now;
        }

        // 检查是否超过限制
        if entry.0 >= self.max_requests {
            return false;
        }

        // 增加计数
        entry.0 += 1;
        true
    }
}

// 速率限制中间件
pub async fn rate_limit_middleware(
    State(limiter): State<Arc<RateLimiter>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从 header 获取用户标识
    let user_id = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous");

    // 检查速率限制
    if !limiter.check(user_id).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}
```

**更高级的限流策略**：

```rust
// 令牌桶算法
pub struct TokenBucket {
    tokens: Arc<Mutex<f64>>,
    capacity: f64,
    refill_rate: f64,  // 每秒补充的令牌数
    last_refill: Arc<Mutex<Instant>>,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: Arc::new(Mutex::new(capacity)),
            capacity,
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn try_consume(&self, tokens: f64) -> bool {
        let mut current_tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;

        // 计算应该补充的令牌
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let refill = elapsed * self.refill_rate;

        *current_tokens = (*current_tokens + refill).min(self.capacity);
        *last_refill = now;

        // 尝试消费令牌
        if *current_tokens >= tokens {
            *current_tokens -= tokens;
            true
        } else {
            false
        }
    }
}
```

### 3. 配置管理

```rust
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // 服务器配置
    pub server: ServerConfig,
    
    // 数据库配置
    pub database: DatabaseConfig,
    
    // AI 提供商配置
    pub ai: AiConfig,
    
    // 认证配置
    pub auth: AuthConfig,
    
    // 日志配置
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiConfig {
    pub openai_api_key: String,
    pub anthropic_api_key: String,
    pub default_model: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,  // "json" or "pretty"
}

impl Config {
    // 从环境变量加载
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .map_err(|e| format!("Invalid port: {}", e))?,
                workers: env::var("SERVER_WORKERS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()
                    .unwrap_or(4),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .map_err(|_| "DATABASE_URL not set")?,
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            ai: AiConfig {
                openai_api_key: env::var("OPENAI_API_KEY")
                    .unwrap_or_default(),
                anthropic_api_key: env::var("ANTHROPIC_API_KEY")
                    .unwrap_or_default(),
                default_model: env::var("DEFAULT_AI_MODEL")
                    .unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
            },
            auth: AuthConfig {
                jwt_secret: env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "change-me-in-production".to_string()),
                token_expiry_hours: env::var("TOKEN_EXPIRY_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
            },
            logging: LoggingConfig {
                level: env::var("LOG_LEVEL")
                    .unwrap_or_else(|_| "info".to_string()),
                format: env::var("LOG_FORMAT")
                    .unwrap_or_else(|_| "json".to_string()),
            },
        })
    }

    // 从 TOML 文件加载
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }
}
```

**配置文件示例（config.toml）**：
```toml
[server]
host = "0.0.0.0"
port = 3000
workers = 4

[database]
url = "postgres://user:pass@localhost/db"
max_connections = 10

[ai]
openai_api_key = "sk-..."
anthropic_api_key = "sk-ant-..."
default_model = "gpt-4"

[auth]
jwt_secret = "your-secret-key"
token_expiry_hours = 24

[logging]
level = "info"
format = "json"
```

### 4. 健康检查

```rust
use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub checks: HealthChecks,
}

#[derive(Serialize)]
pub struct HealthChecks {
    pub database: String,
    pub ai_providers: String,
}

pub async fn health_check(
    State(app_state): State<Arc<AppState>>,
) -> (StatusCode, Json<HealthResponse>) {
    let uptime = app_state.start_time.elapsed().as_secs();

    // 检查数据库
    let db_status = check_database(&app_state.db_pool).await;

    // 检查 AI 提供商
    let ai_status = check_ai_providers(&app_state.ai_client).await;

    let all_healthy = db_status == "healthy" && ai_status == "healthy";

    let status_code = if all_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let response = HealthResponse {
        status: if all_healthy { "healthy".to_string() } else { "unhealthy".to_string() },
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
        checks: HealthChecks {
            database: db_status,
            ai_providers: ai_status,
        },
    };

    (status_code, Json(response))
}

async fn check_database(pool: &PgPool) -> String {
    match sqlx::query("SELECT 1").fetch_one(pool).await {
        Ok(_) => "healthy".to_string(),
        Err(e) => format!("unhealthy: {}", e),
    }
}

async fn check_ai_providers(client: &AiClient) -> String {
    // 简单的连接测试
    "healthy".to_string()
}
```

### 5. 优雅关闭

```rust
use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, shutting down...");
        },
    }

    // 执行清理工作
    tracing::info!("Performing cleanup...");
    
    // 等待正在处理的请求完成
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    tracing::info!("Shutdown complete");
}
```

## 💻 实战项目：完整的生产级 AI Gateway

### 步骤 1：完整的 main.rs

```rust
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 加载配置
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    // 2. 初始化日志
    init_tracing(&config.logging);

    tracing::info!("Starting AI Gateway v{}", env!("CARGO_PKG_VERSION"));

    // 3. 初始化数据库
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    tracing::info!("Database connected");

    // 4. 运行迁移
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;

    // 5. 初始化 AI 客户端
    let ai_client = create_ai_client(&config.ai);

    // 6. 初始化工具注册表
    let tool_registry = create_tool_registry();

    // 7. 初始化速率限制器
    let rate_limiter = Arc::new(RateLimiter::new(
        100,  // 每分钟 100 次请求
        Duration::from_secs(60),
    ));

    // 8. 创建应用状态
    let app_state = Arc::new(AppState {
        config: config.clone(),
        db_pool,
        ai_client,
        tool_registry,
        start_time: Instant::now(),
    });

    // 9. 创建路由
    let app = Router::new()
        // 健康检查（无需认证）
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // API 路由（需要认证）
        .route("/api/chat", post(chat_handler))
        .route("/api/chat/stream", post(chat_stream_handler))
        .route("/api/tools", get(list_tools))
        
        // 添加中间件
        .layer(middleware::from_fn_with_state(
            rate_limiter.clone(),
            rate_limit_middleware,
        ))
        .layer(middleware::from_fn(api_key_auth))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(CorsLayer::permissive())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
        )
        .with_state(app_state);

    // 10. 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);

    // 11. 优雅关闭
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn init_tracing(config: &LoggingConfig) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.level));

    let fmt_layer = if config.format == "json" {
        tracing_subscriber::fmt::layer().json().boxed()
    } else {
        tracing_subscriber::fmt::layer().pretty().boxed()
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}
```

### 步骤 2：Docker 部署

**Dockerfile**：
```dockerfile
# 构建阶段
FROM rust:1.75 as builder

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟 main.rs 以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制源代码
COPY . .

# 构建应用
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/ai-gateway /app/ai-gateway

# 复制配置文件
COPY config.toml /app/config.toml

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["/app/ai-gateway"]
```

**docker-compose.yml**：
```yaml
version: '3.8'

services:
  ai-gateway:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgres://user:pass@postgres:5432/ai_gateway
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
      - LOG_LEVEL=info
      - LOG_FORMAT=json
    depends_on:
      - postgres
    restart: unless-stopped

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
      - POSTGRES_DB=ai_gateway
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  postgres_data:
```

**.env 文件**：
```bash
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
DATABASE_URL=postgres://user:pass@localhost:5432/ai_gateway
JWT_SECRET=your-secret-key
LOG_LEVEL=info
```

### 步骤 3：部署命令

```bash
# 构建镜像
docker-compose build

# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f ai-gateway

# 停止服务
docker-compose down

# 健康检查
curl http://localhost:3000/health
```

## 🔍 深入理解

### 生产环境检查清单

```
✅ 安全性
  - API 认证
  - HTTPS/TLS
  - 输入验证
  - SQL 注入防护
  - XSS 防护

✅ 可靠性
  - 错误处理
  - 重试机制
  - 超时控制
  - 优雅关闭
  - 健康检查

✅ 性能
  - 连接池
  - 缓存
  - 速率限制
  - 异步处理

✅ 可观测性
  - 结构化日志
  - 指标收集
  - 分布式追踪
  - 告警

✅ 可维护性
  - 配置管理
  - 文档
  - 测试覆盖
  - CI/CD
```

### 监控指标

```rust
// 关键指标
- 请求数（QPS）
- 响应时间（P50, P95, P99）
- 错误率
- 数据库连接数
- AI API 调用次数
- 令牌使用量
- 内存使用
- CPU 使用
```

## 📝 练习题

### 练习 1：实现 API Key 管理
创建 API key 的 CRUD 接口，支持创建、列表、撤销。

### 练习 2：添加请求日志
记录所有 API 请求的详细信息（用户、端点、参数、响应时间）。

### 练习 3：实现缓存
为常见查询添加 Redis 缓存。

### 练习 4：添加指标导出
使用 Prometheus 格式导出指标。

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 实现 API 认证和授权
- [ ] 添加速率限制保护
- [ ] 管理配置和环境变量
- [ ] 实现健康检查
- [ ] 实现优雅关闭
- [ ] 使用 Docker 容器化
- [ ] 编写 docker-compose 配置
- [ ] 部署到生产环境
- [ ] 监控应用状态
- [ ] 处理生产环境问题

## 🔗 延伸阅读

- [Axum 生产最佳实践](https://docs.rs/axum/)
- [Docker 最佳实践](https://docs.docker.com/develop/dev-best-practices/)
- [Kubernetes 部署](https://kubernetes.io/docs/tutorials/)
- [Prometheus 监控](https://prometheus.io/docs/introduction/overview/)

## 🎉 恭喜完成！

你已经完成了整个 AI Gateway 项目的学习！现在你具备了：

✅ 构建生产级 Rust Web 服务的能力
✅ 集成多个 AI 提供商的经验
✅ 实现 Agent 系统的知识
✅ 部署和运维的技能

**下一步建议**：
1. 将项目部署到云平台（AWS、GCP、Azure）
2. 添加更多 AI 提供商支持
3. 实现更复杂的 Agent 功能
4. 优化性能和成本
5. 开源你的项目！

---

**掌握生产特性，构建可靠的 AI 服务！** 🚀
