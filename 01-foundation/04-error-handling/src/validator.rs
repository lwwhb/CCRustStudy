use crate::csv::Record;
use crate::errors::{AppError, AppResult};

/// 数据验证器
pub struct Validator;

impl Validator {
    /// 验证单条记录
    ///
    /// # 验证规则
    /// - 姓名：不能为空，长度 2-50
    /// - 年龄：18-100
    /// - 邮箱：必须包含 @
    /// - 分数：0-100
    pub fn validate_record(record: &Record, line_num: usize) -> AppResult<()> {
        // 验证姓名
        if record.name.is_empty() {
            return Err(AppError::validation(line_num, "name", "姓名不能为空"));
        }
        if record.name.len() < 2 || record.name.len() > 50 {
            return Err(AppError::validation(
                line_num,
                "name",
                "姓名长度必须在 2-50 之间",
            ));
        }

        // 验证年龄
        if record.age < 18 || record.age > 100 {
            return Err(AppError::validation(
                line_num,
                "age",
                format!("年龄必须在 18-100 之间，实际值: {}", record.age),
            ));
        }

        // 验证邮箱
        if !record.email.contains('@') {
            return Err(AppError::validation(
                line_num,
                "email",
                "邮箱格式无效，必须包含 @",
            ));
        }

        // 验证分数
        if record.score < 0.0 || record.score > 100.0 {
            return Err(AppError::validation(
                line_num,
                "score",
                format!("分数必须在 0-100 之间，实际值: {:.2}", record.score),
            ));
        }

        Ok(())
    }

    /// 验证所有记录
    ///
    /// 返回第一个验证失败的错误
    pub fn validate_all(records: &[Record]) -> AppResult<()> {
        for (i, record) in records.iter().enumerate() {
            Self::validate_record(record, i + 2)?; // +2 因为有标题行，且从 1 开始
        }
        Ok(())
    }

    /// 验证所有记录并收集所有错误
    ///
    /// 不会在第一个错误处停止，而是收集所有错误
    pub fn validate_all_collect_errors(records: &[Record]) -> Result<(), Vec<AppError>> {
        let errors: Vec<AppError> = records
            .iter()
            .enumerate()
            .filter_map(|(i, record)| Self::validate_record(record, i + 2).err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 过滤出有效的记录
    ///
    /// 返回 (有效记录, 错误列表)
    pub fn filter_valid(records: Vec<Record>) -> (Vec<Record>, Vec<(usize, AppError)>) {
        let mut valid = Vec::new();
        let mut errors = Vec::new();

        for (i, record) in records.into_iter().enumerate() {
            match Self::validate_record(&record, i + 2) {
                Ok(_) => valid.push(record),
                Err(e) => errors.push((i + 2, e)),
            }
        }

        (valid, errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_record() -> Record {
        Record {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
            score: 95.5,
        }
    }

    #[test]
    fn test_validate_valid_record() {
        let record = create_valid_record();
        assert!(Validator::validate_record(&record, 1).is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let mut record = create_valid_record();
        record.name = "".to_string();
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation { .. }));
    }

    #[test]
    fn test_validate_short_name() {
        let mut record = create_valid_record();
        record.name = "A".to_string();
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_age() {
        let mut record = create_valid_record();
        record.age = 15;
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());

        record.age = 101;
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_email() {
        let mut record = create_valid_record();
        record.email = "invalid-email".to_string();
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_score() {
        let mut record = create_valid_record();
        record.score = -1.0;
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());

        record.score = 101.0;
        let result = Validator::validate_record(&record, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_all() {
        let records = vec![create_valid_record(), create_valid_record()];
        assert!(Validator::validate_all(&records).is_ok());
    }

    #[test]
    fn test_validate_all_with_error() {
        let mut invalid = create_valid_record();
        invalid.age = 15;
        let records = vec![create_valid_record(), invalid];
        assert!(Validator::validate_all(&records).is_err());
    }

    #[test]
    fn test_filter_valid() {
        let mut invalid = create_valid_record();
        invalid.age = 15;
        let records = vec![create_valid_record(), invalid, create_valid_record()];

        let (valid, errors) = Validator::filter_valid(records);
        assert_eq!(valid.len(), 2);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_validate_all_collect_errors() {
        let mut invalid1 = create_valid_record();
        invalid1.age = 15;
        let mut invalid2 = create_valid_record();
        invalid2.email = "invalid".to_string();

        let records = vec![create_valid_record(), invalid1, invalid2];
        let result = Validator::validate_all_collect_errors(&records);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }
}
