use std::collections::HashSet;

/// 文本过滤器
///
/// 提供各种文本过滤和转换功能
pub struct TextFilter;

impl TextFilter {
    /// 过滤停用词
    pub fn remove_stop_words(words: Vec<String>) -> Vec<String> {
        let stop_words: HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "of", "with", "by", "from", "as", "is", "was", "are", "were",
        ]
        .iter()
        .cloned()
        .collect();

        words
            .into_iter()
            .filter(|word| !stop_words.contains(word.as_str()))
            .collect()
    }

    /// 过滤短单词
    pub fn filter_short_words(words: Vec<String>, min_length: usize) -> Vec<String> {
        words
            .into_iter()
            .filter(|word| word.len() >= min_length)
            .collect()
    }

    /// 转换为大写
    pub fn to_uppercase(words: Vec<String>) -> Vec<String> {
        words.into_iter().map(|word| word.to_uppercase()).collect()
    }

    /// 转换为小写
    pub fn to_lowercase(words: Vec<String>) -> Vec<String> {
        words.into_iter().map(|word| word.to_lowercase()).collect()
    }

    /// 反转单词
    pub fn reverse_words(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .map(|word| word.chars().rev().collect())
            .collect()
    }

    /// 去重
    pub fn deduplicate(words: Vec<String>) -> Vec<String> {
        let mut seen = HashSet::new();
        words
            .into_iter()
            .filter(|word| seen.insert(word.clone()))
            .collect()
    }

    /// 按长度排序
    pub fn sort_by_length(mut words: Vec<String>) -> Vec<String> {
        words.sort_by_key(|word| word.len());
        words
    }

    /// 按字母顺序排序
    pub fn sort_alphabetically(mut words: Vec<String>) -> Vec<String> {
        words.sort();
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_stop_words() {
        let words = vec![
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
        ];
        let filtered = TextFilter::remove_stop_words(words);
        assert_eq!(filtered, vec!["quick", "brown", "fox"]);
    }

    #[test]
    fn test_filter_short_words() {
        let words = vec![
            "hi".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        let filtered = TextFilter::filter_short_words(words, 4);
        assert_eq!(filtered, vec!["hello", "world"]);
    }

    #[test]
    fn test_to_uppercase() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let upper = TextFilter::to_uppercase(words);
        assert_eq!(upper, vec!["HELLO", "WORLD"]);
    }

    #[test]
    fn test_reverse_words() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let reversed = TextFilter::reverse_words(words);
        assert_eq!(reversed, vec!["olleh", "dlrow"]);
    }

    #[test]
    fn test_deduplicate() {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
        ];
        let unique = TextFilter::deduplicate(words);
        assert_eq!(unique, vec!["hello", "world"]);
    }

    #[test]
    fn test_sort_by_length() {
        let words = vec![
            "world".to_string(),
            "hi".to_string(),
            "hello".to_string(),
        ];
        let sorted = TextFilter::sort_by_length(words);
        assert_eq!(sorted, vec!["hi", "world", "hello"]);
    }

    #[test]
    fn test_sort_alphabetically() {
        let words = vec![
            "zebra".to_string(),
            "apple".to_string(),
            "banana".to_string(),
        ];
        let sorted = TextFilter::sort_alphabetically(words);
        assert_eq!(sorted, vec!["apple", "banana", "zebra"]);
    }
}
