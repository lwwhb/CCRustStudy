/// 数学模块
///
/// 提供基本数学运算和统计函数

pub mod basic;
pub mod stats;

// 重导出常用函数，方便使用者直接从 math 模块访问
pub use basic::{add, divide, gcd, is_prime, lcm, multiply, power, subtract};
pub use stats::{mean, median, std_dev};
