// Stream Trait用于读操作，它模拟Rust标准库的Iterator，
// 可进行迭代式读取和迭代式操作，非常具有Rust的风味

use tokio_stream::{self as stream, StreamExt};

async fn s1() {
    // 目前不能对Stream执行for value in stream{}的迭代操作，
    // 只能不断显式地调用next()方法来读取。比如可以使用下面两种循环读取的方式
    let mut stream = stream::iter(vec![0, 1, 2]);
    while let Some(value) = stream.next().await {
        println!("Got {}", value);
    }

    let mut stream = stream::iter(vec![0, 1, 2]);
    loop {
        match stream.next().await {
            Some(value) => println!("Got {}", value),
            None => break,
        }
    }
}

#[tokio::main]
async fn main() {
    s1().await;
}
