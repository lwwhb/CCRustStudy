# 模块 7.3：Agent 核心 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解 AI Agent 的工作原理
2. 实现工具调用机制
3. 掌握状态管理
4. 实现多步推理循环
5. 处理对话历史

## 🎯 为什么需要 Agent？

### 简单 LLM vs Agent

**简单 LLM（受限）**：
```
用户：今天北京天气怎么样？
LLM：抱歉，我无法获取实时天气信息。

问题：
- 只能基于训练数据回答
- 无法访问外部信息
- 无法执行操作
- 功能受限
```

**Agent（强大）**：
```
用户：今天北京天气怎么样？
Agent：
  1. 识别需要调用天气 API
  2. 调用 get_weather("北京")
  3. 获取结果：晴天，25°C
  4. 生成回答：今天北京天气晴朗，温度 25°C

优势：
- 可以使用工具
- 访问实时数据
- 执行实际操作
- 功能可扩展
```

### Agent 的工作流程

```
┌─────────────────────────────────────┐
│         用户输入                     │
└──────────────┬──────────────────────┘
               ↓
┌─────────────────────────────────────┐
│    LLM 分析：需要什么工具？          │
└──────────────┬──────────────────────┘
               ↓
        ┌──────┴──────┐
        │  需要工具？  │
        └──────┬──────┘
          是 ↙   ↘ 否
            ↓       ↓
    ┌───────────┐  ┌──────────┐
    │ 调用工具  │  │ 直接回答 │
    └─────┬─────┘  └────┬─────┘
          ↓              ↓
    ┌───────────┐  ┌──────────┐
    │ 获取结果  │  │   结束   │
    └─────┬─────┘  └──────────┘
          ↓
    ┌───────────┐
    │ 继续推理  │
    └─────┬─────┘
          ↓
      （循环）
```

## 📖 核心概念详解

### 1. 工具（Tool）

工具是 Agent 可以调用的函数。

#### 工具定义

```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

// 工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
}

// 工具参数（JSON Schema）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: serde_json::Map<String, Value>,
    pub required: Vec<String>,
}

// 工具调用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

// 工具调用结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub output: String,
}
```

#### 工具示例

```rust
// 天气查询工具
let weather_tool = Tool {
    name: "get_weather".to_string(),
    description: "获取指定城市的天气信息".to_string(),
    parameters: ToolParameters {
        param_type: "object".to_string(),
        properties: serde_json::json!({
            "city": {
                "type": "string",
                "description": "城市名称，如：北京、上海"
            }
        }).as_object().unwrap().clone(),
        required: vec!["city".to_string()],
    },
};

// 计算器工具
let calculator_tool = Tool {
    name: "calculate".to_string(),
    description: "执行数学计算".to_string(),
    parameters: ToolParameters {
        param_type: "object".to_string(),
        properties: serde_json::json!({
            "expression": {
                "type": "string",
                "description": "数学表达式，如：2 + 2"
            }
        }).as_object().unwrap().clone(),
        required: vec!["expression".to_string()],
    },
};
```

### 2. 对话历史

Agent 需要维护对话上下文。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

// 对话历史
pub struct ConversationHistory {
    messages: Vec<Message>,
    max_messages: usize,
}

impl ConversationHistory {
    pub fn new(max_messages: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_messages,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);

        // 保持历史在限制内（保留系统消息）
        if self.messages.len() > self.max_messages {
            let system_messages: Vec<_> = self.messages
                .iter()
                .filter(|m| matches!(m.role, MessageRole::System))
                .cloned()
                .collect();

            let recent_messages: Vec<_> = self.messages
                .iter()
                .filter(|m| !matches!(m.role, MessageRole::System))
                .rev()
                .take(self.max_messages - system_messages.len())
                .cloned()
                .collect();

            self.messages = system_messages;
            self.messages.extend(recent_messages.into_iter().rev());
        }
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
```

### 3. Agent 状态

```rust
#[derive(Debug, Clone)]
pub enum AgentState {
    Idle,                    // 空闲
    Thinking,                // 思考中
    CallingTool(String),     // 调用工具
    Responding,              // 生成响应
    Error(String),           // 错误状态
    Finished,                // 完成
}

pub struct AgentContext {
    pub state: AgentState,
    pub history: ConversationHistory,
    pub current_iteration: usize,
    pub max_iterations: usize,
}

impl AgentContext {
    pub fn new(max_iterations: usize, max_history: usize) -> Self {
        Self {
            state: AgentState::Idle,
            history: ConversationHistory::new(max_history),
            current_iteration: 0,
            max_iterations,
        }
    }

    pub fn can_continue(&self) -> bool {
        self.current_iteration < self.max_iterations
            && !matches!(self.state, AgentState::Finished | AgentState::Error(_))
    }
}
```

### 4. 推理循环

Agent 的核心是推理循环（ReAct 模式）。

```
ReAct = Reasoning + Acting

循环步骤：
1. Reason（推理）：分析当前情况，决定下一步
2. Act（行动）：执行工具调用或生成回答
3. Observe（观察）：查看行动结果
4. 重复直到完成
```

**示例对话**：
```
用户：帮我查一下北京和上海的天气，然后告诉我哪个更适合旅游

迭代 1:
  Reason: 需要查询两个城市的天气
  Act: 调用 get_weather("北京")
  Observe: 北京：晴天，25°C

迭代 2:
  Reason: 还需要查询上海天气
  Act: 调用 get_weather("上海")
  Observe: 上海：雨天，18°C

迭代 3:
  Reason: 已有所有信息，可以回答
  Act: 生成回答
  Output: 北京天气更好，更适合旅游
```

## 💻 实战项目：完整的 Agent 系统

### 步骤 1：工具注册表

```rust
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

// 工具执行器 trait
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(&self, arguments: Value) -> Result<String, String>;
}

// 工具注册表
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn ToolExecutor>>,
    definitions: HashMap<String, Tool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    // 注册工具
    pub fn register(
        &mut self,
        tool: Tool,
        executor: Arc<dyn ToolExecutor>,
    ) {
        let name = tool.name.clone();
        self.definitions.insert(name.clone(), tool);
        self.tools.insert(name, executor);
    }

    // 执行工具
    pub async fn execute(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<String, String> {
        let executor = self.tools
            .get(name)
            .ok_or_else(|| format!("工具不存在: {}", name))?;

        executor.execute(arguments).await
    }

    // 获取所有工具定义
    pub fn get_tool_definitions(&self) -> Vec<Tool> {
        self.definitions.values().cloned().collect()
    }
}
```

### 步骤 2：实现具体工具

```rust
// 天气工具
pub struct WeatherTool;

#[async_trait]
impl ToolExecutor for WeatherTool {
    async fn execute(&self, arguments: Value) -> Result<String, String> {
        let city = arguments
            .get("city")
            .and_then(|v| v.as_str())
            .ok_or("缺少 city 参数")?;

        // 模拟 API 调用
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 返回模拟数据
        let weather = match city {
            "北京" => "晴天，25°C",
            "上海" => "多云，22°C",
            "深圳" => "雨天，28°C",
            _ => "未知城市",
        };

        Ok(format!("{}的天气：{}", city, weather))
    }
}

// 计算器工具
pub struct CalculatorTool;

#[async_trait]
impl ToolExecutor for CalculatorTool {
    async fn execute(&self, arguments: Value) -> Result<String, String> {
        let expression = arguments
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or("缺少 expression 参数")?;

        // 简单的计算器实现
        let result = evaluate_expression(expression)
            .map_err(|e| format!("计算错误: {}", e))?;

        Ok(format!("{} = {}", expression, result))
    }
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // 简化实现，实际应使用表达式解析库
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("无效的表达式".to_string());
    }

    let a: f64 = parts[0].parse().map_err(|_| "无效的数字")?;
    let op = parts[1];
    let b: f64 = parts[2].parse().map_err(|_| "无效的数字")?;

    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("除数不能为零".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("不支持的运算符: {}", op)),
    }
}
```

### 步骤 3：实现 Agent 核心

```rust
use crate::ai_client::{AiProvider, Message as AiMessage, ChatOptions};

pub struct Agent {
    ai_provider: Arc<dyn AiProvider>,
    tool_registry: Arc<ToolRegistry>,
    system_prompt: String,
}

impl Agent {
    pub fn new(
        ai_provider: Arc<dyn AiProvider>,
        tool_registry: Arc<ToolRegistry>,
    ) -> Self {
        let system_prompt = "你是一个有用的助手。\
            当需要获取信息或执行操作时，你可以使用可用的工具。\
            仔细分析用户的请求，决定是否需要使用工具。".to_string();

        Self {
            ai_provider,
            tool_registry,
            system_prompt,
        }
    }

    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = prompt;
        self
    }

    // 运行 Agent
    pub async fn run(
        &self,
        user_input: String,
        max_iterations: usize,
    ) -> Result<String, AgentError> {
        let mut context = AgentContext::new(max_iterations, 50);

        // 添加系统消息
        context.history.add_message(Message {
            role: MessageRole::System,
            content: self.system_prompt.clone(),
            tool_calls: None,
            tool_call_id: None,
        });

        // 添加用户消息
        context.history.add_message(Message {
            role: MessageRole::User,
            content: user_input,
            tool_calls: None,
            tool_call_id: None,
        });

        // 推理循环
        while context.can_continue() {
            context.current_iteration += 1;
            context.state = AgentState::Thinking;

            tracing::debug!(
                iteration = context.current_iteration,
                "Agent 推理循环"
            );

            // 调用 LLM
            let response = self.call_llm(&context).await?;

            // 检查是否需要调用工具
            if let Some(tool_calls) = response.tool_calls {
                context.state = AgentState::CallingTool(
                    tool_calls[0].name.clone()
                );

                // 添加助手消息（包含工具调用）
                context.history.add_message(Message {
                    role: MessageRole::Assistant,
                    content: String::new(),
                    tool_calls: Some(tool_calls.clone()),
                    tool_call_id: None,
                });

                // 执行所有工具调用
                for tool_call in tool_calls {
                    let result = self.execute_tool(&tool_call).await?;

                    // 添加工具结果
                    context.history.add_message(Message {
                        role: MessageRole::Tool,
                        content: result.output,
                        tool_calls: None,
                        tool_call_id: Some(result.tool_call_id),
                    });
                }
            } else {
                // 没有工具调用，返回最终答案
                context.state = AgentState::Finished;

                context.history.add_message(Message {
                    role: MessageRole::Assistant,
                    content: response.content.clone(),
                    tool_calls: None,
                    tool_call_id: None,
                });

                return Ok(response.content);
            }
        }

        Err(AgentError::MaxIterationsReached)
    }

    // 调用 LLM
    async fn call_llm(
        &self,
        context: &AgentContext,
    ) -> Result<AgentResponse, AgentError> {
        // 转换消息格式
        let messages: Vec<AiMessage> = context
            .history
            .get_messages()
            .iter()
            .map(|m| AiMessage {
                role: match m.role {
                    MessageRole::System => crate::ai_client::Role::System,
                    MessageRole::User => crate::ai_client::Role::User,
                    MessageRole::Assistant => crate::ai_client::Role::Assistant,
                    MessageRole::Tool => crate::ai_client::Role::User,
                },
                content: m.content.clone(),
            })
            .collect();

        // 获取工具定义
        let tools = self.tool_registry.get_tool_definitions();

        // 调用 AI
        let options = ChatOptions {
            model: None,
            max_tokens: Some(1000),
            temperature: Some(0.7),
            system_prompt: None,
        };

        let response = self.ai_provider
            .chat(messages, options)
            .await
            .map_err(|e| AgentError::AiError(e.to_string()))?;

        // 解析响应
        Ok(self.parse_response(response))
    }

    // 执行工具
    async fn execute_tool(
        &self,
        tool_call: &ToolCall,
    ) -> Result<ToolResult, AgentError> {
        tracing::info!(
            tool = %tool_call.name,
            arguments = ?tool_call.arguments,
            "执行工具"
        );

        let output = self.tool_registry
            .execute(&tool_call.name, tool_call.arguments.clone())
            .await
            .map_err(|e| AgentError::ToolError(e))?;

        Ok(ToolResult {
            tool_call_id: tool_call.id.clone(),
            output,
        })
    }

    // 解析 AI 响应
    fn parse_response(&self, response: ChatResponse) -> AgentResponse {
        // 简化实现，实际需要解析工具调用
        AgentResponse {
            content: response.choices[0].message.content.clone(),
            tool_calls: None,
        }
    }
}

#[derive(Debug)]
struct AgentResponse {
    content: String,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("AI 错误: {0}")]
    AiError(String),

    #[error("工具错误: {0}")]
    ToolError(String),

    #[error("达到最大迭代次数")]
    MaxIterationsReached,
}
```

### 步骤 4：使用示例

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化追踪
    tracing_subscriber::fmt::init();

    println!("=== Agent 系统演示 ===\n");

    // 创建 AI 客户端
    let ai_client = Arc::new(OpenAiClient::new(
        std::env::var("OPENAI_API_KEY")?
    ));

    // 创建工具注册表
    let mut registry = ToolRegistry::new();

    // 注册天气工具
    registry.register(
        Tool {
            name: "get_weather".to_string(),
            description: "获取指定城市的天气信息".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "city": {
                        "type": "string",
                        "description": "城市名称"
                    }
                }).as_object().unwrap().clone(),
                required: vec!["city".to_string()],
            },
        },
        Arc::new(WeatherTool),
    );

    // 注册计算器工具
    registry.register(
        Tool {
            name: "calculate".to_string(),
            description: "执行数学计算".to_string(),
            parameters: ToolParameters {
                param_type: "object".to_string(),
                properties: serde_json::json!({
                    "expression": {
                        "type": "string",
                        "description": "数学表达式"
                    }
                }).as_object().unwrap().clone(),
                required: vec!["expression".to_string()],
            },
        },
        Arc::new(CalculatorTool),
    );

    let registry = Arc::new(registry);

    // 创建 Agent
    let agent = Agent::new(ai_client, registry);

    // 测试用例
    let test_cases = vec![
        "北京今天天气怎么样？",
        "帮我计算 123 + 456",
        "查一下北京和上海的天气，然后告诉我哪个更适合旅游",
    ];

    for (i, input) in test_cases.iter().enumerate() {
        println!("测试 {}: {}", i + 1, input);
        println!("---");

        match agent.run(input.to_string(), 5).await {
            Ok(response) => {
                println!("回答: {}\n", response);
            }
            Err(e) => {
                eprintln!("错误: {}\n", e);
            }
        }
    }

    Ok(())
}
```

## 🔍 深入理解

### Agent 的设计模式

```
1. ReAct（Reasoning + Acting）
   - 交替进行推理和行动
   - 适合需要多步骤的任务

2. Plan-and-Execute
   - 先制定完整计划
   - 再逐步执行
   - 适合复杂任务

3. Reflexion
   - 执行后反思
   - 从错误中学习
   - 适合需要优化的任务
```

### 工具调用的挑战

```
1. 参数验证
   - JSON Schema 验证
   - 类型检查
   - 必填字段检查

2. 错误处理
   - 工具执行失败
   - 超时处理
   - 重试机制

3. 安全性
   - 权限控制
   - 输入清理
   - 输出过滤

4. 性能
   - 并行执行工具
   - 缓存结果
   - 限流
```

## 📝 练习题

### 练习 1：添加新工具
实现一个搜索工具：
```rust
pub struct SearchTool;

#[async_trait]
impl ToolExecutor for SearchTool {
    async fn execute(&self, arguments: Value) -> Result<String, String> {
        // 你的代码
    }
}
```

### 练习 2：改进推理循环
添加以下功能：
- 并行执行多个工具
- 工具执行超时
- 自动重试失败的工具

### 练习 3：实现 Plan-and-Execute
实现一个先规划再执行的 Agent：
```rust
pub struct PlanExecuteAgent {
    // 你的代码
}
```

## 🎯 学习检查清单

- [ ] 理解 Agent 的工作原理
- [ ] 实现工具定义和注册
- [ ] 实现工具执行器
- [ ] 管理对话历史
- [ ] 实现推理循环
- [ ] 处理工具调用结果
- [ ] 实现错误处理和重试
- [ ] 理解不同的 Agent 模式

## 🔗 延伸阅读

- [ReAct 论文](https://arxiv.org/abs/2210.03629)
- [LangChain Agents](https://python.langchain.com/docs/modules/agents/)
- [OpenAI Function Calling](https://platform.openai.com/docs/guides/function-calling)

## 🚀 下一步

完成本模块后，继续学习：
- 模块 7.4：工具插件系统
- 模块 7.5：生产特性

---

**掌握 Agent 核心，构建智能系统！** 🤖
