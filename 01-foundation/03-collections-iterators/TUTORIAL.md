# 模块 1.3：集合与迭代器 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 掌握 Rust 的三大集合类型
2. 理解迭代器模式和惰性求值
3. 学习函数式编程风格
4. 掌握零成本抽象
5. 构建文本分析工具

## 🎯 为什么需要集合和迭代器？

### 集合的重要性

**问题场景**：
```rust
// 没有集合，只能用固定大小的数组
let numbers = [1, 2, 3, 4, 5];  // 大小固定
// 无法添加新元素！

// 需要存储用户输入的数据？
// 需要动态增长的列表？
// 需要快速查找？
```

**使用集合**：
```rust
// Vec - 动态数组
let mut numbers = vec![1, 2, 3];
numbers.push(4);  // 可以动态增长

// HashMap - 快速查找
let mut scores = HashMap::new();
scores.insert("Alice", 100);
let score = scores.get("Alice");  // O(1) 查找

// HashSet - 唯一值集合
let mut unique = HashSet::new();
unique.insert(1);
unique.insert(1);  // 自动去重
```

### 迭代器的优势

**传统循环 vs 迭代器**：

```rust
// 传统方式（命令式）
let numbers = vec![1, 2, 3, 4, 5];
let mut sum = 0;
for i in 0..numbers.len() {
    if numbers[i] % 2 == 0 {
        sum += numbers[i] * numbers[i];
    }
}

// 迭代器方式（函数式）
let sum: i32 = numbers.iter()
    .filter(|&&x| x % 2 == 0)  // 过滤偶数
    .map(|&x| x * x)            // 计算平方
    .sum();                     // 求和

// 更清晰、更简洁、性能相同！
```

**性能对比**：
```
传统循环：
- 手动索引
- 边界检查
- 可能的错误

迭代器：
- 零成本抽象
- 编译器优化
- 类型安全
- 性能相同或更好
```

## 📖 核心概念详解

### 1. Vec（动态数组）

Vec 是最常用的集合类型，类似其他语言的 ArrayList 或 List。

#### 创建 Vec

```rust
// 方式 1：使用 new()
let mut v1: Vec<i32> = Vec::new();

// 方式 2：使用 vec! 宏
let v2 = vec![1, 2, 3, 4, 5];

// 方式 3：使用 with_capacity 预分配
let mut v3 = Vec::with_capacity(10);  // 预分配 10 个元素的空间

// 方式 4：从迭代器收集
let v4: Vec<i32> = (1..=5).collect();
```

**为什么要预分配容量？**

```
不预分配：
Vec [1]       → 容量 1
添加 2 → 重新分配 → Vec [1, 2]       → 容量 2
添加 3 → 重新分配 → Vec [1, 2, 3, _] → 容量 4
添加 4 → 不需要分配 → Vec [1, 2, 3, 4] → 容量 4

预分配：
Vec [_, _, _, _] → 容量 4
添加 1, 2, 3, 4 → 不需要重新分配
```

#### 操作 Vec

```rust
let mut v = vec![1, 2, 3];

// 添加元素
v.push(4);           // 末尾添加
v.insert(0, 0);      // 指定位置插入

// 删除元素
v.pop();             // 删除末尾，返回 Option<T>
v.remove(0);         // 删除指定位置

// 访问元素
let third = v[2];              // 直接索引（可能 panic）
let third = v.get(2);          // 返回 Option<&T>（安全）
let third = v.get(2).unwrap(); // 解包 Option

// 修改元素
v[0] = 10;

// 长度和容量
println!("长度: {}", v.len());
println!("容量: {}", v.capacity());

// 清空
v.clear();
```

**索引 vs get 的区别**：

```rust
let v = vec![1, 2, 3];

// 使用索引 - 可能 panic
let x = v[10];  // panic: index out of bounds

// 使用 get - 安全
match v.get(10) {
    Some(x) => println!("值: {}", x),
    None => println!("索引越界"),
}
```

#### 遍历 Vec

```rust
let v = vec![1, 2, 3, 4, 5];

// 不可变遍历
for i in &v {
    println!("{}", i);
}

// 可变遍历
let mut v = vec![1, 2, 3];
for i in &mut v {
    *i += 10;  // 每个元素加 10
}

// 获取所有权
for i in v {
    println!("{}", i);
}
// v 已失效
```

### 2. HashMap（哈希映射）

HashMap 存储键值对，提供 O(1) 的查找性能。

#### 创建 HashMap

```rust
use std::collections::HashMap;

// 方式 1：new()
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 方式 2：从元组向量创建
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter()
    .zip(initial_scores.iter())
    .collect();

// 方式 3：使用 from_iter
let scores: HashMap<_, _> = [
    ("Blue", 10),
    ("Yellow", 50),
].iter().cloned().collect();
```

#### 操作 HashMap

```rust
let mut scores = HashMap::new();

// 插入
scores.insert(String::from("Blue"), 10);

// 访问
let team = String::from("Blue");
let score = scores.get(&team);  // 返回 Option<&V>

match score {
    Some(s) => println!("分数: {}", s),
    None => println!("队伍不存在"),
}

// 遍历
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新值
scores.insert(String::from("Blue"), 25);  // 覆盖旧值

// 只在键不存在时插入
scores.entry(String::from("Blue")).or_insert(50);

// 基于旧值更新
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
// map: {"hello": 1, "world": 2, "wonderful": 1}
```

**entry API 的强大之处**：

```rust
// 传统方式
if !map.contains_key(&key) {
    map.insert(key, default_value);
}
let value = map.get_mut(&key).unwrap();
*value += 1;

// 使用 entry
let value = map.entry(key).or_insert(default_value);
*value += 1;

// 更简洁、更高效（只查找一次）
```

### 3. HashSet（哈希集合）

HashSet 存储唯一值的集合。

```rust
use std::collections::HashSet;

let mut set = HashSet::new();

// 插入
set.insert(1);
set.insert(2);
set.insert(2);  // 重复值被忽略

// 检查
if set.contains(&1) {
    println!("包含 1");
}

// 集合操作
let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
let b: HashSet<_> = [2, 3, 4].iter().cloned().collect();

// 并集
let union: HashSet<_> = a.union(&b).cloned().collect();
// {1, 2, 3, 4}

// 交集
let intersection: HashSet<_> = a.intersection(&b).cloned().collect();
// {2, 3}

// 差集
let difference: HashSet<_> = a.difference(&b).cloned().collect();
// {1}

// 对称差集
let sym_diff: HashSet<_> = a.symmetric_difference(&b).cloned().collect();
// {1, 4}
```

### 4. 迭代器（Iterators）

迭代器是 Rust 中处理序列的核心抽象。

#### 迭代器的三种形式

```rust
let v = vec![1, 2, 3];

// iter() - 不可变借用
for i in v.iter() {
    println!("{}", i);  // i 的类型是 &i32
}
// v 仍然有效

// iter_mut() - 可变借用
let mut v = vec![1, 2, 3];
for i in v.iter_mut() {
    *i += 10;  // i 的类型是 &mut i32
}

// into_iter() - 获取所有权
for i in v.into_iter() {
    println!("{}", i);  // i 的类型是 i32
}
// v 已失效
```

#### 惰性求值

迭代器是惰性的，只有在消费时才会执行。

```rust
let v = vec![1, 2, 3, 4, 5];

// 这行代码不会执行任何计算
let iter = v.iter().map(|x| {
    println!("处理 {}", x);
    x * 2
});

// 只有在消费时才会执行
let result: Vec<_> = iter.collect();  // 现在才打印 "处理 1", "处理 2", ...
```

**为什么要惰性求值？**

```rust
// 假设有一个大列表
let numbers: Vec<i32> = (1..=1_000_000).collect();

// 只需要前 10 个偶数
let result: Vec<_> = numbers.iter()
    .filter(|&&x| x % 2 == 0)  // 不会遍历所有元素
    .take(10)                   // 找到 10 个就停止
    .collect();

// 如果不是惰性的，会先过滤所有 100 万个数字，然后再取前 10 个
// 惰性求值只处理需要的元素
```

#### 迭代器适配器

```rust
let v = vec![1, 2, 3, 4, 5];

// map - 转换每个元素
let doubled: Vec<_> = v.iter()
    .map(|x| x * 2)
    .collect();
// [2, 4, 6, 8, 10]

// filter - 过滤元素
let evens: Vec<_> = v.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();
// [2, 4]

// filter_map - 过滤并转换
let parsed: Vec<i32> = vec!["1", "two", "3"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();
// [1, 3]

// take - 取前 n 个
let first_three: Vec<_> = v.iter()
    .take(3)
    .collect();
// [1, 2, 3]

// skip - 跳过前 n 个
let after_two: Vec<_> = v.iter()
    .skip(2)
    .collect();
// [3, 4, 5]

// enumerate - 获取索引
for (i, val) in v.iter().enumerate() {
    println!("索引 {}: 值 {}", i, val);
}

// zip - 组合两个迭代器
let names = vec!["Alice", "Bob", "Carol"];
let ages = vec![25, 30, 35];
let people: Vec<_> = names.iter()
    .zip(ages.iter())
    .collect();
// [("Alice", 25), ("Bob", 30), ("Carol", 35)]

// chain - 连接两个迭代器
let v1 = vec![1, 2, 3];
let v2 = vec![4, 5, 6];
let combined: Vec<_> = v1.iter()
    .chain(v2.iter())
    .collect();
// [1, 2, 3, 4, 5, 6]
```

#### 消费适配器

```rust
let v = vec![1, 2, 3, 4, 5];

// sum - 求和
let sum: i32 = v.iter().sum();
// 15

// product - 求积
let product: i32 = v.iter().product();
// 120

// collect - 收集到集合
let collected: Vec<_> = v.iter().collect();

// fold - 累积操作
let sum = v.iter().fold(0, |acc, x| acc + x);
// 等同于 sum()

// reduce - 类似 fold，但返回 Option
let sum = v.iter().reduce(|acc, x| acc + x);
// Some(15)

// find - 查找第一个满足条件的元素
let found = v.iter().find(|&&x| x > 3);
// Some(&4)

// any - 是否有元素满足条件
let has_even = v.iter().any(|x| x % 2 == 0);
// true

// all - 是否所有元素都满足条件
let all_positive = v.iter().all(|x| *x > 0);
// true

// count - 计数
let count = v.iter().filter(|&&x| x % 2 == 0).count();
// 2

// max / min - 最大/最小值
let max = v.iter().max();
// Some(&5)
```

### 5. 链式操作

迭代器的真正威力在于链式操作。

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 复杂的链式操作
let result: i32 = numbers.iter()
    .filter(|&&x| x % 2 == 0)      // 过滤偶数: [2, 4, 6, 8, 10]
    .map(|&x| x * x)                // 计算平方: [4, 16, 36, 64, 100]
    .filter(|&x| x > 20)            // 过滤大于 20: [36, 64, 100]
    .sum();                         // 求和: 200

// 文本处理示例
let text = "Hello World Rust Programming";
let long_words: Vec<String> = text
    .split_whitespace()             // 分割单词
    .filter(|w| w.len() > 5)        // 过滤长单词
    .map(|w| w.to_lowercase())      // 转小写
    .collect();
// ["programming"]
```

**性能说明**：
```
这些链式操作会被编译器优化成一个循环，
性能与手写循环相同，但代码更清晰！
```

### 6. 自定义迭代器

实现 Iterator trait 创建自定义迭代器。

```rust
// 斐波那契数列迭代器
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// 使用
let fib: Vec<_> = Fibonacci::new()
    .take(10)
    .collect();
// [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

## 💻 实战项目：文本分析工具

### 项目需求

构建一个文本分析工具，支持：
1. 词频统计
2. 字符统计
3. 行统计
4. 单词搜索
5. 文本过滤

### 步骤 1：定义数据结构

```rust
use std::collections::HashMap;

pub struct TextAnalyzer {
    text: String,
}

pub struct CharStats {
    pub total: usize,
    pub letters: usize,
    pub digits: usize,
    pub spaces: usize,
    pub punctuation: usize,
}

pub struct LineStats {
    pub total_lines: usize,
    pub non_empty_lines: usize,
    pub avg_length: f64,
}
```

### 步骤 2：实现词频统计

```rust
impl TextAnalyzer {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    // 获取所有单词
    pub fn words(&self) -> Vec<String> {
        self.text
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| !w.is_empty())
            .collect()
    }

    // 词频统计
    pub fn word_frequency(&self) -> HashMap<String, usize> {
        let mut freq = HashMap::new();
        
        for word in self.words() {
            *freq.entry(word).or_insert(0) += 1;
        }
        
        freq
    }

    // 获取最常见的 n 个单词
    pub fn top_words(&self, n: usize) -> Vec<(String, usize)> {
        let freq = self.word_frequency();
        let mut words: Vec<_> = freq.into_iter().collect();
        
        // 按频率降序排序
        words.sort_by(|a, b| b.1.cmp(&a.1));
        
        words.into_iter().take(n).collect()
    }
}
```

### 步骤 3：实现字符统计

```rust
impl TextAnalyzer {
    pub fn char_stats(&self) -> CharStats {
        let total = self.text.len();
        let letters = self.text.chars().filter(|c| c.is_alphabetic()).count();
        let digits = self.text.chars().filter(|c| c.is_numeric()).count();
        let spaces = self.text.chars().filter(|c| c.is_whitespace()).count();
        let punctuation = self.text.chars()
            .filter(|c| c.is_ascii_punctuation())
            .count();

        CharStats {
            total,
            letters,
            digits,
            spaces,
            punctuation,
        }
    }
}
```

### 步骤 4：实现行统计

```rust
impl TextAnalyzer {
    pub fn line_stats(&self) -> LineStats {
        let lines: Vec<&str> = self.text.lines().collect();
        let total_lines = lines.len();
        let non_empty_lines = lines.iter()
            .filter(|line| !line.trim().is_empty())
            .count();
        
        let total_length: usize = lines.iter()
            .map(|line| line.len())
            .sum();
        
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
}
```

### 步骤 5：实现搜索功能

```rust
impl TextAnalyzer {
    // 搜索包含指定单词的行
    pub fn search_word(&self, word: &str) -> Vec<(usize, String)> {
        self.text
            .lines()
            .enumerate()
            .filter(|(_, line)| {
                line.to_lowercase().contains(&word.to_lowercase())
            })
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect()
    }
}
```

### 步骤 6：实现过滤器

```rust
pub struct TextFilter;

impl TextFilter {
    // 移除停用词
    pub fn remove_stop_words(words: Vec<String>) -> Vec<String> {
        let stop_words: HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for"
        ].iter().cloned().collect();

        words.into_iter()
            .filter(|w| !stop_words.contains(w.as_str()))
            .collect()
    }

    // 过滤短单词
    pub fn filter_short_words(words: Vec<String>, min_length: usize) -> Vec<String> {
        words.into_iter()
            .filter(|w| w.len() >= min_length)
            .collect()
    }
}
```

### 步骤 7：主程序

```rust
fn main() {
    let text = "The quick brown fox jumps over the lazy dog.\n\
                The dog was really lazy. The fox was very quick.";

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

    // 搜索
    println!("\n=== 搜索 \"fox\" ===");
    for (line_num, line) in analyzer.search_word("fox") {
        println!("  第 {} 行: {}", line_num, line);
    }
}
```

## 🔍 深入理解

### 零成本抽象

Rust 的迭代器是零成本抽象的典范。

```rust
// 手写循环
let mut sum = 0;
for i in 0..v.len() {
    sum += v[i];
}

// 迭代器
let sum: i32 = v.iter().sum();

// 编译后的汇编代码几乎相同！
```

**编译器优化**：
- 内联函数调用
- 消除边界检查
- 向量化（SIMD）
- 循环展开

### 迭代器 vs 循环的性能

```rust
use std::time::Instant;

let numbers: Vec<i32> = (1..=1_000_000).collect();

// 测试循环
let start = Instant::now();
let mut sum = 0;
for i in &numbers {
    sum += i;
}
println!("循环: {:?}", start.elapsed());

// 测试迭代器
let start = Instant::now();
let sum: i32 = numbers.iter().sum();
println!("迭代器: {:?}", start.elapsed());

// 结果：性能几乎相同！
```

## 📝 练习题

### 练习 1：实现单词反转

```rust
// 反转每个单词的字符顺序
fn reverse_words(text: &str) -> String {
    text.split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn test_reverse_words() {
    assert_eq!(reverse_words("hello world"), "olleh dlrow");
}
```

### 练习 2：查找重复元素

```rust
// 找出列表中的重复元素
fn find_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for num in numbers {
        if !seen.insert(num) {
            duplicates.insert(num);
        }
    }

    duplicates.into_iter().collect()
}
```

### 练习 3：实现分组

```rust
// 按首字母分组单词
fn group_by_first_letter(words: Vec<String>) -> HashMap<char, Vec<String>> {
    let mut groups = HashMap::new();

    for word in words {
        if let Some(first) = word.chars().next() {
            groups.entry(first.to_lowercase().next().unwrap())
                .or_insert_with(Vec::new)
                .push(word);
        }
    }

    groups
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 使用 Vec、HashMap、HashSet 存储数据
- [ ] 理解集合的所有权规则
- [ ] 创建和使用迭代器
- [ ] 使用 map、filter、fold 等适配器
- [ ] 理解惰性求值
- [ ] 编写链式操作
- [ ] 实现自定义迭代器
- [ ] 理解零成本抽象

## 🔗 延伸阅读

- [The Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator Trait 文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

## 🚀 下一步

完成本模块后，继续学习 [模块 1.4：错误处理](../04-error-handling/TUTORIAL.md)。

---

**掌握集合和迭代器，开启函数式编程之旅！** 🦀
