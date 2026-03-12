use crate::history::History;

/// 计算器结构体
///
/// 包含计算历史记录，演示所有权和借用的概念
pub struct Calculator {
    history: History,
}

impl Calculator {
    /// 创建新的计算器实例
    pub fn new() -> Self {
        Calculator {
            history: History::new(),
        }
    }

    /// 执行计算
    ///
    /// # 参数
    /// * `input` - 输入字符串，格式为 "数字 运算符 数字"
    ///
    /// # 返回
    /// * `Ok(f64)` - 计算结果
    /// * `Err(String)` - 错误信息
    pub fn calculate(&mut self, input: &str) -> Result<f64, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            return Err("格式错误！请使用：<数字> <运算符> <数字>".to_string());
        }

        // 解析第一个数字
        let num1 = parts[0]
            .parse::<f64>()
            .map_err(|_| format!("无效的数字: {}", parts[0]))?;

        // 获取运算符
        let operator = parts[1];

        // 解析第二个数字
        let num2 = parts[2]
            .parse::<f64>()
            .map_err(|_| format!("无效的数字: {}", parts[2]))?;

        // 执行运算
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    return Err("错误：除数不能为零".to_string());
                }
                num1 / num2
            }
            _ => return Err(format!("不支持的运算符: {}", operator)),
        };

        // 将计算记录添加到历史
        // 这里演示了所有权：创建新的 String 并转移所有权给 history
        let record = format!("{} {} {} = {}", num1, operator, num2, result);
        self.history.add(record);

        Ok(result)
    }

    /// 显示计算历史
    ///
    /// 这里演示了借用：我们借用 history 来读取数据，而不获取所有权
    pub fn show_history(&self) {
        if self.history.is_empty() {
            println!("暂无历史记录");
        } else {
            println!("计算历史:");
            // 借用 history 的引用来遍历
            for (i, record) in self.history.iter().enumerate() {
                println!("{}. {}", i + 1, record);
            }
        }
    }

    /// 清除历史记录
    ///
    /// 这里演示了可变借用：我们需要修改 history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut calc = Calculator::new();
        let result = calc.calculate("5 + 3").unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_subtraction() {
        let mut calc = Calculator::new();
        let result = calc.calculate("10 - 4").unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_multiplication() {
        let mut calc = Calculator::new();
        let result = calc.calculate("6 * 7").unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_division() {
        let mut calc = Calculator::new();
        let result = calc.calculate("20 / 4").unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::new();
        let result = calc.calculate("10 / 0");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_operator() {
        let mut calc = Calculator::new();
        let result = calc.calculate("5 % 3");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format() {
        let mut calc = Calculator::new();
        let result = calc.calculate("5 +");
        assert!(result.is_err());
    }

    #[test]
    fn test_history() {
        let mut calc = Calculator::new();
        calc.calculate("5 + 3").unwrap();
        calc.calculate("10 * 2").unwrap();

        // 历史记录应该有两条
        assert_eq!(calc.history.len(), 2);
    }

    #[test]
    fn test_clear_history() {
        let mut calc = Calculator::new();
        calc.calculate("5 + 3").unwrap();
        calc.clear_history();

        assert!(calc.history.is_empty());
    }
}
