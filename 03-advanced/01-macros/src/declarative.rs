/// 声明宏示例
///
/// 演示 macro_rules! 的各种用法

/// 简单的打印宏
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

/// 创建函数的宏
#[macro_export]
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

/// 计算表达式的宏
#[macro_export]
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val = $e;
            println!("{} = {}", stringify!($e), val);
            val
        }
    };
}

/// 创建 Vec 的宏（类似标准库的 vec!）
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// 创建 HashMap 的宏
#[macro_export]
macro_rules! hashmap {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

/// 日志宏
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        println!("[{}] {}", $level, format!($($arg)*));
    };
}

/// 调试宏
#[macro_export]
macro_rules! debug_print {
    ($val:expr) => {
        println!("{} = {:?}", stringify!($val), $val);
    };
}

/// 断言宏（带自定义消息）
#[macro_export]
macro_rules! assert_eq_msg {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            panic!("{}: expected {:?}, got {:?}", $msg, $right, $left);
        }
    };
}

/// 重复执行宏
#[macro_export]
macro_rules! repeat {
    ($n:expr, $body:block) => {
        for _ in 0..$n {
            $body
        }
    };
}

/// 测量执行时间的宏
#[macro_export]
macro_rules! time_it {
    ($name:expr, $body:block) => {
        {
            let start = std::time::Instant::now();
            let result = $body;
            let elapsed = start.elapsed();
            println!("{} took {:?}", $name, elapsed);
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_say_hello() {
        say_hello!();
        say_hello!("Rust");
    }

    #[test]
    fn test_create_function() {
        create_function!(foo);
        foo();
    }

    #[test]
    fn test_calculate() {
        let result = calculate!(eval 2 + 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_my_vec() {
        let v = my_vec![1, 2, 3, 4, 5];
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let empty: Vec<i32> = my_vec![];
        assert!(empty.is_empty());
    }

    #[test]
    fn test_hashmap() {
        let map = hashmap! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
        };

        assert_eq!(map.get("one"), Some(&1));
        assert_eq!(map.get("two"), Some(&2));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_log() {
        log!("INFO", "This is a log message");
        log!("ERROR", "Error code: {}", 404);
    }

    #[test]
    fn test_debug_print() {
        let x = 42;
        debug_print!(x);
        debug_print!(2 + 3);
    }

    #[test]
    fn test_assert_eq_msg() {
        assert_eq_msg!(2 + 2, 4, "Math is broken");
    }

    #[test]
    #[should_panic(expected = "Math is broken")]
    fn test_assert_eq_msg_fail() {
        assert_eq_msg!(2 + 2, 5, "Math is broken");
    }

    #[test]
    fn test_repeat() {
        let mut count = 0;
        repeat!(5, {
            count += 1;
        });
        assert_eq!(count, 5);
    }

    #[test]
    fn test_time_it() {
        let result = time_it!("computation", {
            let mut sum = 0;
            for i in 0..1000 {
                sum += i;
            }
            sum
        });
        assert_eq!(result, 499500);
    }
}
