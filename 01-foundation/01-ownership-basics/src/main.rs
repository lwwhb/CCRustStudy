mod calculator;
mod history;

use std::io::{self, Write};
use calculator::Calculator;

fn main() {
    let mut calc = Calculator::new();

    println!("欢迎使用 Rust 计算器！");
    println!("支持的命令：");
    println!("  <数字> <运算符> <数字>  - 执行计算（例如：5 + 3）");
    println!("  history                  - 查看历史记录");
    println!("  clear                    - 清除历史记录");
    println!("  quit                     - 退出程序");
    println!();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "quit" | "exit" => {
                println!("再见！");
                break;
            }
            "history" => {
                calc.show_history();
            }
            "clear" => {
                calc.clear_history();
                println!("历史记录已清除");
            }
            _ => {
                // 解析并执行计算
                match calc.calculate(input) {
                    Ok(result) => println!("结果: {}", result),
                    Err(e) => println!("错误: {}", e),
                }
            }
        }
    }
}
