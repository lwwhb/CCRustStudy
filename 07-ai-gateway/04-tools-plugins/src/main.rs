use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;
use tokio::time::timeout;

// ============================================================================
// 错误类型
// ============================================================================

/// 工具系统错误
#[derive(Debug, Error)]
pub enum ToolError {
    #[error("工具未找到: {0}")]
    ToolNotFound(String),

    #[error("工具执行失败: {0}")]
    ExecutionError(String),

    #[error("参数验证失败: {0}")]
    ValidationError(String),

    #[error("工具超时")]
    Timeout,

    #[error("工具已存在: {0}")]
    ToolAlreadyExists(String),

    #[error("依赖未满足: {0}")]
    DependencyNotMet(String),
}

// ============================================================================
// 工具元数据
// ============================================================================

/// 工具版本
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterSchema {
    /// 参数名称
    pub name: String,
    /// 参数描述
    pub description: String,
    /// 是否必需
    pub required: bool,
    /// 参数类型
    pub param_type: String,
}

/// 工具元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 版本
    pub version: Version,
    /// 作者
    pub author: String,
    /// 参数模式
    pub parameters: Vec<ParameterSchema>,
    /// 依赖的其他工具
    pub dependencies: Vec<String>,
    /// 标签
    pub tags: Vec<String>,
}

/// 工具执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// 是否成功
    pub success: bool,
    /// 输出内容
    pub output: String,
    /// 错误信息
    pub error: Option<String>,
    /// 执行时间（毫秒）
    pub execution_time_ms: u64,
}

impl ToolResult {
    pub fn success(output: String, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            output,
            error: None,
            execution_time_ms,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
            execution_time_ms: 0,
        }
    }
}

// ============================================================================
// 工具接口
// ============================================================================

/// 工具 Trait
#[async_trait]
pub trait Tool: Send + Sync {
    /// 获取工具元数据
    fn metadata(&self) -> &ToolMetadata;

    /// 验证输入参数
    fn validate_input(&self, input: &str) -> Result<(), ToolError> {
        // 默认实现：不验证
        let _ = input;
        Ok(())
    }

    /// 执行工具
    async fn execute(&self, input: &str) -> Result<ToolResult, ToolError>;
}

// ============================================================================
// 工具注册表
// ============================================================================

/// 工具注册表
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Box<dyn Tool>>>>,
    execution_stats: Arc<RwLock<HashMap<String, ExecutionStats>>>,
}

/// 执行统计
#[derive(Debug, Clone)]
struct ExecutionStats {
    total_calls: u64,
    successful_calls: u64,
    failed_calls: u64,
    total_execution_time_ms: u64,
    last_execution: Option<DateTime<Utc>>,
}

impl ExecutionStats {
    fn new() -> Self {
        Self {
            total_calls: 0,
            successful_calls: 0,
            failed_calls: 0,
            total_execution_time_ms: 0,
            last_execution: None,
        }
    }

    fn record_execution(&mut self, success: bool, execution_time_ms: u64) {
        self.total_calls += 1;
        if success {
            self.successful_calls += 1;
        } else {
            self.failed_calls += 1;
        }
        self.total_execution_time_ms += execution_time_ms;
        self.last_execution = Some(Utc::now());
    }

    fn average_execution_time_ms(&self) -> f64 {
        if self.total_calls == 0 {
            0.0
        } else {
            self.total_execution_time_ms as f64 / self.total_calls as f64
        }
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            execution_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册工具
    pub async fn register(&self, tool: Box<dyn Tool>) -> Result<(), ToolError> {
        let name = tool.metadata().name.clone();

        // 检查工具是否已存在
        {
            let tools = self.tools.read().await;
            if tools.contains_key(&name) {
                return Err(ToolError::ToolAlreadyExists(name));
            }
        }

        // 检查依赖
        for dep in &tool.metadata().dependencies {
            let tools = self.tools.read().await;
            if !tools.contains_key(dep) {
                return Err(ToolError::DependencyNotMet(dep.clone()));
            }
        }

        // 注册工具
        {
            let mut tools = self.tools.write().await;
            tools.insert(name.clone(), tool);
        }

        // 初始化统计
        {
            let mut stats = self.execution_stats.write().await;
            stats.insert(name, ExecutionStats::new());
        }

        Ok(())
    }

    /// 注销工具
    pub async fn unregister(&self, name: &str) -> Result<(), ToolError> {
        let mut tools = self.tools.write().await;
        tools
            .remove(name)
            .ok_or_else(|| ToolError::ToolNotFound(name.to_string()))?;

        let mut stats = self.execution_stats.write().await;
        stats.remove(name);

        Ok(())
    }

    /// 获取工具
    pub async fn get(&self, name: &str) -> Result<ToolMetadata, ToolError> {
        let tools = self.tools.read().await;
        let tool = tools
            .get(name)
            .ok_or_else(|| ToolError::ToolNotFound(name.to_string()))?;
        Ok(tool.metadata().clone())
    }

    /// 列出所有工具
    pub async fn list(&self) -> Vec<ToolMetadata> {
        let tools = self.tools.read().await;
        tools.values().map(|t| t.metadata().clone()).collect()
    }

    /// 按标签搜索工具
    pub async fn search_by_tag(&self, tag: &str) -> Vec<ToolMetadata> {
        let tools = self.tools.read().await;
        tools
            .values()
            .filter(|t| t.metadata().tags.contains(&tag.to_string()))
            .map(|t| t.metadata().clone())
            .collect()
    }

    /// 执行工具（带超时）
    pub async fn execute(
        &self,
        name: &str,
        input: &str,
        timeout_duration: Duration,
    ) -> Result<ToolResult, ToolError> {
        let start = std::time::Instant::now();

        // 获取工具
        let tool = {
            let tools = self.tools.read().await;
            tools
                .get(name)
                .ok_or_else(|| ToolError::ToolNotFound(name.to_string()))?;
            // 注意：这里我们不能直接返回引用，因为锁会被释放
            // 实际执行在下面
        };

        // 验证输入
        {
            let tools = self.tools.read().await;
            let tool = tools.get(name).unwrap();
            tool.validate_input(input)?;
        }

        // 执行工具（带超时）
        let result = {
            let tools = self.tools.read().await;
            let tool = tools.get(name).unwrap();

            match timeout(timeout_duration, tool.execute(input)).await {
                Ok(Ok(result)) => Ok(result),
                Ok(Err(e)) => Err(e),
                Err(_) => Err(ToolError::Timeout),
            }
        };

        // 记录统计
        let execution_time_ms = start.elapsed().as_millis() as u64;
        {
            let mut stats = self.execution_stats.write().await;
            if let Some(stat) = stats.get_mut(name) {
                stat.record_execution(result.is_ok(), execution_time_ms);
            }
        }

        result
    }

    /// 获取工具统计信息
    pub async fn get_stats(&self, name: &str) -> Option<String> {
        let stats = self.execution_stats.read().await;
        stats.get(name).map(|s| {
            format!(
                "总调用: {}, 成功: {}, 失败: {}, 平均耗时: {:.2}ms",
                s.total_calls,
                s.successful_calls,
                s.failed_calls,
                s.average_execution_time_ms()
            )
        })
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 内置工具实现
// ============================================================================

/// 字符串处理工具
pub struct StringTool;

impl StringTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for StringTool {
    fn metadata(&self) -> &ToolMetadata {
        static METADATA: std::sync::OnceLock<ToolMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| ToolMetadata {
            name: "string".to_string(),
            description: "字符串处理工具，支持大小写转换、反转等操作".to_string(),
            version: Version::new(1, 0, 0),
            author: "System".to_string(),
            parameters: vec![
                ParameterSchema {
                    name: "operation".to_string(),
                    description: "操作类型: upper, lower, reverse".to_string(),
                    required: true,
                    param_type: "string".to_string(),
                },
                ParameterSchema {
                    name: "text".to_string(),
                    description: "要处理的文本".to_string(),
                    required: true,
                    param_type: "string".to_string(),
                },
            ],
            dependencies: vec![],
            tags: vec!["string".to_string(), "text".to_string()],
        })
    }

    fn validate_input(&self, input: &str) -> Result<(), ToolError> {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(ToolError::ValidationError(
                "输入格式: <operation> <text>".to_string(),
            ));
        }

        let operation = parts[0];
        if !["upper", "lower", "reverse"].contains(&operation) {
            return Err(ToolError::ValidationError(
                "操作必须是: upper, lower, reverse".to_string(),
            ));
        }

        Ok(())
    }

    async fn execute(&self, input: &str) -> Result<ToolResult, ToolError> {
        let start = std::time::Instant::now();

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let operation = parts[0];
        let text = parts[1];

        let result = match operation {
            "upper" => text.to_uppercase(),
            "lower" => text.to_lowercase(),
            "reverse" => text.chars().rev().collect(),
            _ => unreachable!(),
        };

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(ToolResult::success(result, execution_time_ms))
    }
}

/// JSON 处理工具
pub struct JsonTool;

impl JsonTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for JsonTool {
    fn metadata(&self) -> &ToolMetadata {
        static METADATA: std::sync::OnceLock<ToolMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| ToolMetadata {
            name: "json".to_string(),
            description: "JSON 处理工具，支持格式化和压缩".to_string(),
            version: Version::new(1, 0, 0),
            author: "System".to_string(),
            parameters: vec![
                ParameterSchema {
                    name: "operation".to_string(),
                    description: "操作类型: format, minify".to_string(),
                    required: true,
                    param_type: "string".to_string(),
                },
                ParameterSchema {
                    name: "json".to_string(),
                    description: "JSON 字符串".to_string(),
                    required: true,
                    param_type: "string".to_string(),
                },
            ],
            dependencies: vec![],
            tags: vec!["json".to_string(), "data".to_string()],
        })
    }

    async fn execute(&self, input: &str) -> Result<ToolResult, ToolError> {
        let start = std::time::Instant::now();

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(ToolError::ValidationError(
                "输入格式: <operation> <json>".to_string(),
            ));
        }

        let operation = parts[0];
        let json_str = parts[1];

        // 解析 JSON
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| ToolError::ExecutionError(format!("JSON 解析失败: {}", e)))?;

        let result = match operation {
            "format" => serde_json::to_string_pretty(&value)
                .map_err(|e| ToolError::ExecutionError(format!("格式化失败: {}", e)))?,
            "minify" => serde_json::to_string(&value)
                .map_err(|e| ToolError::ExecutionError(format!("压缩失败: {}", e)))?,
            _ => {
                return Err(ToolError::ValidationError(
                    "操作必须是: format, minify".to_string(),
                ))
            }
        };

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(ToolResult::success(result, execution_time_ms))
    }
}

/// 数学计算工具
pub struct MathTool;

impl MathTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for MathTool {
    fn metadata(&self) -> &ToolMetadata {
        static METADATA: std::sync::OnceLock<ToolMetadata> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| ToolMetadata {
            name: "math".to_string(),
            description: "数学计算工具".to_string(),
            version: Version::new(1, 0, 0),
            author: "System".to_string(),
            parameters: vec![ParameterSchema {
                name: "expression".to_string(),
                description: "数学表达式，如: 5 + 3".to_string(),
                required: true,
                param_type: "string".to_string(),
            }],
            dependencies: vec![],
            tags: vec!["math".to_string(), "calculator".to_string()],
        })
    }

    async fn execute(&self, input: &str) -> Result<ToolResult, ToolError> {
        let start = std::time::Instant::now();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ToolError::ValidationError(
                "输入格式: <数字1> <运算符> <数字2>".to_string(),
            ));
        }

        let a: f64 = parts[0]
            .parse()
            .map_err(|_| ToolError::ValidationError("无效的数字".to_string()))?;

        let b: f64 = parts[2]
            .parse()
            .map_err(|_| ToolError::ValidationError("无效的数字".to_string()))?;

        let result = match parts[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return Err(ToolError::ExecutionError("除数不能为零".to_string()));
                }
                a / b
            }
            _ => {
                return Err(ToolError::ValidationError(format!(
                    "不支持的运算符: {}",
                    parts[1]
                )))
            }
        };

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(ToolResult::success(result.to_string(), execution_time_ms))
    }
}

// ============================================================================
// 主函数和测试
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("工具与插件系统演示\n");

    // 创建工具注册表
    let registry = ToolRegistry::new();

    // 注册工具
    println!("注册工具...");
    registry.register(Box::new(StringTool::new())).await?;
    registry.register(Box::new(JsonTool::new())).await?;
    registry.register(Box::new(MathTool::new())).await?;
    println!("✓ 已注册 3 个工具\n");

    // 列出所有工具
    println!("可用工具:");
    for tool in registry.list().await {
        println!("  - {} v{}: {}", tool.name, tool.version, tool.description);
    }
    println!();

    // 示例 1: 字符串处理
    println!("{}", "=".repeat(60));
    println!("示例 1: 字符串处理");
    println!("{}", "=".repeat(60));

    let result = registry
        .execute("string", "upper hello world", Duration::from_secs(5))
        .await?;
    println!("输入: upper hello world");
    println!("输出: {}", result.output);
    println!("耗时: {}ms\n", result.execution_time_ms);

    // 示例 2: JSON 格式化
    println!("{}", "=".repeat(60));
    println!("示例 2: JSON 格式化");
    println!("{}", "=".repeat(60));

    let json_input = r#"format {"name":"Alice","age":30}"#;
    let result = registry
        .execute("json", json_input, Duration::from_secs(5))
        .await?;
    println!("输入: {}", json_input);
    println!("输出:\n{}", result.output);
    println!("耗时: {}ms\n", result.execution_time_ms);

    // 示例 3: 数学计算
    println!("{}", "=".repeat(60));
    println!("示例 3: 数学计算");
    println!("{}", "=".repeat(60));

    let result = registry
        .execute("math", "15 + 27", Duration::from_secs(5))
        .await?;
    println!("输入: 15 + 27");
    println!("输出: {}", result.output);
    println!("耗时: {}ms\n", result.execution_time_ms);

    // 显示统计信息
    println!("{}", "=".repeat(60));
    println!("工具统计信息");
    println!("{}", "=".repeat(60));
    for tool in registry.list().await {
        if let Some(stats) = registry.get_stats(&tool.name).await {
            println!("{}: {}", tool.name, stats);
        }
    }

    Ok(())
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_registration() {
        let registry = ToolRegistry::new();

        // 注册工具
        let result = registry.register(Box::new(StringTool::new())).await;
        assert!(result.is_ok());

        // 重复注册应该失败
        let result = registry.register(Box::new(StringTool::new())).await;
        assert!(matches!(result, Err(ToolError::ToolAlreadyExists(_))));
    }

    #[tokio::test]
    async fn test_tool_execution() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(StringTool::new())).await.unwrap();

        let result = registry
            .execute("string", "upper hello", Duration::from_secs(5))
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.output, "HELLO");
    }

    #[tokio::test]
    async fn test_tool_validation() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(StringTool::new())).await.unwrap();

        // 无效输入应该失败
        let result = registry
            .execute("string", "invalid", Duration::from_secs(5))
            .await;

        assert!(matches!(result, Err(ToolError::ValidationError(_))));
    }

    #[tokio::test]
    async fn test_tool_not_found() {
        let registry = ToolRegistry::new();

        let result = registry
            .execute("nonexistent", "test", Duration::from_secs(5))
            .await;

        assert!(matches!(result, Err(ToolError::ToolNotFound(_))));
    }

    #[tokio::test]
    async fn test_math_tool() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(MathTool::new())).await.unwrap();

        let result = registry
            .execute("math", "10 + 5", Duration::from_secs(5))
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.output, "15");
    }

    #[tokio::test]
    async fn test_json_tool() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(JsonTool::new())).await.unwrap();

        let result = registry
            .execute("json", r#"minify {"a": 1}"#, Duration::from_secs(5))
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.output, r#"{"a":1}"#);
    }

    #[tokio::test]
    async fn test_tool_stats() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(MathTool::new())).await.unwrap();

        // 执行几次
        for _ in 0..3 {
            let _ = registry
                .execute("math", "5 + 3", Duration::from_secs(5))
                .await;
        }

        let stats = registry.get_stats("math").await;
        assert!(stats.is_some());
        assert!(stats.unwrap().contains("总调用: 3"));
    }

    #[tokio::test]
    async fn test_search_by_tag() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(StringTool::new())).await.unwrap();
        registry.register(Box::new(MathTool::new())).await.unwrap();

        let tools = registry.search_by_tag("string").await;
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "string");
    }
}
