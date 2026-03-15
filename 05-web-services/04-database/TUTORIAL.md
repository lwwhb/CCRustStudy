# 模块 5.4：数据库集成 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解异步数据库访问
2. 掌握 sqlx 的使用
3. 学习连接池管理
4. 实现事务处理
5. 掌握数据库迁移

## 🎯 为什么需要异步数据库？

### 同步 vs 异步数据库访问

**同步数据库（传统方式）**：
```
请求 1 → 线程 1 → 查询数据库（阻塞）→ 响应
请求 2 → 线程 2 → 查询数据库（阻塞）→ 响应
请求 3 → 线程 3 → 查询数据库（阻塞）→ 响应

问题：
- 每个请求占用一个线程
- 线程在等待数据库时被阻塞
- 资源浪费
- 并发能力受限
```

**异步数据库**：
```
请求 1 → 任务 1 → 查询数据库（不阻塞）→ 响应
请求 2 → 任务 2 → 查询数据库（不阻塞）→ 响应
请求 3 → 任务 3 → 查询数据库（不阻塞）→ 响应

优势：
- 单线程处理多个请求
- 等待时可以处理其他任务
- 高并发能力
- 资源利用率高
```

### 性能对比

```
同步模式：
- 1000 并发 → 1000 个线程 → 1GB+ 内存
- 数据库连接数 = 线程数

异步模式：
- 1000 并发 → 几个线程 → 几 MB 内存
- 数据库连接数 = 连接池大小（如 10-20）
```

## 📖 核心概念详解

### 1. sqlx 简介

sqlx 是 Rust 的异步 SQL 工具包。

**特点**：
- 完全异步
- 编译时检查 SQL
- 支持多种数据库（PostgreSQL、MySQL、SQLite）
- 类型安全
- 连接池内置

**与其他库对比**：

```rust
// Diesel（同步 ORM）
let results = users
    .filter(age.gt(18))
    .load::<User>(&conn)?;

// sqlx（异步，原生 SQL）
let results = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE age > $1"
)
.bind(18)
.fetch_all(&pool)
.await?;
```

**sqlx 的优势**：
- 异步支持
- 更接近 SQL（学习曲线低）
- 编译时验证
- 灵活性高

### 2. 连接池

连接池管理数据库连接的生命周期。

```
连接池工作原理：

应用启动 → 创建连接池 → 预创建 N 个连接
                           ↓
请求 1 → 获取连接 → 执行查询 → 归还连接
请求 2 → 获取连接 → 执行查询 → 归还连接
请求 3 → 等待连接 → 执行查询 → 归还连接
```

**配置参数**：
```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(5)        // 最大连接数
    .min_connections(1)        // 最小连接数
    .acquire_timeout(Duration::from_secs(3))  // 获取超时
    .idle_timeout(Duration::from_secs(600))   // 空闲超时
    .max_lifetime(Duration::from_secs(1800))  // 最大生命周期
    .connect("postgres://...")
    .await?;
```

**为什么需要连接池？**
```
没有连接池：
- 每次查询创建新连接（慢）
- 连接数不受控制
- 资源浪费

有连接池：
- 复用连接（快）
- 限制连接数
- 资源高效利用
```

### 3. 查询方式

sqlx 提供多种查询方式：

#### 方式 1：query（动态查询）

```rust
use sqlx::{query, Row};

// 执行查询
let rows = query("SELECT id, name FROM users WHERE age > $1")
    .bind(18)
    .fetch_all(&pool)
    .await?;

// 手动提取字段
for row in rows {
    let id: i32 = row.get("id");
    let name: String = row.get("name");
    println!("{}: {}", id, name);
}
```

**特点**：
- 灵活
- 需要手动提取字段
- 运行时类型检查

#### 方式 2：query_as（映射到结构体）

```rust
use sqlx::FromRow;

#[derive(FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// 自动映射
let users = query_as::<_, User>(
    "SELECT id, name, email FROM users WHERE age > $1"
)
.bind(18)
.fetch_all(&pool)
.await?;

for user in users {
    println!("{}: {}", user.id, user.name);
}
```

**特点**：
- 自动映射
- 类型安全
- 代码简洁

#### 方式 3：query!（编译时检查）

```rust
// 编译时验证 SQL 和类型
let users = sqlx::query!(
    "SELECT id, name, email FROM users WHERE age > $1",
    18
)
.fetch_all(&pool)
.await?;

for user in users {
    // 字段类型在编译时确定
    println!("{}: {}", user.id, user.name);
}
```

**特点**：
- 编译时检查 SQL 语法
- 编译时检查字段类型
- 最安全
- 需要数据库连接（编译时）

### 4. 事务

事务保证一组操作的原子性。

```rust
use sqlx::Transaction;

// 开始事务
let mut tx = pool.begin().await?;

// 执行多个操作
query("INSERT INTO users (name) VALUES ($1)")
    .bind("Alice")
    .execute(&mut *tx)
    .await?;

query("UPDATE accounts SET balance = balance - 100 WHERE user_id = $1")
    .bind(1)
    .execute(&mut *tx)
    .await?;

// 提交事务
tx.commit().await?;

// 如果出错，事务会自动回滚
```

**ACID 特性**：
```
Atomicity（原子性）：
- 全部成功或全部失败

Consistency（一致性）：
- 数据保持一致状态

Isolation（隔离性）：
- 事务之间互不干扰

Durability（持久性）：
- 提交后永久保存
```

### 5. 迁移

迁移管理数据库 schema 的变更。

```bash
# 创建迁移
sqlx migrate add create_users_table

# 生成文件：migrations/20240101000000_create_users_table.sql
```

```sql
-- migrations/20240101000000_create_users_table.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

```rust
// 运行迁移
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

**迁移的好处**：
- 版本控制
- 可重复执行
- 团队协作
- 回滚支持

## 💻 实战项目：用户管理 API

### 项目需求

构建一个用户管理 API，支持：
1. CRUD 操作（创建、读取、更新、删除）
2. 分页查询
3. 搜索功能
4. 事务处理
5. 错误处理

### 步骤 1：项目设置

```toml
# Cargo.toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
```

### 步骤 2：数据库设置

```sql
-- migrations/20240101000000_create_users.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    age INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_name ON users(name);
```

### 步骤 3：定义模型

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// 数据库模型
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
}

// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}

// 分页参数
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page();
        (page - 1) * per_page
    }

    pub fn per_page(&self) -> i64 {
        self.per_page.unwrap_or(10).clamp(1, 100)
    }
}
```

### 步骤 4：数据库操作层

```rust
use sqlx::{PgPool, Result};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // 创建用户
    pub async fn create(&self, req: CreateUserRequest) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, age)
            VALUES ($1, $2, $3)
            RETURNING id, name, email, age, created_at, updated_at
            "#
        )
        .bind(&req.name)
        .bind(&req.email)
        .bind(req.age)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // 根据 ID 查询
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // 分页查询所有用户
    pub async fn find_all(
        &self,
        params: &PaginationParams,
    ) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(params.per_page())
        .bind(params.offset())
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // 搜索用户（按名字）
    pub async fn search_by_name(&self, name: &str) -> Result<Vec<User>> {
        let pattern = format!("%{}%", name);
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE name ILIKE $1 ORDER BY id"
        )
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // 更新用户
    pub async fn update(
        &self,
        id: i32,
        req: UpdateUserRequest,
    ) -> Result<Option<User>> {
        // 构建动态更新查询
        let mut query = String::from("UPDATE users SET updated_at = CURRENT_TIMESTAMP");
        let mut bind_count = 1;

        if req.name.is_some() {
            query.push_str(&format!(", name = ${}", bind_count));
            bind_count += 1;
        }
        if req.email.is_some() {
            query.push_str(&format!(", email = ${}", bind_count));
            bind_count += 1;
        }
        if req.age.is_some() {
            query.push_str(&format!(", age = ${}", bind_count));
            bind_count += 1;
        }

        query.push_str(&format!(" WHERE id = ${} RETURNING *", bind_count));

        // 执行查询
        let mut q = sqlx::query_as::<_, User>(&query);

        if let Some(name) = req.name {
            q = q.bind(name);
        }
        if let Some(email) = req.email {
            q = q.bind(email);
        }
        if let Some(age) = req.age {
            q = q.bind(age);
        }

        q = q.bind(id);

        let user = q.fetch_optional(&self.pool).await?;
        Ok(user)
    }

    // 删除用户
    pub async fn delete(&self, id: i32) -> Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 统计用户数量
    pub async fn count(&self) -> Result<i64> {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }
}
```

**关键点**：
- 使用 `query_as` 自动映射到结构体
- `fetch_one` - 获取一条记录（不存在会报错）
- `fetch_optional` - 获取可选记录
- `fetch_all` - 获取所有记录
- `execute` - 执行不返回数据的查询

### 步骤 5：API 处理器

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

// 应用状态
pub struct AppState {
    repo: UserRepository,
}

// 创建用户
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    match state.repo.create(req).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            eprintln!("创建用户失败: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "创建用户失败".to_string(),
            ))
        }
    }
}

// 获取用户
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    match state.repo.find_by_id(id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("查询用户失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 列出用户
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<User>>, StatusCode> {
    match state.repo.find_all(&params).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("查询用户列表失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 搜索用户
pub async fn search_users(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let name = params.get("name").map(|s| s.as_str()).unwrap_or("");

    match state.repo.search_by_name(name).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("搜索用户失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 更新用户
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    match state.repo.update(id, req).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("更新用户失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 删除用户
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> StatusCode {
    match state.repo.delete(id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(e) => {
            eprintln!("删除用户失败: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
```

### 步骤 6：主程序

```rust
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 用户管理 API ===\");

    // 数据库连接
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:pass@localhost/mydb".to_string());

    // 创建连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("数据库连接成功");

    // 运行迁移
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("数据库迁移完成");

    // 创建应用状态
    let state = Arc::new(AppState {
        repo: UserRepository::new(pool),
    });

    // 创建路由
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/users/search", get(search_users))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .with_state(state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;

    println!("服务器运行在 http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
```

### 步骤 7：测试 API

```bash
# 创建用户
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com","age":25}'

# 获取用户
curl http://localhost:3000/users/1

# 列出用户（分页）
curl "http://localhost:3000/users?page=1&per_page=10"

# 搜索用户
curl "http://localhost:3000/users/search?name=alice"

# 更新用户
curl -X PUT http://localhost:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice Updated"}'

# 删除用户
curl -X DELETE http://localhost:3000/users/1
```

## 🔍 深入理解

### 连接池的工作原理

```
初始化：
┌─────────────┐
│ 连接池      │
│ ┌─────┐    │
│ │Conn1│    │  空闲连接
│ │Conn2│    │
│ │Conn3│    │
│ └─────┘    │
└─────────────┘

请求到达：
┌─────────────┐
│ 连接池      │
│ ┌─────┐    │
│ │Conn2│    │  空闲
│ │Conn3│    │
│ └─────┘    │
└─────────────┘
      ↓
   Conn1 被借出

归还连接：
┌─────────────┐
│ 连接池      │
│ ┌─────┐    │
│ │Conn1│ ←─ 归还
│ │Conn2│    │
│ │Conn3│    │
│ └─────┘    │
└─────────────┘
```

### SQL 注入防护

```rust
// ❌ 危险：SQL 注入
let name = "'; DROP TABLE users; --";
let query = format!("SELECT * FROM users WHERE name = '{}'", name);
// 结果：SELECT * FROM users WHERE name = ''; DROP TABLE users; --'

// ✅ 安全：参数绑定
let name = "'; DROP TABLE users; --";
sqlx::query("SELECT * FROM users WHERE name = $1")
    .bind(name)  // 自动转义
    .fetch_all(&pool)
    .await?;
```

**sqlx 如何防护**：
- 使用参数绑定（`$1`, `$2`）
- 自动转义特殊字符
- 类型检查

### 事务隔离级别

```rust
use sqlx::postgres::PgConnection;

// 设置隔离级别
let mut tx = pool.begin().await?;
sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
    .execute(&mut *tx)
    .await?;

// 执行操作
// ...

tx.commit().await?;
```

**隔离级别**：
```
READ UNCOMMITTED（读未提交）：
- 最低级别
- 可能读到脏数据

READ COMMITTED（读已提交）：
- PostgreSQL 默认
- 只读已提交的数据

REPEATABLE READ（可重复读）：
- 事务内多次读取结果一致

SERIALIZABLE（串行化）：
- 最高级别
- 完全隔离
```

## 📝 练习题

### 练习 1：添加软删除

```rust
// 修改表结构
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMP;

// 实现软删除
pub async fn soft_delete(&self, id: i32) -> Result<bool> {
    // 你的代码
}

// 查询时排除已删除
pub async fn find_all_active(&self) -> Result<Vec<User>> {
    // 你的代码
}
```

### 练习 2：实现批量插入

```rust
pub async fn create_batch(
    &self,
    users: Vec<CreateUserRequest>,
) -> Result<Vec<User>> {
    // 提示：使用事务和循环
    // 你的代码
}
```

### 练习 3：实现全文搜索

```sql
-- 创建全文搜索索引
CREATE INDEX idx_users_search ON users 
USING gin(to_tsvector('english', name || ' ' || email));
```

```rust
pub async fn full_text_search(&self, query: &str) -> Result<Vec<User>> {
    // 使用 PostgreSQL 全文搜索
    // 你的代码
}
```

### 练习 4：实现乐观锁

```rust
// 添加版本字段
ALTER TABLE users ADD COLUMN version INTEGER DEFAULT 0;

pub async fn update_with_version(
    &self,
    id: i32,
    version: i32,
    req: UpdateUserRequest,
) -> Result<Option<User>> {
    // 检查版本号，防止并发冲突
    // 你的代码
}
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解异步数据库访问的优势
- [ ] 使用 sqlx 连接数据库
- [ ] 配置和使用连接池
- [ ] 执行 CRUD 操作
- [ ] 使用事务保证数据一致性
- [ ] 实现分页查询
- [ ] 处理数据库错误
- [ ] 使用迁移管理 schema
- [ ] 防止 SQL 注入
- [ ] 理解连接池的工作原理

## 🔗 延伸阅读

- [sqlx 官方文档](https://docs.rs/sqlx/)
- [PostgreSQL 文档](https://www.postgresql.org/docs/)
- [数据库事务](https://en.wikipedia.org/wiki/Database_transaction)
- [SQL 注入防护](https://owasp.org/www-community/attacks/SQL_Injection)

## 🚀 下一步

完成本模块后，你可以：
1. 继续学习模块 5.5（可观测性）
2. 学习高级查询优化
3. 探索 NoSQL 数据库集成

---

**掌握数据库集成，构建数据驱动的应用！** 🚀
