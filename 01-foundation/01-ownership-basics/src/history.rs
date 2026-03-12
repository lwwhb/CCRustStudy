/// 历史记录管理
///
/// 这个模块演示了 Rust 的所有权和借用概念
pub struct History {
    records: Vec<String>,
}

impl History {
    /// 创建新的历史记录实例
    pub fn new() -> Self {
        History {
            records: Vec::new(),
        }
    }

    /// 添加一条记录
    ///
    /// # 参数
    /// * `record` - 要添加的记录字符串
    ///
    /// # 所有权说明
    /// 这个方法接受 String 的所有权，而不是借用
    /// 这意味着调用者将失去对该字符串的所有权
    pub fn add(&mut self, record: String) {
        self.records.push(record);
    }

    /// 获取历史记录的迭代器
    ///
    /// # 借用说明
    /// 这个方法返回一个不可变引用的迭代器
    /// 调用者可以读取数据，但不能修改
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.records.iter()
    }

    /// 检查历史记录是否为空
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// 获取历史记录的数量
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// 清除所有历史记录
    ///
    /// # 可变借用说明
    /// 这个方法需要可变引用，因为它会修改内部状态
    pub fn clear(&mut self) {
        self.records.clear();
    }

    /// 获取指定索引的记录
    ///
    /// # 参数
    /// * `index` - 记录的索引
    ///
    /// # 返回
    /// * `Some(&String)` - 如果索引有效，返回记录的引用
    /// * `None` - 如果索引无效
    ///
    /// # 借用说明
    /// 返回的是字符串的引用，而不是所有权
    /// 这样调用者可以读取数据，但原始数据仍然属于 History
    pub fn get(&self, index: usize) -> Option<&String> {
        self.records.get(index)
    }

    /// 获取最后一条记录
    pub fn last(&self) -> Option<&String> {
        self.records.last()
    }
}

// 实现 Default trait，提供默认构造方式
impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_history_is_empty() {
        let history = History::new();
        assert!(history.is_empty());
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_add_record() {
        let mut history = History::new();
        history.add("5 + 3 = 8".to_string());

        assert!(!history.is_empty());
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_multiple_records() {
        let mut history = History::new();
        history.add("5 + 3 = 8".to_string());
        history.add("10 * 2 = 20".to_string());
        history.add("15 - 7 = 8".to_string());

        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_get_record() {
        let mut history = History::new();
        history.add("5 + 3 = 8".to_string());
        history.add("10 * 2 = 20".to_string());

        assert_eq!(history.get(0), Some(&"5 + 3 = 8".to_string()));
        assert_eq!(history.get(1), Some(&"10 * 2 = 20".to_string()));
        assert_eq!(history.get(2), None);
    }

    #[test]
    fn test_last_record() {
        let mut history = History::new();
        assert_eq!(history.last(), None);

        history.add("5 + 3 = 8".to_string());
        assert_eq!(history.last(), Some(&"5 + 3 = 8".to_string()));

        history.add("10 * 2 = 20".to_string());
        assert_eq!(history.last(), Some(&"10 * 2 = 20".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut history = History::new();
        history.add("5 + 3 = 8".to_string());
        history.add("10 * 2 = 20".to_string());

        assert_eq!(history.len(), 2);

        history.clear();

        assert!(history.is_empty());
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mut history = History::new();
        history.add("5 + 3 = 8".to_string());
        history.add("10 * 2 = 20".to_string());

        let records: Vec<&String> = history.iter().collect();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], "5 + 3 = 8");
        assert_eq!(records[1], "10 * 2 = 20");
    }

    #[test]
    fn test_ownership_transfer() {
        let mut history = History::new();
        let record = String::from("5 + 3 = 8");

        // 所有权转移给 history
        history.add(record);

        // 下面这行会导致编译错误，因为 record 的所有权已经转移
        // println!("{}", record);

        // 但我们可以通过借用来访问
        assert_eq!(history.get(0), Some(&"5 + 3 = 8".to_string()));
    }
}
