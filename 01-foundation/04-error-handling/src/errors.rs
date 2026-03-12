use thiserror::Error;

/// 应用程序错误类型
///
/// 使用 thiserror 宏自动实现 Display 和 Error trait
/// 这是 Rust 中定义自定义错误的推荐方式
#[derive(Error, Debug)]
pub enum AppError {
    /// IO 错误（文件读写等）
    /// #[from] 自动实现 From<io::Error> for AppError
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// CSV 解析错误，包含行号信息
    #[error("CSV 解析错误（第 {line} 行）: {message}")]
    CsvParse { line: usize, message: String },

    /// 数据验证错误
    #[error("数据验证失败（第 {line} 行，字段 '{field}'）: {message}")]
    Validation {
        line: usize,
        field: String,
        message: String,
    },

    /// 文件格式错误
    #[error("文件格式错误: {0}")]
    InvalidFormat(String),

    /// 数字解析错误
    #[error("数字解析错误: '{value}' 不是有效的数字")]
    ParseNumber { value: String },

    /// 空文件错误
    #[error("文件为空: {0}")]
    EmptyFile(String),
}

impl AppError {
    /// 创建 CSV 解析错误
    pub fn csv_parse(line: usize, message: impl Into<String>) -> Self {
        AppError::CsvParse {
            line,
            message: message.into(),
        }
    }

    /// 创建验证错误
    pub fn validation(line: usize, field: impl Into<String>, message: impl Into<String>) -> Self {
        AppError::Validation {
            line,
            field: field.into(),
            message: message.into(),
        }
    }

    /// 创建数字解析错误
    pub fn parse_number(value: impl Into<String>) -> Self {
        AppError::ParseNumber {
            value: value.into(),
        }
    }
}

/// 应用程序 Result 类型别名
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::csv_parse(5, "缺少字段");
        assert_eq!(err.to_string(), "CSV 解析错误（第 5 行）: 缺少字段");
    }

    #[test]
    fn test_validation_error() {
        let err = AppError::validation(3, "age", "年龄必须大于 0");
        assert!(err.to_string().contains("第 3 行"));
        assert!(err.to_string().contains("age"));
    }

    #[test]
    fn test_parse_number_error() {
        let err = AppError::parse_number("abc");
        assert!(err.to_string().contains("abc"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "文件未找到");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Io(_)));
    }
}
