/// 统计计算函数

/// 计算平均值
///
/// # 示例
///
/// ```
/// use modules_cargo::math::stats::mean;
/// assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
/// assert_eq!(mean(&[]), None);
/// ```
pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    Some(data.iter().sum::<f64>() / data.len() as f64)
}

/// 计算中位数
pub fn median(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    if len % 2 == 0 {
        Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
    } else {
        Some(sorted[len / 2])
    }
}

/// 计算标准差
pub fn std_dev(data: &[f64]) -> Option<f64> {
    let m = mean(data)?;
    let variance = data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / data.len() as f64;
    Some(variance.sqrt())
}

/// 计算数据范围（最大值 - 最小值）
pub fn range(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some(max - min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(median(&[3.0, 1.0, 5.0, 2.0, 4.0]), Some(3.0));
    }

    #[test]
    fn test_median_even() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }

    #[test]
    fn test_std_dev() {
        let result = std_dev(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).unwrap();
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_range() {
        assert_eq!(range(&[1.0, 5.0, 3.0, 9.0, 2.0]), Some(8.0));
        assert_eq!(range(&[]), None);
    }
}
