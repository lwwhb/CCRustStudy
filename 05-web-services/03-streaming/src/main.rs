use axum::{
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _;

/// SSE 端点 - 发送时间戳
async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Event::default().data(timestamp)
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// SSE 端点 - 计数器
async fn counter_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::iter(0..10)
        .throttle(Duration::from_millis(500))
        .map(|count| Event::default().data(format!("Count: {}", count)))
        .map(Ok);

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// SSE 端点 - JSON 数据
async fn json_stream_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::iter(0..5)
        .throttle(Duration::from_secs(1))
        .map(|i| {
            let json = serde_json::json!({
                "id": i,
                "message": format!("Message {}", i),
                "timestamp": chrono::Local::now().timestamp()
            });
            Event::default()
                .json_data(json)
                .unwrap_or_else(|_| Event::default().data("error"))
        })
        .map(Ok);

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// 健康检查
async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    println!("=== 流式处理与 SSE 演示 ===\n");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/sse/time", get(sse_handler))
        .route("/sse/counter", get(counter_handler))
        .route("/sse/json", get(json_stream_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("服务器运行在 http://127.0.0.1:3001");
    println!("\n可用端点:");
    println!("  GET /health        - 健康检查");
    println!("  GET /sse/time      - 时间戳流");
    println!("  GET /sse/counter   - 计数器流");
    println!("  GET /sse/json      - JSON 数据流");
    println!("\n使用 curl 测试:");
    println!("  curl http://127.0.0.1:3001/sse/counter");
    println!("\n按 Ctrl+C 停止服务器");

    axum::serve(listener, app).await.unwrap();
}

// 由于 SSE 需要实际的 HTTP 连接，我们只测试基本功能
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_health() {
        let result = health_check().await;
        assert_eq!(result, "OK");
    }
}
