use std::collections::HashMap;

/// 文本分析器
///
/// 提供各种文本分析功能，演示集合和迭代器的使用
pub struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    /// 创建新的文本分析器
    pub fn new(text: String) -> Self {
        TextAnalyzer { text }
    }

    /// 获取所有单词（转换为小写，去除标点）
    pub fn words(&self) -> Vec<String> {
        self.text
            .to_lowercase()
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect()
            })
            .filter(|word: &String| !word.is_empty())
            .collect()
    }

    /// 词频统计
    ///
    /// 返回每个单词出现的次数
    pub fn word_frequency(&self) -> HashMap<String, usize> {
        let mut freq = HashMap::new();

        for word in self.words() {
            *freq.entry(word).or_insert(0) += 1;
        }

        freq
    }

    /// 获取最常见的 n 个单词
    pub fn top_words(&self, n: usize) -> Vec<(String, usize)> {
        let freq = self.word_frequency();
        let mut word_counts: Vec<_> = freq.into_iter().collect();

        // 按频率降序排序
        word_counts.sort_by(|a, b| b.1.cmp(&a.1));

        word_counts.into_iter().take(n).collect()
    }

    /// 统计字符类型
    pub fn char_stats(&self) -> CharStats {
        let total = self.text.len();
        let letters = self.text.chars().filter(|c| c.is_alphabetic()).count();
        let digits = self.text.chars().filter(|c| c.is_numeric()).count();
        let spaces = self.text.chars().filter(|c| c.is_whitespace()).count();
        let punctuation = self.text.chars().filter(|c| c.is_ascii_punctuation()).count();

        CharStats {
            total,
            letters,
            digits,
            spaces,
            punctuation,
        }
    }

    /// 统计行信息
    pub fn line_stats(&self) -> LineStats {
        let lines: Vec<&str> = self.text.lines().collect();
        let total_lines = lines.len();
        let non_empty_lines = lines.iter().filter(|line| !line.trim().is_empty()).count();

        let total_length: usize = lines.iter().map(|line| line.len()).sum();
        let avg_length = if total_lines > 0 {
            total_length as f64 / total_lines as f64
        } else {
            0.0
        };

        LineStats {
            total_lines,
            non_empty_lines,
            avg_length,
        }
    }

    /// 搜索包含指定单词的行
    pub fn search_word(&self, word: &str) -> Vec<(usize, String)> {
        let search_word = word.to_lowercase();

        self.text
            .lines()
            .enumerate()
            .filter(|(_, line)| {
                line.to_lowercase().contains(&search_word)
            })
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect()
    }

    /// 过滤长单词
    pub fn long_words(&self, min_length: usize) -> Vec<String> {
        self.words()
            .into_iter()
            .filter(|word| word.len() >= min_length)
            .collect()
    }

    /// 获取唯一单词数量
    pub fn unique_word_count(&self) -> usize {
        self.word_frequency().len()
    }

    /// 获取总单词数
    pub fn total_word_count(&self) -> usize {
        self.words().len()
    }
}

/// 字符统计信息
#[derive(Debug, PartialEq)]
pub struct CharStats {
    pub total: usize,
    pub letters: usize,
    pub digits: usize,
    pub spaces: usize,
    pub punctuation: usize,
}

/// 行统计信息
#[derive(Debug)]
pub struct LineStats {
    pub total_lines: usize,
    pub non_empty_lines: usize,
    pub avg_length: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words() {
        let analyzer = TextAnalyzer::new("Hello, World! Hello Rust.".to_string());
        let words = analyzer.words();
        assert_eq!(words, vec!["hello", "world", "hello", "rust"]);
    }

    #[test]
    fn test_word_frequency() {
        let analyzer = TextAnalyzer::new("the quick brown fox jumps over the lazy dog".to_string());
        let freq = analyzer.word_frequency();
        assert_eq!(freq.get("the"), Some(&2));
        assert_eq!(freq.get("quick"), Some(&1));
    }

    #[test]
    fn test_top_words() {
        let analyzer = TextAnalyzer::new("the the the quick quick brown".to_string());
        let top = analyzer.top_words(2);
        assert_eq!(top[0].0, "the");
        assert_eq!(top[0].1, 3);
        assert_eq!(top[1].0, "quick");
        assert_eq!(top[1].1, 2);
    }

    #[test]
    fn test_char_stats() {
        let analyzer = TextAnalyzer::new("Hello, World!".to_string());
        let stats = analyzer.char_stats();
        assert_eq!(stats.letters, 10);
        assert_eq!(stats.spaces, 1);
        assert_eq!(stats.punctuation, 2);
    }

    #[test]
    fn test_line_stats() {
        let analyzer = TextAnalyzer::new("Line 1\nLine 2\nLine 3".to_string());
        let stats = analyzer.line_stats();
        assert_eq!(stats.total_lines, 3);
        assert_eq!(stats.non_empty_lines, 3);
    }

    #[test]
    fn test_search_word() {
        let analyzer = TextAnalyzer::new("Hello World\nHello Rust\nGoodbye World".to_string());
        let results = analyzer.search_word("hello");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1);
        assert_eq!(results[1].0, 2);
    }

    #[test]
    fn test_long_words() {
        let analyzer = TextAnalyzer::new("the quick brown fox".to_string());
        let long = analyzer.long_words(5);
        assert_eq!(long, vec!["quick", "brown"]);
    }

    #[test]
    fn test_unique_word_count() {
        let analyzer = TextAnalyzer::new("the the quick brown".to_string());
        assert_eq!(analyzer.unique_word_count(), 3);
    }

    #[test]
    fn test_total_word_count() {
        let analyzer = TextAnalyzer::new("the the quick brown".to_string());
        assert_eq!(analyzer.total_word_count(), 4);
    }
}
