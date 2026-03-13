mod config;
mod parser;
mod validator;

use config::{
    ApiResponse, AppConfig, DatabaseConfig, LogLevel, LogOutput, LoggingConfig, ServerConfig,
    User,
};
use parser::{ConfigFormat, ConfigParser};
use validator::ConfigValidator;

fn main() {
    println!("=== 序列化与反序列化演示 ===\n");

    // 演示 1：基本序列化
    println!("=== 演示 1：基本序列化 ===");
    demonstrate_basic_serialization();

    // 演示 2：JSON 序列化
    println!("\n=== 演示 2：JSON 序列化 ===");
    demonstrate_json();

    // 演示 3：TOML 序列化
    println!("\n=== 演示 3：TOML 序列化 ===");
    demonstrate_toml();

    // 演示 4：YAML 序列化
    println!("\n=== 演示 4：YAML 序列化 ===");
    demonstrate_yaml();

    // 演示 5：字段属性
    println!("\n=== 演示 5：字段属性（rename、skip）===");
    demonstrate_field_attributes();

    // 演示 6：枚举序列化
    println!("\n=== 演示 6：枚举序列化 ===");
    demonstrate_enums();

    // 演示 7：泛型序列化
    println!("\n=== 演示 7：泛型序列化 ===");
    demonstrate_generics();

    // 演示 8：配置验证
    println!("\n=== 演示 8：配置验证 ===");
    demonstrate_validation();
}

/// 演示基本序列化
fn demonstrate_basic_serialization() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());

    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("用户对象序列化为 JSON:\n{}", json);
}

/// 演示 JSON 序列化
fn demonstrate_json() {
    let config = create_sample_config();

    match ConfigParser::to_json(&config) {
        Ok(json) => println!("配置序列化为 JSON:\n{}", json),
        Err(e) => println!("错误: {}", e),
    }
}

/// 演示 TOML 序列化
fn demonstrate_toml() {
    let config = create_sample_config();

    match ConfigParser::to_toml(&config) {
        Ok(toml) => println!("配置序列化为 TOML:\n{}", toml),
        Err(e) => println!("错误: {}", e),
    }
}

/// 演示 YAML 序列化
fn demonstrate_yaml() {
    let config = create_sample_config();

    match ConfigParser::to_yaml(&config) {
        Ok(yaml) => println!("配置序列化为 YAML:\n{}", yaml),
        Err(e) => println!("错误: {}", e),
    }
}

/// 演示字段属性
fn demonstrate_field_attributes() {
    let user = User::new(1, "Bob".to_string(), "bob@example.com".to_string());

    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("注意字段名变化:");
    println!("- email -> emailAddress (rename)");
    println!("- password_hash 被跳过 (skip)");
    println!("\n{}", json);
}

/// 演示枚举序列化
fn demonstrate_enums() {
    let levels = vec![
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];

    println!("日志级别序列化:");
    for level in levels {
        let json = serde_json::to_string(&level).unwrap();
        println!("  {:?} -> {}", level, json);
    }

    println!("\n日志输出序列化:");
    let outputs = vec![
        LogOutput::Console,
        LogOutput::File("/var/log/app.log".to_string()),
    ];

    for output in outputs {
        let json = serde_json::to_string(&output).unwrap();
        println!("  {:?} -> {}", output, json);
    }
}

/// 演示泛型序列化
fn demonstrate_generics() {
    let success: ApiResponse<String> = ApiResponse::success("操作成功".to_string());
    let json1 = serde_json::to_string_pretty(&success).unwrap();
    println!("成功响应:\n{}", json1);

    let error: ApiResponse<String> = ApiResponse::error("未找到资源".to_string());
    let json2 = serde_json::to_string_pretty(&error).unwrap();
    println!("\n错误响应:\n{}", json2);
}

/// 演示配置验证
fn demonstrate_validation() {
    let valid_config = create_sample_config();

    match ConfigValidator::validate(&valid_config) {
        Ok(_) => println!("✓ 配置验证通过"),
        Err(errors) => {
            println!("✗ 配置验证失败:");
            for error in errors {
                println!("  - {}: {}", error.field, error.message);
            }
        }
    }

    // 创建无效配置
    let mut invalid_config = create_sample_config();
    invalid_config.server.host = String::new();
    invalid_config.server.port = 0;

    println!("\n测试无效配置:");
    match ConfigValidator::validate(&invalid_config) {
        Ok(_) => println!("✓ 配置验证通过"),
        Err(errors) => {
            println!("✗ 配置验证失败:");
            for error in errors {
                println!("  - {}: {}", error.field, error.message);
            }
        }
    }
}

/// 创建示例配置
fn create_sample_config() -> AppConfig {
    AppConfig {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User::new(1, "Test".to_string(), "test@example.com".to_string());
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_config_roundtrip() {
        let config = create_sample_config();
        let json = ConfigParser::to_json(&config).unwrap();
        let parsed = ConfigParser::parse_json(&json).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_validation() {
        let config = create_sample_config();
        assert!(ConfigValidator::validate(&config).is_ok());
    }

    #[test]
    fn test_api_response() {
        let response: ApiResponse<i32> = ApiResponse::success(42);
        assert!(response.success);
        assert_eq!(response.data, Some(42));
    }
}

