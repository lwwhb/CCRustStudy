# 模块 5.1：Axum Web 框架 - 详细学习指南

## 📚 学习目标

通过本模块，你将：
1. 理解现代 Web 框架的架构
2. 掌握 Axum 的核心概念
3. 学习路由和处理器
4. 掌握提取器（Extractors）
5. 实现状态管理
6. 构建完整的 RESTful API

## 🎯 为什么选择 Axum？

### Web 框架对比

**传统同步框架（如 Flask/Express）**：
```
请求 1 → 线程 1 → 阻塞等待数据库 → 响应
请求 2 → 线程 2 → 阻塞等待数据库 → 响应
请求 3 → 线程 3 → 阻塞等待数据库 → 响应

问题：每个请求占用一个线程，资源消耗大
```

**异步框架（如 Axum）**：
```
请求 1 → 任务 1 → 等待数据库（不阻塞）→ 响应
请求 2 → 任务 2 → 等待数据库（不阻塞）→ 响应
请求 3 → 任务 3 → 等待数据库（不阻塞）→ 响应

优势：单线程处理数千个并发请求
```

**Axum 的特点**：
- 基于 Tokio 异步运行时
- 类型安全的提取器
- 零成本抽象
- 与 Tower 生态系统集成
- 编译时错误检查

### 性能对比

```
传统框架：
- 1000 并发 → 1000 个线程 → 1GB+ 内存
- 上下文切换开销大

Axum：
- 1000 并发 → 几个线程 → 几 MB 内存
- 任务切换开销小
```

## 📖 核心概念详解

### 1. 路由（Routing）

路由将 HTTP 请求映射到处理函数。

#### 基础路由

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

// 创建路由
let app = Router::new()
    .route("/", get(root))              // GET /
    .route("/users", get(list_users))   // GET /users
    .route("/users", post(create_user)) // POST /users
    .route("/users/:id", get(get_user)) // GET /users/123
    .route("/users/:id", put(update_user))    // PUT /users/123
    .route("/users/:id", delete(delete_user)); // DELETE /users/123

// 处理函数
async fn root() -> &'static str {
    "Hello, World!"
}
```

**路由匹配规则**：
```
/users          → 精确匹配
/users/:id      → 路径参数（:id 可以是任何值）
/files/*path    → 通配符（匹配剩余所有路径）
```

#### 路由组合

```rust
// 方法链式组合
let app = Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id",
        get(get_user)
        .put(update_user)
        .delete(delete_user)
    );

// 嵌套路由
let api_routes = Router::new()
    .route("/users", get(list_users))
    .route("/posts", get(list_posts));

let app = Router::new()
    .nest("/api/v1", api_routes)  // 所有路由加上 /api/v1 前缀
    .route("/health", get(health_check));

// 最终路由：
// GET /api/v1/users
// GET /api/v1/posts
// GET /health
```

### 2. 处理器（Handlers）

处理器是处理 HTTP 请求的异步函数。

#### 处理器签名

```rust
// 最简单的处理器
async fn hello() -> &'static str {
    "Hello!"
}

// 返回 JSON
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn json_handler() -> Json<Message> {
    Json(Message {
        text: "Hello, JSON!".to_string(),
    })
}

// 返回状态码和 JSON
use axum::http::StatusCode;

async fn create_handler() -> (StatusCode, Json<Message>) {
    (
        StatusCode::CREATED,
        Json(Message {
            text: "Created!".to_string(),
        })
    )
}

// 返回 Result
async fn fallible_handler() -> Result<Json<Message>, StatusCode> {
    if some_condition() {
        Ok(Json(Message { text: "Success".to_string() }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

**处理器的灵活性**：
```rust
// Axum 支持多种返回类型：
&'static str
String
Json<T>
Html<String>
(StatusCode, Json<T>)
Result<T, E>
// ... 任何实现了 IntoResponse 的类型
```

### 3. 提取器（Extractors）

提取器从请求中提取数据，类型安全且零成本。

#### 路径参数提取

```rust
use axum::extract::Path;

// 单个参数
async fn get_user(Path(id): Path<u64>) -> String {
    format!("User ID: {}", id)
}

// 多个参数
async fn get_post(
    Path((user_id, post_id)): Path<(u64, u64)>
) -> String {
    format!("User {} Post {}", user_id, post_id)
}

// 使用结构体
#[derive(Deserialize)]
struct Params {
    user_id: u64,
    post_id: u64,
}

async fn get_post_struct(Path(params): Path<Params>) -> String {
    format!("User {} Post {}", params.user_id, params.post_id)
}
```

#### 查询参数提取

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u64>,
    per_page: Option<u64>,
}

async fn list_users(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    format!("Page {} with {} items", page, per_page)
}

// 请求: GET /users?page=2&per_page=20
```

#### JSON 请求体提取

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(user))
}
```

#### 组合多个提取器

```rust
async fn complex_handler(
    Path(id): Path<u64>,
    Query(params): Query<Pagination>,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    // 可以同时使用多个提取器
    // ...
}
```

**提取器的顺序很重要**：
```rust
// ✅ 正确：State 在前
async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<Data>,
) { }

// ❌ 错误：消费性提取器（如 String）会消费请求体
async fn handler(
    body: String,  // 消费了请求体
    Json(payload): Json<Data>,  // 无法再提取 JSON
) { }
```

### 4. 状态管理（State）

在处理器之间共享数据。

#### 定义应用状态

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<u64, User>>>,
    next_id: Arc<Mutex<u64>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}
```

**为什么需要 Arc 和 Mutex？**

```
Arc (Atomic Reference Counting):
- 允许多个所有者共享数据
- 线程安全的引用计数
- 不可变共享

Mutex (Mutual Exclusion):
- 提供内部可变性
- 同一时间只有一个线程可以访问
- 防止数据竞争

Arc<Mutex<T>> 组合：
- Arc 允许多个处理器共享状态
- Mutex 允许修改共享状态
```

#### 使用状态

```rust
use axum::extract::State;

async fn get_users(
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    let all_users: Vec<User> = users.values().cloned().collect();
    Json(all_users)
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // 获取下一个 ID
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    // 创建用户
    let user = User {
        id,
        name: req.name,
        email: req.email,
    };

    // 保存用户
    state.users.lock().unwrap().insert(id, user.clone());

    (StatusCode::CREATED, Json(user))
}

// 将状态附加到路由
let state = AppState::new();
let app = Router::new()
    .route("/users", get(get_users).post(create_user))
    .with_state(state);
```

### 5. 错误处理

#### 使用 Result

```rust
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().unwrap();

    users.get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// 请求不存在的用户 → 404 Not Found
// 请求存在的用户 → 200 OK + JSON
```

#### 自定义错误类型

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

enum AppError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Resource not found")
            }
            AppError::DatabaseError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str())
            }
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, msg.as_str())
            }
        };

        (status, message).into_response()
    }
}

async fn handler() -> Result<Json<User>, AppError> {
    // 可以返回自定义错误
    Err(AppError::NotFound)
}
```

## 💻 实战项目：RESTful API

### 项目需求

构建一个用户管理 API，支持：
1. 列出所有用户（支持分页）
2. 获取单个用户
3. 创建用户
4. 更新用户
5. 删除用户
6. 健康检查端点

### API 设计

```
GET    /health           → 健康检查
GET    /users            → 列出用户（支持 ?page=1&per_page=10）
POST   /users            → 创建用户
GET    /users/:id        → 获取用户
PUT    /users/:id        → 更新用户
DELETE /users/:id        → 删除用户
```

### 步骤 1：定义数据结构

```rust
use serde::{Deserialize, Serialize};

// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// 创建用户请求
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// 更新用户请求（字段可选）
#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

// 分页参数
#[derive(Debug, Deserialize)]
struct PaginationParams {
    page: Option<u64>,
    per_page: Option<u64>,
}
```

### 步骤 2：定义应用状态

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<u64, User>>>,
    next_id: Arc<Mutex<u64>>,
}

impl AppState {
    fn new() -> Self {
        // 初始化一些测试数据
        let mut users = HashMap::new();
        users.insert(1, User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });
        users.insert(2, User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        });

        Self {
            users: Arc::new(Mutex::new(users)),
            next_id: Arc::new(Mutex::new(3)),
        }
    }
}
```

### 步骤 3：实现处理器

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};

// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "1.0.0"
    }))
}

// 列出用户（支持分页）
async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    // 转换为 Vec 并排序
    let mut all_users: Vec<User> = users.values().cloned().collect();
    all_users.sort_by_key(|u| u.id);

    // 分页
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(all_users.len());

    Json(all_users[start..end].to_vec())
}

// 获取单个用户
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().unwrap();

    users.get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// 创建用户
async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    // 生成新 ID
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    // 创建用户
    let user = User {
        id,
        name: req.name,
        email: req.email,
    };

    // 保存用户
    state.users.lock().unwrap().insert(id, user.clone());

    (StatusCode::CREATED, Json(user))
}

// 更新用户
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.lock().unwrap();

    if let Some(user) = users.get_mut(&id) {
        // 更新字段（如果提供）
        if let Some(name) = req.name {
            user.name = name;
        }
        if let Some(email) = req.email {
            user.email = email;
        }

        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// 删除用户
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut users = state.users.lock().unwrap();

    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
```

### 步骤 4：创建路由

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users",
            get(get_users)
            .post(create_user)
        )
        .route("/users/:id",
            get(get_user)
            .put(update_user)
            .delete(delete_user)
        )
        .with_state(state)
}
```

### 步骤 5：启动服务器

```rust
#[tokio::main]
async fn main() {
    println!("=== Axum Web 服务演示 ===");

    // 创建应用状态
    let state = AppState::new();

    // 创建路由
    let app = create_router(state);

    // 绑定地址
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("服务器运行在 http://127.0.0.1:3000");

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}
```

### 步骤 6：测试 API

```bash
# 健康检查
curl http://localhost:3000/health

# 列出用户
curl http://localhost:3000/users

# 分页
curl "http://localhost:3000/users?page=1&per_page=5"

# 获取用户
curl http://localhost:3000/users/1

# 创建用户
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Charlie","email":"charlie@example.com"}'

# 更新用户
curl -X PUT http://localhost:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice Updated"}'

# 删除用户
curl -X DELETE http://localhost:3000/users/1
```

## 🔍 深入理解

### Axum 的工作原理

```
1. 请求到达
   ↓
2. 路由匹配
   ↓
3. 提取器提取数据（编译时类型检查）
   ↓
4. 调用处理器
   ↓
5. 处理器返回响应
   ↓
6. 转换为 HTTP 响应
   ↓
7. 发送给客户端
```

### 类型安全的好处

```rust
// ❌ 运行时错误（其他框架）
app.get("/users/:id", (req, res) => {
    const id = req.params.id;  // 字符串！
    const user = db.get(id);   // 类型不匹配
});

// ✅ 编译时检查（Axum）
async fn get_user(Path(id): Path<u64>) -> Json<User> {
    // id 保证是 u64
    // 如果 URL 中不是数字，Axum 自动返回 400
}
```

### 零成本抽象

```rust
// 提取器在编译时展开，没有运行时开销
async fn handler(
    Path(id): Path<u64>,
    Query(params): Query<Params>,
) {
    // 编译后等价于直接访问请求数据
    // 没有反射，没有动态查找
}
```

## 📝 练习题

### 练习 1：添加搜索功能

```rust
// 实现按名字搜索用户
// GET /users/search?name=alice

#[derive(Deserialize)]
struct SearchParams {
    name: String,
}

async fn search_users(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Json<Vec<User>> {
    // 你的代码
}
```

### 练习 2：添加验证

```rust
// 验证邮箱格式
fn is_valid_email(email: &str) -> bool {
    // 你的代码
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    // 添加验证逻辑
}
```

### 练习 3：添加中间件

```rust
// 实现请求日志中间件
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/users", get(get_users))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
    );
```

## 🎯 学习检查清单

完成本模块后，你应该能够：

- [ ] 理解 Axum 的核心概念
- [ ] 创建和配置路由
- [ ] 编写异步处理器
- [ ] 使用各种提取器
- [ ] 管理应用状态
- [ ] 处理错误
- [ ] 返回不同类型的响应
- [ ] 测试 API 端点
- [ ] 理解类型安全的好处

## 🔗 延伸阅读

- [Axum 官方文档](https://docs.rs/axum/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)
- [Tower 中间件](https://docs.rs/tower/)
- [RESTful API 设计最佳实践](https://restfulapi.net/)

## 🚀 下一步

完成本模块后，你可以：
1. 继续学习模块 5.2（HTTP 客户端）
2. 学习模块 5.3（流式处理）
3. 深入学习中间件和认证

---

**掌握 Axum，构建高性能 Web 服务！** 🚀
