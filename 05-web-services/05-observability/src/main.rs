use tracing::{debug, error, info, warn, instrument, Level};
use tracing_subscriber;

/// 简单的业务逻辑函数
#[instrument]
fn process_order(order_id: u64, amount: f64) -> Result<String, String> {
    info!("开始处理订单");
    debug!(order_id, amount, "订单详情");

    if amount <= 0.0 {
        error!("订单金额无效");
        return Err("Invalid amount".to_string());
    }

    if amount > 10000.0 {
        warn!("订单金额较大，需要审核");
    }

    info!("订单处理成功");
    Ok(format!("Order {} processed", order_id))
}

/// 模拟数据库操作
#[instrument]
async fn fetch_user(user_id: u64) -> Result<String, String> {
    info!("查询用户信息");
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    if user_id == 0 {
        error!("用户 ID 无效");
        return Err("Invalid user ID".to_string());
    }

    debug!(user_id, "用户查询成功");
    Ok(format!("User {}", user_id))
}

/// 模拟 API 调用
#[instrument]
async fn call_external_api(endpoint: &str) -> Result<String, String> {
    info!(endpoint, "调用外部 API");
    
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    if endpoint.is_empty() {
        error!("API 端点为空");
        return Err("Empty endpoint".to_string());
    }

    info!("API 调用成功");
    Ok("API response".to_string())
}

/// 复杂的业务流程
#[instrument]
async fn complex_workflow(user_id: u64, order_id: u64) -> Result<(), String> {
    info!("开始复杂工作流");

    // 步骤 1：获取用户
    let user = fetch_user(user_id).await?;
    info!(user, "用户信息获取成功");

    // 步骤 2：处理订单
    let order_result = process_order(order_id, 99.99)?;
    info!(order_result, "订单处理完成");

    // 步骤 3：调用外部 API
    let api_result = call_external_api("https://api.example.com/notify").await?;
    info!(api_result, "外部 API 调用完成");

    info!("工作流完成");
    Ok(())
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .init();

    println!("=== 可观测性演示 ===\n");

    // 演示 1：基本日志
    info!("应用启动");
    debug!("调试信息");
    warn!("警告信息");

    // 演示 2：结构化日志
    info!(
        user_id = 123,
        action = "login",
        "用户登录"
    );

    // 演示 3：函数追踪
    match process_order(1001, 99.99) {
        Ok(result) => info!(result, "订单处理结果"),
        Err(e) => error!(error = %e, "订单处理失败"),
    }

    // 演示 4：异步函数追踪
    match fetch_user(42).await {
        Ok(user) => info!(user, "用户查询结果"),
        Err(e) => error!(error = %e, "用户查询失败"),
    }

    // 演示 5：复杂工作流
    match complex_workflow(100, 2001).await {
        Ok(_) => info!("工作流执行成功"),
        Err(e) => error!(error = %e, "工作流执行失败"),
    }

    // 演示 6：错误场景
    info!("测试错误场景");
    let _ = process_order(9999, -10.0);
    let _ = fetch_user(0).await;

    info!("应用结束");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_order_valid() {
        let result = process_order(1, 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_order_invalid() {
        let result = process_order(1, -10.0);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fetch_user_valid() {
        let result = fetch_user(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_user_invalid() {
        let result = fetch_user(0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_call_external_api() {
        let result = call_external_api("https://api.example.com").await;
        assert!(result.is_ok());
    }
}
