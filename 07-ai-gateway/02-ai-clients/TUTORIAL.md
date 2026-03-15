# 模块 7.2：AI 客户端 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 AI API 的工作原理
2. 实现 OpenAI 客户端
3. 实现 Anthropic Claude 客户端
4. 设计统一的 AI 接口抽象
5. 处理流式响应

## 🎯 为什么需要 AI 客户端抽象？

### 直接调用 vs 抽象层

**直接调用（问题）**：
```rust
// 每次都要处理 HTTP 细节
let response = reqwest::Client::new()
    .post("https://api.openai.com/v1/chat/completions")
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&serde_json::json!({
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Hello"}]
    }))
    .send()
    .await?;

// 问题：
// - 重复代码
// - 难以切换提供商
// - 错误处理分散
// - 难以测试
```

**抽象层（解决方案）**：
```rust
// 统一接口
let response = ai_client.chat("Hello").await?;

// 优势：
// - 代码简洁
// - 轻松切换提供商
// - 集中错误处理
// - 易于测试和模拟
```

### AI 提供商对比

```
OpenAI:
- 模型：GPT-4, GPT-3.5-turbo
- 特点：功能强大，生态丰富
- API：/v1/chat/completions

Anthropic Claude:
- 模型：Claude-3-opus, Claude-3-sonnet
- 特点：长上下文，安全性高
- API：/v1/messages

统一抽象的价值：
- 一套代码，多个提供商
- A/B 测试不同模型
- 故障转移
- 成本优化
```

## 📖 核心概念详解

### 1. Chat Completion API

大多数 AI 提供商使用类似的 Chat API。

#### 请求结构

```rust
use serde::{Deserialize, Serialize};

// 消息角色
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

// 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

// 聊天请求
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
}
```

#### 响应结构

```rust
// 聊天响应
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
```

### 2. 流式响应

AI 模型通常支持流式输出（逐字生成）。

```
非流式：
客户端 → 请求 → 服务器
                  ↓
              生成全部内容（等待 5 秒）
                  ↓
客户端 ← 完整响应

流式：
客户端 → 请求 → 服务器
客户端 ← "你" ← 服务器（立即）
客户端 ← "好" ← 服务器
客户端 ← "！" ← 服务器
```

**SSE 流式格式**：
```
data: {"choices":[{"delta":{"content":"你"}}]}

data: {"choices":[{"delta":{"content":"好"}}]}

data: {"choices":[{"delta":{"content":"！"}}]}

data: [DONE]
```

### 3. 统一接口设计

```rust
use async_trait::async_trait;

// 统一的 AI 提供商接口
#[async_trait]
pub trait AiProvider: Send + Sync {
    // 普通聊天
    async fn chat(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
    ) -> Result<ChatResponse, AiError>;

    // 流式聊天
    async fn chat_stream(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
    ) -> Result<impl Stream<Item = Result<String, AiError>>, AiError>;

    // 获取提供商名称
    fn provider_name(&self) -> &str;

    // 获取可用模型列表
    fn available_models(&self) -> Vec<String>;
}

// 聊天选项
#[derive(Debug, Default)]
pub struct ChatOptions {
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub system_prompt: Option<String>,
}
```

## 💻 实战项目：多提供商 AI 客户端

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "stream"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-trait = "0.1"
futures = "0.3"
tokio-stream = "0.1"
thiserror = "1"
```

### 步骤 2：定义错误类型

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AiError {
    #[error("HTTP 请求失败: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON 解析失败: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API 错误 {status}: {message}")]
    ApiError { status: u16, message: String },

    #[error("认证失败")]
    AuthError,

    #[error("速率限制")]
    RateLimitError,

    #[error("模型不可用: {0}")]
    ModelNotAvailable(String),

    #[error("流式响应错误: {0}")]
    StreamError(String),
}
```

### 步骤 3：实现 OpenAI 客户端

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

// OpenAI 特定的请求/响应结构
#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    id: String,
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
    finish_reason: String,
}

#[derive(Deserialize)]
struct OpenAiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[async_trait::async_trait]
impl AiProvider for OpenAiClient {
    async fn chat(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
    ) -> Result<ChatResponse, AiError> {
        let model = options.model
            .unwrap_or_else(|| "gpt-3.5-turbo".to_string());

        // 转换消息格式
        let openai_messages: Vec<OpenAiMessage> = messages
            .into_iter()
            .map(|m| OpenAiMessage {
                role: format!("{:?}", m.role).to_lowercase(),
                content: m.content,
            })
            .collect();

        let request = OpenAiRequest {
            model: model.clone(),
            messages: openai_messages,
            max_tokens: options.max_tokens,
            temperature: options.temperature,
            stream: None,
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // 处理错误状态码
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();

            return Err(match status {
                401 => AiError::AuthError,
                429 => AiError::RateLimitError,
                _ => AiError::ApiError { status, message: body },
            });
        }

        let openai_resp: OpenAiResponse = response.json().await?;

        // 转换为统一格式
        Ok(ChatResponse {
            id: openai_resp.id,
            content: openai_resp.choices
                .into_iter()
                .next()
                .map(|c| c.message.content)
                .unwrap_or_default(),
            model,
            usage: TokenUsage {
                prompt_tokens: openai_resp.usage.prompt_tokens,
                completion_tokens: openai_resp.usage.completion_tokens,
                total_tokens: openai_resp.usage.total_tokens,
            },
        })
    }

    fn provider_name(&self) -> &str {
        "openai"
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "gpt-4".to_string(),
            "gpt-4-turbo".to_string(),
            "gpt-3.5-turbo".to_string(),
        ]
    }
}
```

### 步骤 4：实现 Anthropic 客户端

```rust
pub struct AnthropicClient {
    client: Client,
    api_key: String,
    base_url: String,
}

// Anthropic 特定结构
#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    id: String,
    content: Vec<AnthropicContent>,
    model: String,
    usage: AnthropicUsage,
}

#[derive(Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl AiProvider for AnthropicClient {
    async fn chat(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
    ) -> Result<ChatResponse, AiError> {
        let model = options.model
            .unwrap_or_else(|| "claude-3-sonnet-20240229".to_string());

        // 分离 system 消息
        let (system_messages, chat_messages): (Vec<_>, Vec<_>) = messages
            .into_iter()
            .partition(|m| matches!(m.role, Role::System));

        let system = system_messages
            .into_iter()
            .next()
            .map(|m| m.content)
            .or(options.system_prompt);

        let anthropic_messages: Vec<AnthropicMessage> = chat_messages
            .into_iter()
            .map(|m| AnthropicMessage {
                role: format!("{:?}", m.role).to_lowercase(),
                content: m.content,
            })
            .collect();

        let request = AnthropicRequest {
            model: model.clone(),
            messages: anthropic_messages,
            max_tokens: options.max_tokens.unwrap_or(1024),
            system,
            temperature: options.temperature,
        };

        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(match status {
                401 => AiError::AuthError,
                429 => AiError::RateLimitError,
                _ => AiError::ApiError { status, message: body },
            });
        }

        let anthropic_resp: AnthropicResponse = response.json().await?;

        Ok(ChatResponse {
            id: anthropic_resp.id,
            content: anthropic_resp.content
                .into_iter()
                .filter(|c| c.content_type == "text")
                .map(|c| c.text)
                .collect::<Vec<_>>()
                .join(""),
            model,
            usage: TokenUsage {
                prompt_tokens: anthropic_resp.usage.input_tokens,
                completion_tokens: anthropic_resp.usage.output_tokens,
                total_tokens: anthropic_resp.usage.input_tokens
                    + anthropic_resp.usage.output_tokens,
            },
        })
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
        ]
    }
}
```

### 步骤 5：AI 路由器（多提供商）

```rust
use std::collections::HashMap;
use std::sync::Arc;

pub struct AiRouter {
    providers: HashMap<String, Arc<dyn AiProvider>>,
    default_provider: String,
}

impl AiRouter {
    pub fn new(default_provider: String) -> Self {
        Self {
            providers: HashMap::new(),
            default_provider,
        }
    }

    pub fn add_provider(
        mut self,
        name: String,
        provider: Arc<dyn AiProvider>,
    ) -> Self {
        self.providers.insert(name, provider);
        self
    }

    pub async fn chat(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
        provider_name: Option<&str>,
    ) -> Result<ChatResponse, AiError> {
        let name = provider_name.unwrap_or(&self.default_provider);

        let provider = self.providers.get(name)
            .ok_or_else(|| AiError::ModelNotAvailable(name.to_string()))?;

        provider.chat(messages, options).await
    }

    // 故障转移：尝试多个提供商
    pub async fn chat_with_fallback(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
        providers: Vec<&str>,
    ) -> Result<ChatResponse, AiError> {
        let mut last_error = None;

        for provider_name in providers {
            match self.chat(
                messages.clone(),
                options.clone(),
                Some(provider_name),
            ).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    tracing::warn!(
                        provider = provider_name,
                        error = %e,
                        "Provider failed, trying next"
                    );
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or(AiError::ModelNotAvailable(
            "No providers available".to_string()
        )))
    }
}
```

### 步骤 6：使用示例

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let openai = Arc::new(OpenAiClient::new(
        std::env::var("OPENAI_API_KEY")?
    ));

    let anthropic = Arc::new(AnthropicClient::new(
        std::env::var("ANTHROPIC_API_KEY")?
    ));

    // 创建路由器
    let router = AiRouter::new("openai".to_string())
        .add_provider("openai".to_string(), openai)
        .add_provider("anthropic".to_string(), anthropic);

    // 发送消息
    let messages = vec![
        Message {
            role: Role::User,
            content: "你好！请介绍一下 Rust 语言。".to_string(),
        }
    ];

    let options = ChatOptions {
        max_tokens: Some(500),
        temperature: Some(0.7),
        ..Default::default()
    };

    // 使用默认提供商
    let response = router.chat(messages.clone(), options.clone(), None).await?;
    println!("OpenAI 响应: {}", response.content);
    println!("Token 使用: {}", response.usage.total_tokens);

    // 使用特定提供商
    let response = router.chat(
        messages.clone(),
        options.clone(),
        Some("anthropic"),
    ).await?;
    println!("Anthropic 响应: {}", response.content);

    // 故障转移
    let response = router.chat_with_fallback(
        messages,
        options,
        vec!["openai", "anthropic"],
    ).await?;
    println!("最终响应: {}", response.content);

    Ok(())
}
```

## 🔍 深入理解

### 流式响应实现

```rust
use futures::Stream;
use tokio_stream::StreamExt;

impl OpenAiClient {
    pub async fn chat_stream(
        &self,
        messages: Vec<Message>,
        options: ChatOptions,
    ) -> Result<impl Stream<Item = Result<String, AiError>>, AiError> {
        let request = OpenAiRequest {
            // ...
            stream: Some(true),
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let stream = response.bytes_stream()
            .map(|chunk| {
                let chunk = chunk.map_err(AiError::HttpError)?;
                let text = String::from_utf8_lossy(&chunk);

                // 解析 SSE 格式
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data == "[DONE]" {
                            return Ok(String::new());
                        }

                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                return Ok(content.to_string());
                            }
                        }
                    }
                }

                Ok(String::new())
            })
            .filter(|result| {
                !matches!(result, Ok(s) if s.is_empty())
            });

        Ok(stream)
    }
}
```

### 重试机制

```rust
use std::time::Duration;
use tokio::time::sleep;

pub async fn chat_with_retry(
    provider: &dyn AiProvider,
    messages: Vec<Message>,
    options: ChatOptions,
    max_retries: u32,
) -> Result<ChatResponse, AiError> {
    let mut attempts = 0;

    loop {
        match provider.chat(messages.clone(), options.clone()).await {
            Ok(response) => return Ok(response),
            Err(AiError::RateLimitError) if attempts < max_retries => {
                attempts += 1;
                let delay = Duration::from_secs(2_u64.pow(attempts));
                tracing::warn!(
                    attempt = attempts,
                    delay_secs = delay.as_secs(),
                    "Rate limited, retrying"
                );
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## 📝 练习题

### 练习 1：添加缓存
```rust
// 实现响应缓存，相同的请求直接返回缓存结果
pub struct CachedAiProvider<P: AiProvider> {
    inner: P,
    cache: Arc<Mutex<HashMap<String, ChatResponse>>>,
}
```

### 练习 2：添加成本追踪
```rust
// 追踪每个请求的 token 使用和成本
pub struct CostTracker {
    total_tokens: AtomicU64,
    cost_per_token: f64,
}
```

### 练习 3：实现负载均衡
```rust
// 在多个 API key 之间轮询
pub struct LoadBalancedClient {
    clients: Vec<OpenAiClient>,
    current: AtomicUsize,
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解 AI API 的请求/响应格式
- [ ] 实现 OpenAI 客户端
- [ ] 实现 Anthropic 客户端
- [ ] 设计统一的 AI 接口
- [ ] 处理流式响应
- [ ] 实现故障转移
- [ ] 添加重试机制
- [ ] 处理认证和速率限制

## 🔗 延伸阅读

- [OpenAI API 文档](https://platform.openai.com/docs)
- [Anthropic API 文档](https://docs.anthropic.com)
- [async-trait 文档](https://docs.rs/async-trait)
- [reqwest 流式响应](https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.bytes_stream)

## 🚀 下一步

完成本模块后，继续学习：
1. 模块 7.3（Agent 核心）- 实现 Agent 推理循环
2. 模块 7.4（工具插件）- 可扩展的工具系统

---

**掌握 AI 客户端，构建智能应用！** 🤖
