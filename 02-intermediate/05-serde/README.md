# 模块 2.5：序列化与反序列化

## 🎯 学习目标

- 掌握 Serde 框架的使用
- 处理 JSON、TOML、YAML 格式
- 自定义序列化逻辑
- 使用 derive 宏
- 处理复杂数据结构

## 📚 核心概念

### 1. 基本序列化

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

let user = User {
    name: "Alice".to_string(),
    age: 30,
    email: "alice@example.com".to_string(),
};

// 序列化为 JSON
let json = serde_json::to_string(&user)?;

// 反序列化
let user: User = serde_json::from_str(&json)?;
```

### 2. 字段属性

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "userName")]
    user_name: String,

    #[serde(skip)]
    password: String,

    #[serde(default)]
    timeout: u32,
}
```

### 3. 枚举序列化

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Text { content: String },
    Image { url: String },
}
```

### 4. 自定义序列化

```rust
use serde::ser::{Serialize, Serializer};

impl Serialize for MyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 自定义逻辑
    }
}
```

## 💻 实战项目：配置文件管理器

构建一个支持多种格式的配置文件管理器。

### 功能需求

1. 读取 JSON、TOML、YAML 配置
2. 类型安全的配置结构
3. 配置验证
4. 配置合并
5. 环境变量覆盖

### 项目结构

```
serde-demo/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs     # 配置结构
│   ├── parser.rs     # 解析器
│   └── validator.rs  # 验证器
└── README.md
```

## 🧪 练习题

### 练习 1：基本序列化

```rust
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

// 序列化和反序列化
```

### 练习 2：自定义字段名

```rust
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    #[serde(rename = "userId")]
    user_id: u32,
}
```

## 📖 深入阅读

- [Serde Documentation](https://serde.rs/)
- [serde_json](https://docs.rs/serde_json)
- [toml](https://docs.rs/toml)

## ✅ 检查清单

- [ ] 使用 Serde derive 宏
- [ ] 序列化和反序列化 JSON
- [ ] 处理 TOML 和 YAML
- [ ] 使用字段属性
- [ ] 自定义序列化逻辑
- [ ] 处理嵌套结构
- [ ] 错误处理

## 🚀 下一步

完成中级篇后，继续学习 [模块 3.1：宏编程](../../03-advanced/01-macros/)。
