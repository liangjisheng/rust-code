use async_std::task::{sleep, spawn};
use std::time::Duration;

// 采用单一线程进行无阻塞调用

async fn sleepus() {
    for i in 1..=10 {
        println!("Sleepus {}", i);
        sleep(Duration::from_millis(50)).await;
    }
}

async fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(100)).await;
    }
}

#[async_std::main]
async fn main() {
    let sleepus = spawn(sleepus());
    interruptus().await;

    sleepus.await;
}
