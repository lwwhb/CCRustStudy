use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

/// 简单的内存数据库（模拟真实数据库）
struct InMemoryDB {
    users: Arc<Mutex<HashMap<i32, User>>>,
    next_id: Arc<Mutex<i32>>,
}

impl InMemoryDB {
    fn new() -> Self {
        InMemoryDB {
            users: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// 插入用户
    fn insert_user(&self, name: String, email: String) -> Result<User, String> {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;

        let user = User { id, name, email };
        self.users.lock().unwrap().insert(id, user.clone());
        Ok(user)
    }

    /// 查询所有用户
    fn get_all_users(&self) -> Vec<User> {
        self.users
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    /// 根据 ID 查询用户
    fn get_user_by_id(&self, id: i32) -> Option<User> {
        self.users.lock().unwrap().get(&id).cloned()
    }

    /// 更新用户
    fn update_user(&self, id: i32, name: Option<String>, email: Option<String>) -> Result<User, String> {
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&id) {
            if let Some(n) = name {
                user.name = n;
            }
            if let Some(e) = email {
                user.email = e;
            }
            Ok(user.clone())
        } else {
            Err("User not found".to_string())
        }
    }

    /// 删除用户
    fn delete_user(&self, id: i32) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if users.remove(&id).is_some() {
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }
}

fn main() {
    println!("=== 数据库集成演示 ===\n");

    let db = InMemoryDB::new();

    // 插入数据
    println!("=== 插入用户 ===");
    let user1 = db.insert_user("Alice".to_string(), "alice@example.com".to_string()).unwrap();
    println!("插入用户: {:?}", user1);

    let user2 = db.insert_user("Bob".to_string(), "bob@example.com".to_string()).unwrap();
    println!("插入用户: {:?}", user2);

    // 查询所有用户
    println!("\n=== 查询所有用户 ===");
    let users = db.get_all_users();
    for user in &users {
        println!("  - [{}] {} ({})", user.id, user.name, user.email);
    }

    // 根据 ID 查询
    println!("\n=== 根据 ID 查询 ===");
    if let Some(user) = db.get_user_by_id(1) {
        println!("找到用户: {:?}", user);
    }

    // 更新用户
    println!("\n=== 更新用户 ===");
    let updated = db.update_user(1, Some("Alice Smith".to_string()), None).unwrap();
    println!("更新后: {:?}", updated);

    // 删除用户
    println!("\n=== 删除用户 ===");
    db.delete_user(2).unwrap();
    println!("已删除用户 ID: 2");

    // 再次查询所有用户
    println!("\n=== 最终用户列表 ===");
    let users = db.get_all_users();
    for user in &users {
        println!("  - [{}] {} ({})", user.id, user.name, user.email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_user() {
        let db = InMemoryDB::new();
        let user = db.insert_user("Test".to_string(), "test@example.com".to_string()).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Test");
    }

    #[test]
    fn test_get_all_users() {
        let db = InMemoryDB::new();
        db.insert_user("User1".to_string(), "user1@example.com".to_string()).unwrap();
        db.insert_user("User2".to_string(), "user2@example.com".to_string()).unwrap();
        
        let users = db.get_all_users();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_get_user_by_id() {
        let db = InMemoryDB::new();
        let user = db.insert_user("Test".to_string(), "test@example.com".to_string()).unwrap();
        
        let found = db.get_user_by_id(user.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");
    }

    #[test]
    fn test_update_user() {
        let db = InMemoryDB::new();
        let user = db.insert_user("Test".to_string(), "test@example.com".to_string()).unwrap();
        
        let updated = db.update_user(user.id, Some("Updated".to_string()), None).unwrap();
        assert_eq!(updated.name, "Updated");
    }

    #[test]
    fn test_delete_user() {
        let db = InMemoryDB::new();
        let user = db.insert_user("Test".to_string(), "test@example.com".to_string()).unwrap();
        
        let result = db.delete_user(user.id);
        assert!(result.is_ok());
        
        let found = db.get_user_by_id(user.id);
        assert!(found.is_none());
    }
}
