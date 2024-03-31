// broadcast通道是一种广播通道，可以有多个Sender端以及多个Receiver端，
// 可以发送多个数据，且任何一个Sender发送的每一个数据都能被所有的Receiver端看到。
// 使用mpsc::broadcast()创建广播通道，要求指定一个通道容量作为参数。
// 它返回Sender和Receiver。Sender可以克隆得到多个Sender，
// 可以调用Sender的subscribe()方法来创建新的Receiver

use tokio::sync::broadcast;

async fn demo1() {
    // 最多存放16个消息
    // tx是Sender，rx1是Receiver
    let (tx, mut rx1) = broadcast::channel(16);

    // Sender的subscribe()方法可生成新的Receiver
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}

// Receiver端可通过recv()去接收消息，如果所有的Sender端都已经关闭，
// 则该方法返回RecvError::Closed错误。该方法还可能返回RecvError::Lagged错误，
// 该错误表示接收端已经落后于发送端。

// 虽然broadcast通道也指定容量，但是通道已满的情况下还可以继续写入新数据而不会等待
// (因此上面示例中的send()无需await)，此时通道中最旧的(头部的)数据将被剔除，
// 并且新数据添加在尾部。就像是FIFO队列一样。出现这种情况时，就意味着接收端已经落后于发送端。

// 当接收端已经开始落后于发送端时，下一次的recv()操作将直接返回RecvError::Lagged错误。
// 如果紧跟着再执行recv()且落后现象未再次发生，那么这次的recv()将取得队列头部的消息

async fn demo2() {
    // 通道容量2
    let (tx, mut rx) = broadcast::channel(2);

    // 写入3个数据，将出现接收端落后于发送端的情况，
    // 此时，第一个数据(10)将被剔除，剔除后，20将位于队列的头部
    tx.send(10).unwrap();
    tx.send(20).unwrap();
    tx.send(30).unwrap();

    // 落后于发送端之后的第一次recv()操作，返回RecvError::Lagged错误
    assert!(rx.recv().await.is_err());

    // 之后可正常获取通道中的数据
    assert_eq!(20, rx.recv().await.unwrap());
    assert_eq!(30, rx.recv().await.unwrap());
}

// 另外，tokio::broadcast的任何一个Receiver都可以看到每一次发送的消息，
// 且它们都可以去recv()同一个消息，tokio::broadcast对此的处理方式是消息克隆：
// 每一个Receiver调用recv()去接收一个消息的时候，都会克隆通道中的该消息一次，
// 直到所有存活的Receiver都克隆了该消息，该消息才会从通道中被移除，进而释放一个通道空闲位置。

// 这可能会导致一种现象：某个ReceiverA已经接收了通道中的第10个消息，
// 但另一个ReceiverB可能尚未接收第一个消息，由于第一个消息还未被全部接收者所克隆，
// 它仍会保留在通道中并占用通道的位置，假如该通道的最大容量为10，
// 此时Sender再发送一个消息，那么第一个数据将被踢掉，ReceiverB接收到消息的时候
// 将收到RecvError::Lagged错误并永远地错过第一个消息

async fn do_something() {
    // 创建一个broadcast channel
    let (tx, _) = broadcast::channel(10);
    let tx1 = tx.clone();

    // 在一个异步任务中发送消息
    let task = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });

    // 在多个异步任务中接收消息
    for _ in 0..3 {
        let mut rx = tx1.subscribe();
        // let mut rx = rx.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(msg) => println!("recv msg = {}", msg),
                    Err(_) => break,
                }
            }
        });
    }

    drop(tx1);
    let _ = task.await;
}

#[tokio::main]
async fn main() {
    // demo1().await;
    // demo2().await;
    do_something().await;
}
