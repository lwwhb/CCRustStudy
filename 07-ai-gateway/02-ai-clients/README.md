# 模块 7.2：AI Provider 客户端

## 🎯 学习目标

- 理解多 AI Provider 集成模式
- 掌握统一接口抽象设计
- 学习流式响应处理
- 实现可扩展的客户端架构

## 📚 核心概念

### 1. Provider 抽象

使用 Trait 定义统一的 AI Provider 接口：
```rust
#[async_trait]
trait AIProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<Response>;
    async fn stream(&self, messages: Vec<Message>) -> impl Stream<Item = Chunk>;
}
```

### 2. 适配器模式

为不同的 AI Provider 实现适配器：
- OpenAI 适配器
- Anthropic Claude 适配器
- 本地模型适配器

### 3. 流式响应

处理 Server-Sent Events (SSE) 流式响应：
- 逐块接收数据
- 解析 JSON 数据
- 错误处理和重试

## 🚀 实战项目：多 Provider 客户端

实现支持多个 AI Provider 的客户端系统：
- OpenAI 客户端（模拟）
- Anthropic 客户端（模拟）
- 统一的接口抽象
- 流式响应支持

## 📖 代码说明

### 主要组件

1. **AIProvider Trait** - 统一接口
2. **OpenAIClient** - OpenAI 实现
3. **AnthropicClient** - Anthropic 实现
4. **Message/Response** - 数据结构

## 🧪 测试

```bash
cargo test
```

## 🔍 运行示例

```bash
cargo run
```

## 📝 练习

1. 添加对 Google Gemini 的支持
2. 实现请求重试机制
3. 添加响应缓存
4. 实现负载均衡

## 🔗 深入阅读

- [OpenAI API 文档](https://platform.openai.com/docs)
- [Anthropic API 文档](https://docs.anthropic.com/)
- [async-trait](https://docs.rs/async-trait/)

## ✅ 检查清单

- [ ] 理解 Provider 抽象模式
- [ ] 实现多个 Provider 客户端
- [ ] 掌握流式响应处理
- [ ] 完成所有测试
- [ ] 运行示例程序
- [ ] 完成练习题
