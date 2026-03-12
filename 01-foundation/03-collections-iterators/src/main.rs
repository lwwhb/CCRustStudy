mod analyzer;
mod filters;
mod stats;

use analyzer::TextAnalyzer;
use filters::TextFilter;
use stats::{NumberSequence, Statistics};

fn main() {
    println!("=== 文本分析工具 ===\n");

    let text = "The quick brown fox jumps over the lazy dog.\n\
                The dog was really lazy. The fox was very quick.\n\
                Rust is a systems programming language focused on safety.";

    println!("分析文本:\n\"{}\"\n", text);

    let analyzer = TextAnalyzer::new(text.to_string());

    // 词频统计
    println!("=== 词频统计（前 5 名）===");
    for (word, count) in analyzer.top_words(5) {
        println!("  \"{}\": {} 次", word, count);
    }

    // 字符统计
    println!("\n=== 字符统计 ===");
    let char_stats = analyzer.char_stats();
    println!("  总字符数: {}", char_stats.total);
    println!("  字母: {}", char_stats.letters);
    println!("  数字: {}", char_stats.digits);
    println!("  空格: {}", char_stats.spaces);
    println!("  标点: {}", char_stats.punctuation);

    // 行统计
    println!("\n=== 行统计 ===");
    let line_stats = analyzer.line_stats();
    println!("  总行数: {}", line_stats.total_lines);
    println!("  非空行数: {}", line_stats.non_empty_lines);
    println!("  平均行长: {:.1}", line_stats.avg_length);

    // 单词搜索
    println!("\n=== 搜索 \"fox\" ===");
    for (line_num, line) in analyzer.search_word("fox") {
        println!("  第 {} 行: {}", line_num, line);
    }

    // 迭代器演示
    println!("\n=== 迭代器演示 ===");
    let words = analyzer.words();
    println!("  总单词数: {}", words.len());
    println!("  唯一单词数: {}", analyzer.unique_word_count());

    // 过滤器演示
    let filtered = TextFilter::remove_stop_words(words.clone());
    println!("  去除停用词后: {} 个单词", filtered.len());

    let long_words = TextFilter::filter_short_words(words.clone(), 5);
    println!("  长单词（≥5字符）: {:?}", long_words);

    // 数字统计演示
    println!("\n=== 数字统计演示 ===");
    let numbers = vec![4, 7, 2, 9, 1, 5, 8, 3, 6, 5, 5];
    let stats = Statistics::from_numbers(&numbers).unwrap();
    println!("  数据: {:?}", numbers);
    println!("  平均值: {:.2}", stats.mean);
    println!("  中位数: {:.1}", stats.median);
    println!("  众数: {:?}", stats.mode);
    println!("  最小值: {}, 最大值: {}", stats.min, stats.max);

    // 数列生成演示
    println!("\n=== 数列生成演示 ===");
    let fib = NumberSequence::fibonacci(8);
    println!("  斐波那契数列: {:?}", fib);

    let primes = NumberSequence::primes(6);
    println!("  前 6 个质数: {:?}", primes);

    let arith = NumberSequence::arithmetic(1, 2, 5);
    println!("  等差数列(1, +2, 5项): {:?}", arith);

    // 迭代器链式操作演示
    println!("\n=== 迭代器链式操作 ===");
    let numbers: Vec<i32> = (1..=10).collect();

    let result: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)   // 过滤偶数
        .map(|&x| x * x)             // 计算平方
        .sum();                       // 求和
    println!("  1-10 中偶数的平方和: {}", result);

    let words_demo = vec!["rust", "is", "awesome", "and", "fast"];
    let long_upper: Vec<String> = words_demo.iter()
        .filter(|w| w.len() > 3)     // 过滤长单词
        .map(|w| w.to_uppercase())   // 转大写
        .collect();
    println!("  长单词转大写: {:?}", long_upper);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_even_squares_sum() {
        let result: i32 = (1..=10)
            .filter(|x| x % 2 == 0)
            .map(|x| x * x)
            .sum();
        assert_eq!(result, 220);
    }

    #[test]
    fn test_group_by_parity() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
            .into_iter()
            .partition(|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_flatten() {
        let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let flat: Vec<i32> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_zip_and_unzip() {
        let names = vec!["Alice", "Bob", "Carol"];
        let scores = vec![95, 87, 92];
        let combined: Vec<_> = names.into_iter().zip(scores.into_iter()).collect();
        assert_eq!(combined.len(), 3);
        assert_eq!(combined[0], ("Alice", 95));
    }
}

