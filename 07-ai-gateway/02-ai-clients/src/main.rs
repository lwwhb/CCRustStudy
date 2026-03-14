use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use futures::Stream;
use thiserror::Error;

// ============================================================================
// 错误类型
// ============================================================================

/// AI 客户端错误
#[derive(Debug, Error)]
pub enum AIError {
    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("API 错误: {0}")]
    ApiError(String),

    #[error("解析错误: {0}")]
    ParseError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),
}

// ============================================================================
// 数据结构
// ============================================================================

/// 消息角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// AI 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    /// 响应内容
    pub content: String,
    /// 使用的模型
    pub model: String,
    /// Token 使用情况
    pub usage: Option<TokenUsage>,
}

/// Token 使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// 流式响应块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// 内容片段
    pub content: String,
    /// 是否完成
    pub done: bool,
}

// ============================================================================
// AI Provider Trait
// ============================================================================

/// AI Provider 统一接口
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// 发送聊天请求（非流式）
    async fn chat(&self, messages: Vec<Message>) -> Result<AIResponse, AIError>;

    /// 发送聊天请求（流式）
    async fn stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk, AIError>> + Send>>, AIError>;

    /// 获取 Provider 名称
    fn name(&self) -> &str;

    /// 获取默认模型
    fn default_model(&self) -> &str;
}

// ============================================================================
// OpenAI 客户端
// ============================================================================

/// OpenAI 客户端
pub struct OpenAIClient {
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-4".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl AIProvider for OpenAIClient {
    async fn chat(&self, messages: Vec<Message>) -> Result<AIResponse, AIError> {
        // 模拟 API 调用
        println!("[OpenAI] 发送请求到模型: {}", self.model);
        println!("[OpenAI] 消息数量: {}", messages.len());

        // 模拟延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 构造模拟响应
        let last_message = messages.last()
            .ok_or_else(|| AIError::ApiError("没有消息".to_string()))?;

        let response_content = format!(
            "[OpenAI 模拟响应] 收到您的消息: {}",
            last_message.content
        );

        Ok(AIResponse {
            content: response_content,
            model: self.model.clone(),
            usage: Some(TokenUsage {
                prompt_tokens: 50,
                completion_tokens: 30,
                total_tokens: 80,
            }),
        })
    }

    async fn stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk, AIError>> + Send>>, AIError> {
        println!("[OpenAI] 开始流式响应");

        let last_message = messages.last()
            .ok_or_else(|| AIError::ApiError("没有消息".to_string()))?
            .content
            .clone();

        // 创建模拟流
        let stream = futures::stream::iter(vec![
            Ok(StreamChunk {
                content: "[OpenAI] ".to_string(),
                done: false,
            }),
            Ok(StreamChunk {
                content: "流式".to_string(),
                done: false,
            }),
            Ok(StreamChunk {
                content: "响应: ".to_string(),
                done: false,
            }),
            Ok(StreamChunk {
                content: last_message,
                done: false,
            }),
            Ok(StreamChunk {
                content: String::new(),
                done: true,
            }),
        ]);

        Ok(Box::pin(stream))
    }

    fn name(&self) -> &str {
        "OpenAI"
    }

    fn default_model(&self) -> &str {
        &self.model
    }
}

// ============================================================================
// Anthropic 客户端
// ============================================================================

/// Anthropic Claude 客户端
pub struct AnthropicClient {
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-3-5-sonnet-20241022".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl AIProvider for AnthropicClient {
    async fn chat(&self, messages: Vec<Message>) -> Result<AIResponse, AIError> {
        // 模拟 API 调用
        println!("[Anthropic] 发送请求到模型: {}", self.model);
        println!("[Anthropic] 消息数量: {}", messages.len());

        // 模拟延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(120)).await;

        // 构造模拟响应
        let last_message = messages.last()
            .ok_or_else(|| AIError::ApiError("没有消息".to_string()))?;

        let response_content = format!(
            "[Anthropic 模拟响应] 理解您的问题: {}",
            last_message.content
        );

        Ok(AIResponse {
            content: response_content,
            model: self.model.clone(),
            usage: Some(TokenUsage {
                prompt_tokens: 45,
                completion_tokens: 35,
                total_tokens: 80,
            }),
        })
    }

    async fn stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk, AIError>> + Send>>, AIError> {
        println!("[Anthropic] 开始流式响应");

        let last_message = messages.last()
            .ok_or_else(|| AIError::ApiError("没有消息".to_string()))?
            .content
            .clone();

        // 创建模拟流
        let stream = futures::stream::iter(vec![
            Ok(StreamChunk {
                content: "[Claude] ".to_string(),
                done: false,
            }),
            Ok(StreamChunk {
                content: "正在".to_string(),
                done: false,
            }),
            Ok(StreamChunk {
                content: "思考: ".to_string(),
                done: false,
            }),
            Ok(StreamChunk {
                content: last_message,
                done: false,
            }),
            Ok(StreamChunk {
                content: String::new(),
                done: true,
            }),
        ]);

        Ok(Box::pin(stream))
    }

    fn name(&self) -> &str {
        "Anthropic"
    }

    fn default_model(&self) -> &str {
        &self.model
    }
}

// ============================================================================
// Provider 工厂
// ============================================================================

/// Provider 类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProviderType {
    OpenAI,
    Anthropic,
}

/// Provider 工厂
pub struct ProviderFactory;

impl ProviderFactory {
    /// 创建 Provider
    pub fn create(
        provider_type: ProviderType,
        api_key: String,
    ) -> Box<dyn AIProvider> {
        match provider_type {
            ProviderType::OpenAI => Box::new(OpenAIClient::new(api_key)),
            ProviderType::Anthropic => Box::new(AnthropicClient::new(api_key)),
        }
    }
}

// ============================================================================
// 主函数
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), AIError> {
    println!("=== AI Provider 客户端演示 ===\n");

    // 创建测试消息
    let messages = vec![
        Message::system("你是一个有帮助的助手"),
        Message::user("什么是 Rust 语言？"),
    ];

    // 测试 OpenAI
    println!("--- OpenAI 客户端 ---");
    let openai = OpenAIClient::new("test-key".to_string());
    let response = openai.chat(messages.clone()).await?;
    println!("响应: {}", response.content);
    println!("模型: {}", response.model);
    if let Some(usage) = response.usage {
        println!("Token 使用: {}", usage.total_tokens);
    }
    println!();

    // 测试 Anthropic
    println!("--- Anthropic 客户端 ---");
    let anthropic = AnthropicClient::new("test-key".to_string());
    let response = anthropic.chat(messages.clone()).await?;
    println!("响应: {}", response.content);
    println!("模型: {}", response.model);
    println!();

    // 测试流式响应
    println!("--- 流式响应测试 ---");
    use futures::StreamExt;

    let mut stream = openai.stream(messages.clone()).await?;
    print!("OpenAI 流式: ");
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(chunk) => {
                if !chunk.done {
                    print!("{}", chunk.content);
                }
            }
            Err(e) => eprintln!("错误: {}", e),
        }
    }
    println!("\n");

    // 测试工厂模式
    println!("--- Provider 工厂 ---");
    let provider = ProviderFactory::create(ProviderType::OpenAI, "test-key".to_string());
    println!("创建的 Provider: {}", provider.name());
    println!("默认模型: {}", provider.default_model());

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_openai_chat() {
        let client = OpenAIClient::new("test-key".to_string());
        let messages = vec![Message::user("Hello")];

        let response = client.chat(messages).await.unwrap();
        assert!(response.content.contains("OpenAI"));
        assert_eq!(response.model, "gpt-4");
    }

    #[tokio::test]
    async fn test_anthropic_chat() {
        let client = AnthropicClient::new("test-key".to_string());
        let messages = vec![Message::user("Hello")];

        let response = client.chat(messages).await.unwrap();
        assert!(response.content.contains("Anthropic"));
    }

    #[tokio::test]
    async fn test_stream() {
        let client = OpenAIClient::new("test-key".to_string());
        let messages = vec![Message::user("Test")];

        let mut stream = client.stream(messages).await.unwrap();
        let mut chunks = Vec::new();

        while let Some(chunk) = stream.next().await {
            chunks.push(chunk.unwrap());
        }

        assert!(!chunks.is_empty());
        assert!(chunks.last().unwrap().done);
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::user("test");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content, "test");
    }

    #[test]
    fn test_provider_factory() {
        let provider = ProviderFactory::create(
            ProviderType::OpenAI,
            "test-key".to_string(),
        );
        assert_eq!(provider.name(), "OpenAI");
    }
}
