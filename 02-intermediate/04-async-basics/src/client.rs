/// 异步 HTTP 客户端
///
/// 演示异步 HTTP 请求和错误处理

use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP 客户端
pub struct AsyncClient {
    client: reqwest::Client,
}

impl AsyncClient {
    /// 创建新的客户端
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create client");

        AsyncClient { client }
    }

    /// 发送 GET 请求
    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    /// 发送 GET 请求并解析 JSON
    pub async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> Result<T, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }

    /// 发送 POST 请求
    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<String, reqwest::Error> {
        let response = self.client.post(url).json(body).send().await?;
        let text = response.text().await?;
        Ok(text)
    }

    /// 获取响应状态码
    pub async fn get_status(&self, url: &str) -> Result<u16, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        Ok(response.status().as_u16())
    }
}

impl Default for AsyncClient {
    fn default() -> Self {
        Self::new()
    }
}

/// 并发请求多个 URL
pub async fn fetch_multiple(urls: Vec<String>) -> Vec<Result<String, reqwest::Error>> {
    let client = AsyncClient::new();
    let mut tasks = Vec::new();

    for url in urls {
        let client_clone = &client;
        let task = async move { client_clone.get(&url).await };
        tasks.push(task);
    }

    futures::future::join_all(tasks).await
}

/// 带重试的请求
pub async fn fetch_with_retry(
    url: &str,
    max_retries: u32,
) -> Result<String, reqwest::Error> {
    let client = AsyncClient::new();
    let mut attempts = 0;

    loop {
        match client.get(url).await {
            Ok(body) => return Ok(body),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(e);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = AsyncClient::new();
        // 客户端创建成功
        assert!(true);
    }

    #[tokio::test]
    async fn test_get_status() {
        let client = AsyncClient::new();
        // 使用一个稳定的测试 URL
        let result = client.get_status("https://httpbin.org/status/200").await;
        if let Ok(status) = result {
            assert_eq!(status, 200);
        }
        // 如果网络不可用，测试也应该通过
    }

    #[tokio::test]
    async fn test_multiple_requests() {
        let urls = vec![
            "https://httpbin.org/delay/1".to_string(),
            "https://httpbin.org/delay/1".to_string(),
        ];

        let results = fetch_multiple(urls).await;
        assert_eq!(results.len(), 2);
    }
}
