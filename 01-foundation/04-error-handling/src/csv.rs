use crate::errors::{AppError, AppResult};

/// CSV 记录
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub score: f64,
}

/// CSV 解析器
pub struct CsvParser;

impl CsvParser {
    /// 从字符串解析 CSV 内容
    ///
    /// # 错误处理演示
    /// 这个函数展示了多层错误处理：
    /// - 使用 ? 传播错误
    /// - 使用自定义错误类型提供上下文
    /// - 错误包含行号信息
    pub fn parse(content: &str) -> AppResult<Vec<Record>> {
        let mut records = Vec::new();
        let mut lines = content.lines().enumerate();

        // 跳过标题行
        let header = lines.next();
        if header.is_none() {
            return Err(AppError::EmptyFile("CSV 内容为空".to_string()));
        }

        // 解析数据行
        for (line_num, line) in lines {
            let line_num = line_num + 1; // 从 1 开始计数

            if line.trim().is_empty() {
                continue; // 跳过空行
            }

            let record = Self::parse_line(line_num, line)?;
            records.push(record);
        }

        Ok(records)
    }

    /// 解析单行 CSV 数据
    fn parse_line(line_num: usize, line: &str) -> AppResult<Record> {
        let fields: Vec<&str> = line.split(',').map(|f| f.trim()).collect();

        if fields.len() != 4 {
            return Err(AppError::csv_parse(
                line_num,
                format!("期望 4 个字段，实际 {} 个", fields.len()),
            ));
        }

        let name = fields[0].to_string();
        if name.is_empty() {
            return Err(AppError::csv_parse(line_num, "姓名不能为空"));
        }

        // 解析年龄 - 演示错误转换
        let age = fields[1]
            .parse::<u32>()
            .map_err(|_| AppError::parse_number(fields[1]))?;

        let email = fields[2].to_string();

        // 解析分数
        let score = fields[3]
            .parse::<f64>()
            .map_err(|_| AppError::parse_number(fields[3]))?;

        Ok(Record {
            name,
            age,
            email,
            score,
        })
    }

    /// 从文件读取并解析 CSV
    pub fn parse_file(path: &str) -> AppResult<Vec<Record>> {
        // std::io::Error 通过 #[from] 自动转换为 AppError::Io
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// 将记录写入 CSV 字符串
    pub fn to_csv(records: &[Record]) -> String {
        let mut lines = vec!["name,age,email,score".to_string()];

        for record in records {
            lines.push(format!(
                "{},{},{},{:.2}",
                record.name, record.age, record.email, record.score
            ));
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CSV: &str = "name,age,email,score
Alice,30,alice@example.com,95.5
Bob,25,bob@example.com,87.0
Carol,35,carol@example.com,92.3";

    #[test]
    fn test_parse_valid_csv() {
        let records = CsvParser::parse(VALID_CSV).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].name, "Alice");
        assert_eq!(records[0].age, 30);
        assert_eq!(records[0].score, 95.5);
    }

    #[test]
    fn test_parse_empty_csv() {
        let result = CsvParser::parse("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::EmptyFile(_)));
    }

    #[test]
    fn test_parse_invalid_age() {
        let csv = "name,age,email,score\nAlice,abc,alice@example.com,95.5";
        let result = CsvParser::parse(csv);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::ParseNumber { .. }));
    }

    #[test]
    fn test_parse_missing_fields() {
        let csv = "name,age,email,score\nAlice,30";
        let result = CsvParser::parse(csv);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::CsvParse { .. }));
    }

    #[test]
    fn test_to_csv() {
        let records = vec![Record {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
            score: 95.5,
        }];
        let csv = CsvParser::to_csv(&records);
        assert!(csv.contains("Alice,30,alice@example.com,95.50"));
    }

    #[test]
    fn test_roundtrip() {
        let records = CsvParser::parse(VALID_CSV).unwrap();
        let csv = CsvParser::to_csv(&records);
        let records2 = CsvParser::parse(&csv).unwrap();
        assert_eq!(records, records2);
    }
}
