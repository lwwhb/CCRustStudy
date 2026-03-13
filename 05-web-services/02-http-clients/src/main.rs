use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[derive(Debug, Serialize)]
struct CreatePost {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

async fn fetch_post(id: u32) -> Result<Post, reqwest::Error> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
    let post = reqwest::get(&url).await?.json::<Post>().await?;
    Ok(post)
}

async fn fetch_all_posts() -> Result<Vec<Post>, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let posts = reqwest::get(url).await?.json::<Vec<Post>>().await?;
    Ok(posts)
}

async fn create_post(post: CreatePost) -> Result<Post, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&post)
        .send()
        .await?
        .json::<Post>()
        .await?;
    Ok(response)
}

async fn fetch_with_timeout(url: &str, timeout_secs: u64) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;
    
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

async fn fetch_multiple_posts(ids: Vec<u32>) -> Vec<Result<Post, String>> {
    let mut handles = vec![];
    
    for id in ids {
        let handle = tokio::spawn(async move {
            fetch_post(id).await
        });
        handles.push(handle);
    }
    
    let mut results = vec![];
    for handle in handles {
        match handle.await {
            Ok(Ok(post)) => results.push(Ok(post)),
            Ok(Err(e)) => results.push(Err(e.to_string())),
            Err(e) => results.push(Err(e.to_string())),
        }
    }
    
    results
}

async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, reqwest::Error> {
    let mut retries = 0;
    
    loop {
        match reqwest::get(url).await {
            Ok(response) => {
                return response.text().await;
            }
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(e);
                }
                println!("请求失败，重试 {}/{}", retries, max_retries);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("=== HTTP 客户端演示 ===\n");

    println!("=== 演示 1：获取单个文章 ===");
    match fetch_post(1).await {
        Ok(post) => {
            println!("文章 ID: {}", post.id);
            println!("标题: {}", post.title);
            println!("内容: {}", &post.body[..50.min(post.body.len())]);
        }
        Err(e) => println!("错误: {}", e),
    }
    println!();

    println!("=== 演示 2：获取所有文章 ===");
    match fetch_all_posts().await {
        Ok(posts) => {
            println!("获取到 {} 篇文章", posts.len());
            for post in posts.iter().take(3) {
                println!("  - [{}] {}", post.id, post.title);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    println!();

    println!("=== 演示 3：创建新文章 ===");
    let new_post = CreatePost {
        title: "测试文章".to_string(),
        body: "这是一篇测试文章的内容".to_string(),
        user_id: 1,
    };
    match create_post(new_post).await {
        Ok(post) => {
            println!("创建成功！");
            println!("文章 ID: {}", post.id);
            println!("标题: {}", post.title);
        }
        Err(e) => println!("错误: {}", e),
    }
    println!();

    println!("=== 演示 4：并发请求多个文章 ===");
    let ids = vec![1, 2, 3, 4, 5];
    let results = fetch_multiple_posts(ids).await;
    println!("成功获取 {} 个结果", results.iter().filter(|r| r.is_ok()).count());
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(post) => println!("  [{}] {}", i + 1, post.title),
            Err(e) => println!("  [{}] 错误: {}", i + 1, e),
        }
    }
    println!();

    println!("演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_post() {
        let result = fetch_post(1).await;
        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.id, 1);
    }

    #[tokio::test]
    async fn test_fetch_all_posts() {
        let result = fetch_all_posts().await;
        assert!(result.is_ok());
        let posts = result.unwrap();
        assert!(!posts.is_empty());
    }

    #[tokio::test]
    async fn test_create_post() {
        let new_post = CreatePost {
            title: "Test".to_string(),
            body: "Test body".to_string(),
            user_id: 1,
        };
        let result = create_post(new_post).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_post_serialization() {
        let post = Post {
            user_id: 1,
            id: 1,
            title: "Test".to_string(),
            body: "Body".to_string(),
        };
        let json = serde_json::to_string(&post).unwrap();
        assert!(json.contains("Test"));
    }
}
