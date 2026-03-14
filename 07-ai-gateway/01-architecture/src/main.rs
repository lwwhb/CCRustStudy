use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use uuid::Uuid;

// ============================================================================
// 数据结构
// ============================================================================

/// Gateway 应用状态
#[derive(Clone)]
struct GatewayState {
    /// 请求计数器
    request_count: Arc<RwLock<u64>>,
    /// Gateway 配置
    config: Arc<GatewayConfig>,
}

/// Gateway 配置
#[derive(Debug, Clone)]
struct GatewayConfig {
    /// 服务名称
    service_name: String,
    /// 版本号
    version: String,
    /// 最大请求大小（字节）
    max_request_size: usize,
}

/// 聊天请求
#[derive(Debug, Deserialize)]
struct ChatRequest {
    /// 用户消息
    message: String,
    /// 可选的模型名称
    model: Option<String>,
}

/// 聊天响应
#[derive(Debug, Serialize)]
struct ChatResponse {
    /// 请求 ID
    request_id: String,
    /// AI 响应
    response: String,
    /// 使用的模型
    model: String,
}

/// 健康检查响应
#[derive(Debug, Serialize)]
struct HealthResponse {
    /// 服务状态
    status: String,
    /// 服务名称
    service: String,
    /// 版本号
    version: String,
    /// 总请求数
    total_requests: u64,
}

/// 错误响应
#[derive(Debug, Serialize)]
struct ErrorResponse {
    /// 错误消息
    error: String,
    /// 错误代码
    code: String,
}

// ============================================================================
// 中间件
// ============================================================================

/// 请求日志中间件
async fn logging_middleware(
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    info!("收到请求: {} {}", method, uri);

    let response = next.run(request).await;

    info!("响应状态: {}", response.status());

    response
}

/// 请求 ID 中间件
async fn request_id_middleware(
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();

    // 将请求 ID 添加到请求头
    request.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );

    info!("请求 ID: {}", request_id);

    next.run(request).await
}

// ============================================================================
// 路由处理器
// ============================================================================

/// 健康检查端点
async fn health_handler(
    State(state): State<GatewayState>,
) -> Json<HealthResponse> {
    let count = *state.request_count.read().await;

    Json(HealthResponse {
        status: "healthy".to_string(),
        service: state.config.service_name.clone(),
        version: state.config.version.clone(),
        total_requests: count,
    })
}

/// 聊天端点
async fn chat_handler(
    State(state): State<GatewayState>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, AppError> {
    // 增加请求计数
    {
        let mut count = state.request_count.write().await;
        *count += 1;
    }

    // 验证请求
    if payload.message.is_empty() {
        return Err(AppError::BadRequest("消息不能为空".to_string()));
    }

    if payload.message.len() > state.config.max_request_size {
        return Err(AppError::BadRequest("消息过长".to_string()));
    }

    // 模拟 AI 处理
    let model = payload.model.unwrap_or_else(|| "gpt-4".to_string());
    let response_text = format!("收到消息: {}。这是一个模拟响应。", payload.message);

    info!("处理聊天请求，使用模型: {}", model);

    // 模拟处理延迟
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(Json(ChatResponse {
        request_id: Uuid::new_v4().to_string(),
        response: response_text,
        model,
    }))
}

/// 根路径处理器
async fn root_handler() -> &'static str {
    "AI Gateway - 欢迎使用"
}

// ============================================================================
// 错误处理
// ============================================================================

/// 应用错误类型
#[derive(Debug)]
enum AppError {
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg)
            }
            AppError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg)
            }
        };

        warn!("错误: {} - {}", code, message);

        let body = Json(ErrorResponse {
            error: message,
            code: code.to_string(),
        });

        (status, body).into_response()
    }
}

// ============================================================================
// 应用构建
// ============================================================================

/// 创建 Gateway 应用
fn create_app(state: GatewayState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/chat", post(chat_handler))
        .layer(middleware::from_fn(logging_middleware))
        .layer(middleware::from_fn(request_id_middleware))
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

    // 创建配置
    let config = GatewayConfig {
        service_name: "AI Gateway".to_string(),
        version: "0.1.0".to_string(),
        max_request_size: 10000,
    };

    // 创建应用状态
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

    info!("🚀 AI Gateway 启动在 http://127.0.0.1:3000");

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

    fn create_test_state() -> GatewayState {
        let config = GatewayConfig {
            service_name: "Test Gateway".to_string(),
            version: "0.1.0".to_string(),
            max_request_size: 1000,
        };

        GatewayState {
            request_count: Arc::new(RwLock::new(0)),
            config: Arc::new(config),
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
    async fn test_chat_endpoint() {
        let state = create_test_state();
        let app = create_app(state);

        let request_body = serde_json::json!({
            "message": "Hello",
            "model": "gpt-4"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_empty_message_error() {
        let state = create_test_state();
        let app = create_app(state);

        let request_body = serde_json::json!({
            "message": ""
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_request_counter() {
        let state = create_test_state();
        let app = create_app(state.clone());

        // 发送第一个请求
        let request_body = serde_json::json!({
            "message": "Test"
        });

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/chat")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 检查计数器
        let count = *state.request_count.read().await;
        assert_eq!(count, 1);
    }
}
