mod csv;
mod errors;
mod validator;

use csv::{CsvParser, Record};
use errors::{AppError, AppResult};
use validator::Validator;

fn main() {
    println!("=== CSV 文件处理工具 ===\n");

    // 示例 CSV 数据
    let csv_data = "name,age,email,score
Alice,30,alice@example.com,95.5
Bob,25,bob@example.com,87.0
Carol,17,carol@example.com,92.3
David,35,david-invalid-email,88.5
Eve,45,eve@example.com,105.0
Frank,28,frank@example.com,91.2";

    println!("原始 CSV 数据:");
    println!("{}\n", csv_data);

    // 演示 1：基本解析
    println!("=== 演示 1：解析 CSV ===");
    match parse_and_display(csv_data) {
        Ok(count) => println!("✓ 成功解析 {} 条记录\n", count),
        Err(e) => println!("✗ 解析失败: {}\n", e),
    }

    // 演示 2：验证数据
    println!("=== 演示 2：数据验证 ===");
    match parse_and_validate(csv_data) {
        Ok(count) => println!("✓ 所有 {} 条记录验证通过\n", count),
        Err(e) => println!("✗ 验证失败: {}\n", e),
    }

    // 演示 3：过滤有效记录
    println!("=== 演示 3：过滤有效记录 ===");
    filter_and_display(csv_data);

    // 演示 4：文件操作（使用 anyhow）
    println!("\n=== 演示 4：文件操作 ===");
    demonstrate_file_operations();

    // 演示 5：错误链
    println!("\n=== 演示 5：错误传播链 ===");
    demonstrate_error_chain();
}

/// 解析并显示记录
fn parse_and_display(csv_data: &str) -> AppResult<usize> {
    let records = CsvParser::parse(csv_data)?;

    for (i, record) in records.iter().enumerate() {
        println!(
            "  {}. {} ({}岁) - {} - 分数: {:.1}",
            i + 1,
            record.name,
            record.age,
            record.email,
            record.score
        );
    }

    Ok(records.len())
}

/// 解析并验证所有记录
fn parse_and_validate(csv_data: &str) -> AppResult<usize> {
    let records = CsvParser::parse(csv_data)?;
    Validator::validate_all(&records)?;
    Ok(records.len())
}

/// 过滤有效记录并显示结果
fn filter_and_display(csv_data: &str) {
    match CsvParser::parse(csv_data) {
        Ok(records) => {
            let (valid, errors) = Validator::filter_valid(records);

            println!("有效记录 ({} 条):", valid.len());
            for record in &valid {
                println!("  ✓ {} - {}", record.name, record.email);
            }

            if !errors.is_empty() {
                println!("\n无效记录 ({} 条):", errors.len());
                for (line, error) in errors {
                    println!("  ✗ 第 {} 行: {}", line, error);
                }
            }
        }
        Err(e) => println!("解析失败: {}", e),
    }
}

/// 演示文件操作（使用 anyhow）
fn demonstrate_file_operations() {
    use anyhow::{Context, Result};

    fn process_file(path: &str) -> Result<Vec<Record>> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("无法读取文件: {}", path))?;

        let records = CsvParser::parse(&content)
            .with_context(|| "CSV 解析失败")?;

        Validator::validate_all(&records)
            .with_context(|| "数据验证失败")?;

        Ok(records)
    }

    // 尝试读取不存在的文件
    match process_file("nonexistent.csv") {
        Ok(_) => println!("文件处理成功"),
        Err(e) => {
            println!("文件处理失败:");
            println!("  错误: {}", e);
            // 显示错误链
            for cause in e.chain().skip(1) {
                println!("  原因: {}", cause);
            }
        }
    }
}

/// 演示错误传播链
fn demonstrate_error_chain() {
    fn level3() -> AppResult<i32> {
        Err(AppError::parse_number("abc"))
    }

    fn level2() -> AppResult<i32> {
        level3()?;
        Ok(42)
    }

    fn level1() -> AppResult<i32> {
        level2()?;
        Ok(100)
    }

    match level1() {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误传播: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CSV: &str = "name,age,email,score
Alice,30,alice@example.com,95.5
Bob,25,bob@example.com,87.0";

    const INVALID_CSV: &str = "name,age,email,score
Alice,30,alice@example.com,95.5
Bob,abc,bob@example.com,87.0";

    #[test]
    fn test_parse_and_display() {
        let result = parse_and_display(VALID_CSV);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_parse_invalid_csv() {
        let result = parse_and_display(INVALID_CSV);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_and_validate() {
        let result = parse_and_validate(VALID_CSV);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_propagation() {
        fn inner() -> AppResult<()> {
            Err(AppError::parse_number("test"))
        }

        fn outer() -> AppResult<()> {
            inner()?;
            Ok(())
        }

        assert!(outer().is_err());
    }

    #[test]
    fn test_option_to_result() {
        fn find_value(values: &[i32], target: i32) -> AppResult<usize> {
            values
                .iter()
                .position(|&x| x == target)
                .ok_or_else(|| AppError::InvalidFormat("未找到目标值".to_string()))
        }

        let values = vec![1, 2, 3, 4, 5];
        assert!(find_value(&values, 3).is_ok());
        assert!(find_value(&values, 10).is_err());
    }
}

