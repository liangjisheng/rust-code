// Receiver端可通过recv()去接收消息，如果所有的Sender端都已经关闭，
// 则该方法返回RecvError::Closed错误。该方法还可能返回RecvError::Lagged错误，
// 该错误表示接收端已经落后于发送端。

// 虽然broadcast通道也指定容量，但是通道已满的情况下还可以继续写入新数据而不会等待
// (因此上面示例中的send()无需await)，此时通道中最旧的(头部的)数据将被剔除，
// 并且新数据添加在尾部。就像是FIFO队列一样。出现这种情况时，就意味着接收端已经落后于发送端。

// 当接收端已经开始落后于发送端时，下一次的recv()操作将直接返回RecvError::Lagged错误。
// 如果紧跟着再执行recv()且落后现象未再次发生，那么这次的recv()将取得队列头部的消息

use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
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
