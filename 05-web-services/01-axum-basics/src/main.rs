use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<u64, User>>>,
    pub next_id: Arc<Mutex<u64>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(1, User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() });
        users.insert(2, User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() });

        AppState {
            users: Arc::new(Mutex::new(users)),
            next_id: Arc::new(Mutex::new(3)),
        }
    }
}

pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Json<Vec<User>> {
    let users = state.users.lock().unwrap();
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let mut all_users: Vec<User> = users.values().cloned().collect();
    all_users.sort_by_key(|u| u.id);

    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(all_users.len());

    Json(all_users[start..end].to_vec())
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.lock().unwrap();
    users.get(&id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let user = User {
        id,
        name: req.name,
        email: req.email,
    };

    state.users.lock().unwrap().insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.lock().unwrap();

    if let Some(user) = users.get_mut(&id) {
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

pub async fn delete_user(
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

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok", "version": "1.0.0"}))
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    println!("=== Axum Web 服务演示 ===");
    let state = AppState::new();
    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("服务器运行在 http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state() {
        let state = AppState::new();
        let users = state.users.lock().unwrap();
        assert_eq!(users.len(), 2);
        assert!(users.contains_key(&1));
        assert!(users.contains_key(&2));
    }

    #[tokio::test]
    async fn test_create_user_logic() {
        let state = AppState::new();
        let initial_id = *state.next_id.lock().unwrap();
        
        let req = CreateUserRequest {
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        };
        
        let (status, Json(user)) = create_user(State(state.clone()), Json(req)).await;
        
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(user.id, initial_id);
        assert_eq!(user.name, "Charlie");
        
        let users = state.users.lock().unwrap();
        assert!(users.contains_key(&initial_id));
    }

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: 1,
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
        };
        
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("test@example.com"));
    }
}
