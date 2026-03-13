/// 配置验证器
///
/// 验证配置的有效性

use crate::config::{AppConfig, DatabaseConfig, ServerConfig};

/// 验证错误
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// 配置验证器
pub struct ConfigValidator;

impl ConfigValidator {
    /// 验证完整配置
    pub fn validate(config: &AppConfig) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // 验证服务器配置
        if let Err(mut e) = Self::validate_server(&config.server) {
            errors.append(&mut e);
        }

        // 验证数据库配置
        if let Err(mut e) = Self::validate_database(&config.database) {
            errors.append(&mut e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证服务器配置
    pub fn validate_server(config: &ServerConfig) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // 验证主机名
        if config.host.is_empty() {
            errors.push(ValidationError::new("server.host", "主机名不能为空"));
        }

        // 验证端口
        if config.port == 0 {
            errors.push(ValidationError::new("server.port", "端口号无效"));
        }

        // 验证工作线程数
        if config.workers == 0 {
            errors.push(ValidationError::new("server.workers", "工作线程数必须大于 0"));
        }

        // 验证最大连接数
        if config.max_connections == 0 {
            errors.push(ValidationError::new(
                "server.max_connections",
                "最大连接数必须大于 0",
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 验证数据库配置
    pub fn validate_database(config: &DatabaseConfig) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // 验证 URL
        if config.url.is_empty() {
            errors.push(ValidationError::new("database.url", "数据库 URL 不能为空"));
        }

        // 验证连接池大小
        if config.pool_size > 100 {
            errors.push(ValidationError::new(
                "database.pool_size",
                "连接池大小不应超过 100",
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{LogLevel, LogOutput, LoggingConfig};

    fn create_valid_config() -> AppConfig {
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
    fn test_valid_config() {
        let config = create_valid_config();
        assert!(ConfigValidator::validate(&config).is_ok());
    }

    #[test]
    fn test_invalid_host() {
        let mut config = create_valid_config();
        config.server.host = String::new();

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "server.host"));
    }

    #[test]
    fn test_invalid_port() {
        let mut config = create_valid_config();
        config.server.port = 0;

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_workers() {
        let mut config = create_valid_config();
        config.server.workers = 0;

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_database_url() {
        let mut config = create_valid_config();
        config.database.url = String::new();

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "database.url"));
    }

    #[test]
    fn test_pool_size_warning() {
        let mut config = create_valid_config();
        config.database.pool_size = 150;

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "database.pool_size"));
    }

    #[test]
    fn test_multiple_errors() {
        let mut config = create_valid_config();
        config.server.host = String::new();
        config.server.port = 0;
        config.database.url = String::new();

        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors.len() >= 3);
    }
}
