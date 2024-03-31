// watch通道的特性是：只能有单个Sender，可以有多个Receiver，且通道永远只保存一个数据。
// Sender每次向通道中发送数据时，都会修改通道中的那个数据。
// 通道中的这个数据可以被Receiver进行引用读取

use tokio::sync::watch;

async fn demo1() {
    // 创建watch通道时，需指定一个初始值存放在通道中
    let (tx, mut rx) = watch::channel("hello");

    // Receiver 端，通过changed()来等待通道的数据发生变化
    // 通过borrow()引用通道中的数据
    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("received = {:?}", *rx.borrow());
        }
    });

    // 向通道中发送数据，实际上是修改通道中的那个数据
    if tx.send("world").is_err() {
        println!("send err");
    }
}

async fn do_something() {
    // 创建一个watch channel
    let (tx, mut rx) = watch::channel(0);

    // 在一个异步任务中发送消息
    let task = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });

    // 在多个异步任务中接收消息
    for _ in 0..3 {
        let mut rx = rx.clone();
        tokio::spawn(async move {
            // while rx.changed().await.is_ok() {
            //     println!("received = {:?}", *rx.borrow());
            // }

            loop {
                let ok = rx.changed().await.is_ok();
                if ok {
                    println!("recv msg = {}", *rx.borrow());
                } else {
                    break;
                }
            }
        });
    }

    // 有没有下面的这个 drop 都可以, 有的话就自动释放，没有的话就自动释放
    // drop(rx);

    let _ = task.await;
}

#[tokio::main]
async fn main() {
    // demo1().await;
    do_something().await;
}

// Sender端可通过subscribe()创建新的Receiver端。
// 当所有Receiver端均已关闭时，send()方法将返回错误。也就是说，
// send()必须要在有Receiver存活的情况下才能发送数据。
// 但是Sender端还有一个send_replace()方法，它可以在没有Receiver
// 的情况下将数据写入通道，并且该方法会返回通道中原来保存的值。

// 无论是Sender端还是Receiver端，都可以通过borrow()方法取得通道中当前的值。
// 由于可以有多个Receiver，为了避免读写时的数据不一致，watch内部使用了读写锁：
// Sender端要发送数据修改通道中的数据时，需要申请写锁，论是Sender还是Receiver端，
// 在调用borrow()或其它一些方式访问通道数据时，都需要申请读锁。因此，
// 访问通道数据时要尽快释放读锁，否则可能会长时间阻塞Sender端的发送操作
