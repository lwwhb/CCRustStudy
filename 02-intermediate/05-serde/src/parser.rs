/// 配置解析器
///
/// 支持 JSON、TOML、YAML 格式

use crate::config::AppConfig;
use std::fs;

/// 配置格式
#[derive(Debug, Clone, Copy)]
pub enum ConfigFormat {
    Json,
    Toml,
    Yaml,
}

/// 配置解析器
pub struct ConfigParser;

impl ConfigParser {
    /// 从文件加载配置
    pub fn load_from_file(path: &str) -> Result<AppConfig, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("无法读取文件: {}", e))?;

        let format = Self::detect_format(path)?;
        Self::parse(&content, format)
    }

    /// 解析配置字符串
    pub fn parse(content: &str, format: ConfigFormat) -> Result<AppConfig, String> {
        match format {
            ConfigFormat::Json => Self::parse_json(content),
            ConfigFormat::Toml => Self::parse_toml(content),
            ConfigFormat::Yaml => Self::parse_yaml(content),
        }
    }

    /// 解析 JSON
    pub fn parse_json(content: &str) -> Result<AppConfig, String> {
        serde_json::from_str(content)
            .map_err(|e| format!("JSON 解析错误: {}", e))
    }

    /// 解析 TOML
    pub fn parse_toml(content: &str) -> Result<AppConfig, String> {
        toml::from_str(content)
            .map_err(|e| format!("TOML 解析错误: {}", e))
    }

    /// 解析 YAML
    pub fn parse_yaml(content: &str) -> Result<AppConfig, String> {
        serde_yaml::from_str(content)
            .map_err(|e| format!("YAML 解析错误: {}", e))
    }

    /// 序列化为 JSON
    pub fn to_json(config: &AppConfig) -> Result<String, String> {
        serde_json::to_string_pretty(config)
            .map_err(|e| format!("JSON 序列化错误: {}", e))
    }

    /// 序列化为 TOML
    pub fn to_toml(config: &AppConfig) -> Result<String, String> {
        toml::to_string(config)
            .map_err(|e| format!("TOML 序列化错误: {}", e))
    }

    /// 序列化为 YAML
    pub fn to_yaml(config: &AppConfig) -> Result<String, String> {
        serde_yaml::to_string(config)
            .map_err(|e| format!("YAML 序列化错误: {}", e))
    }

    /// 检测文件格式
    fn detect_format(path: &str) -> Result<ConfigFormat, String> {
        if path.ends_with(".json") {
            Ok(ConfigFormat::Json)
        } else if path.ends_with(".toml") {
            Ok(ConfigFormat::Toml)
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            Ok(ConfigFormat::Yaml)
        } else {
            Err("无法识别的文件格式".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DatabaseConfig, LogLevel, LogOutput, LoggingConfig, ServerConfig};

    fn create_test_config() -> AppConfig {
        AppConfig {
            server: ServerConfig {
                host: "localhost".to_string(),
                port: 8080,
                workers: 4,
                max_connections: 1000,
            },
            database: DatabaseConfig {
                url: "postgres://localhost/mydb".to_string(),
                pool_size: 10,
                password: None,
            },
            logging: LoggingConfig {
                level: LogLevel::Info,
                output: LogOutput::Console,
            },
        }
    }

    #[test]
    fn test_json_roundtrip() {
        let config = create_test_config();
        let json = ConfigParser::to_json(&config).unwrap();
        let parsed = ConfigParser::parse_json(&json).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_toml_roundtrip() {
        let config = create_test_config();
        let toml = ConfigParser::to_toml(&config).unwrap();
        let parsed = ConfigParser::parse_toml(&toml).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_yaml_roundtrip() {
        let config = create_test_config();
        let yaml = ConfigParser::to_yaml(&config).unwrap();
        let parsed = ConfigParser::parse_yaml(&yaml).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_json_parsing() {
        let json = r#"{
            "server": {
                "host": "0.0.0.0",
                "port": 3000,
                "maxConnections": 500
            },
            "database": {
                "url": "sqlite://data.db",
                "pool_size": 5
            },
            "logging": {
                "level": "debug",
                "output": {
                    "type": "Console"
                }
            }
        }"#;

        let config = ConfigParser::parse_json(json).unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.server.workers, 4); // 默认值
    }

    #[test]
    fn test_detect_format() {
        assert!(matches!(
            ConfigParser::detect_format("config.json"),
            Ok(ConfigFormat::Json)
        ));
        assert!(matches!(
            ConfigParser::detect_format("config.toml"),
            Ok(ConfigFormat::Toml)
        ));
        assert!(matches!(
            ConfigParser::detect_format("config.yaml"),
            Ok(ConfigFormat::Yaml)
        ));
        assert!(ConfigParser::detect_format("config.txt").is_err());
    }
}
