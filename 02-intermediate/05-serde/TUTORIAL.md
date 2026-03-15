# 模块 2.5：序列化与反序列化 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 掌握 Serde 框架的使用
2. 处理 JSON、TOML、YAML 格式
3. 学习自定义序列化逻辑
4. 使用 derive 宏简化代码
5. 构建配置文件管理器

## 🎯 为什么需要序列化？

### 序列化的应用场景

**问题场景**：
```rust
// 如何保存这个结构体到文件？
struct User {
    name: String,
    age: u32,
    email: String,
}

// 如何通过网络发送？
// 如何存储到数据库？
// 如何与其他语言交互？
```

**使用序列化**：
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// 转换为 JSON
let json = serde_json::to_string(&user)?;
// {"name":"Alice","age":30,"email":"alice@example.com"}

// 保存到文件、发送网络、存储数据库...
```

### 其他语言的对比

**Python**：
```python
import json

user = {"name": "Alice", "age": 30}
json_str = json.dumps(user)  # 序列化
user_obj = json.loads(json_str)  # 反序列化

# 问题：没有类型检查，运行时错误
```

**Java**：
```java
// 需要大量样板代码
class User {
    private String name;
    private int age;
    
    // getter/setter...
    // toString()...
    // equals()...
}

// 使用 Jackson 或 Gson
ObjectMapper mapper = new ObjectMapper();
String json = mapper.writeValueAsString(user);
```

**Rust + Serde**：
```rust
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

// 一行代码搞定，类型安全
let json = serde_json::to_string(&user)?;
```

## 📖 核心概念详解

### 1. Serde 基础

Serde 是 Rust 的序列化/反序列化框架。

#### 基本用法

```rust
use serde::{Serialize, Deserialize};

// 自动实现序列化和反序列化
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // 序列化为 JSON
    let json = serde_json::to_string(&user).unwrap();
    println!("JSON: {}", json);

    // 反序列化
    let user2: User = serde_json::from_str(&json).unwrap();
    println!("User: {:?}", user2);
}
```

#### Cargo.toml 配置

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"  # JSON 支持
toml = "0.8"        # TOML 支持
serde_yaml = "0.9"  # YAML 支持
```

### 2. JSON 序列化

JSON 是最常用的数据交换格式。

#### 基础操作

```rust
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

// 序列化
let person = Person {
    name: "Bob".to_string(),
    age: 25,
    hobbies: vec!["reading".to_string(), "coding".to_string()],
};

// 紧凑格式
let json = serde_json::to_string(&person)?;
// {"name":"Bob","age":25,"hobbies":["reading","coding"]}

// 美化格式
let json_pretty = serde_json::to_string_pretty(&person)?;
// {
//   "name": "Bob",
//   "age": 25,
//   "hobbies": [
//     "reading",
//     "coding"
//   ]
// }

// 反序列化
let person2: Person = serde_json::from_str(&json)?;
```

#### 处理 JSON 值

```rust
use serde_json::{Value, json};

// 动态 JSON
let data = json!({
    "name": "Alice",
    "age": 30,
    "active": true,
    "scores": [95, 87, 92]
});

// 访问字段
if let Some(name) = data["name"].as_str() {
    println!("Name: {}", name);
}

if let Some(age) = data["age"].as_u64() {
    println!("Age: {}", age);
}

// 修改 JSON
let mut data = data;
data["age"] = json!(31);
```

### 3. 字段属性

Serde 提供了丰富的属性来控制序列化行为。

#### rename - 重命名字段

```rust
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "userName")]
    user_name: String,
    
    #[serde(rename = "emailAddress")]
    email: String,
}

// JSON: {"userName":"Alice","emailAddress":"alice@example.com"}
```

#### skip - 跳过字段

```rust
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    
    #[serde(skip)]
    password: String,  // 不会被序列化
    
    #[serde(skip_serializing)]
    internal_id: u64,  // 只在反序列化时使用
}
```

#### default - 默认值

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    
    #[serde(default)]
    port: u16,  // 如果 JSON 中没有，使用默认值 0
    
    #[serde(default = "default_timeout")]
    timeout: u32,
}

fn default_timeout() -> u32 {
    30
}

// JSON: {"host":"localhost"}
// 反序列化后: Config { host: "localhost", port: 0, timeout: 30 }
```

#### flatten - 展平嵌套

```rust
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    
    #[serde(flatten)]
    contact: Contact,
}

#[derive(Serialize, Deserialize)]
struct Contact {
    email: String,
    phone: String,
}

// JSON: {"name":"Alice","email":"alice@example.com","phone":"123-456"}
// 而不是: {"name":"Alice","contact":{"email":"...","phone":"..."}}
```

### 4. 枚举序列化

枚举有多种序列化方式。

#### 外部标记（默认）

```rust
#[derive(Serialize, Deserialize)]
enum Message {
    Text(String),
    Number(i32),
}

// JSON: {"Text":"hello"} 或 {"Number":42}
```

#### 内部标记

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Text { content: String },
    Image { url: String },
}

// JSON: {"type":"Text","content":"hello"}
// JSON: {"type":"Image","url":"http://..."}
```

#### 相邻标记

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum Message {
    Text(String),
    Number(i32),
}

// JSON: {"type":"Text","data":"hello"}
// JSON: {"type":"Number","data":42}
```

#### 无标记

```rust
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Value {
    String(String),
    Number(i32),
    Bool(bool),
}

// JSON: "hello" 或 42 或 true
```

### 5. TOML 序列化

TOML 常用于配置文件。

```rust
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    title: String,
    
    #[serde(rename = "owner")]
    owner_info: Owner,
    
    database: Database,
}

#[derive(Serialize, Deserialize)]
struct Owner {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Database {
    host: String,
    port: u16,
    enabled: bool,
}

// 序列化
let config = Config { /* ... */ };
let toml_string = toml::to_string(&config)?;

// TOML 格式:
// title = "My App"
//
// [owner]
// name = "Alice"
// email = "alice@example.com"
//
// [database]
// host = "localhost"
// port = 5432
// enabled = true

// 反序列化
let config: Config = toml::from_str(&toml_string)?;
```

### 6. YAML 序列化

YAML 也常用于配置文件。

```rust
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    version: String,
    dependencies: Vec<String>,
}

// 序列化
let config = Config {
    name: "my-app".to_string(),
    version: "1.0.0".to_string(),
    dependencies: vec!["serde".to_string(), "tokio".to_string()],
};

let yaml = serde_yaml::to_string(&config)?;

// YAML 格式:
// name: my-app
// version: 1.0.0
// dependencies:
//   - serde
//   - tokio

// 反序列化
let config: Config = serde_yaml::from_str(&yaml)?;
```

### 7. 自定义序列化

有时需要自定义序列化逻辑。

#### 自定义序列化函数

```rust
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    
    #[serde(serialize_with = "serialize_age")]
    #[serde(deserialize_with = "deserialize_age")]
    age: u32,
}

fn serialize_age<S>(age: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 将年龄序列化为字符串
    serializer.serialize_str(&format!("{} years old", age))
}

fn deserialize_age<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // 从 "30 years old" 解析出 30
    s.split_whitespace()
        .next()
        .and_then(|n| n.parse().ok())
        .ok_or_else(|| serde::de::Error::custom("invalid age format"))
}
```

#### 实现 Serialize trait

```rust
use serde::ser::{Serialize, Serializer, SerializeStruct};

struct Point {
    x: i32,
    y: i32,
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Point", 2)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.end()
    }
}
```

### 8. 泛型序列化

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Response<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// 使用
let response: Response<User> = Response {
    success: true,
    data: Some(user),
    error: None,
};

let json = serde_json::to_string(&response)?;
```

## 💻 实战项目：配置文件管理器

### 项目需求

构建一个支持多种格式的配置文件管理器：
1. 读取 JSON、TOML、YAML 配置
2. 类型安全的配置结构
3. 配置验证
4. 环境变量覆盖

### 步骤 1：定义配置结构

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default)]
    pub workers: usize,
}

fn default_port() -> u16 {
    8080
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: LogLevel,
    
    pub output: LogOutput,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Stdout,
    File(String),
}
```

### 步骤 2：实现配置解析器

```rust
use std::fs;
use std::path::Path;

pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}

pub struct ConfigParser;

impl ConfigParser {
    pub fn load<P: AsRef<Path>>(path: P, format: ConfigFormat) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        
        match format {
            ConfigFormat::Json => Self::from_json(&content),
            ConfigFormat::Toml => Self::from_toml(&content),
            ConfigFormat::Yaml => Self::from_yaml(&content),
        }
    }
    
    pub fn from_json(s: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(s)?)
    }
    
    pub fn from_toml(s: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
        Ok(toml::from_str(s)?)
    }
    
    pub fn from_yaml(s: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
        Ok(serde_yaml::from_str(s)?)
    }
    
    pub fn to_json(config: &AppConfig) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(config)?)
    }
    
    pub fn to_toml(config: &AppConfig) -> Result<String, Box<dyn std::error::Error>> {
        Ok(toml::to_string(config)?)
    }
    
    pub fn to_yaml(config: &AppConfig) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_yaml::to_string(config)?)
    }
}
```

### 步骤 3：配置验证

```rust
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(config: &AppConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // 验证服务器配置
        if config.server.host.is_empty() {
            errors.push("服务器主机不能为空".to_string());
        }
        
        if config.server.port == 0 {
            errors.push("服务器端口必须大于 0".to_string());
        }
        
        // 验证数据库配置
        if config.database.url.is_empty() {
            errors.push("数据库 URL 不能为空".to_string());
        }
        
        if config.database.max_connections == 0 {
            errors.push("最大连接数必须大于 0".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### 步骤 4：使用示例

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从 JSON 加载
    let config = ConfigParser::load("config.json", ConfigFormat::Json)?;
    
    // 验证配置
    match ConfigValidator::validate(&config) {
        Ok(_) => println!("配置验证通过"),
        Err(errors) => {
            println!("配置验证失败:");
            for error in errors {
                println!("  - {}", error);
            }
            return Ok(());
        }
    }
    
    // 使用配置
    println!("服务器: {}:{}", config.server.host, config.server.port);
    println!("数据库: {}", config.database.url);
    println!("日志级别: {:?}", config.logging.level);
    
    // 转换为其他格式
    let toml = ConfigParser::to_toml(&config)?;
    println!("\nTOML 格式:\n{}", toml);
    
    Ok(())
}
```

## 🔍 深入理解

### Serde 的工作原理

```
数据结构 ──Serialize trait──> 数据模型 ──Format──> 字节流
                                  ↓
                            (JSON, TOML, etc)

字节流 ──Format──> 数据模型 ──Deserialize trait──> 数据结构
```

**关键点**：
- Serde 提供统一的数据模型
- 格式库只需实现序列化器/反序列化器
- 数据结构通过 derive 宏自动实现 trait

### 性能考虑

```rust
// ✅ 高效：直接序列化
let json = serde_json::to_string(&data)?;

// ❌ 低效：先转为 Value 再序列化
let value = serde_json::to_value(&data)?;
let json = serde_json::to_string(&value)?;

// ✅ 流式处理大文件
let file = File::create("output.json")?;
serde_json::to_writer(file, &data)?;
```

## 📝 常见问题

### 问题 1：字段缺失

```rust
// 错误：JSON 中缺少字段
// {"name": "Alice"}  // 缺少 age

#[derive(Deserialize)]
struct User {
    name: String,
    age: u32,  // 错误！
}

// 解决方案 1：使用 Option
#[derive(Deserialize)]
struct User {
    name: String,
    age: Option<u32>,
}

// 解决方案 2：使用 default
#[derive(Deserialize)]
struct User {
    name: String,
    #[serde(default)]
    age: u32,
}
```

### 问题 2：类型不匹配

```rust
// JSON: {"age": "30"}  // 字符串而不是数字

#[derive(Deserialize)]
struct User {
    #[serde(deserialize_with = "deserialize_age")]
    age: u32,
}

fn deserialize_age<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}
```

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 使用 derive 宏实现序列化
- [ ] 处理 JSON、TOML、YAML 格式
- [ ] 使用字段属性控制序列化
- [ ] 序列化枚举和泛型
- [ ] 实现自定义序列化逻辑
- [ ] 处理序列化错误
- [ ] 验证反序列化的数据

## 🔗 延伸阅读

- [Serde 官方文档](https://serde.rs/)
- [serde_json 文档](https://docs.rs/serde_json/)
- [TOML 规范](https://toml.io/)
- [YAML 规范](https://yaml.org/)

---

**掌握 Serde，轻松处理数据序列化！** 🦀🚀
