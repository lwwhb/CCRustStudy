/// 函数组合器
///
/// 演示高阶函数和函数组合

/// 组合两个函数
///
/// # 示例
///
/// ```
/// use closures_functional::combinators::compose;
/// let add_one = |x| x + 1;
/// let double = |x| x * 2;
/// let f = compose(double, add_one);
/// assert_eq!(f(5), 12); // (5 + 1) * 2
/// ```
pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

/// 管道组合（从左到右）
pub fn pipe<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

/// 部分应用（柯里化）
pub fn partial<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

/// 创建常量函数
pub fn constant<T: Clone>(value: T) -> impl Fn() -> T {
    move || value.clone()
}

/// 创建恒等函数
pub fn identity<T>() -> impl Fn(T) -> T {
    |x| x
}

/// 条件执行
pub fn if_then_else<F, G, T>(
    condition: bool,
    then_fn: F,
    else_fn: G,
) -> impl Fn() -> T
where
    F: Fn() -> T,
    G: Fn() -> T,
{
    move || {
        if condition {
            then_fn()
        } else {
            else_fn()
        }
    }
}

/// 重复执行函数
pub fn repeat<F, T>(f: F, n: usize) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    T: Clone,
{
    move |mut x| {
        for _ in 0..n {
            x = f(x.clone());
        }
        x
    }
}

/// 函数缓存（记忆化）
pub struct Memoized<F, A, R>
where
    F: FnMut(A) -> R,
    A: std::hash::Hash + Eq + Clone,
    R: Clone,
{
    func: F,
    cache: std::collections::HashMap<A, R>,
}

impl<F, A, R> Memoized<F, A, R>
where
    F: FnMut(A) -> R,
    A: std::hash::Hash + Eq + Clone,
    R: Clone,
{
    pub fn new(func: F) -> Self {
        Memoized {
            func,
            cache: std::collections::HashMap::new(),
        }
    }

    pub fn call(&mut self, arg: A) -> R {
        if let Some(result) = self.cache.get(&arg) {
            result.clone()
        } else {
            let result = (self.func)(arg.clone());
            self.cache.insert(arg, result.clone());
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let f = compose(double, add_one);
        assert_eq!(f(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_pipe() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let f = pipe(add_one, double);
        assert_eq!(f(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_partial() {
        let add = |x, y| x + y;
        let add_five = partial(add, 5);
        assert_eq!(add_five(3), 8);
        assert_eq!(add_five(10), 15);
    }

    #[test]
    fn test_constant() {
        let get_42 = constant(42);
        assert_eq!(get_42(), 42);
        assert_eq!(get_42(), 42);
    }

    #[test]
    fn test_identity() {
        let id = identity();
        assert_eq!(id(42), 42);

        let id_str = identity();
        assert_eq!(id_str("hello"), "hello");
    }

    #[test]
    fn test_if_then_else() {
        let f = if_then_else(true, || 1, || 2);
        assert_eq!(f(), 1);

        let g = if_then_else(false, || 1, || 2);
        assert_eq!(g(), 2);
    }

    #[test]
    fn test_repeat() {
        let double = |x| x * 2;
        let f = repeat(double, 3);
        assert_eq!(f(1), 8); // 1 * 2 * 2 * 2
    }

    #[test]
    fn test_memoized() {
        let mut fib = Memoized::new(|n: u32| {
            if n <= 1 {
                n as u64
            } else {
                // 简化版，实际应该递归调用缓存版本
                n as u64
            }
        });

        assert_eq!(fib.call(0), 0);
        assert_eq!(fib.call(1), 1);
        assert_eq!(fib.call(5), 5);
    }

    #[test]
    fn test_compose_multiple() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let square = |x| x * x;

        let f = compose(square, compose(double, add_one));
        assert_eq!(f(2), 36); // ((2 + 1) * 2)^2 = 6^2 = 36
    }
}
