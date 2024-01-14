use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn m1() {
    let (tx, rx) = mpsc::channel();

    // send 方法返回一个 Result<T, E> 类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误。
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // send 函数获取其参数的所有权并移动这个值归接收者所有。
        // 这可以防止在发送后再次意外地使用这个值；所有权系统检查一切是否合乎规则。
        // println!("val is {}", val);
    });

    // recv 会在一个 Result<T, E> 中返回它。当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn m2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 发送线程结束后，tx 被移除，相当于信道被关闭了
    // 在主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器。
    // 对于每一个接收到的值，我们将其打印出来。当信道被关闭时，迭代器也将结束。
    for received in rx {
        println!("Got: {}", received);
    }
}

fn m3() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
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
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    // m1();
    // m2();
    m3();
}
