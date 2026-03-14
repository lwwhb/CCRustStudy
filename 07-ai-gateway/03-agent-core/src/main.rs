use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ============================================================================
// 错误类型
// ============================================================================

/// Agent 错误
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("工具未找到: {0}")]
    ToolNotFound(String),

    #[error("工具执行失败: {0}")]
    ToolExecutionError(String),

    #[error("推理失败: {0}")]
    ReasoningError(String),

    #[error("达到最大迭代次数")]
    MaxIterationsReached,
}

// ============================================================================
// 工具系统
// ============================================================================

/// 工具执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// 是否成功
    pub success: bool,
    /// 结果内容
    pub output: String,
    /// 错误信息（如果失败）
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
        }
    }
}

/// 工具接口
#[async_trait]
pub trait Tool: Send + Sync {
    /// 工具名称
    fn name(&self) -> &str;

    /// 工具描述
    fn description(&self) -> &str;

    /// 执行工具
    async fn execute(&self, input: &str) -> Result<ToolResult, AgentError>;
}

// ============================================================================
// 内置工具
// ============================================================================

/// 计算器工具
pub struct Calculator;

#[async_trait]
impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "执行数学计算。输入格式: '数字1 运算符 数字2'，例如 '5 + 3'"
    }

    async fn execute(&self, input: &str) -> Result<ToolResult, AgentError> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            return Ok(ToolResult::error(
                "输入格式错误，应为: 数字1 运算符 数字2".to_string()
            ));
        }

        let a: f64 = parts[0].parse().map_err(|_| {
            AgentError::ToolExecutionError("无法解析第一个数字".to_string())
        })?;

        let b: f64 = parts[2].parse().map_err(|_| {
            AgentError::ToolExecutionError("无法解析第二个数字".to_string())
        })?;

        let result = match parts[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return Ok(ToolResult::error("除数不能为零".to_string()));
                }
                a / b
            }
            _ => {
                return Ok(ToolResult::error(
                    format!("不支持的运算符: {}", parts[1])
                ));
            }
        };

        Ok(ToolResult::success(result.to_string()))
    }
}

/// 搜索工具（模拟）
pub struct SearchTool;

#[async_trait]
impl Tool for SearchTool {
    fn name(&self) -> &str {
        "search"
    }

    fn description(&self) -> &str {
        "搜索信息。输入搜索查询，返回相关结果"
    }

    async fn execute(&self, input: &str) -> Result<ToolResult, AgentError> {
        // 模拟搜索延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // 模拟搜索结果
        let result = format!(
            "搜索 '{}' 的结果:\n1. 相关信息A\n2. 相关信息B\n3. 相关信息C",
            input
        );

        Ok(ToolResult::success(result))
    }
}

/// 天气工具（模拟）
pub struct WeatherTool;

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "weather"
    }

    fn description(&self) -> &str {
        "查询天气信息。输入城市名称，返回天气情况"
    }

    async fn execute(&self, input: &str) -> Result<ToolResult, AgentError> {
        // 模拟 API 调用延迟
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // 模拟天气数据
        let result = format!(
            "{} 的天气: 晴天，温度 22°C，湿度 60%",
            input
        );

        Ok(ToolResult::success(result))
    }
}

// ============================================================================
// 对话历史
// ============================================================================

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// 用户输入
    UserInput(String),
    /// Agent 思考
    AgentThought(String),
    /// 工具调用
    ToolCall { tool: String, input: String },
    /// 工具结果
    ToolResult(ToolResult),
    /// 最终答案
    FinalAnswer(String),
}

/// 对话记忆
#[derive(Debug, Clone)]
pub struct ConversationMemory {
    messages: Vec<MessageType>,
    max_history: usize,
}

impl ConversationMemory {
    pub fn new(max_history: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_history,
        }
    }

    pub fn add(&mut self, message: MessageType) {
        self.messages.push(message);

        // 保持历史记录在限制内
        if self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    pub fn get_history(&self) -> &[MessageType] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn format_history(&self) -> String {
        self.messages
            .iter()
            .map(|msg| match msg {
                MessageType::UserInput(s) => format!("用户: {}", s),
                MessageType::AgentThought(s) => format!("思考: {}", s),
                MessageType::ToolCall { tool, input } => {
                    format!("调用工具 [{}]: {}", tool, input)
                }
                MessageType::ToolResult(r) => {
                    if r.success {
                        format!("工具结果: {}", r.output)
                    } else {
                        format!("工具错误: {}", r.error.as_ref().unwrap())
                    }
                }
                MessageType::FinalAnswer(s) => format!("答案: {}", s),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ============================================================================
// Agent 核心
// ============================================================================

/// Agent 配置
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// 最大推理迭代次数
    pub max_iterations: usize,
    /// 是否启用详细日志
    pub verbose: bool,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: 5,
            verbose: true,
        }
    }
}

/// Agent 核心
pub struct Agent {
    /// 工具注册表
    tools: HashMap<String, Box<dyn Tool>>,
    /// 对话记忆
    memory: ConversationMemory,
    /// 配置
    config: AgentConfig,
}

impl Agent {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            tools: HashMap::new(),
            memory: ConversationMemory::new(100),
            config,
        }
    }

    /// 注册工具
    pub fn register_tool(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }

    /// 获取所有工具描述
    pub fn list_tools(&self) -> String {
        self.tools
            .values()
            .map(|tool| format!("- {}: {}", tool.name(), tool.description()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 执行推理循环
    pub async fn run(&mut self, input: &str) -> Result<String, AgentError> {
        // 添加用户输入到记忆
        self.memory.add(MessageType::UserInput(input.to_string()));

        if self.config.verbose {
            println!("\n=== Agent 开始推理 ===");
            println!("用户输入: {}", input);
            println!("可用工具:\n{}", self.list_tools());
        }

        // 推理循环
        for iteration in 0..self.config.max_iterations {
            if self.config.verbose {
                println!("\n--- 迭代 {} ---", iteration + 1);
            }

            // 分析并决定下一步行动
            let action = self.decide_action(input, iteration).await?;

            match action {
                Action::UseTool { tool_name, input: tool_input } => {
                    // 记录工具调用
                    self.memory.add(MessageType::ToolCall {
                        tool: tool_name.clone(),
                        input: tool_input.clone(),
                    });

                    if self.config.verbose {
                        println!("调用工具: {} ({})", tool_name, tool_input);
                    }

                    // 执行工具
                    let result = self.execute_tool(&tool_name, &tool_input).await?;

                    // 记录工具结果
                    self.memory.add(MessageType::ToolResult(result.clone()));

                    if self.config.verbose {
                        if result.success {
                            println!("工具结果: {}", result.output);
                        } else {
                            println!("工具错误: {}", result.error.as_ref().unwrap());
                        }
                    }
                }
                Action::Finish(answer) => {
                    // 记录最终答案
                    self.memory.add(MessageType::FinalAnswer(answer.clone()));

                    if self.config.verbose {
                        println!("\n=== Agent 完成推理 ===");
                        println!("最终答案: {}", answer);
                    }

                    return Ok(answer);
                }
            }
        }

        Err(AgentError::MaxIterationsReached)
    }

    /// 决定下一步行动
    async fn decide_action(&self, input: &str, iteration: usize) -> Result<Action, AgentError> {
        // 简化的决策逻辑（实际应该使用 LLM）

        // 第一次迭代：分析输入
        if iteration == 0 {
            // 检查是否需要计算
            if input.contains("计算") || input.contains("+") || input.contains("-")
                || input.contains("*") || input.contains("/") {
                // 提取计算表达式
                let expr = self.extract_calculation(input);
                return Ok(Action::UseTool {
                    tool_name: "calculator".to_string(),
                    input: expr,
                });
            }

            // 检查是否需要搜索
            if input.contains("搜索") || input.contains("查找") || input.contains("什么是") {
                let query = input.replace("搜索", "").replace("查找", "").trim().to_string();
                return Ok(Action::UseTool {
                    tool_name: "search".to_string(),
                    input: query,
                });
            }

            // 检查是否需要天气
            if input.contains("天气") {
                let city = self.extract_city(input);
                return Ok(Action::UseTool {
                    tool_name: "weather".to_string(),
                    input: city,
                });
            }
        }

        // 如果有工具结果，生成最终答案
        let history = self.memory.get_history();
        if let Some(MessageType::ToolResult(result)) = history.last() {
            if result.success {
                let answer = format!("根据查询结果: {}", result.output);
                return Ok(Action::Finish(answer));
            }
        }

        // 默认：直接回答
        Ok(Action::Finish(format!("收到您的消息: {}", input)))
    }

    /// 提取计算表达式
    fn extract_calculation(&self, input: &str) -> String {
        // 简单的提取逻辑
        for op in &["+", "-", "*", "/"] {
            if let Some(pos) = input.find(op) {
                let before = &input[..pos].trim();
                let after = &input[pos + 1..].trim();

                // 尝试提取数字
                if let (Some(a), Some(b)) = (
                    before.split_whitespace().last(),
                    after.split_whitespace().next()
                ) {
                    return format!("{} {} {}", a, op, b);
                }
            }
        }

        "0 + 0".to_string()
    }

    /// 提取城市名称
    fn extract_city(&self, input: &str) -> String {
        // 简单的提取逻辑
        let words: Vec<&str> = input.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if *word == "天气" && i > 0 {
                return words[i - 1].to_string();
            }
        }
        "北京".to_string()
    }

    /// 执行工具
    async fn execute_tool(&self, tool_name: &str, input: &str) -> Result<ToolResult, AgentError> {
        let tool = self.tools.get(tool_name)
            .ok_or_else(|| AgentError::ToolNotFound(tool_name.to_string()))?;

        tool.execute(input).await
    }

    /// 获取对话历史
    pub fn get_memory(&self) -> &ConversationMemory {
        &self.memory
    }

    /// 清空对话历史
    pub fn clear_memory(&mut self) {
        self.memory.clear();
    }
}

/// Agent 行动
#[derive(Debug)]
enum Action {
    /// 使用工具
    UseTool { tool_name: String, input: String },
    /// 完成并返回答案
    Finish(String),
}

// ============================================================================
// 主函数和示例
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), AgentError> {
    println!("=== Agent 系统演示 ===\n");

    // 创建 Agent
    let mut agent = Agent::new(AgentConfig {
        max_iterations: 5,
        verbose: true,
    });

    // 注册工具
    agent.register_tool(Box::new(Calculator));
    agent.register_tool(Box::new(SearchTool));
    agent.register_tool(Box::new(WeatherTool));

    // 示例 1: 计算
    println!("\n{}", "=".repeat(60));
    println!("示例 1: 数学计算");
    println!("{}", "=".repeat(60));
    let result = agent.run("帮我计算 15 + 27").await?;
    println!("\n最终结果: {}", result);

    // 清空记忆
    agent.clear_memory();

    // 示例 2: 搜索
    println!("\n\n{}", "=".repeat(60));
    println!("示例 2: 信息搜索");
    println!("{}", "=".repeat(60));
    let result = agent.run("搜索 Rust 编程语言").await?;
    println!("\n最终结果: {}", result);

    // 清空记忆
    agent.clear_memory();

    // 示例 3: 天气查询
    println!("\n\n{}", "=".repeat(60));
    println!("示例 3: 天气查询");
    println!("{}", "=".repeat(60));
    let result = agent.run("上海天气怎么样").await?;
    println!("\n最终结果: {}", result);

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculator_tool() {
        let calc = Calculator;

        let result = calc.execute("5 + 3").await.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "8");

        let result = calc.execute("10 * 2").await.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "20");

        let result = calc.execute("10 / 0").await.unwrap();
        assert!(!result.success);
    }

    #[tokio::test]
    async fn test_search_tool() {
        let search = SearchTool;
        let result = search.execute("Rust").await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Rust"));
    }

    #[tokio::test]
    async fn test_weather_tool() {
        let weather = WeatherTool;
        let result = weather.execute("北京").await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("北京"));
    }

    #[tokio::test]
    async fn test_conversation_memory() {
        let mut memory = ConversationMemory::new(3);

        memory.add(MessageType::UserInput("Hello".to_string()));
        memory.add(MessageType::AgentThought("Thinking...".to_string()));
        memory.add(MessageType::FinalAnswer("Answer".to_string()));

        assert_eq!(memory.get_history().len(), 3);

        // 测试历史限制
        memory.add(MessageType::UserInput("New".to_string()));
        assert_eq!(memory.get_history().len(), 3);
    }

    #[tokio::test]
    async fn test_agent_calculation() {
        let mut agent = Agent::new(AgentConfig {
            max_iterations: 5,
            verbose: false,
        });

        agent.register_tool(Box::new(Calculator));

        let result = agent.run("计算 5 + 3").await.unwrap();
        assert!(result.contains("8"));
    }

    #[tokio::test]
    async fn test_agent_search() {
        let mut agent = Agent::new(AgentConfig {
            max_iterations: 5,
            verbose: false,
        });

        agent.register_tool(Box::new(SearchTool));

        let result = agent.run("搜索 Rust").await.unwrap();
        assert!(result.contains("Rust"));
    }

    #[tokio::test]
    async fn test_agent_weather() {
        let mut agent = Agent::new(AgentConfig {
            max_iterations: 5,
            verbose: false,
        });

        agent.register_tool(Box::new(WeatherTool));

        let result = agent.run("北京天气").await.unwrap();
        assert!(result.contains("北京") || result.contains("天气"));
    }
}
