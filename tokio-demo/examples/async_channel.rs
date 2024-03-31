// 在异步编程中，背压和有界队列是非常重要的概念。背压是一种流量控制机制，用于控制消息发送的速度，
// 以避免消息积压和内存耗尽。有界队列是一种限制队列长度的机制，用于控制消息的数量，以避免队列溢出
// 和内存耗尽。在 Tokio 中，我们可以使用 async-channel 模块和 crossbeam-channel 模块来
// 实现背压和有界队列

use async_channel::{bounded, Receiver, Sender};

async fn demo1() {
    let (tx, rx) = bounded(10);

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    let msg = rx.recv().await.unwrap();
    println!("{}", msg);
}

async fn try_send() {
    let (tx, rx) = bounded(10);
    tokio::spawn(async move {
        loop {
            if let Err(_) = tx.try_send("hello") {
                // Channel is full, wait for a moment
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    });

    loop {
        let msg = rx.recv().await.unwrap();
        // Process the message
    }
}

#[tokio::main]
async fn main() {
    // demo1().await;
    try_send().await;
}
