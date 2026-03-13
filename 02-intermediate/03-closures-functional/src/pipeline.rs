/// 数据处理管道
///
/// 演示函数式编程和闭包的使用

/// 数据管道构建器
pub struct Pipeline<T> {
    data: Vec<T>,
}

impl<T> Pipeline<T> {
    /// 创建新的管道
    pub fn new(data: Vec<T>) -> Self {
        Pipeline { data }
    }

    /// 映射操作
    pub fn map<F, U>(self, f: F) -> Pipeline<U>
    where
        F: Fn(T) -> U,
    {
        Pipeline {
            data: self.data.into_iter().map(f).collect(),
        }
    }

    /// 过滤操作
    pub fn filter<F>(self, predicate: F) -> Pipeline<T>
    where
        F: Fn(&T) -> bool,
    {
        Pipeline {
            data: self.data.into_iter().filter(predicate).collect(),
        }
    }

    /// 收集结果
    pub fn collect(self) -> Vec<T> {
        self.data
    }

    /// 归约操作
    pub fn fold<F, B>(self, init: B, f: F) -> B
    where
        F: Fn(B, T) -> B,
    {
        self.data.into_iter().fold(init, f)
    }

    /// 获取第一个元素
    pub fn first(self) -> Option<T> {
        self.data.into_iter().next()
    }

    /// 获取最后一个元素
    pub fn last(self) -> Option<T> {
        self.data.into_iter().last()
    }

    /// 获取管道长度
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Clone> Pipeline<T> {
    /// 去重
    pub fn unique(self) -> Pipeline<T>
    where
        T: Eq + std::hash::Hash,
    {
        use std::collections::HashSet;
        let set: HashSet<_> = self.data.into_iter().collect();
        Pipeline {
            data: set.into_iter().collect(),
        }
    }
}

/// 创建计数器闭包
///
/// # 示例
///
/// ```
/// use closures_functional::pipeline::make_counter;
/// let mut counter = make_counter();
/// assert_eq!(counter(), 1);
/// assert_eq!(counter(), 2);
/// ```
pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// 创建累加器闭包
pub fn make_accumulator(initial: i32) -> impl FnMut(i32) -> i32 {
    let mut sum = initial;
    move |x| {
        sum += x;
        sum
    }
}

/// 应用函数 n 次
pub fn apply_n_times<F>(mut f: F, n: usize, initial: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    let mut result = initial;
    for _ in 0..n {
        result = f(result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_map() {
        let result = Pipeline::new(vec![1, 2, 3])
            .map(|x| x * 2)
            .collect();
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_pipeline_filter() {
        let result = Pipeline::new(vec![1, 2, 3, 4, 5])
            .filter(|&x| x % 2 == 0)
            .collect();
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_pipeline_chain() {
        let result = Pipeline::new(vec![1, 2, 3, 4, 5])
            .filter(|&x| x % 2 == 0)
            .map(|x| x * x)
            .collect();
        assert_eq!(result, vec![4, 16]);
    }

    #[test]
    fn test_pipeline_fold() {
        let sum = Pipeline::new(vec![1, 2, 3, 4, 5])
            .fold(0, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_make_counter() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_make_accumulator() {
        let mut acc = make_accumulator(0);
        assert_eq!(acc(5), 5);
        assert_eq!(acc(10), 15);
        assert_eq!(acc(3), 18);
    }

    #[test]
    fn test_apply_n_times() {
        let double = |x| x * 2;
        let result = apply_n_times(double, 3, 1);
        assert_eq!(result, 8); // 1 * 2 * 2 * 2
    }

    #[test]
    fn test_pipeline_first_last() {
        let data = vec![1, 2, 3, 4, 5];
        let pipeline1 = Pipeline::new(data.clone());
        let pipeline2 = Pipeline::new(data);

        assert_eq!(pipeline1.first(), Some(1));
        assert_eq!(pipeline2.last(), Some(5));
    }
}
