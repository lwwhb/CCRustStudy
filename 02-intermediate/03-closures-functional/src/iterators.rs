/// 自定义迭代器
///
/// 演示迭代器模式和惰性求值

/// 斐波那契数列迭代器
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

/// 范围步进迭代器
pub struct StepBy {
    current: i32,
    end: i32,
    step: i32,
}

impl StepBy {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        StepBy {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for StepBy {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

/// 窗口迭代器
pub struct Windows<'a, T> {
    slice: &'a [T],
    window_size: usize,
    index: usize,
}

impl<'a, T> Windows<'a, T> {
    pub fn new(slice: &'a [T], window_size: usize) -> Self {
        Windows {
            slice,
            window_size,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + self.window_size <= self.slice.len() {
            let window = &self.slice[self.index..self.index + self.window_size];
            self.index += 1;
            Some(window)
        } else {
            None
        }
    }
}

/// 惰性映射迭代器
pub struct LazyMap<I, F> {
    iter: I,
    func: F,
}

impl<I, F, B> Iterator for LazyMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.func)
    }
}

/// 创建惰性映射
pub fn lazy_map<I, F, B>(iter: I, func: F) -> LazyMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    LazyMap { iter, func }
}

/// 无限循环迭代器
pub struct Cycle<I: Clone> {
    original: I,
    current: I,
}

impl<I: Clone + Iterator> Cycle<I> {
    pub fn new(iter: I) -> Self {
        Cycle {
            original: iter.clone(),
            current: iter,
        }
    }
}

impl<I: Clone + Iterator> Iterator for Cycle<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            Some(item) => Some(item),
            None => {
                self.current = self.original.clone();
                self.current.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_step_by() {
        let result: Vec<i32> = StepBy::new(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_windows() {
        let data = vec![1, 2, 3, 4, 5];
        let windows: Vec<&[i32]> = Windows::new(&data, 3).collect();
        assert_eq!(windows, vec![&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..]]);
    }

    #[test]
    fn test_lazy_map() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = lazy_map(numbers.into_iter(), |x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_cycle() {
        let data = vec![1, 2, 3];
        let cycled: Vec<i32> = Cycle::new(data.into_iter()).take(8).collect();
        assert_eq!(cycled, vec![1, 2, 3, 1, 2, 3, 1, 2]);
    }

    #[test]
    fn test_iterator_chain() {
        let result: i32 = Fibonacci::new()
            .take(10)
            .filter(|&x| x % 2 == 0)
            .map(|x| x as i32)
            .sum();
        assert_eq!(result, 44); // 0 + 2 + 8 + 34
    }

    #[test]
    fn test_windows_small() {
        let data = vec![1, 2];
        let windows: Vec<&[i32]> = Windows::new(&data, 2).collect();
        assert_eq!(windows, vec![&[1, 2][..]]);
    }
}
