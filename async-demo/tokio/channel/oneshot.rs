use tokio::{self, sync, time};

#[tokio::main]
async fn main() {
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
