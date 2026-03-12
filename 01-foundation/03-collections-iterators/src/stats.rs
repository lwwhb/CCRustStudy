/// 统计信息结构体
#[derive(Debug, Clone)]
pub struct Statistics {
    pub mean: f64,
    pub median: f64,
    pub mode: Vec<i32>,
    pub min: i32,
    pub max: i32,
    pub range: i32,
}

impl Statistics {
    /// 从数字列表计算统计信息
    pub fn from_numbers(numbers: &[i32]) -> Option<Self> {
        if numbers.is_empty() {
            return None;
        }

        let mean = Self::calculate_mean(numbers);
        let median = Self::calculate_median(numbers);
        let mode = Self::calculate_mode(numbers);
        let min = *numbers.iter().min().unwrap();
        let max = *numbers.iter().max().unwrap();
        let range = max - min;

        Some(Statistics {
            mean,
            median,
            mode,
            min,
            max,
            range,
        })
    }

    /// 计算平均值
    fn calculate_mean(numbers: &[i32]) -> f64 {
        let sum: i32 = numbers.iter().sum();
        sum as f64 / numbers.len() as f64
    }

    /// 计算中位数
    fn calculate_median(numbers: &[i32]) -> f64 {
        let mut sorted = numbers.to_vec();
        sorted.sort();

        let len = sorted.len();
        if len % 2 == 0 {
            let mid1 = sorted[len / 2 - 1];
            let mid2 = sorted[len / 2];
            (mid1 + mid2) as f64 / 2.0
        } else {
            sorted[len / 2] as f64
        }
    }

    /// 计算众数（可能有多个）
    fn calculate_mode(numbers: &[i32]) -> Vec<i32> {
        use std::collections::HashMap;

        let mut frequency = HashMap::new();
        for &num in numbers {
            *frequency.entry(num).or_insert(0) += 1;
        }

        let max_freq = frequency.values().max().copied().unwrap_or(0);

        frequency
            .into_iter()
            .filter(|(_, freq)| *freq == max_freq)
            .map(|(num, _)| num)
            .collect()
    }
}

/// 数字序列生成器
pub struct NumberSequence;

impl NumberSequence {
    /// 生成斐波那契数列
    pub fn fibonacci(n: usize) -> Vec<u64> {
        let mut fib = vec![0, 1];

        for i in 2..n {
            let next = fib[i - 1] + fib[i - 2];
            fib.push(next);
        }

        fib.into_iter().take(n).collect()
    }

    /// 生成质数序列
    pub fn primes(n: usize) -> Vec<u32> {
        let mut primes = Vec::new();
        let mut candidate = 2;

        while primes.len() < n {
            if Self::is_prime(candidate) {
                primes.push(candidate);
            }
            candidate += 1;
        }

        primes
    }

    /// 判断是否为质数
    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let sqrt = (n as f64).sqrt() as u32;
        for i in (3..=sqrt).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }

        true
    }

    /// 生成等差数列
    pub fn arithmetic(start: i32, diff: i32, count: usize) -> Vec<i32> {
        (0..count)
            .map(|i| start + diff * i as i32)
            .collect()
    }

    /// 生成等比数列
    pub fn geometric(start: f64, ratio: f64, count: usize) -> Vec<f64> {
        (0..count)
            .map(|i| start * ratio.powi(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_mean() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = Statistics::from_numbers(&numbers).unwrap();
        assert_eq!(stats.mean, 3.0);
    }

    #[test]
    fn test_statistics_median_odd() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = Statistics::from_numbers(&numbers).unwrap();
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_statistics_median_even() {
        let numbers = vec![1, 2, 3, 4];
        let stats = Statistics::from_numbers(&numbers).unwrap();
        assert_eq!(stats.median, 2.5);
    }

    #[test]
    fn test_statistics_mode() {
        let numbers = vec![1, 2, 2, 3, 3, 3, 4];
        let stats = Statistics::from_numbers(&numbers).unwrap();
        assert_eq!(stats.mode, vec![3]);
    }

    #[test]
    fn test_statistics_range() {
        let numbers = vec![1, 5, 3, 9, 2];
        let stats = Statistics::from_numbers(&numbers).unwrap();
        assert_eq!(stats.min, 1);
        assert_eq!(stats.max, 9);
        assert_eq!(stats.range, 8);
    }

    #[test]
    fn test_fibonacci() {
        let fib = NumberSequence::fibonacci(7);
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_primes() {
        let primes = NumberSequence::primes(5);
        assert_eq!(primes, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_arithmetic() {
        let seq = NumberSequence::arithmetic(2, 3, 5);
        assert_eq!(seq, vec![2, 5, 8, 11, 14]);
    }

    #[test]
    fn test_geometric() {
        let seq = NumberSequence::geometric(2.0, 3.0, 4);
        assert_eq!(seq, vec![2.0, 6.0, 18.0, 54.0]);
    }
}
