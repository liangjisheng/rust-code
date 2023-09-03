// broadcast通道是一种广播通道，可以有多个Sender端以及多个Receiver端，
// 可以发送多个数据，且任何一个Sender发送的每一个数据都能被所有的Receiver端看到。
// 使用mpsc::broadcast()创建广播通道，要求指定一个通道容量作为参数。
// 它返回Sender和Receiver。Sender可以克隆得到多个Sender，
// 可以调用Sender的subscribe()方法来创建新的Receiver

use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
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
