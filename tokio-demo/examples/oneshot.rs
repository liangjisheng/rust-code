// oneshot channel 是一种只能发送一次消息的 channel。它的特点是发送端只能发送一次消息，
// 接收端只能接收一次消息。一旦消息被发送或接收，channel 就会被关闭。

use tokio::{self, sync, time};

async fn do_something() -> i32 {
    // 创建一个oneshot channel
    let (tx, rx) = sync::oneshot::channel();

    // 在一个异步任务中发送消息
    tokio::spawn(async move {
        let result = 42;
        tx.send(result).unwrap();
    });

    // 在当前任务中接收消息
    let result = rx.await.unwrap();
    result
}

async fn demo1() {
    let (tx, rx) = sync::oneshot::channel();

    tokio::spawn(async move {
        if tx.send(33).is_err() {
            println!("receiver dropped");
        }
    });

    match rx.await {
        Ok(value) => println!("received: {:?}", value),
        Err(_) => println!("sender dropped"),
    };
}

async fn demo2() {
    // 另一个比较常见的使用场景是结合select!宏，此时应在 recv 前面加上 &mut
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));

    // 注意 mut
    let (tx, mut rx) = sync::oneshot::channel::<i32>();

    tokio::spawn(async move {
        time::sleep(time::Duration::from_millis(500)).await;
        if tx.send(1).is_err() {
            println!("receiver dropped");
        }
    });

    loop {
        // 注意，select!中无需await，因为select!会自动轮询推进每一个分支的任务进度
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut rx => {
                println!("Got message: {}", msg.unwrap());
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // demo1().await;
    // demo2().await;

    let result = do_something().await;
    println!("result = {}", result);
}
