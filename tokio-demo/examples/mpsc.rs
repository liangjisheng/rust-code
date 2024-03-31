// 通过mpsc::channel()创建有界通道，需传递一个大于1的usize值作为其参数
// mpsc通道只能有一个Receiver端，但可以tx.clone()得到多个Sender端，
// clone 得到的Sender都可以使用send()方法向该通道发送消息。
// 发送消息时，如果通道已满，发送消息的任务将等待直到通道中有空闲的位置
// 发送消息时，如果Receiver端已经关闭，则发送消息的操作将返回SendError
// 如果所有的Sender端都已经关闭，则Receiver端接收消息的方法recv()将返回None

use chrono::Local;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use tokio::time::{self, Duration};
use tokio::{
    self, join,
    sync::{self, mpsc},
};

async fn mpsc_send_num() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);

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

async fn mpsc_send_string() {
    let (mut tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("hello".to_string()).await.unwrap();
        tx.send("world".to_string()).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

async fn mpsc_send_struct() {
    let (mut tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send(Point { x: 1, y: 2 }).await.unwrap();
        tx.send(Point { x: 3, y: 4 }).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("{:?}", msg);
    }
}

async fn mpsc_send_tuple() {
    let (mut tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send((1, 2)).await.unwrap();
        tx.send((3, 4)).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("{:?}", msg);
    }
}

enum Message {
    Text(String),
    Number(i32),
}

async fn mpsc_send_enum() {
    let (mut tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send(Message::Text("hello".to_string())).await.unwrap();
        tx.send(Message::Number(123)).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Text(s) => println!("{}", s),
            Message::Number(n) => println!("{}", n),
        }
    }
}

async fn mpsc_demo() {
    let (tx1, mut rx) = mpsc::channel(32);
    let tx2 = tx1.clone();
    let tx3 = tx1.clone();

    tokio::spawn(async move {
        tx1.send("hello".to_string()).await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("world".to_string()).await.unwrap();
    });

    tokio::spawn(async move {
        tx3.send("!".to_string()).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }
}

async fn mpsc_demo1() {
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

async fn buf_write() {
    let (mut tx, mut rx) = mpsc::channel::<String>(32);

    let task = tokio::spawn(async move {
        let mut writer = BufWriter::new(tokio::io::stdout());
        while let Some(msg) = rx.recv().await {
            let _ = writer.write_all(msg.as_bytes()).await;
            let _ = writer.flush().await;
        }
    });

    tx.send("hello\n".to_string()).await.unwrap();
    tx.send("world\n".to_string()).await.unwrap();
    drop(tx);

    let _ = task.await;
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

// 如果通道已满，Sender通过send()发送消息时将等待。例如下面的示例中，
// 通道容量为5，但要发送7个数据，前5个数据会立即发送，发送第6个消息的时候将等待，
// 直到1秒后Receiver开始从通道中消费数据

async fn mpsc2() {
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

async fn mpsc3() {
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

async fn unbounded() {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        tx.send("hello").unwrap();
    });

    assert_eq!(Some("hello"), rx.recv().await);
    assert_eq!(None, rx.recv().await);
}

#[tokio::main]
async fn main() {
    // mpsc_send_num().await;
    // mpsc_send_string().await;
    // mpsc_send_struct().await;
    // mpsc_send_tuple().await;
    // mpsc_send_enum().await;
    // mpsc_demo().await;
    // mpsc_demo1().await;
    // buf_write().await;
    // mpsc2().await;
    // mpsc3().await;
    unbounded().await;
}
