# 模块 1.3：集合与迭代器

## 🎯 学习目标

- 掌握 Rust 标准库中的集合类型（Vec、HashMap、HashSet）
- 理解迭代器模式和迭代器适配器
- 学习函数式编程风格和链式操作
- 掌握零成本抽象的概念
- 实践迭代器组合子的使用

## 📚 核心概念

### 1. 动态数组（Vec）

Vec 是 Rust 中最常用的集合类型，可以存储可变数量的同类型值。

```rust
// 创建 Vec
let mut v1: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3, 4, 5];

// 添加元素
v1.push(1);
v1.push(2);

// 访问元素
let third = &v2[2];           // 直接索引（可能 panic）
let third = v2.get(2);        // 返回 Option<&T>

// 遍历
for i in &v2 {
    println!("{}", i);
}

// 可变遍历
for i in &mut v1 {
    *i += 50;
}
```

### 2. 哈希映射（HashMap）

HashMap 存储键值对，提供快速的查找性能。

```rust
use std::collections::HashMap;

// 创建 HashMap
let mut scores = HashMap::new();

// 插入键值对
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 访问值
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // 返回 Option<&V>

// 遍历
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新值
scores.entry(String::from("Blue")).or_insert(50);
```

### 3. 哈希集合（HashSet）

HashSet 存储唯一值的集合。

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(2);  // 重复值会被忽略

// 检查是否包含
if set.contains(&1) {
    println!("包含 1");
}

// 集合操作
let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
let b: HashSet<_> = [2, 3, 4].iter().cloned().collect();

let union: HashSet<_> = a.union(&b).cloned().collect();
let intersection: HashSet<_> = a.intersection(&b).cloned().collect();
```

### 4. 迭代器（Iterators）

迭代器是 Rust 中处理序列的强大工具。

```rust
// 创建迭代器
let v = vec![1, 2, 3];
let iter = v.iter();

// 消费适配器
let sum: i32 = v.iter().sum();
let collected: Vec<_> = v.iter().collect();

// 迭代器适配器
let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
let v3: Vec<_> = v.iter().filter(|x| *x > 1).collect();

// 链式操作
let result: i32 = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .sum();
```

### 5. 常用迭代器方法

```rust
let v = vec![1, 2, 3, 4, 5];

// map - 转换每个元素
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();

// filter - 过滤元素
let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();

// fold - 累积操作
let sum = v.iter().fold(0, |acc, x| acc + x);

// take - 取前 n 个元素
let first_three: Vec<_> = v.iter().take(3).collect();

// skip - 跳过前 n 个元素
let after_two: Vec<_> = v.iter().skip(2).collect();

// enumerate - 获取索引和值
for (i, val) in v.iter().enumerate() {
    println!("索引 {}: 值 {}", i, val);
}

// zip - 组合两个迭代器
let names = vec!["Alice", "Bob", "Carol"];
let ages = vec![25, 30, 35];
let combined: Vec<_> = names.iter().zip(ages.iter()).collect();

// find - 查找第一个满足条件的元素
let found = v.iter().find(|&&x| x > 3);

// any / all - 检查条件
let has_even = v.iter().any(|x| x % 2 == 0);
let all_positive = v.iter().all(|x| *x > 0);
```

### 6. 自定义迭代器

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 使用
let sum: u32 = Counter::new().sum();
```

## 💻 实战项目：文本分析工具

构建一个文本分析工具，使用集合和迭代器进行词频统计、文本处理等操作。

### 功能需求

1. 词频统计（最常见的单词）
2. 字符统计（字母、数字、标点符号）
3. 行统计（总行数、非空行数、平均行长）
4. 单词搜索和过滤
5. 文本转换（大小写、反转等）

### 项目结构

```
collections-iterators/
├── Cargo.toml
├── src/
│   ├── main.rs          # 主程序入口
│   ├── analyzer.rs      # 文本分析器
│   ├── stats.rs         # 统计信息
│   └── filters.rs       # 文本过滤器
└── README.md
```

### 运行项目

```bash
cargo run
```

### 使用示例

```
=== 文本分析工具 ===

分析文本:
"The quick brown fox jumps over the lazy dog.
The dog was really lazy."

=== 词频统计 ===
"the": 3 次
"lazy": 2 次
"dog": 2 次
"quick": 1 次
"brown": 1 次

=== 字符统计 ===
总字符数: 58
字母: 48
数字: 0
空格: 9
标点: 2

=== 行统计 ===
总行数: 2
非空行数: 2
平均行长: 29.0
```

## 🧪 练习题

### 练习 1：实现自定义过滤器

```rust
// 实现一个函数，过滤出长度大于 n 的单词
fn filter_long_words(words: Vec<&str>, min_length: usize) -> Vec<&str> {
    // 你的代码
}

#[test]
fn test_filter_long_words() {
    let words = vec!["hello", "hi", "world", "rust"];
    let result = filter_long_words(words, 4);
    assert_eq!(result, vec!["hello", "world", "rust"]);
}
```

### 练习 2：使用迭代器计算

```rust
// 使用迭代器计算 1 到 100 中所有偶数的平方和
fn sum_of_even_squares() -> i32 {
    // 你的代码
}

#[test]
fn test_sum_of_even_squares() {
    assert_eq!(sum_of_even_squares(), 171700);
}
```

### 练习 3：实现分组功能

```rust
// 将数字列表按奇偶分组
fn group_by_parity(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // 返回 (偶数列表, 奇数列表)
    // 你的代码
}

#[test]
fn test_group_by_parity() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds) = group_by_parity(numbers);
    assert_eq!(evens, vec![2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5]);
}
```

## 📖 深入阅读

- [The Rust Book - Chapter 8: Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [The Rust Book - Chapter 13: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust by Example - Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
- [Iterator Trait Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 使用 Vec、HashMap、HashSet 存储和操作数据
- [ ] 理解集合的所有权和借用规则
- [ ] 创建和使用迭代器
- [ ] 使用迭代器适配器进行链式操作
- [ ] 理解惰性求值的概念
- [ ] 实现自定义迭代器
- [ ] 使用函数式编程风格处理数据
- [ ] 理解零成本抽象

## 🚀 下一步

完成本模块后，继续学习 [模块 1.4：错误处理](../04-error-handling/)。
