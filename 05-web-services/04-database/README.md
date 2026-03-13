# 模块 5.4：数据库集成

## 🎯 学习目标

- 使用 sqlx 进行异步数据库访问
- 连接池管理
- 事务处理
- 数据库迁移
- 类型安全的查询

## 📚 核心概念

### 1. 连接数据库

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://user:pass@localhost/db")
    .await?;
```

### 2. 查询数据

```rust
#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    .fetch_all(&pool)
    .await?;
```

### 3. 插入数据

```rust
sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    .bind("Alice")
    .bind("alice@example.com")
    .execute(&pool)
    .await?;
```

### 4. 事务

```rust
let mut tx = pool.begin().await?;

sqlx::query("INSERT INTO users (name) VALUES ($1)")
    .bind("Bob")
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

## 💻 实战项目：持久化 API 服务

将 Axum API 与数据库集成。

### 功能需求

1. 数据库连接池
2. CRUD 操作
3. 事务支持
4. 迁移管理

## ✅ 检查清单

- [ ] 连接数据库
- [ ] 实现 CRUD
- [ ] 事务处理
- [ ] 连接池管理
- [ ] 数据库迁移

## 🚀 下一步

完成本模块后，继续学习 [模块 5.5：可观测性](../05-observability/)。
