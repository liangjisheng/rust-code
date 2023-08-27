// Rust 中一个实现消息传递并发的主要工具是通道(channel) 通道有两部分组成
// 一个发送者（transmitter）和一个接收者(receiver)
// std::sync::mpsc 包含了消息传递的方法

use std::sync::mpsc;
use std::thread;

// 子线程获得了主线程的发送者 tx，并调用了它的 send 方法发送了一个字符串
// 然后主线程就通过对应的接收者 rx 接收到了

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
