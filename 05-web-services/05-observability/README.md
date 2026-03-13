# 模块 5.5：可观测性

## 🎯 学习目标

- 使用 tracing 框架
- 结构化日志
- 分布式追踪
- 指标收集
- 性能监控

## 📚 核心概念

### 1. 基本日志

```rust
use tracing::{info, warn, error};

#[tracing::instrument]
async fn process_request(id: u64) {
    info!("Processing request");
    // ...
    warn!("Something unusual");
}
```

### 2. 结构化日志

```rust
use tracing::info;

info!(
    user_id = 123,
    action = "login",
    "User logged in successfully"
);
```

### 3. Span 追踪

```rust
use tracing::info_span;

let span = info_span!("request", method = "GET", path = "/api/users");
let _enter = span.enter();
```

### 4. 订阅器配置

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();
```

## 💻 实战项目：可观测的 Web 服务

为 Web 服务添加完整的可观测性。

### 功能需求

1. 结构化日志
2. 请求追踪
3. 性能监控
4. 错误追踪

## ✅ 检查清单

- [ ] 配置 tracing
- [ ] 添加日志
- [ ] 实现追踪
- [ ] 性能监控
- [ ] 错误追踪

## 🚀 下一步

完成本模块后，你已掌握 Web 服务开发的核心技能！可以继续学习最终项目。
