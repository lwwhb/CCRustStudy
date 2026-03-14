use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Debug, Error)]
pub enum AppError {
    #[error("未授权")]
    Unauthorized,

    #[error("速率限制超出")]
    RateLimitExceeded,

    #[error("请求无效: {0}")]
    BadRequest(String),

    #[error("内部错误: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未授权"),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "速率限制超出"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "请求无效"),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "内部错误"),
        };

        let body = Json(serde_json::json!({
            "error": message,
            "details": self.to_string(),
        }));

        (status, body).into_response()
    }
}

// ============================================================================
// 认证系统
// ============================================================================

/// API Key 存储
#[derive(Clone)]
struct ApiKeyStore {
    keys: Arc<RwLock<HashMap<String, ApiKeyInfo>>>,
}

/// API Key 信息
#[derive(Debug, Clone)]
struct ApiKeyInfo {
    key: String,
    user_id: String,
    created_at: DateTime<Utc>,
    rate_limit: u32, // 每分钟请求数
}

impl ApiKeyStore {
    fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn add_key(&self, key: String, user_id: String, rate_limit: u32) {
        let info = ApiKeyInfo {
            key: key.clone(),
            user_id,
            created_at: Utc::now(),
            rate_limit,
        };
        self.keys.write().await.insert(key, info);
    }

    async fn validate(&self, key: &str) -> Option<ApiKeyInfo> {
        self.keys.read().await.get(key).cloned()
    }
}

/// 认证中间件
async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 提取 API Key
    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    // 验证 API Key
    let key_info = state
        .api_keys
        .validate(api_key)
        .await
        .ok_or(AppError::Unauthorized)?;

    info!("认证成功: user_id={}", key_info.user_id);

    // 继续处理请求
    Ok(next.run(request).await)
}

// ============================================================================
// 速率限制
// ============================================================================

/// 速率限制器（令牌桶算法）
#[derive(Clone)]
struct RateLimiter {
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
}

/// 令牌桶
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64, // 每秒补充的令牌数
    last_refill: DateTime<Utc>,
}

impl TokenBucket {
    fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_rate,
            last_refill: Utc::now(),
        }
    }

    fn refill(&mut self) {
        let now = Utc::now();
        let elapsed = (now - self.last_refill).num_milliseconds() as f64 / 1000.0;
        let new_tokens = elapsed * self.refill_rate;

        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill = now;
    }

    fn consume(&mut self, tokens: f64) -> bool {
        self.refill();

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn check(&self, key: &str, capacity: f64, refill_rate: f64) -> bool {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets
            .entry(key.to_string())
            .or_insert_with(|| TokenBucket::new(capacity, refill_rate));

        bucket.consume(1.0)
    }
}

/// 速率限制中间件
async fn rate_limit_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 获取 API Key
    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous");

    // 检查速率限制（每秒 10 个请求，容量 20）
    let allowed = state.rate_limiter.check(api_key, 20.0, 10.0).await;

    if !allowed {
        warn!("速率限制超出: api_key={}", api_key);
        return Err(AppError::RateLimitExceeded);
    }

    Ok(next.run(request).await)
}

// ============================================================================
// 应用状态
// ============================================================================

#[derive(Clone)]
struct AppState {
    api_keys: ApiKeyStore,
    rate_limiter: RateLimiter,
    metrics: Arc<RwLock<Metrics>>,
}

/// 指标
#[derive(Debug, Clone, Default)]
struct Metrics {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    unauthorized_requests: u64,
    rate_limited_requests: u64,
}

impl Metrics {
    fn record_request(&mut self, success: bool) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
    }

    fn record_unauthorized(&mut self) {
        self.unauthorized_requests += 1;
    }

    fn record_rate_limited(&mut self) {
        self.rate_limited_requests += 1;
    }
}

// ============================================================================
// 请求/响应类型
// ============================================================================

#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
    model: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
    model: String,
    request_id: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
struct MetricsResponse {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    unauthorized_requests: u64,
    rate_limited_requests: u64,
    success_rate: f64,
}

// ============================================================================
// 路由处理器
// ============================================================================

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime_seconds: 0, // 简化实现
    })
}

async fn metrics_handler(State(state): State<AppState>) -> Json<MetricsResponse> {
    let metrics = state.metrics.read().await;

    let success_rate = if metrics.total_requests > 0 {
        metrics.successful_requests as f64 / metrics.total_requests as f64 * 100.0
    } else {
        0.0
    };

    Json(MetricsResponse {
        total_requests: metrics.total_requests,
        successful_requests: metrics.successful_requests,
        failed_requests: metrics.failed_requests,
        unauthorized_requests: metrics.unauthorized_requests,
        rate_limited_requests: metrics.rate_limited_requests,
        success_rate,
    })
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    // 记录指标
    {
        let mut metrics = state.metrics.write().await;
        metrics.record_request(true);
    }

    // 验证请求
    if payload.message.is_empty() {
        return Err(AppError::BadRequest("消息不能为空".to_string()));
    }

    // 模拟处理
    tokio::time::sleep(Duration::from_millis(100)).await;

    let model = payload.model.unwrap_or_else(|| "gpt-4".to_string());
    let response_text = format!("收到消息: {}。这是模拟响应。", payload.message);

    info!("处理聊天请求成功");

    Ok(Json(ChatResponse {
        response: response_text,
        model,
        request_id: uuid::Uuid::new_v4().to_string(),
    }))
}

// ============================================================================
// 应用构建
// ============================================================================

fn create_app(state: AppState) -> Router {
    // 公开路由（无需认证）
    let public_routes = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler));

    // 受保护路由（需要认证和速率限制）
    let protected_routes = Router::new()
        .route("/api/chat", post(chat_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // 合并路由
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

// ============================================================================
// 主函数
// ============================================================================

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("启动生产级 AI Gateway");

    // 创建 API Key 存储
    let api_keys = ApiKeyStore::new();

    // 添加测试 API Key
    api_keys
        .add_key("test-key-123".to_string(), "user-1".to_string(), 100)
        .await;
    api_keys
        .add_key("test-key-456".to_string(), "user-2".to_string(), 50)
        .await;

    info!("已添加测试 API Keys");

    // 创建应用状态
    let state = AppState {
        api_keys,
        rate_limiter: RateLimiter::new(),
        metrics: Arc::new(RwLock::new(Metrics::default())),
    };

    // 创建应用
    let app = create_app(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("服务器运行在 http://127.0.0.1:3000");
    info!("健康检查: http://127.0.0.1:3000/health");
    info!("指标: http://127.0.0.1:3000/metrics");
    info!("聊天 API: POST http://127.0.0.1:3000/api/chat");
    info!("使用 Header: x-api-key: test-key-123");

    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn create_test_state() -> AppState {
        let api_keys = ApiKeyStore::new();
        AppState {
            api_keys,
            rate_limiter: RateLimiter::new(),
            metrics: Arc::new(RwLock::new(Metrics::default())),
        }
    }

    #[tokio::test]
    async fn test_health_endpoint() {
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
    async fn test_metrics_endpoint() {
        let state = create_test_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_chat_without_auth() {
        let state = create_test_state();
        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"message":"hello"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_chat_with_valid_auth() {
        let state = create_test_state();

        // 添加测试 API Key
        state
            .api_keys
            .add_key("test-key".to_string(), "test-user".to_string(), 100)
            .await;

        let app = create_app(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .header("x-api-key", "test-key")
                    .body(Body::from(r#"{"message":"hello"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_token_bucket() {
        let mut bucket = TokenBucket::new(10.0, 5.0);

        // 消耗 5 个令牌
        for _ in 0..5 {
            assert!(bucket.consume(1.0));
        }

        // 还剩 5 个令牌
        assert!(bucket.consume(5.0));

        // 没有令牌了
        assert!(!bucket.consume(1.0));
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new();

        // 前 20 个请求应该成功（容量为 20）
        for _ in 0..20 {
            assert!(limiter.check("test-key", 20.0, 10.0).await);
        }

        // 第 21 个请求应该失败
        assert!(!limiter.check("test-key", 20.0, 10.0).await);
    }

    #[tokio::test]
    async fn test_api_key_validation() {
        let store = ApiKeyStore::new();

        store
            .add_key("valid-key".to_string(), "user-1".to_string(), 100)
            .await;

        assert!(store.validate("valid-key").await.is_some());
        assert!(store.validate("invalid-key").await.is_none());
    }

    #[tokio::test]
    async fn test_metrics_recording() {
        let mut metrics = Metrics::default();

        metrics.record_request(true);
        metrics.record_request(true);
        metrics.record_request(false);

        assert_eq!(metrics.total_requests, 3);
        assert_eq!(metrics.successful_requests, 2);
        assert_eq!(metrics.failed_requests, 1);
    }
}
