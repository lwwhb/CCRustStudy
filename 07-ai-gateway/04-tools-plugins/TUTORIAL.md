# 模块 7.4：工具与插件系统 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 设计可扩展的工具系统
2. 实现工具注册表
3. 创建常用工具插件
4. 实现类型安全的工具调用
5. 处理工具执行的错误和超时

## 🎯 为什么需要插件系统？

### 硬编码 vs 插件系统

**硬编码工具（问题）**：
```rust
match tool_name {
    "weather" => get_weather(args),
    "calculator" => calculate(args),
    "search" => search_web(args),
    // 添加新工具需要修改核心代码
}

问题：
- 难以扩展
- 代码耦合
- 难以测试
- 无法动态加载
```

**插件系统（解决方案）**：
```rust
// 注册工具
registry.register(WeatherTool::new());
registry.register(CalculatorTool::new());
registry.register(SearchTool::new());

// 动态调用
let result = registry.execute(tool_name, args).await?;

优势：
- 易于扩展
- 解耦
- 易于测试
- 支持动态加载
```

### 插件系统的价值

```
1. 可扩展性
   - 轻松添加新工具
   - 不修改核心代码

2. 模块化
   - 每个工具独立
   - 可以单独测试

3. 可配置
   - 启用/禁用工具
   - 权限控制

4. 可维护
   - 清晰的接口
   - 统一的错误处理
```

## 📖 核心概念详解

### 1. 工具 Trait

定义统一的工具接口。

```rust
use async_trait::async_trait;
use serde_json::Value;

// 工具执行结果
#[derive(Debug)]
pub enum ToolResult {
    Success(String),
    Error(String),
}

// 工具 trait
#[async_trait]
pub trait Tool: Send + Sync {
    // 工具名称
    fn name(&self) -> &str;

    // 工具描述
    fn description(&self) -> &str;

    // 参数 schema（JSON Schema 格式）
    fn parameters_schema(&self) -> Value;

    // 执行工具
    async fn execute(&self, args: Value) -> Result<ToolResult, ToolError>;

    // 验证参数（可选）
    fn validate_args(&self, args: &Value) -> Result<(), ToolError> {
        Ok(())
    }

    // 工具是否需要认证
    fn requires_auth(&self) -> bool {
        false
    }

    // 工具类别
    fn category(&self) -> ToolCategory {
        ToolCategory::General
    }
}

// 工具类别
#[derive(Debug, Clone, PartialEq)]
pub enum ToolCategory {
    General,      // 通用
    Web,          // 网络
    Data,         // 数据处理
    System,       // 系统操作
    External,     // 外部 API
}

// 工具错误
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("参数验证失败: {0}")]
    ValidationError(String),

    #[error("执行失败: {0}")]
    ExecutionError(String),

    #[error("超时")]
    Timeout,

    #[error("未授权")]
    Unauthorized,

    #[error("工具不存在: {0}")]
    NotFound(String),
}
```

### 2. 工具注册表

管理所有可用的工具。

```rust
use std::collections::HashMap;
use std::sync::Arc;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    // 注册工具
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tools.insert(name, Arc::new(tool));
    }

    // 获取工具
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    // 列出所有工具
    pub fn list_tools(&self) -> Vec<ToolInfo> {
        self.tools
            .values()
            .map(|tool| ToolInfo {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                category: tool.category(),
                requires_auth: tool.requires_auth(),
            })
            .collect()
    }

    // 按类别获取工具
    pub fn get_by_category(&self, category: ToolCategory) -> Vec<Arc<dyn Tool>> {
        self.tools
            .values()
            .filter(|tool| tool.category() == category)
            .cloned()
            .collect()
    }

    // 执行工具
    pub async fn execute(
        &self,
        name: &str,
        args: Value,
    ) -> Result<ToolResult, ToolError> {
        let tool = self.get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?;

        // 验证参数
        tool.validate_args(&args)?;

        // 执行工具
        tool.execute(args).await
    }

    // 获取工具的 schema（用于 LLM）
    pub fn get_tools_schema(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|tool| {
                serde_json::json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "parameters": tool.parameters_schema()
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub requires_auth: bool,
}
```

### 3. 参数验证

使用 JSON Schema 验证参数。

```rust
use jsonschema::{Draft, JSONSchema};

pub fn validate_parameters(
    schema: &Value,
    args: &Value,
) -> Result<(), ToolError> {
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema)
        .map_err(|e| ToolError::ValidationError(e.to_string()))?;

    if let Err(errors) = compiled.validate(args) {
        let error_messages: Vec<String> = errors
            .map(|e| e.to_string())
            .collect();

        return Err(ToolError::ValidationError(
            error_messages.join(", ")
        ));
    }

    Ok(())
}
```

## 💻 实战项目：常用工具插件

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json"] }
thiserror = "1"
jsonschema = "0.17"
chrono = "0.4"
```

### 步骤 2：计算器工具

```rust
use serde_json::{json, Value};

pub struct CalculatorTool;

impl CalculatorTool {
    pub fn new() -> Self {
        Self
    }

    fn evaluate(&self, expression: &str) -> Result<f64, String> {
        // 简单的表达式求值（实际应使用专门的解析库）
        match meval::eval_str(expression) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("计算错误: {}", e)),
        }
    }
}

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "执行数学计算，支持基本运算符 (+, -, *, /, ^) 和函数 (sin, cos, sqrt 等)"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "要计算的数学表达式，如 '2 + 2' 或 'sqrt(16)'"
                }
            },
            "required": ["expression"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult, ToolError> {
        let expression = args["expression"]
            .as_str()
            .ok_or_else(|| ToolError::ValidationError(
                "缺少 expression 参数".to_string()
            ))?;

        match self.evaluate(expression) {
            Ok(result) => Ok(ToolResult::Success(
                format!("{} = {}", expression, result)
            )),
            Err(e) => Ok(ToolResult::Error(e)),
        }
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::General
    }
}
```

### 步骤 3：天气查询工具

```rust
pub struct WeatherTool {
    api_key: String,
    client: reqwest::Client,
}

impl WeatherTool {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    async fn fetch_weather(&self, city: &str) -> Result<String, String> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=zh_cn",
            city, self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API 错误: {}", response.status()));
        }

        let data: Value = response
            .json()
            .await
            .map_err(|e| format!("解析失败: {}", e))?;

        // 提取天气信息
        let temp = data["main"]["temp"].as_f64().unwrap_or(0.0);
        let description = data["weather"][0]["description"]
            .as_str()
            .unwrap_or("未知");
        let humidity = data["main"]["humidity"].as_i64().unwrap_or(0);

        Ok(format!(
            "{}的天气：{}，温度 {:.1}°C，湿度 {}%",
            city, description, temp, humidity
        ))
    }
}

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "get_weather"
    }

    fn description(&self) -> &str {
        "获取指定城市的实时天气信息"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "城市名称，如：北京、上海、深圳"
                }
            },
            "required": ["city"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult, ToolError> {
        let city = args["city"]
            .as_str()
            .ok_or_else(|| ToolError::ValidationError(
                "缺少 city 参数".to_string()
            ))?;

        match self.fetch_weather(city).await {
            Ok(weather) => Ok(ToolResult::Success(weather)),
            Err(e) => Ok(ToolResult::Error(e)),
        }
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::External
    }

    fn requires_auth(&self) -> bool {
        true
    }
}
```

### 步骤 4：网页搜索工具

```rust
pub struct WebSearchTool {
    api_key: String,
    search_engine_id: String,
    client: reqwest::Client,
}

impl WebSearchTool {
    pub fn new(api_key: String, search_engine_id: String) -> Self {
        Self {
            api_key,
            search_engine_id,
            client: reqwest::Client::new(),
        }
    }

    async fn search(&self, query: &str, num_results: usize) -> Result<Vec<SearchResult>, String> {
        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num={}",
            self.api_key, self.search_engine_id, query, num_results
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("搜索失败: {}", e))?;

        let data: Value = response
            .json()
            .await
            .map_err(|e| format!("解析失败: {}", e))?;

        let items = data["items"]
            .as_array()
            .ok_or_else(|| "没有搜索结果".to_string())?;

        let results: Vec<SearchResult> = items
            .iter()
            .map(|item| SearchResult {
                title: item["title"].as_str().unwrap_or("").to_string(),
                link: item["link"].as_str().unwrap_or("").to_string(),
                snippet: item["snippet"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(results)
    }
}

#[derive(Debug)]
struct SearchResult {
    title: String,
    link: String,
    snippet: String,
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "在网络上搜索信息"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "搜索关键词"
                },
                "num_results": {
                    "type": "integer",
                    "description": "返回结果数量（1-10）",
                    "default": 5
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult, ToolError> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| ToolError::ValidationError(
                "缺少 query 参数".to_string()
            ))?;

        let num_results = args["num_results"]
            .as_u64()
            .unwrap_or(5)
            .min(10) as usize;

        match self.search(query, num_results).await {
            Ok(results) => {
                let output = results
                    .iter()
                    .enumerate()
                    .map(|(i, r)| {
                        format!("{}. {}\n   {}\n   {}", i + 1, r.title, r.snippet, r.link)
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n");

                Ok(ToolResult::Success(output))
            }
            Err(e) => Ok(ToolResult::Error(e)),
        }
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::Web
    }

    fn requires_auth(&self) -> bool {
        true
    }
}
```

### 步骤 5：文件操作工具

```rust
use tokio::fs;

pub struct FileReadTool {
    allowed_paths: Vec<String>,
}

impl FileReadTool {
    pub fn new(allowed_paths: Vec<String>) -> Self {
        Self { allowed_paths }
    }

    fn is_path_allowed(&self, path: &str) -> bool {
        self.allowed_paths.iter().any(|allowed| {
            path.starts_with(allowed)
        })
    }

    async fn read_file(&self, path: &str) -> Result<String, String> {
        if !self.is_path_allowed(path) {
            return Err("路径不在允许列表中".to_string());
        }

        fs::read_to_string(path)
            .await
            .map_err(|e| format!("读取文件失败: {}", e))
    }
}

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "读取文件内容"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "文件路径"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult, ToolError> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| ToolError::ValidationError(
                "缺少 path 参数".to_string()
            ))?;

        match self.read_file(path).await {
            Ok(content) => Ok(ToolResult::Success(content)),
            Err(e) => Ok(ToolResult::Error(e)),
        }
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::System
    }
}
```

### 步骤 6：时间工具

```rust
use chrono::{Local, Utc};

pub struct TimeTool;

impl TimeTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for TimeTool {
    fn name(&self) -> &str {
        "get_time"
    }

    fn description(&self) -> &str {
        "获取当前时间"
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "时区，'local' 或 'utc'",
                    "default": "local"
                },
                "format": {
                    "type": "string",
                    "description": "时间格式，如 '%Y-%m-%d %H:%M:%S'",
                    "default": "%Y-%m-%d %H:%M:%S"
                }
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult, ToolError> {
        let timezone = args["timezone"]
            .as_str()
            .unwrap_or("local");

        let format = args["format"]
            .as_str()
            .unwrap_or("%Y-%m-%d %H:%M:%S");

        let time_str = match timezone {
            "utc" => Utc::now().format(format).to_string(),
            _ => Local::now().format(format).to_string(),
        };

        Ok(ToolResult::Success(time_str))
    }

    fn category(&self) -> ToolCategory {
        ToolCategory::General
    }
}
```

### 步骤 7：组装工具注册表

```rust
pub fn create_default_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    // 注册基础工具
    registry.register(CalculatorTool::new());
    registry.register(TimeTool::new());

    // 注册需要 API 密钥的工具（从环境变量读取）
    if let Ok(weather_key) = std::env::var("WEATHER_API_KEY") {
        registry.register(WeatherTool::new(weather_key));
    }

    if let (Ok(search_key), Ok(search_engine_id)) = (
        std::env::var("GOOGLE_API_KEY"),
        std::env::var("GOOGLE_SEARCH_ENGINE_ID"),
    ) {
        registry.register(WebSearchTool::new(search_key, search_engine_id));
    }

    // 注册文件工具（限制访问路径）
    registry.register(FileReadTool::new(vec![
        "/tmp".to_string(),
        "/var/log".to_string(),
    ]));

    registry
}
```

### 步骤 8：使用示例

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 工具插件系统演示 ===\n");

    // 创建工具注册表
    let registry = create_default_registry();

    // 列出所有工具
    println!("可用工具:");
    for tool_info in registry.list_tools() {
        println!("  - {}: {}", tool_info.name, tool_info.description);
    }
    println!();

    // 示例 1: 计算器
    println!("=== 示例 1: 计算器 ===");
    let result = registry.execute(
        "calculator",
        serde_json::json!({
            "expression": "2 + 2 * 3"
        })
    ).await?;
    println!("结果: {:?}\n", result);

    // 示例 2: 获取时间
    println!("=== 示例 2: 获取时间 ===");
    let result = registry.execute(
        "get_time",
        serde_json::json!({
            "timezone": "local"
        })
    ).await?;
    println!("结果: {:?}\n", result);

    // 示例 3: 天气查询（需要 API 密钥）
    if registry.get("get_weather").is_some() {
        println!("=== 示例 3: 天气查询 ===");
        let result = registry.execute(
            "get_weather",
            serde_json::json!({
                "city": "北京"
            })
        ).await?;
        println!("结果: {:?}\n", result);
    }

    // 获取工具 schema（用于 LLM）
    println!("=== 工具 Schema（用于 LLM）===");
    let schemas = registry.get_tools_schema();
    println!("{}", serde_json::to_string_pretty(&schemas)?);

    Ok(())
}
```

## 🔍 深入理解

### 工具执行的安全性

```rust
// 1. 超时控制
use tokio::time::{timeout, Duration};

pub async fn execute_with_timeout(
    tool: &dyn Tool,
    args: Value,
    timeout_secs: u64,
) -> Result<ToolResult, ToolError> {
    match timeout(
        Duration::from_secs(timeout_secs),
        tool.execute(args)
    ).await {
        Ok(result) => result,
        Err(_) => Err(ToolError::Timeout),
    }
}

// 2. 权限检查
pub fn check_permission(
    tool: &dyn Tool,
    user_permissions: &[String],
) -> Result<(), ToolError> {
    if tool.requires_auth() && !user_permissions.contains(&tool.name().to_string()) {
        return Err(ToolError::Unauthorized);
    }
    Ok(())
}

// 3. 速率限制
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct RateLimitedRegistry {
    registry: ToolRegistry,
    semaphore: Arc<Semaphore>,
}

impl RateLimitedRegistry {
    pub fn new(registry: ToolRegistry, max_concurrent: usize) -> Self {
        Self {
            registry,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn execute(
        &self,
        name: &str,
        args: Value,
    ) -> Result<ToolResult, ToolError> {
        let _permit = self.semaphore.acquire().await.unwrap();
        self.registry.execute(name, args).await
    }
}
```

### 工具组合

```rust
// 组合多个工具的结果
pub async fn execute_pipeline(
    registry: &ToolRegistry,
    steps: Vec<(String, Value)>,
) -> Result<Vec<ToolResult>, ToolError> {
    let mut results = Vec::new();

    for (tool_name, args) in steps {
        let result = registry.execute(&tool_name, args).await?;
        results.push(result);
    }

    Ok(results)
}
```

## 📝 练习题

### 练习 1: 实现 HTTP 请求工具
创建一个工具，可以发送 HTTP GET 请求。

### 练习 2: 实现数据转换工具
创建一个工具，可以在 JSON、YAML、TOML 之间转换。

### 练习 3: 实现缓存层
为工具执行添加缓存，避免重复调用。

### 练习 4: 实现工具链
允许一个工具的输出作为另一个工具的输入。

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 设计工具 trait 接口
- [ ] 实现工具注册表
- [ ] 创建自定义工具
- [ ] 验证工具参数
- [ ] 处理工具执行错误
- [ ] 实现超时控制
- [ ] 添加权限检查
- [ ] 实现速率限制
- [ ] 组合多个工具
- [ ] 生成工具 schema

## 🔗 延伸阅读

- [JSON Schema 规范](https://json-schema.org/)
- [OpenAI Function Calling](https://platform.openai.com/docs/guides/function-calling)
- [Anthropic Tool Use](https://docs.anthropic.com/claude/docs/tool-use)

## 🚀 下一步

完成本模块后，继续学习：
1. 模块 7.5（生产特性）- 认证、监控、部署
2. 将工具系统集成到完整的 AI Gateway

---

**掌握工具系统，构建强大的 AI Agent！** 🤖
