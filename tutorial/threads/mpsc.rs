// Rust 中一个实现消息传递的主要工具是 通道（channel）
// 通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）
// 当发送者或接收者任一被丢弃（dropped） 时可以认为通道被关闭（closed）
// mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn c1() {
    // 由于历史原因，`tx`和`rx`通常作为发送者（transmitter）和接收者（receiver）的缩写，
    // 所以这就是我们将用来绑定这两端变量的名字。
    // 这里使用了一个`let`语句和模式来解构了此元组；
    let (tx, rx) = mpsc::channel();
    // 使用`move`将`tx`移动到闭包中这样新建线程就拥有`tx`了
    thread::spawn(move || {
        // `send`方法返回一个`Result<T, E>`类型
        tx.send("receive").unwrap();
        tx.send("try receive").unwrap();
    });

    // recv() 这个方法会阻塞主线程执行直到从信道中接收一个值
    // 当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了
    // let msg = rx.recv().unwrap();
    // println!("message is {}", msg);

    // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>
    // Ok 值包含可用的信息，而 Err 值代表此时没有任何消息
    // 由于 try_recv 不会阻塞主线程执行，新线程如果没有执行完毕就无法接收到消息
    let msg = rx.try_recv();
    if msg.is_ok() {
        println!("message is {}", msg.unwrap());
    } else {
        // receiving on an empty channel
        println!("{}", msg.unwrap_err());
    }
}

fn c2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // 线程中每次循环单独的发送每一个字符串
            tx.send(val).unwrap();
            // 线程中每次循环调用`thread::sleep`函数来暂停一秒
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 主线程中的`for`循环里并没有任何暂停或等待的代码，所以可以说主线程是在等待从新建线程中接收值
    for received in rx {
        // 不再显式调用 rx.recv() 函数：而是将 rx 当作一个迭代器
        println!("Got: {}", received);
    }
}

// 通过克隆发送者来创建多个生产者
fn c3() {
    let (tx, rx) = mpsc::channel();
    // 对发送者调用了`clone`方法克隆一个发送者
    let tx1 = tx.clone();
    // 和上面的代码等价
    // let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // 线程1的发送端句柄使用克隆的tx1，向同一接收者rx发送值
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // 线程2的发送端句柄使用原始的tx，向同一接收者rx发送值
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 两个发送者 tx 和 tx1 发送的值，都由同一个接收者 rx 接收
    for received in rx {
        println!("Got: {}", received);
    }

    // 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
    // 这里虽然用了clone但是并不会影响性能，因为它并不在热点代码路径中，仅仅会被执行一次
    // 由于两个子线程谁先创建完成是未知的，因此哪条消息先发送也是未知的，最终主线程的输出顺序也不确定
}

fn main() {
    // c1();
    // c2();
    c3();
}
