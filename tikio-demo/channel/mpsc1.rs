// 对于Sender端
// send_timeout(): 向通道中发送消息，通道已满时只等待指定的时长
// try_send(): 向通道中发送消息，但不等待，如果发送不成功，则返回错误
// reserve(): 等待并申请一个通道中的空闲位置，返回一个Permit，申请的空闲位置被占位，
// 且该位置只留给该Permit实例，之后该Permit可以直接向通道中发送消息，并释放其占位的位置。
// 申请成功时，通道空闲容量减1，释放位置时，通道容量会加1
// try_reserve(): 尝试申请一个空闲位置且不等待，如果无法申请，则返回错误
// reserve_owned(): 与reserve()类似，它返回OwnedPermit，但会Move Sender
// try_reserve_owned(): reserve_owned()的不等待版本，尝试申请空闲位置失败时会立即返回错误

// if tx.send_timeout(33, Duration::from_secs(1)).await.is_err() {
// println!("receiver closed or timeout");
// }

// 需要特别注意的是，Receiver端调用close()方法关闭通道后，只是半关闭状态，Receiver
// 端仍然可以继续读取可能已经缓冲在通道中的消息，close()只能保证Sender端无法再发送普通的消息，
// 但Permit或OwnedPermit仍然可以向通道发送消息。只有通道已空且所有Sender端
// (包括Permit和OwnedPermit)都已经关闭的情况下，recv()才会返回None，此时代表通道完全关闭

// Receiver的try_recv()方法在无法立即接收消息时会立即返回错误。返回的错误分为两种:
// TryRecvError::Empty错误: 表示通道已空，但Sender端尚未全部关闭
// TryRecvError::Disconnected错误: 表示通道已空，且Sender端(包括Permit和OwnedPermit)已经全部关闭

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 创建容量为1的通道
    let (tx, mut rx) = mpsc::channel(1);
    // 申请并占有唯一的空闲位置
    let permit = tx.reserve().await.unwrap();
    // 唯一的位置已被permit占有，tx.send()无法发送消息
    assert!(tx.try_send(123).is_err());
    // Permit可以通过send()方法向它占有的那个位置发送消息
    permit.send(456);
    // Receiver端接收到消息
    assert_eq!(rx.recv().await.unwrap(), 456);

    // 创建容量为1的通道
    let (tx, mut rx) = mpsc::channel(1);
    // tx.reserve_owned()会消费掉tx
    let permit = tx.reserve_owned().await.unwrap();
    // 通过permit.send()发送消息，它又返回一个Sender
    let tx = permit.send(456);
    assert_eq!(rx.recv().await.unwrap(), 456);
    //可以继续使用返回的Sender发送消息
    tx.send(789).await.unwrap();
}
