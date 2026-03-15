# 模块 1.4：错误处理 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 深入理解 Result 和 Option
2. 掌握 ? 操作符的使用
3. 学习自定义错误类型
4. 使用 thiserror 和 anyhow
5. 构建健壮的错误处理系统

## 🎯 为什么 Rust 的错误处理与众不同？

### 其他语言的错误处理

**Java/Python（异常）**：
```java
try {
    String content = readFile("data.txt");
    int number = parseInt(content);
    return number * 2;
} catch (IOException e) {
    // 处理 IO 错误
} catch (NumberFormatException e) {
    // 处理解析错误
}

问题：
- 异常可能被忽略
- 不知道函数会抛出什么异常
- 性能开销（栈展开）
- 控制流不清晰
```

**C/Go（错误码）**：
```c
int result;
int error = read_file("data.txt", &result);
if (error != 0) {
    // 处理错误
    return error;
}

问题：
- 容易忘记检查错误
- 错误码含义不明确
- 没有类型安全
```

**Rust（Result 类型）**：
```rust
fn process() -> Result<i32, MyError> {
    let content = read_file("data.txt")?;  // 必须处理错误
    let number = parse_int(&content)?;      // 编译器强制
    Ok(number * 2)
}

优势：
- 编译时强制错误处理
- 类型安全
- 零成本抽象
- 控制流清晰
```

### Rust 错误处理的哲学

```
1. 可恢复错误 → Result<T, E>
   - 文件不存在
   - 网络超时
   - 解析失败

2. 不可恢复错误 → panic!
   - 数组越界
   - 除以零
   - 断言失败

原则：让错误显式化，让程序员做决定
```

## 📖 核心概念详解

### 1. Option<T> - 可能不存在的值

Option 用于表示值可能存在或不存在。

#### 基础用法

```rust
enum Option<T> {
    Some(T),
    None,
}

// 示例：查找用户
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

// 使用 match 处理
match find_user(1) {
    Some(name) => println!("找到用户: {}", name),
    None => println!("用户不存在"),
}

// 使用 if let
if let Some(name) = find_user(1) {
    println!("找到用户: {}", name);
}
```

#### Option 的方法

```rust
let x: Option<i32> = Some(5);

// unwrap - 解包（如果是 None 会 panic）
let value = x.unwrap();  // 5

// unwrap_or - 提供默认值
let value = x.unwrap_or(0);  // 5
let value = None.unwrap_or(0);  // 0

// unwrap_or_else - 使用闭包计算默认值
let value = x.unwrap_or_else(|| {
    println!("计算默认值");
    0
});

// expect - 带自定义消息的 unwrap
let value = x.expect("x 应该有值");

// map - 转换 Some 中的值
let y = x.map(|v| v * 2);  // Some(10)

// and_then - 链式操作
let result = x.and_then(|v| {
    if v > 0 {
        Some(v * 2)
    } else {
        None
    }
});

// filter - 过滤
let result = x.filter(|&v| v > 3);  // Some(5)

// ok_or - 转换为 Result
let result: Result<i32, &str> = x.ok_or("没有值");
```

**何时使用 Option？**

```rust
// ✅ 好的使用场景
fn get_first_element(v: &Vec<i32>) -> Option<&i32> {
    v.first()  // 可能为空
}

fn parse_config(key: &str) -> Option<String> {
    // 配置可能不存在
}

// ❌ 不好的使用
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None  // 应该使用 Result，因为这是错误
    } else {
        Some(a / b)
    }
}
```

### 2. Result<T, E> - 可能失败的操作

Result 用于表示操作可能成功或失败。

#### 基础用法

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 示例：解析数字
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("无法解析 '{}'", s)),
    }
}

// 使用 match 处理
match parse_number("42") {
    Ok(n) => println!("数字: {}", n),
    Err(e) => println!("错误: {}", e),
}
```

#### Result 的方法

```rust
let x: Result<i32, &str> = Ok(5);

// unwrap / expect（同 Option）
let value = x.unwrap();
let value = x.expect("应该成功");

// unwrap_or / unwrap_or_else
let value = x.unwrap_or(0);

// map - 转换 Ok 中的值
let y = x.map(|v| v * 2);  // Ok(10)

// map_err - 转换 Err 中的值
let y = x.map_err(|e| format!("错误: {}", e));

// and_then - 链式操作
let result = x.and_then(|v| {
    if v > 0 {
        Ok(v * 2)
    } else {
        Err("值必须为正")
    }
});

// or_else - 错误恢复
let result = x.or_else(|_| Ok(0));
```

### 3. ? 操作符 - 错误传播

? 操作符是 Rust 错误处理的核心，用于简化错误传播。

#### 基础用法

```rust
// 不使用 ? 操作符
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用 ? 操作符
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更简洁
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 最简洁
fn read_username_from_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("username.txt")
}
```

#### ? 操作符的工作原理

```rust
// 这段代码：
let value = some_function()?;

// 等价于：
let value = match some_function() {
    Ok(v) => v,
    Err(e) => return Err(e.into()),  // 注意 into()
};
```

**关键点**：
1. 只能在返回 Result 或 Option 的函数中使用
2. 自动调用 `into()` 进行错误转换
3. 提前返回错误

#### ? 用于 Option

```rust
fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()  // 返回 Option<char>
}

fn process() -> Option<String> {
    let first = get_first_char("hello")?;  // 如果是 None，提前返回
    Some(first.to_uppercase().to_string())
}
```

### 4. 自定义错误类型

为应用定义专门的错误类型。

#### 手动实现

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(String),
    NotFound(String),
    ValidationError(String),
}

// 实现 Display trait
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO 错误: {}", e),
            AppError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            AppError::NotFound(item) => write!(f, "未找到: {}", item),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

// 实现 Error trait
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

// 实现 From trait 用于错误转换
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}
```

#### 使用自定义错误

```rust
type AppResult<T> = Result<T, AppError>;

fn read_config(path: &str) -> AppResult<String> {
    // std::io::Error 自动转换为 AppError
    let content = std::fs::read_to_string(path)?;
    
    if content.is_empty() {
        return Err(AppError::ValidationError(
            "配置文件为空".to_string()
        ));
    }
    
    Ok(content)
}
```

### 5. thiserror - 简化错误定义

thiserror 是一个过程宏，简化自定义错误类型的定义。

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误: {0}")]
    Parse(String),
    
    #[error("未找到: {0}")]
    NotFound(String),
    
    #[error("验证错误: {field}: {message}")]
    Validation {
        field: String,
        message: String,
    },
    
    #[error("数据库错误")]
    Database(#[from] sqlx::Error),
}

// 自动实现了：
// - Display trait
// - Error trait
// - From trait（对于标记了 #[from] 的变体）
```

**thiserror 的优势**：
- 自动实现 Display
- 自动实现 Error
- 自动实现 From（错误转换）
- 支持格式化字符串
- 减少样板代码

### 6. anyhow - 简化错误传播

anyhow 提供了一个通用的错误类型，适合应用程序（不适合库）。

```rust
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.toml")
        .context("无法读取配置文件")?;
    
    if content.is_empty() {
        anyhow::bail!("配置文件为空");
    }
    
    Ok(content)
}

fn process() -> Result<()> {
    let config = read_config()
        .context("加载配置失败")?;
    
    // ... 更多操作
    
    Ok(())
}
```

**anyhow 的特点**：
- 单一错误类型 `anyhow::Error`
- 自动错误转换
- 添加上下文信息
- 错误链追踪
- 适合应用程序，不适合库

**thiserror vs anyhow**：

```
thiserror:
- 用于库
- 定义具体的错误类型
- 类型安全
- 调用者可以匹配具体错误

anyhow:
- 用于应用程序
- 通用错误类型
- 简化错误处理
- 关注错误消息而非类型
```

## 💻 实战项目：CSV 文件处理工具

### 项目需求

构建一个 CSV 处理工具，演示完整的错误处理：
1. 解析 CSV 数据
2. 验证数据（年龄、邮箱、分数）
3. 过滤有效/无效记录
4. 详细的错误报告

### 步骤 1：定义错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("CSV 解析错误: {0}")]
    CsvParse(String),
    
    #[error("第 {line} 行: 字段数量错误，期望 {expected}，实际 {actual}")]
    FieldCount {
        line: usize,
        expected: usize,
        actual: usize,
    },
    
    #[error("第 {line} 行: 无效的 {field}: {message}")]
    Validation {
        line: usize,
        field: String,
        message: String,
    },
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
}

pub type AppResult<T> = Result<T, AppError>;
```

### 步骤 2：实现 CSV 解析器

```rust
pub struct Record {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub score: f64,
}

pub struct CsvParser;

impl CsvParser {
    pub fn parse(data: &str) -> AppResult<Vec<Record>> {
        let lines: Vec<&str> = data.lines().collect();
        
        if lines.is_empty() {
            return Err(AppError::CsvParse("空文件".to_string()));
        }
        
        // 跳过标题行
        let mut records = Vec::new();
        
        for (i, line) in lines.iter().skip(1).enumerate() {
            let line_num = i + 2;  // 从第 2 行开始
            let record = Self::parse_line(line, line_num)?;
            records.push(record);
        }
        
        Ok(records)
    }
    
    fn parse_line(line: &str, line_num: usize) -> AppResult<Record> {
        let fields: Vec<&str> = line.split(',').collect();
        
        if fields.len() != 4 {
            return Err(AppError::FieldCount {
                line: line_num,
                expected: 4,
                actual: fields.len(),
            });
        }
        
        let age = fields[1].parse::<u32>()
            .map_err(|_| AppError::Validation {
                line: line_num,
                field: "age".to_string(),
                message: format!("无效的数字: {}", fields[1]),
            })?;
        
        let score = fields[3].parse::<f64>()
            .map_err(|_| AppError::Validation {
                line: line_num,
                field: "score".to_string(),
                message: format!("无效的分数: {}", fields[3]),
            })?;
        
        Ok(Record {
            name: fields[0].to_string(),
            age,
            email: fields[2].to_string(),
            score,
        })
    }
}
```

### 步骤 3：实现数据验证器

```rust
pub struct Validator;

impl Validator {
    pub fn validate_all(records: &[Record]) -> AppResult<()> {
        for (i, record) in records.iter().enumerate() {
            let line_num = i + 2;
            Self::validate_record(record, line_num)?;
        }
        Ok(())
    }
    
    fn validate_record(record: &Record, line_num: usize) -> AppResult<()> {
        // 验证年龄
        if record.age < 18 {
            return Err(AppError::Validation {
                line: line_num,
                field: "age".to_string(),
                message: "年龄必须 >= 18".to_string(),
            });
        }
        
        // 验证邮箱
        if !record.email.contains('@') {
            return Err(AppError::Validation {
                line: line_num,
                field: "email".to_string(),
                message: "无效的邮箱格式".to_string(),
            });
        }
        
        // 验证分数
        if record.score < 0.0 || record.score > 100.0 {
            return Err(AppError::Validation {
                line: line_num,
                field: "score".to_string(),
                message: "分数必须在 0-100 之间".to_string(),
            });
        }
        
        Ok(())
    }
    
    // 过滤有效记录，返回 (有效记录, 错误列表)
    pub fn filter_valid(records: Vec<Record>) -> (Vec<Record>, Vec<(usize, String)>) {
        let mut valid = Vec::new();
        let mut errors = Vec::new();
        
        for (i, record) in records.into_iter().enumerate() {
            let line_num = i + 2;
            match Self::validate_record(&record, line_num) {
                Ok(_) => valid.push(record),
                Err(e) => errors.push((line_num, e.to_string())),
            }
        }
        
        (valid, errors)
    }
}
```

### 步骤 4：主程序

```rust
fn main() {
    let csv_data = "name,age,email,score
Alice,30,alice@example.com,95.5
Bob,25,bob@example.com,87.0
Carol,17,carol@example.com,92.3
David,35,david-invalid,88.5";
    
    // 解析并验证
    match process_csv(csv_data) {
        Ok(count) => println!("✓ 成功处理 {} 条记录", count),
        Err(e) => println!("✗ 错误: {}", e),
    }
    
    // 过滤有效记录
    filter_and_display(csv_data);
}

fn process_csv(data: &str) -> AppResult<usize> {
    let records = CsvParser::parse(data)?;
    Validator::validate_all(&records)?;
    Ok(records.len())
}

fn filter_and_display(data: &str) {
    match CsvParser::parse(data) {
        Ok(records) => {
            let (valid, errors) = Validator::filter_valid(records);
            
            println!("有效记录: {}", valid.len());
            for record in valid {
                println!("  ✓ {}", record.name);
            }
            
            if !errors.is_empty() {
                println!("无效记录:");
                for (line, error) in errors {
                    println!("  ✗ 第 {} 行: {}", line, error);
                }
            }
        }
        Err(e) => println!("解析失败: {}", e),
    }
}
```

## 🔍 深入理解

### 错误处理的最佳实践

**1. 选择合适的错误类型**

```rust
// ✅ 库代码：使用 thiserror
#[derive(Error, Debug)]
pub enum MyLibError {
    #[error("配置错误: {0}")]
    Config(String),
}

// ✅ 应用代码：使用 anyhow
use anyhow::Result;

fn main() -> Result<()> {
    // ...
    Ok(())
}
```

**2. 提供有用的错误信息**

```rust
// ❌ 不好
Err("错误")

// ✅ 好
Err(format!("无法打开文件 '{}': 权限被拒绝", path))

// ✅ 更好（使用 context）
std::fs::read_to_string(path)
    .with_context(|| format!("读取配置文件失败: {}", path))?
```

**3. 不要过度使用 unwrap**

```rust
// ❌ 危险
let value = some_option.unwrap();

// ✅ 安全
let value = some_option.expect("这里不应该是 None");

// ✅ 更好
let value = some_option.ok_or_else(|| {
    AppError::NotFound("值不存在".to_string())
})?;
```

**4. 错误恢复策略**

```rust
// 策略 1：提供默认值
let config = load_config().unwrap_or_default();

// 策略 2：重试
let mut attempts = 0;
let result = loop {
    match try_operation() {
        Ok(r) => break Ok(r),
        Err(e) if attempts < 3 => {
            attempts += 1;
            std::thread::sleep(Duration::from_secs(1));
        }
        Err(e) => break Err(e),
    }
};

// 策略 3：降级
let data = fetch_from_cache()
    .or_else(|_| fetch_from_network())
    .or_else(|_| use_default_data())?;
```

### 性能考虑

**Result 是零成本抽象**：

```rust
// 这段代码：
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 编译后的性能等同于：
fn divide(a: i32, b: i32) -> (bool, i32, String) {
    if b == 0 {
        (false, 0, "除数为零".to_string())
    } else {
        (true, a / b, String::new())
    }
}

// 没有异常的栈展开开销
// 没有额外的运行时成本
```

## 📝 练习题

### 练习 1：实现错误转换

```rust
#[derive(Error, Debug)]
enum MyError {
    #[error("IO 错误")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误")]
    Parse(#[from] std::num::ParseIntError),
}

// 使用 ? 操作符自动转换错误
fn read_and_parse(path: &str) -> Result<i32, MyError> {
    let content = std::fs::read_to_string(path)?;  // io::Error -> MyError
    let number = content.trim().parse::<i32>()?;   // ParseIntError -> MyError
    Ok(number)
}
```

### 练习 2：实现重试逻辑

```rust
use std::time::Duration;

fn retry<F, T, E>(mut f: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut attempts = 0;
    loop {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts - 1 => {
                attempts += 1;
                std::thread::sleep(Duration::from_millis(100 * 2_u64.pow(attempts)));
            }
            Err(e) => return Err(e),
        }
    }
}
```

### 练习 3：错误链追踪

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let path = find_config_path()
        .context("查找配置文件失败")?;
    
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("读取配置文件失败: {}", path))?;
    
    let config = parse_config(&content)
        .context("解析配置失败")?;
    
    Ok(config)
}

// 错误输出会显示完整的错误链：
// Error: 加载配置失败
// Caused by:
//     0: 读取配置文件失败: /etc/app/config.toml
//     1: Permission denied (os error 13)
```

## ✅ 检查清单

完成本模块后，你应该能够：

- [ ] 理解 Result 和 Option 的区别和使用场景
- [ ] 熟练使用 ? 操作符传播错误
- [ ] 定义自定义错误类型
- [ ] 使用 thiserror 简化错误定义
- [ ] 使用 anyhow 简化应用程序错误处理
- [ ] 实现错误转换（From trait）
- [ ] 添加错误上下文信息
- [ ] 实现错误恢复策略
- [ ] 编写健壮的错误处理代码

## 🚀 下一步

完成本模块后，继续学习 [模块 1.5：模块系统](../05-modules-cargo/)。

---

**记住**：好的错误处理不是让程序不崩溃，而是让错误清晰、可追踪、可恢复！
