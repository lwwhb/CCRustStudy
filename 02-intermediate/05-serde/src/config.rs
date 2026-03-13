/// 配置结构定义
///
/// 演示 Serde 的各种特性

use serde::{Deserialize, Serialize};

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,

    #[serde(default = "default_workers")]
    pub workers: usize,

    #[serde(rename = "maxConnections")]
    pub max_connections: u32,
}

fn default_workers() -> usize {
    4
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabaseConfig {
    pub url: String,

    #[serde(default)]
    pub pool_size: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub output: LogOutput,
}

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 日志输出
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "path")]
pub enum LogOutput {
    Console,
    File(String),
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,

    #[serde(rename = "emailAddress")]
    pub email: String,

    #[serde(skip)]
    pub password_hash: String,

    #[serde(default)]
    pub active: bool,
}

impl User {
    pub fn new(id: u32, name: String, email: String) -> Self {
        User {
            id,
            name,
            email,
            password_hash: String::new(),
            active: true,
        }
    }
}

/// API 响应
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Alice"));
        assert!(json.contains("emailAddress"));
        assert!(!json.contains("password_hash")); // 应该被跳过
    }

    #[test]
    fn test_user_deserialization() {
        let json = r#"{
            "id": 1,
            "name": "Bob",
            "emailAddress": "bob@example.com",
            "active": true
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Bob");
        assert_eq!(user.email, "bob@example.com");
        assert!(user.active);
    }

    #[test]
    fn test_log_level() {
        let level = LogLevel::Info;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, r#""info""#);

        let parsed: LogLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, LogLevel::Info);
    }

    #[test]
    fn test_log_output() {
        let output = LogOutput::File("/var/log/app.log".to_string());
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("File"));

        let console = LogOutput::Console;
        let json2 = serde_json::to_string(&console).unwrap();
        assert!(json2.contains("Console"));
    }

    #[test]
    fn test_api_response() {
        let response = ApiResponse::success("Hello".to_string());
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("Hello"));

        let error_response: ApiResponse<String> = ApiResponse::error("Not found".to_string());
        let json2 = serde_json::to_string(&error_response).unwrap();
        assert!(json2.contains("false"));
        assert!(json2.contains("Not found"));
    }
}
