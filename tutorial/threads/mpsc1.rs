use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn m1() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么
        // Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());

    // tx,rx对应发送者和接收者，它们的类型由编译器自动推导: tx.send(1)发送
    // 了整数，因此它们分别是mpsc::Sender<i32>和mpsc::Receiver<i32>类型，
    // 需要注意，由于内部是泛型实现，一旦类型被推导确定，该通道就只能传递对应类
    // 型的值, 例如此例中非i32类型的值将导致编译错误
    // 接收消息的操作rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
    // 需要使用move将tx的所有权转移到子线程的闭包中
}

fn m2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_millis(10));
    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_millis(10));
    println!("receive {:?}", rx.try_recv());

    // 由于子线程的创建需要时间，因此println!和try_recv方法会先执行，而此时
    // 子线程的消息还未被发出。try_recv会尝试立即读取一次消息，因为消息没有发
    // 出，此次读取最终会报错

    // 如上，try_recv返回了一个错误，错误内容是Empty，代表通道并没有消息。如果
    // 你尝试把println!复制一些行，就会发现一个有趣的输出

    // 如上，当子线程创建成功且发送消息后，主线程会接收到Ok(1)的消息内容，紧接着
    // 子线程结束，发送者也随着被drop，此时接收者又会报错，但是这次错误原因有所
    // 不同：Disconnected代表发送者已经被关闭
}

// 使用通道来传输数据，一样要遵循 Rust 的所有权规则：
// 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
// 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错

fn m3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        tx.send(s).unwrap();
        // println!("val is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // 以上代码中，String底层的字符串是存储在堆上，并没有实现Copy特征，当它被发送后，
    // 会将所有权从发送端的s转移给接收端的received，之后s将无法被使用:
}

// 之前我们使用的都是异步通道：无论接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞
fn m4() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();

    // 主线程因为睡眠阻塞了 3 秒，因此并没有进行消息接收，而子线程却在此期间轻松完成了
    // 消息的发送。等主线程睡眠结束后，才姗姗来迟的从通道中接收了子线程老早之前发送的消息。
}

// 与异步通道相反，同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞
fn m5() {
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();

    // 睡眠之前
    // 发送之前
    // ···睡眠3秒
    // 睡眠之后
    // receive 1
    // 发送之后

    // mpsc::sync_channel() 参数值，该值可以用来指定同步通道的消息缓存
    // 条数，当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，当消息
    // 缓冲队列满了后，新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的
    // 消息，那么第N+1条消息就将触发发送阻塞)。
}

// 之前我们数次提到了通道关闭，并且提到了当通道关闭后，发送消息或接收消息将会报错。
// 那么如何关闭通道呢？ 很简单：所有发送者被drop或者所有接收者被drop后，通道会自动关闭。
// 神奇的是，这件事是在编译期实现的，完全没有运行期性能损耗！只能说 Rust 的Drop特征 YYDS!

// 之前提到过，一个消息通道只能传输一种类型的数据，如果你想要传输多种类型的数据，
// 可以为每个类型创建一个通道，你也可以使用枚举类型来实现

enum Fruit {
    Apple(u8),
    Orange(String),
}

fn m6() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }

    // 但是有一点需要注意，Rust 会按照枚举中占用内存最大的那个成员进行内存对齐，
    // 这意味着就算你传输的是枚举中占用内存最小的成员，它占用的内存依然和最大的
    // 成员相同, 因此会造成内存上的浪费。
}

fn m7() {
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    drop(send);

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");

    // 以上代码看起来非常正常，但是运行后主线程会一直阻塞，最后一行打印输出
    // 也不会被执行，原因在于： 子线程拿走的是复制后的send的所有权，这些拷贝
    // 会在子线程结束后被drop，因此无需担心，但是send本身却直到main函数的
    // 结束才会被drop。
    // 之前提到，通道关闭的两个条件：发送者全部drop或接收者被drop，要结束
    // for循环显然是要求发送者全部drop，但是由于send自身没有被drop，会
    // 导致该循环永远无法结束，最终主线程会一直阻塞。
    // 解决办法很简单，drop掉send即可：在代码中的注释下面添加一行drop(send);
}

fn main() {
    // m1();
    // m2();
    // m3();
    // m4();
    // m5();
    // m6();
    m7();
}
