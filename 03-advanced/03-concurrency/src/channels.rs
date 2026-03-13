/// 消息传递（Channels）
///
/// 演示使用 channels 在线程间传递消息

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 简单的消息传递
pub fn simple_channel() {
    println!("简单的消息传递：");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("  收到: {}", received);
}

/// 发送多个消息
pub fn multiple_messages() {
    println!("\n发送多个消息：");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("消息1"),
            String::from("消息2"),
            String::from("消息3"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("  收到: {}", received);
    }
}

/// 多个生产者
pub fn multiple_producers() {
    println!("\n多个生产者：");

    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let msg = format!("来自线程 {} 的消息", i);
            tx_clone.send(msg).unwrap();
        });
    }

    drop(tx); // 关闭原始发送者

    for received in rx {
        println!("  收到: {}", received);
    }
}

/// 同步 channel
pub fn sync_channel() {
    println!("\n同步 channel（有界队列）：");

    let (tx, rx) = mpsc::sync_channel(2); // 容量为 2

    thread::spawn(move || {
        for i in 0..5 {
            println!("  发送: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::sleep(Duration::from_millis(100));

    for received in rx {
        println!("  收到: {}", received);
        thread::sleep(Duration::from_millis(50));
    }
}

/// 生产者-消费者模式
pub fn producer_consumer() {
    println!("\n生产者-消费者模式：");

    let (tx, rx) = mpsc::channel();

    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..10 {
            println!("  生产: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 消费者
    let consumer = thread::spawn(move || {
        for received in rx {
            println!("  消费: {}", received);
            thread::sleep(Duration::from_millis(100));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

/// 超时接收
pub fn timeout_receive() {
    println!("\n超时接收：");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("延迟消息").unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(200)) {
        Ok(msg) => println!("  收到: {}", msg),
        Err(_) => println!("  接收超时"),
    }

    match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(msg) => println!("  收到: {}", msg),
        Err(_) => println!("  接收超时"),
    }
}

/// 非阻塞接收
pub fn try_receive() {
    println!("\n非阻塞接收：");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("消息").unwrap();
    });

    // 立即尝试接收
    match rx.try_recv() {
        Ok(msg) => println!("  收到: {}", msg),
        Err(_) => println!("  暂无消息"),
    }

    thread::sleep(Duration::from_millis(200));

    // 再次尝试
    match rx.try_recv() {
        Ok(msg) => println!("  收到: {}", msg),
        Err(_) => println!("  暂无消息"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_channel() {
        simple_channel();
    }

    #[test]
    fn test_multiple_messages() {
        multiple_messages();
    }

    #[test]
    fn test_multiple_producers() {
        multiple_producers();
    }

    #[test]
    fn test_sync_channel() {
        sync_channel();
    }

    #[test]
    fn test_producer_consumer() {
        producer_consumer();
    }

    #[test]
    fn test_timeout_receive() {
        timeout_receive();
    }

    #[test]
    fn test_try_receive() {
        try_receive();
    }

    #[test]
    fn test_channel_send_receive() {
        let (tx, rx) = mpsc::channel();
        tx.send(42).unwrap();
        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_channel_closed() {
        let (tx, rx) = mpsc::channel::<i32>();
        drop(tx);
        assert!(rx.recv().is_err());
    }
}
