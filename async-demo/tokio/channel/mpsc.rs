// 通过mpsc::channel()创建有界通道，需传递一个大于1的usize值作为其参数
// mpsc通道只能有一个Receiver端，但可以tx.clone()得到多个Sender端，
// clone 得到的Sender都可以使用send()方法向该通道发送消息。
// 发送消息时，如果通道已满，发送消息的任务将等待直到通道中有空闲的位置
// 发送消息时，如果Receiver端已经关闭，则发送消息的操作将返回SendError
// 如果所有的Sender端都已经关闭，则Receiver端接收消息的方法recv()将返回None

use chrono::Local;
use tokio::time::{self, Duration};
use tokio::{self, sync};

async fn mpsc1() {
    let (tx, mut rx) = sync::mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        for i in 1..=5 {
            // if let Err(_) = tx.send(i).await {}
            if tx.send(i).await.is_err() {
                println!("receiver closed");
                return;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("received: {}", i);
    }
}

async fn mpsc2() {
    let (tx, mut rx) = sync::mpsc::channel::<i32>(5);

    for i in 1..=5 {
        let tx = tx.clone();
        tokio::spawn(async move {
            if tx.send(i).await.is_err() {
                println!("receiver closed");
            }
        });
    }

    // 另外注意上面示例中的drop(tx)，因为生成的10个异步任务中都拥有clone后的Sender
    // clone出的Sender在每个异步任务完成时自动被drop，但原始任务中还有一个Sender
    // 如果不关闭这个Sender，rx.recv()将不会返回None，而是一直等待
    drop(tx);

    while let Some(i) = rx.recv().await {
        println!("received: {}", i);
    }
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

// 如果通道已满，Sender通过send()发送消息时将等待。例如下面的示例中，
// 通道容量为5，但要发送7个数据，前5个数据会立即发送，发送第6个消息的时候将等待，
// 直到1秒后Receiver开始从通道中消费数据
async fn mpsc3() {
    let (tx, mut rx) = sync::mpsc::channel::<i32>(5);

    tokio::spawn(async move {
        for i in 1..=7 {
            if tx.send(i).await.is_err() {
                println!("receiver closed");
                return;
            }
            println!("sended: {}, {}", i, now());
        }
    });

    time::sleep(Duration::from_secs(1)).await;
    while let Some(i) = rx.recv().await {
        time::sleep(Duration::from_secs(1)).await;
        println!("received: {}", i);
    }
}

#[tokio::main]
async fn main() {
    // mpsc1().await;
    // mpsc2().await;
    mpsc3().await;
}
