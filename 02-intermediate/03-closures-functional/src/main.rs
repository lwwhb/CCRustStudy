mod combinators;
mod iterators;
mod pipeline;

use combinators::{compose, partial, pipe, repeat};
use iterators::{Fibonacci, StepBy, Windows};
use pipeline::{make_accumulator, make_counter, Pipeline};

fn main() {
    println!("=== 闭包与函数式编程演示 ===\n");

    // 演示 1：基本闭包
    println!("=== 演示 1：基本闭包 ===");
    let add = |x, y| x + y;
    let multiply = |x, y| x * y;
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(4, 5) = {}\n", multiply(4, 5));

    // 演示 2：闭包捕获环境
    println!("=== 演示 2：闭包捕获环境 ===");
    let x = 10;
    let add_x = |y| x + y;
    println!("x = {}", x);
    println!("add_x(5) = {}\n", add_x(5));

    // 演示 3：可变闭包
    println!("=== 演示 3：可变闭包（计数器）===");
    let mut counter = make_counter();
    println!("counter() = {}", counter());
    println!("counter() = {}", counter());
    println!("counter() = {}\n", counter());

    // 演示 4：累加器
    println!("=== 演示 4：累加器 ===");
    let mut acc = make_accumulator(0);
    println!("acc(5) = {}", acc(5));
    println!("acc(10) = {}", acc(10));
    println!("acc(3) = {}\n", acc(3));

    // 演示 5：数据管道
    println!("=== 演示 5：数据管道 ===");
    let result = Pipeline::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("偶数的平方: {:?}\n", result);

    // 演示 6：函数组合
    println!("=== 演示 6：函数组合 ===");
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let f = compose(double, add_one);
    println!("compose(double, add_one)(5) = {}", f(5));

    let g = pipe(add_one, double);
    println!("pipe(add_one, double)(5) = {}\n", g(5));

    // 演示 7：部分应用
    println!("=== 演示 7：部分应用（柯里化）===");
    let add = |x, y| x + y;
    let add_ten = partial(add, 10);
    println!("add_ten(5) = {}", add_ten(5));
    println!("add_ten(20) = {}\n", add_ten(20));

    // 演示 8：重复执行
    println!("=== 演示 8：重复执行函数 ===");
    let double = |x| x * 2;
    let f = repeat(double, 3);
    println!("repeat(double, 3)(1) = {}\n", f(1)); // 1 * 2 * 2 * 2

    // 演示 9：斐波那契迭代器
    println!("=== 演示 9：斐波那契迭代器 ===");
    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("前 10 个斐波那契数: {:?}\n", fib);

    // 演示 10：步进迭代器
    println!("=== 演示 10：步进迭代器 ===");
    let steps: Vec<i32> = StepBy::new(0, 20, 3).collect();
    println!("0 到 20，步长 3: {:?}\n", steps);

    // 演示 11：窗口迭代器
    println!("=== 演示 11：窗口迭代器 ===");
    let data = vec![1, 2, 3, 4, 5];
    let windows: Vec<Vec<i32>> = Windows::new(&data, 3)
        .map(|w| w.to_vec())
        .collect();
    println!("大小为 3 的窗口: {:?}\n", windows);

    // 演示 12：迭代器链式操作
    println!("=== 演示 12：迭代器链式操作 ===");
    let result: i32 = (1..=10)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("1-10 中偶数的平方和: {}\n", result);

    // 演示 13：高阶函数
    println!("=== 演示 13：高阶函数 ===");
    demonstrate_higher_order_functions();

    // 演示 14：Fn trait 家族
    println!("\n=== 演示 14：Fn trait 家族 ===");
    demonstrate_fn_traits();
}

/// 演示高阶函数
fn demonstrate_higher_order_functions() {
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    let square = |x| x * x;

    println!("apply(double, 5) = {}", apply(double, 5));
    println!("apply(square, 5) = {}", apply(square, 5));
}

/// 演示 Fn trait 家族
fn demonstrate_fn_traits() {
    // Fn - 不可变借用
    fn call_fn<F>(f: F, x: i32)
    where
        F: Fn(i32) -> i32,
    {
        println!("Fn: {}", f(x));
    }

    let add_one = |x| x + 1;
    call_fn(add_one, 5);

    // FnMut - 可变借用
    fn call_fn_mut<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
    }

    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("FnMut: count = {}", count);
    };
    call_fn_mut(&mut increment);

    // FnOnce - 获取所有权
    fn call_fn_once<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let message = String::from("Hello");
    let consume = move || {
        println!("FnOnce: {}", message);
    };
    call_fn_once(consume);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_basic() {
        let add = |x, y| x + y;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_closure_capture() {
        let x = 10;
        let add_x = |y| x + y;
        assert_eq!(add_x(5), 15);
    }

    #[test]
    fn test_pipeline() {
        let result = Pipeline::new(vec![1, 2, 3, 4, 5])
            .filter(|&x| x % 2 == 0)
            .map(|x| x * x)
            .collect();
        assert_eq!(result, vec![4, 16]);
    }

    #[test]
    fn test_compose() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let f = compose(double, add_one);
        assert_eq!(f(5), 12);
    }

    #[test]
    fn test_fibonacci() {
        let fib: Vec<u64> = Fibonacci::new().take(5).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3]);
    }
}

